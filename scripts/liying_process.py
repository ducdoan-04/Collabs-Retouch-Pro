#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
LiYing Background Replacement & Pose Alignment Bridge for Retouch Studio Pro
Powered by LiYing (https://github.com/aoguai/LiYing)
"""
import os
import sys
import argparse
import time
import json
import numpy as np
import cv2
from PIL import Image

# Search paths for LiYing source
LIYING_DIRS = [
    r"F:\Workspace\Code projects\LiYing-master",
    r"F:\Workspace\Code projects\LiYing",
    os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "LiYing-master"),
    os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "LiYing"),
]

liying_root = None
for d in LIYING_DIRS:
    if os.path.isdir(os.path.join(d, "src", "tool")):
        liying_root = d
        break

if not liying_root:
    cand = r"F:\Workspace\Code projects\LiYing-master"
    if os.path.exists(cand):
        liying_root = cand

if not liying_root:
    sys.stderr.write("ERROR: Could not locate LiYing project directory\n")
    sys.exit(1)

sys.path.insert(0, os.path.join(liying_root, "src"))

try:
    from tool.ImageProcessor import ImageProcessor
    from tool.ImageSegmentation import ImageSegmentation, rgb_to_rgba
    from tool.PhotoEntity import PhotoEntity
except Exception as e:
    sys.stderr.write(f"ERROR: Failed to import LiYing modules: {e}\n")
    sys.exit(1)


def _fixed_postprocess_image(self, result: np.ndarray, im_size) -> np.ndarray:
    result = np.squeeze(result)
    ma = float(result.max())
    mi = float(result.min())
    if ma > mi:
        result = (result - mi) / (ma - mi)
    else:
        result = np.zeros_like(result)
    matte_uint8 = (result * 255.0).clip(0, 255).astype(np.uint8)
    # PIL resize takes (width, height), whereas OpenCV shape[:2] is (height, width)
    h, w = int(im_size[0]), int(im_size[1])
    matte_pil = Image.fromarray(matte_uint8, mode="L").resize((w, h), Image.BILINEAR)
    return np.array(matte_pil)


def _fixed_infer(self, image: np.ndarray) -> np.ndarray:
    if len(image.shape) == 3 and image.shape[2] == 4:
        image = cv2.cvtColor(image, cv2.COLOR_BGRA2BGR)

    orig_im_size = image.shape[0:2]  # (H, W)
    h, w = int(orig_im_size[0]), int(orig_im_size[1])

    image_rgb = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
    image_pil = Image.fromarray(image_rgb)
    image_preprocessed = self.preprocess_image(np.array(image_pil))

    ort_inputs = {self.ort_session.get_inputs()[0].name: image_preprocessed}
    ort_outs = self.ort_session.run(None, ort_inputs)
    result = ort_outs[0]

    result_matte = _fixed_postprocess_image(self, result, orig_im_size)
    pil_mask = Image.fromarray(result_matte, mode="L")
    orig_rgba = Image.fromarray(cv2.cvtColor(image, cv2.COLOR_BGR2RGBA))

    bg_rgba = tuple(int(c) for c in self.rgb_list)
    if len(bg_rgba) == 4 and bg_rgba[3] == 0:
        r_ch, g_ch, b_ch, _ = orig_rgba.split()
        out_pil = Image.merge("RGBA", (r_ch, g_ch, b_ch, pil_mask))
    else:
        out_pil = Image.new("RGBA", (w, h), bg_rgba)
        out_pil.paste(orig_rgba, mask=pil_mask)

    self._last_matte = result_matte
    return cv2.cvtColor(np.array(out_pil), cv2.COLOR_RGBA2BGRA)


ImageSegmentation.postprocess_image = _fixed_postprocess_image
ImageSegmentation.infer = _fixed_infer


def find_model(model_name):
    cands = [
        os.path.join(liying_root, "src", "model", model_name),
        os.path.join(liying_root, "model", model_name),
        os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "models", model_name),
        r"F:\Workspace\Code projects\LiYing-master\src\model\\" + model_name,
    ]
    for c in cands:
        if os.path.isfile(c):
            return os.path.abspath(c)
    return None


def parse_color(color_str, is_transparent):
    if is_transparent:
        return [0, 0, 0, 0]
    if not color_str:
        return [255, 255, 255, 255]
    
    # Preset names from LiYing data/color_en.csv
    presets = {
        "white": [255, 255, 255, 255],
        "blue": [98, 139, 206, 255],         # Standard ID blue
        "passport_blue": [98, 139, 206, 255],
        "red": [215, 69, 50, 255],
        "dark_blue": [0, 71, 171, 255],
        "light_blue": [180, 200, 230, 255],
        "gray": [128, 128, 128, 255],
        "beige": [245, 245, 220, 255],
        "light_green": [144, 238, 144, 255],
        "lavender": [230, 230, 250, 255],
        "sky_blue": [135, 206, 235, 255],
        "transparent": [0, 0, 0, 0],
    }
    key = color_str.strip().lower()
    if key in presets:
        return presets[key]

    if color_str.startswith("#"):
        hex_c = color_str.lstrip("#")
        if len(hex_c) == 6:
            r = int(hex_c[0:2], 16)
            g = int(hex_c[2:4], 16)
            b = int(hex_c[4:6], 16)
            return [r, g, b, 255]
        elif len(hex_c) == 8:
            r = int(hex_c[0:2], 16)
            g = int(hex_c[2:4], 16)
            b = int(hex_c[4:6], 16)
            a = int(hex_c[6:8], 16)
            return [r, g, b, a]

    parts = [p.strip() for p in color_str.split(",") if p.strip()]
    if len(parts) >= 3:
        r = int(parts[0])
        g = int(parts[1])
        b = int(parts[2])
        a = int(parts[3]) if len(parts) >= 4 else 255
        return [r, g, b, a]

    return [255, 255, 255, 255]


def main():
    parser = argparse.ArgumentParser(description="LiYing Background Replacement and Pose Straightening")
    parser.add_argument("--input", "-i", required=True, help="Input image file path")
    parser.add_argument("--output", "-o", required=True, help="Output image file path (PNG/JPG)")
    parser.add_argument("--color", "-c", default="blue", help="Background color (preset name, hex #RRGGBB, or R,G,B)")
    parser.add_argument("--transparent", "-t", action="store_true", help="Force transparent background (PNG alpha)")
    parser.add_argument("--straighten", "-s", action="store_true", help="Auto-level shoulders and center face (LiYing pose alignment)")
    parser.add_argument("--level-only", action="store_true", help="Only level shoulders and center face without replacing background")
    parser.add_argument("--matte-output", "-m", default=None, help="Optional output path for grayscale alpha matte")
    parser.add_argument("--face-ratio", type=float, default=0.30, help="Face height ratio for cropping")
    parser.add_argument("--top-margin", type=float, default=0.175, help="Top margin ratio for cropping")
    
    args = parser.parse_args()

    t0 = time.time()
    if not os.path.exists(args.input):
        sys.stderr.write(f"ERROR: Input file does not exist: {args.input}\n")
        sys.exit(1)

    yolo_model = find_model("yolov8n-pose.onnx")
    yunet_model = find_model("face_detection_yunet_2023mar.onnx")
    rmbg_model = find_model("RMBG-1.4-model.onnx") or find_model("RMBG-2.0-model.onnx")

    if not rmbg_model:
        sys.stderr.write("ERROR: Could not find RMBG segmentation model\n")
        sys.exit(1)

    rgb_list = parse_color(args.color, args.transparent)

    try:
        processor = ImageProcessor(
            img_path=args.input,
            yolov8_model_path=yolo_model,
            yunet_model_path=yunet_model,
            RMBG_model_path=rmbg_model,
            rgb_list=rgb_list,
        )

        was_straightened = False
        if args.straighten or args.level_only:
            try:
                processor.crop_and_correct_image()
                was_straightened = True
            except Exception as e:
                sys.stderr.write(f"WARN: crop_and_correct_image failed ({e}), continuing with uncropped\n")

        if args.level_only:
            out_img = processor.photo.image
        else:
            # Perform LiYing segmentation & background replacement
            photo = processor.change_background(rgb_list=rgb_list)
            out_img = photo.image  # BGRA OpenCV image

        # Save replaced image
        os.makedirs(os.path.dirname(os.path.abspath(args.output)), exist_ok=True)
        is_saved = cv2.imwrite(args.output, out_img)
        if not is_saved:
            raise IOError(f"cv2.imwrite failed to write to {args.output}")

        # Optional matte output
        if args.matte_output:
            try:
                last_matte = getattr(processor.segmentation, "_last_matte", None)
                if last_matte is not None:
                    matte_ch = last_matte
                elif rgb_list[3] == 0 and out_img.shape[2] == 4:
                    matte_ch = out_img[:, :, 3]
                else:
                    orig_for_matte = Image.fromarray(cv2.cvtColor(out_img, cv2.COLOR_BGRA2RGB))
                    im_prep = processor.segmentation.preprocess_image(np.array(orig_for_matte))
                    sess = processor.segmentation.ort_session
                    raw_out = sess.run(None, {sess.get_inputs()[0].name: im_prep})[0]
                    matte_ch = processor.segmentation.postprocess_image(raw_out, out_img.shape[:2])
                
                os.makedirs(os.path.dirname(os.path.abspath(args.matte_output)), exist_ok=True)
                cv2.imwrite(args.matte_output, matte_ch)
            except Exception as me:
                sys.stderr.write(f"WARN: failed to write matte output: {me}\n")

        h, w = out_img.shape[:2]
        elapsed = time.time() - t0
        res = {
            "status": "success",
            "width": w,
            "height": h,
            "elapsed_s": round(elapsed, 3),
            "straightened": was_straightened,
            "color": rgb_list,
        }
        print(f"SUCCESS {json.dumps(res)}")

    except Exception as e:
        sys.stderr.write(f"ERROR: LiYing processing failed: {e}\n")
        import traceback
        traceback.print_exc(file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
