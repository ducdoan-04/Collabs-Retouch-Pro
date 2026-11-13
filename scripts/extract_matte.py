import os
import sys
import base64
import json
import time
from io import BytesIO
import numpy as np
from PIL import Image

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PROJECT_DIR = os.path.dirname(SCRIPT_DIR)
MODEL_CANDIDATES = [
    os.path.join(PROJECT_DIR, "models", "RMBG-1.4-model.onnx"),
    os.path.join(PROJECT_DIR, "src-tauri", "models", "RMBG-1.4-model.onnx"),
    r"F:\Workspace\Code projects\LiYing-master\src\model\RMBG-1.4-model.onnx",
    r"F:\Workspace\Code projects\LiYing\src\model\RMBG-1.4-model.onnx",
]

_session = None

def get_session():
    global _session
    if _session is not None:
        return _session
    import onnxruntime as ort
    model_path = None
    for cand in MODEL_CANDIDATES:
        if os.path.isfile(cand):
            model_path = cand
            break
    if not model_path:
        raise FileNotFoundError(f"RMBG-1.4-model.onnx not found in candidates: {MODEL_CANDIDATES}")
    
    # Try DirectML / CUDA / CPU
    available = ort.get_available_providers()
    providers = []
    if "DmlExecutionProvider" in available:
        providers.append("DmlExecutionProvider")
    if "CUDAExecutionProvider" in available:
        providers.append("CUDAExecutionProvider")
    providers.append("CPUExecutionProvider")
    
    # Optimize session options for low latency
    opts = ort.SessionOptions()
    opts.graph_optimization_level = ort.GraphOptimizationLevel.ORT_ENABLE_ALL
    opts.intra_op_num_threads = min(8, max(2, os.cpu_count() or 4))
    
    _session = ort.InferenceSession(model_path, sess_options=opts, providers=providers)
    return _session

def process_pil_image(img_pil: Image.Image) -> Image.Image:
    sess = get_session()
    w, h = img_pil.size
    if w <= 0 or h <= 0:
        return Image.new("L", (max(1, w), max(1, h)), 0)
    
    # 1. Aspect-ratio preserving letterbox scaling to 1024x1024
    # Eliminates facial/head distortion and preserves natural hair strand geometry
    scale = 1024.0 / max(w, h)
    target_w = max(1, int(round(w * scale)))
    target_h = max(1, int(round(h * scale)))
    
    im_scaled = img_pil.resize((target_w, target_h), Image.BILINEAR)
    im_padded = Image.new("RGB", (1024, 1024), (128, 128, 128))
    pad_x = (1024 - target_w) // 2
    pad_y = (1024 - target_h) // 2
    im_padded.paste(im_scaled, (pad_x, pad_y))
    
    im_np = np.array(im_padded, dtype=np.float32) / 255.0
    im_np = (im_np - 0.5) / 1.0
    im_np = im_np.transpose(2, 0, 1)
    im_np = np.expand_dims(im_np, 0)
    
    inp_name = sess.get_inputs()[0].name
    out = sess.run(None, {inp_name: im_np})[0]
    
    matte = np.squeeze(out)
    ma = float(matte.max())
    mi = float(matte.min())
    if ma > mi:
        matte = (matte - mi) / (ma - mi)
    else:
        matte = np.zeros_like(matte)
        
    matte_uint8 = (matte * 255.0).clip(0, 255).astype(np.uint8)
    matte_padded = Image.fromarray(matte_uint8, mode="L")
    
    # 2. Crop out letterbox padding and resize back to original resolution
    matte_cropped = matte_padded.crop((pad_x, pad_y, pad_x + target_w, pad_y + target_h))
    matte_coarse = matte_cropped.resize((w, h), Image.BILINEAR)
    
    # 3. High-resolution Guided Filter edge refinement
    # Uses high-contrast hair luminance edges from original image to recover fine micro-strands
    try:
        import cv2
        guide_np = np.array(img_pil)
        coarse_np = np.array(matte_coarse)
        
        if hasattr(cv2, "ximgproc") and hasattr(cv2.ximgproc, "guidedFilter"):
            refined_np = cv2.ximgproc.guidedFilter(
                guide=guide_np, src=coarse_np, radius=8, eps=1e-4
            )
        else:
            # Box-filter guided filter fallback
            p = coarse_np.astype(np.float32) / 255.0
            I = cv2.cvtColor(guide_np, cv2.COLOR_RGB2GRAY).astype(np.float32) / 255.0
            r = 8
            eps = 1e-4
            mean_I = cv2.boxFilter(I, -1, (r, r))
            mean_p = cv2.boxFilter(p, -1, (r, r))
            mean_Ip = cv2.boxFilter(I * p, -1, (r, r))
            cov_Ip = mean_Ip - mean_I * mean_p
            mean_II = cv2.boxFilter(I * I, -1, (r, r))
            var_I = mean_II - mean_I * mean_I
            a = cov_Ip / (var_I + eps)
            b = mean_p - a * mean_I
            mean_a = cv2.boxFilter(a, -1, (r, r))
            mean_b = cv2.boxFilter(b, -1, (r, r))
            q = mean_a * I + mean_b
            refined_np = (np.clip(q, 0.0, 1.0) * 255.0).astype(np.uint8)
            
        matte_img = Image.fromarray(refined_np, mode="L")
    except Exception:
        matte_img = matte_coarse
        
    return matte_img

def run_single(input_arg: str, output_path: str):
    t0 = time.time()
    if input_arg.startswith("data:image") or len(input_arg) > 500:
        if "," in input_arg:
            input_arg = input_arg.split(",", 1)[1]
        data = base64.b64decode(input_arg)
        img = Image.open(BytesIO(data)).convert("RGB")
    else:
        img = Image.open(input_arg).convert("RGB")
        
    matte = process_pil_image(img)
    matte.save(output_path, format="PNG")
    t1 = time.time()
    print(f"SUCCESS {t1 - t0:.3f}s {matte.size[0]}x{matte.size[1]}", flush=True)

def run_daemon():
    # Warm up session
    try:
        get_session()
        # Warmup forward pass with dummy tensor
        dummy = Image.new("RGB", (64, 64), (128, 128, 128))
        process_pil_image(dummy)
        print(json.dumps({"status": "ready"}), flush=True)
    except Exception as e:
        print(json.dumps({"status": "error", "error": str(e)}), flush=True)
        return

    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        if line == "quit" or line == "exit":
            break
        try:
            req = json.loads(line)
            req_id = req.get("id", "0")
            t0 = time.time()
            
            if "image_base64" in req:
                b64 = req["image_base64"]
                if "," in b64:
                    b64 = b64.split(",", 1)[1]
                data = base64.b64decode(b64)
                img = Image.open(BytesIO(data)).convert("RGB")
            elif "input_path" in req:
                img = Image.open(req["input_path"]).convert("RGB")
            else:
                raise ValueError("Missing 'image_base64' or 'input_path'")
                
            matte = process_pil_image(img)
            
            resp = {
                "id": req_id,
                "status": "ok",
                "width": matte.size[0],
                "height": matte.size[1],
                "elapsed_ms": round((time.time() - t0) * 1000),
            }
            
            if "output_path" in req:
                matte.save(req["output_path"], format="PNG")
                resp["output_path"] = req["output_path"]
            else:
                bio = BytesIO()
                matte.save(bio, format="PNG")
                resp["matte_base64"] = "data:image/png;base64," + base64.b64encode(bio.getvalue()).decode("ascii")
                
            print(json.dumps(resp), flush=True)
        except Exception as err:
            print(json.dumps({"id": req.get("id", "0"), "status": "error", "error": str(err)}), flush=True)

def main():
    if len(sys.argv) > 1 and sys.argv[1] == "--daemon":
        run_daemon()
    elif len(sys.argv) >= 3:
        run_single(sys.argv[1], sys.argv[2])
    else:
        print("Usage:")
        print("  extract_matte.py <input_img_path_or_base64> <output_png_path>")
        print("  extract_matte.py --daemon")
        sys.exit(1)

if __name__ == "__main__":
    main()
