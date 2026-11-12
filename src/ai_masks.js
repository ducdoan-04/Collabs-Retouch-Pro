import { FaceLandmarker, ImageSegmenter, FilesetResolver } from '@mediapipe/tasks-vision';

let visionResolver = null;
let faceLandmarker = null;
let portraitSegmenter = null;

// Official MediaPipe 478 Face Mesh anatomical polygon indices
export const MESH_INDICES = {
  // Full Face Silhouette Oval (36 points clockwise around forehead, temples, cheeks, jawline, chin)
  FACE_OVAL: [
    10, 338, 297, 332, 284, 251, 389, 356, 454, 323, 361, 288,
    397, 365, 379, 378, 400, 377, 152, 148, 176, 149, 150, 136,
    172, 58, 132, 93, 234, 127, 162, 21, 54, 103, 67, 109
  ],
  // Left Eye Opening Contour (exact eyelid rim)
  LEFT_EYE: [33, 7, 163, 144, 145, 153, 154, 155, 133, 173, 157, 158, 159, 160, 161, 246],
  // Right Eye Opening Contour (exact eyelid rim)
  RIGHT_EYE: [362, 382, 381, 380, 374, 373, 390, 249, 263, 466, 388, 387, 386, 385, 384, 398],
  // Left & Right Eye with Eyelash Protection Ring (outer orbital rim around eyelashes)
  LEFT_EYE_GUARD: [226, 113, 225, 224, 223, 222, 221, 189, 244, 112, 26, 22, 23, 24, 110, 25],
  RIGHT_EYE_GUARD: [446, 342, 445, 444, 443, 442, 441, 413, 464, 341, 256, 252, 253, 254, 339, 255],
  // Left & Right Upper Eyelid Eyeshadow Zone
  LEFT_EYESHADOW: [226, 113, 225, 224, 223, 222, 221, 189, 173, 157, 158, 159, 160, 161, 246, 33],
  RIGHT_EYESHADOW: [446, 342, 445, 444, 443, 442, 441, 413, 398, 384, 385, 386, 387, 388, 466, 263],
  // Left Eyebrow Polygon
  LEFT_EYEBROW: [70, 63, 105, 66, 107, 55, 65, 52, 53, 46],
  // Right Eyebrow Polygon
  RIGHT_EYEBROW: [300, 293, 334, 296, 336, 285, 295, 282, 283, 276],
  // Outer Lips Contour (entire mouth outer border)
  LIPS_OUTER: [61, 185, 40, 39, 37, 0, 267, 269, 270, 409, 291, 375, 321, 405, 314, 17, 84, 181, 91, 146],
  // Inner Lips Contour (the mouth opening where teeth are visible!)
  LIPS_INNER: [78, 191, 80, 81, 82, 13, 312, 311, 310, 415, 308, 324, 318, 402, 317, 14, 87, 178, 88, 95],
  // Nostrils & Nose Base Guard (so dark nostrils are never blurred as acne/blemishes)
  NOSTRILS_GUARD: [48, 115, 220, 45, 4, 275, 440, 344, 278, 294, 327, 326, 2, 97, 98, 64],
  // Under-Eye Dark Circles / Eye Bags Crescent Polygons (strictly below lower eyelid!)
  LEFT_EYE_BAG_CRESCENT: [226, 31, 228, 229, 230, 231, 232, 233, 244, 133, 155, 154, 153, 145, 144, 163, 7, 33],
  RIGHT_EYE_BAG_CRESCENT: [446, 261, 448, 449, 450, 451, 452, 453, 464, 362, 382, 381, 380, 374, 373, 390, 249, 263],
  // Left & Right Cheek Blush Zones
  LEFT_CHEEK: [116, 117, 118, 101, 36, 205, 187, 123],
  RIGHT_CHEEK: [345, 346, 347, 330, 266, 425, 411, 352],
  // Nasolabial Folds (Left & Right Smile Lines)
  LEFT_NASOLABIAL: [98, 203, 206, 216, 207, 57, 186, 92, 165],
  RIGHT_NASOLABIAL: [327, 423, 426, 436, 427, 287, 410, 322, 391],
  // Forehead Zone (above eyebrows up to hairline)
  FOREHEAD: [10, 338, 297, 332, 284, 300, 293, 334, 296, 336, 9, 107, 66, 105, 63, 70, 54, 103, 67, 109],
};

async function ensureAiModelsLoaded() {
  if (!visionResolver) {
    visionResolver = await FilesetResolver.forVisionTasks('/models/wasm');
  }
  if (!faceLandmarker) {
    try {
      faceLandmarker = await FaceLandmarker.createFromOptions(visionResolver, {
        baseOptions: {
          modelAssetPath: '/models/face_landmarker.task',
          delegate: 'CPU', // CPU XNNPACK delegate guarantees 100% deterministic reliability in Tauri WebView
        },
        outputFaceBlendshapes: false,
        runningMode: 'IMAGE',
        numFaces: 1,
        minFaceDetectionConfidence: 0.20,
        minFacePresenceConfidence: 0.20,
      });
    } catch (e) {
      console.error('FaceLandmarker init error:', e);
    }
  }
  if (!portraitSegmenter) {
    try {
      portraitSegmenter = await ImageSegmenter.createFromOptions(visionResolver, {
        baseOptions: {
          modelAssetPath: '/models/selfie_multiclass_256x256.tflite',
          delegate: 'CPU',
        },
        runningMode: 'IMAGE',
        outputCategoryMask: false,
        outputConfidenceMasks: true,
      });
    } catch (e) {
      console.error('ImageSegmenter init error:', e);
    }
  }
}

/**
 * Helper to draw a closed polygon of landmark indices on a 2D canvas, with optional expansion around centroid
 */
function fillLandmarkPolygon(ctx, pts, indices, w, h, fillStyle, blurPx = 0, expandScale = 1.0) {
  if (!pts || indices.length < 3) return;
  let cx = 0;
  let cy = 0;
  let validCount = 0;
  for (const idx of indices) {
    const p = pts[idx];
    if (p) {
      cx += p.x;
      cy += p.y;
      validCount++;
    }
  }
  if (validCount < 3) return;
  cx /= validCount;
  cy /= validCount;

  ctx.save();
  if (blurPx > 0) {
    ctx.filter = `blur(${blurPx}px)`;
  }
  ctx.fillStyle = fillStyle;
  ctx.beginPath();
  for (let i = 0; i < indices.length; i++) {
    const p = pts[indices[i]];
    if (!p) continue;
    const px = cx + (p.x - cx) * expandScale;
    const py = cy + (p.y - cy) * expandScale;
    const x = px * w;
    const y = py * h;
    if (i === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  }
  ctx.closePath();
  ctx.fill();
  ctx.restore();
}

function sampleConfidenceBilinear(plane, w, h, nx, ny) {
  const fx = Math.max(0, Math.min(w - 1, nx * (w - 1)));
  const fy = Math.max(0, Math.min(h - 1, ny * (h - 1)));
  const x0 = Math.floor(fx);
  const y0 = Math.floor(fy);
  const x1 = Math.min(w - 1, x0 + 1);
  const y1 = Math.min(h - 1, y0 + 1);
  const dx = fx - x0;
  const dy = fy - y0;
  const i00 = y0 * w + x0;
  const i10 = y0 * w + x1;
  const i01 = y1 * w + x0;
  const i11 = y1 * w + x1;
  return (
    plane[i00] * (1 - dx) * (1 - dy) +
    plane[i10] * dx * (1 - dy) +
    plane[i01] * (1 - dx) * dy +
    plane[i11] * dx * dy
  );
}

/**
 * Run both Neural Networks (478-Point Face Mesh + 6-Class Semantic Portrait Segmenter)
 * and build a 512x512 RGBA Semantic Map where:
 *   - R channel = Exact Face & Neck Skin Mask (0..255) WITH EYES, EYELASHES, EYEBROWS, NOSTRILS & LIPS STRICTLY CARVED OUT TO 0!
 *   - G channel = Discrete Feature ID Map (drawn with blurPx=0 so codes never bleed into each other):
 *       * 30  = Forehead Wrinkles Zone
 *       * 60  = Under-Eye Dark Circles / Eye Bags Crescent Zone
 *       * 85  = Nasolabial Folds (Smile Lines) Zone
 *       * 110 = Eyebrows Zone
 *       * 125 = Upper Eyelid Eyeshadow Zone
 *       * 135 = Cheek Blush Zone
 *       * 160 = Sclera & Iris (Eyes) Zone
 *       * 205 = Outer Lips (Lipstick Zone - Inner mouth excluded)
 *       * 245 = Inner Mouth Cavity (Teeth Whitening Zone)
 *   - B channel = Neural Hair Mask (from Multiclass Segmenter class 1)
 *   - A channel = Scene Category:
 *       * 0   = Background (class 0)
 *       * 128 = Clothing (class 4)
 *       * 255 = Person Foreground (Skin/Head/Hair)
 */
export async function analyzePhotoWithNeuralAI(imgElement, fallbackFace) {
  await ensureAiModelsLoaded();

  const maskSize = 512;
  const workCanvas = document.createElement('canvas');
  const natW = imgElement.naturalWidth || imgElement.width || 960;
  const natH = imgElement.naturalHeight || imgElement.height || 1160;
  const scale = Math.min(1, 1024 / Math.max(natW, natH));
  workCanvas.width = Math.round(natW * scale);
  workCanvas.height = Math.round(natH * scale);
  const wCtx = workCanvas.getContext('2d');
  wCtx.drawImage(imgElement, 0, 0, workCanvas.width, workCanvas.height);

  // 1. Run 478-Point Face Landmarker
  let pts = null;
  let landmarksDetected = false;
  let face = {
    centerX: fallbackFace?.centerX ?? 0.5,
    centerY: fallbackFace?.centerY ?? 0.42,
    faceRx: fallbackFace?.faceRx ?? 0.18,
    faceRy: fallbackFace?.faceRy ?? 0.24,
    leftEyeX: fallbackFace?.leftEyeX ?? 0.43,
    leftEyeY: fallbackFace?.leftEyeY ?? 0.38,
    rightEyeX: fallbackFace?.rightEyeX ?? 0.57,
    rightEyeY: fallbackFace?.rightEyeY ?? 0.38,
    noseX: fallbackFace?.noseX ?? 0.5,
    noseY: fallbackFace?.noseY ?? 0.44,
    mouthX: fallbackFace?.mouthX ?? 0.5,
    mouthY: fallbackFace?.mouthY ?? 0.51,
    chinX: 0.5,
    chinY: (fallbackFace?.centerY ?? 0.42) + (fallbackFace?.faceRy ?? 0.24) * 0.95,
    foreheadX: 0.5,
    foreheadY: (fallbackFace?.centerY ?? 0.42) - (fallbackFace?.faceRy ?? 0.24) * 0.92,
    jawLeftX: (fallbackFace?.centerX ?? 0.5) - (fallbackFace?.faceRx ?? 0.18) * 0.82,
    jawLeftY: (fallbackFace?.centerY ?? 0.42) + (fallbackFace?.faceRy ?? 0.24) * 0.52,
    jawRightX: (fallbackFace?.centerX ?? 0.5) + (fallbackFace?.faceRx ?? 0.18) * 0.82,
    jawRightY: (fallbackFace?.centerY ?? 0.42) + (fallbackFace?.faceRy ?? 0.24) * 0.52,
    cheekLeftX: (fallbackFace?.centerX ?? 0.5) - (fallbackFace?.faceRx ?? 0.18) * 0.88,
    cheekLeftY: (fallbackFace?.centerY ?? 0.42) + (fallbackFace?.faceRy ?? 0.24) * 0.06,
    cheekRightX: (fallbackFace?.centerX ?? 0.5) + (fallbackFace?.faceRx ?? 0.18) * 0.88,
    cheekRightY: (fallbackFace?.centerY ?? 0.42) + (fallbackFace?.faceRy ?? 0.24) * 0.06,
    mouthLeftX: (fallbackFace?.mouthX ?? 0.5) - (fallbackFace?.faceRx ?? 0.18) * 0.28,
    mouthLeftY: fallbackFace?.mouthY ?? 0.51,
    mouthRightX: (fallbackFace?.mouthX ?? 0.5) + (fallbackFace?.faceRx ?? 0.18) * 0.28,
    mouthRightY: fallbackFace?.mouthY ?? 0.51,
  };

  if (faceLandmarker) {
    try {
      const lmRes = faceLandmarker.detect(workCanvas);
      if (lmRes?.faceLandmarks && lmRes.faceLandmarks.length > 0) {
        pts = lmRes.faceLandmarks[0];
        landmarksDetected = true;

        let minX = 1, maxX = 0, minY = 1, maxY = 0;
        for (const idx of MESH_INDICES.FACE_OVAL) {
          const p = pts[idx];
          if (p.x < minX) minX = p.x;
          if (p.x > maxX) maxX = p.x;
          if (p.y < minY) minY = p.y;
          if (p.y > maxY) maxY = p.y;
        }

        const lEye = pts[468] || {
          x: (pts[33].x + pts[133].x) * 0.5,
          y: (pts[159].y + pts[145].y) * 0.5,
        };
        const rEye = pts[473] || {
          x: (pts[362].x + pts[263].x) * 0.5,
          y: (pts[386].y + pts[374].y) * 0.5,
        };
        const nose = pts[4] || pts[1];
        const mouth = {
          x: (pts[13].x + pts[14].x) * 0.5,
          y: (pts[13].y + pts[14].y) * 0.5,
        };
        const chin = pts[152] || { x: (minX + maxX) * 0.5, y: maxY };
        const forehead = pts[10] || { x: (minX + maxX) * 0.5, y: minY };
        const jawL = pts[172] || pts[136];
        const jawR = pts[397] || pts[365];
        const cheekL = pts[234] || pts[123];
        const cheekR = pts[454] || pts[352];
        const mouthL = pts[61];
        const mouthR = pts[291];

        face = {
          centerX: (minX + maxX) * 0.5,
          centerY: (minY + maxY) * 0.5,
          faceRx: Math.max(0.06, (maxX - minX) * 0.5),
          faceRy: Math.max(0.08, (maxY - minY) * 0.5),
          leftEyeX: lEye.x,
          leftEyeY: lEye.y,
          rightEyeX: rEye.x,
          rightEyeY: rEye.y,
          noseX: nose.x,
          noseY: nose.y,
          mouthX: mouth.x,
          mouthY: mouth.y,
          chinX: chin.x,
          chinY: chin.y,
          foreheadX: forehead.x,
          foreheadY: forehead.y,
          jawLeftX: jawL.x,
          jawLeftY: jawL.y,
          jawRightX: jawR.x,
          jawRightY: jawR.y,
          cheekLeftX: cheekL.x,
          cheekLeftY: cheekL.y,
          cheekRightX: cheekR.x,
          cheekRightY: cheekR.y,
          mouthLeftX: mouthL.x,
          mouthLeftY: mouthL.y,
          mouthRightX: mouthR.x,
          mouthRightY: mouthR.y,
        };
      }
    } catch (e) {
      console.warn('FaceLandmarker detect error:', e);
    }
  }

  // 2. Run 6-Class Semantic Portrait Segmenter (0=BG, 1=Hair, 2=BodySkin, 3=FaceSkin, 4=Clothes, 5=Accessories)
  let segConfs = null;
  let segClasses = null;
  let segW = 0;
  let segH = 0;
  if (portraitSegmenter) {
    let segRes = null;
    try {
      segRes = portraitSegmenter.segment(workCanvas);
      if (segRes?.confidenceMasks && segRes.confidenceMasks.length >= 5) {
        segW = segRes.confidenceMasks[0].width;
        segH = segRes.confidenceMasks[0].height;
        segConfs = segRes.confidenceMasks.map((mask) => new Float32Array(mask.getAsFloat32Array()));
      } else if (segRes?.categoryMask) {
        segW = segRes.categoryMask.width;
        segH = segRes.categoryMask.height;
        segClasses = new Uint8Array(segRes.categoryMask.getAsUint8Array());
      }
    } catch (e) {
      console.warn('PortraitSegmenter error:', e);
    } finally {
      try {
        segRes?.close?.();
      } catch (_) {}
    }
  }

  // 3. Rasterize High-Precision Anatomical Polygons onto 512x512 layers
  // Layer A: Skin Polygon Canvas (White on face oval + forehead, Black carved out on Eyes+Eyelashes, Eyebrows, Nostrils, Lips!)
  const skinCanvas = document.createElement('canvas');
  skinCanvas.width = maskSize;
  skinCanvas.height = maskSize;
  const sCtx = skinCanvas.getContext('2d');
  sCtx.fillStyle = '#000';
  sCtx.fillRect(0, 0, maskSize, maskSize);

  // Layer B: Discrete Facial Feature Zone Canvas (NO CSS blur so discrete grayscale codes never bleed into adjacent codes!)
  const featCanvas = document.createElement('canvas');
  featCanvas.width = maskSize;
  featCanvas.height = maskSize;
  const fCtx = featCanvas.getContext('2d');
  fCtx.imageSmoothingEnabled = false;
  fCtx.fillStyle = '#000';
  fCtx.fillRect(0, 0, maskSize, maskSize);

  if (pts) {
    // Draw Face Oval & Forehead as pure white skin (with soft 6px feathered outer edge)
    fillLandmarkPolygon(sCtx, pts, MESH_INDICES.FACE_OVAL, maskSize, maskSize, '#ffffff', 6, 0.98);
    fillLandmarkPolygon(sCtx, pts, MESH_INDICES.FOREHEAD, maskSize, maskSize, '#ffffff', 6, 1.04);

    // STRICTLY CARVE OUT Eyes + Eyelashes, Eyebrows, Nostrils, and Lips from the Skin Mask so Skin Smoothing / Blemish / Dodge & Burn NEVER touch them!
    fillLandmarkPolygon(sCtx, pts, MESH_INDICES.LEFT_EYE_GUARD, maskSize, maskSize, '#000000', 3, 1.12);
    fillLandmarkPolygon(sCtx, pts, MESH_INDICES.RIGHT_EYE_GUARD, maskSize, maskSize, '#000000', 3, 1.12);
    fillLandmarkPolygon(sCtx, pts, MESH_INDICES.LEFT_EYE, maskSize, maskSize, '#000000', 2, 1.35);
    fillLandmarkPolygon(sCtx, pts, MESH_INDICES.RIGHT_EYE, maskSize, maskSize, '#000000', 2, 1.35);
    fillLandmarkPolygon(sCtx, pts, MESH_INDICES.LEFT_EYEBROW, maskSize, maskSize, '#000000', 3, 1.22);
    fillLandmarkPolygon(sCtx, pts, MESH_INDICES.RIGHT_EYEBROW, maskSize, maskSize, '#000000', 3, 1.22);
    fillLandmarkPolygon(sCtx, pts, MESH_INDICES.NOSTRILS_GUARD, maskSize, maskSize, '#000000', 2, 1.05);
    fillLandmarkPolygon(sCtx, pts, MESH_INDICES.LIPS_OUTER, maskSize, maskSize, '#000000', 3, 1.18);

    // Draw Discrete Feature IDs onto featCanvas (blurPx = 0 so codes are 100% pure):
    // 1) Forehead Wrinkles Zone -> value 30
    fillLandmarkPolygon(fCtx, pts, MESH_INDICES.FOREHEAD, maskSize, maskSize, 'rgb(30,30,30)', 0, 1.0);

    // 2) Cheek Blush Zones -> value 135
    fillLandmarkPolygon(fCtx, pts, MESH_INDICES.LEFT_CHEEK, maskSize, maskSize, 'rgb(135,135,135)', 0, 1.05);
    fillLandmarkPolygon(fCtx, pts, MESH_INDICES.RIGHT_CHEEK, maskSize, maskSize, 'rgb(135,135,135)', 0, 1.05);

    // 3) Nasolabial Folds (Smile Lines) -> value 85
    fillLandmarkPolygon(fCtx, pts, MESH_INDICES.LEFT_NASOLABIAL, maskSize, maskSize, 'rgb(85,85,85)', 0, 1.05);
    fillLandmarkPolygon(fCtx, pts, MESH_INDICES.RIGHT_NASOLABIAL, maskSize, maskSize, 'rgb(85,85,85)', 0, 1.05);

    // 4) Under-Eye Bags / Dark Circles Crescent -> value 60
    fillLandmarkPolygon(fCtx, pts, MESH_INDICES.LEFT_EYE_BAG_CRESCENT, maskSize, maskSize, 'rgb(60,60,60)', 0, 1.0);
    fillLandmarkPolygon(fCtx, pts, MESH_INDICES.RIGHT_EYE_BAG_CRESCENT, maskSize, maskSize, 'rgb(60,60,60)', 0, 1.0);

    // 5) Upper Eyelid Eyeshadow Zone -> value 125
    fillLandmarkPolygon(fCtx, pts, MESH_INDICES.LEFT_EYESHADOW, maskSize, maskSize, 'rgb(125,125,125)', 0, 1.0);
    fillLandmarkPolygon(fCtx, pts, MESH_INDICES.RIGHT_EYESHADOW, maskSize, maskSize, 'rgb(125,125,125)', 0, 1.0);

    // 6) Eyebrows -> value 110
    fillLandmarkPolygon(fCtx, pts, MESH_INDICES.LEFT_EYEBROW, maskSize, maskSize, 'rgb(110,110,110)', 0, 1.05);
    fillLandmarkPolygon(fCtx, pts, MESH_INDICES.RIGHT_EYEBROW, maskSize, maskSize, 'rgb(110,110,110)', 0, 1.05);

    // 7) Eyes (Sclera + Iris) -> value 160 (drawn AFTER under-eye crescent & eyeshadow so eye opening is 100% protected!)
    fillLandmarkPolygon(fCtx, pts, MESH_INDICES.LEFT_EYE, maskSize, maskSize, 'rgb(160,160,160)', 0, 1.0);
    fillLandmarkPolygon(fCtx, pts, MESH_INDICES.RIGHT_EYE, maskSize, maskSize, 'rgb(160,160,160)', 0, 1.0);

    // 8) Outer Lips (Lipstick region) -> value 205
    fillLandmarkPolygon(fCtx, pts, MESH_INDICES.LIPS_OUTER, maskSize, maskSize, 'rgb(205,205,205)', 0, 1.0);

    // 9) Inner Mouth Cavity (Teeth region) -> value 245 (overwrites center of lips so Lipstick never paints teeth!)
    fillLandmarkPolygon(fCtx, pts, MESH_INDICES.LIPS_INNER, maskSize, maskSize, 'rgb(245,245,245)', 0, 0.96);
  } else {
    // Fallback parametric masks if synthetic portrait or face mesh unavailable
    const cx = face.centerX * maskSize;
    const cy = face.centerY * maskSize;
    const rx = face.faceRx * maskSize;
    const ry = face.faceRy * maskSize;

    sCtx.save();
    sCtx.filter = 'blur(6px)';
    sCtx.fillStyle = '#ffffff';
    sCtx.beginPath();
    sCtx.ellipse(cx, cy, rx, ry, 0, 0, Math.PI * 2);
    sCtx.fill();

    // Carve out eyes, eyebrows, and mouth from fallback skin mask
    sCtx.fillStyle = '#000000';
    for (const [ex, ey] of [[face.leftEyeX, face.leftEyeY], [face.rightEyeX, face.rightEyeY]]) {
      sCtx.beginPath();
      sCtx.ellipse(ex * maskSize, ey * maskSize, rx * 0.26, ry * 0.14, 0, 0, Math.PI * 2);
      sCtx.fill();
      sCtx.beginPath();
      sCtx.ellipse(ex * maskSize, (ey - face.faceRy * 0.12) * maskSize, rx * 0.28, ry * 0.08, 0, 0, Math.PI * 2);
      sCtx.fill();
    }
    sCtx.beginPath();
    sCtx.ellipse(face.mouthX * maskSize, face.mouthY * maskSize, rx * 0.34, ry * 0.14, 0, 0, Math.PI * 2);
    sCtx.fill();
    sCtx.restore();

    // Fallback feature codes
    // Forehead (30)
    fCtx.fillStyle = 'rgb(30,30,30)';
    fCtx.beginPath();
    fCtx.ellipse(cx, (face.centerY - face.faceRy * 0.55) * maskSize, rx * 0.72, ry * 0.25, 0, 0, Math.PI * 2);
    fCtx.fill();

    // Cheek Blush Zones (135)
    fCtx.fillStyle = 'rgb(135,135,135)';
    for (const dir of [-1, 1]) {
      fCtx.beginPath();
      fCtx.ellipse((face.centerX + dir * face.faceRx * 0.52) * maskSize, (face.centerY + face.faceRy * 0.08) * maskSize, rx * 0.26, ry * 0.16, 0, 0, Math.PI * 2);
      fCtx.fill();
    }

    // Nasolabial Folds (85)
    fCtx.fillStyle = 'rgb(85,85,85)';
    for (const dir of [-1, 1]) {
      fCtx.beginPath();
      fCtx.ellipse((face.centerX + dir * face.faceRx * 0.28) * maskSize, (face.centerY + face.faceRy * 0.24) * maskSize, rx * 0.12, ry * 0.15, dir * -0.35, 0, Math.PI * 2);
      fCtx.fill();
    }

    // Under-eye bags (60), Eyeshadow (125), Eyebrows (110) & Eyes (160)
    for (const [ex, ey] of [[face.leftEyeX, face.leftEyeY], [face.rightEyeX, face.rightEyeY]]) {
      fCtx.fillStyle = 'rgb(60,60,60)';
      fCtx.beginPath();
      fCtx.ellipse(ex * maskSize, (ey + face.faceRy * 0.10) * maskSize, rx * 0.22, ry * 0.08, 0, 0, Math.PI * 2);
      fCtx.fill();

      fCtx.fillStyle = 'rgb(125,125,125)';
      fCtx.beginPath();
      fCtx.ellipse(ex * maskSize, (ey - face.faceRy * 0.05) * maskSize, rx * 0.20, ry * 0.055, 0, 0, Math.PI * 2);
      fCtx.fill();

      fCtx.fillStyle = 'rgb(110,110,110)';
      fCtx.beginPath();
      fCtx.ellipse(ex * maskSize, (ey - face.faceRy * 0.11) * maskSize, rx * 0.24, ry * 0.055, 0, 0, Math.PI * 2);
      fCtx.fill();

      fCtx.fillStyle = 'rgb(160,160,160)';
      fCtx.beginPath();
      fCtx.ellipse(ex * maskSize, ey * maskSize, rx * 0.18, ry * 0.075, 0, 0, Math.PI * 2);
      fCtx.fill();
    }

    // Outer Lips (205) & Inner Mouth Teeth (245)
    fCtx.fillStyle = 'rgb(205,205,205)';
    fCtx.beginPath();
    fCtx.ellipse(face.mouthX * maskSize, face.mouthY * maskSize, rx * 0.27, ry * 0.095, 0, 0, Math.PI * 2);
    fCtx.fill();

    fCtx.fillStyle = 'rgb(245,245,245)';
    fCtx.beginPath();
    fCtx.ellipse(face.mouthX * maskSize, face.mouthY * maskSize, rx * 0.19, ry * 0.045, 0, 0, Math.PI * 2);
    fCtx.fill();
  }

  const skinPixels = sCtx.getImageData(0, 0, maskSize, maskSize).data;
  const featPixels = fCtx.getImageData(0, 0, maskSize, maskSize).data;

  // Combine 478-Point Polygon Masks + Continuous 6-Class Neural Segmentation into lossless A=255 PNGs
  // Canvas 1 (maskBase64): R=Skin, G=Discrete Feature ID, B=Hair, A=255 (no premultiplication loss!)
  // Canvas 2 (sceneMaskBase64): R=Continuous Subject Matte, G=Clothing, B=Solid Subject Core, A=255
  const hasNeuralSeg = Boolean(segConfs || segClasses);
  const combinedCanvas = document.createElement('canvas');
  combinedCanvas.width = maskSize;
  combinedCanvas.height = maskSize;
  const cCtx = combinedCanvas.getContext('2d');
  const outImg = cCtx.createImageData(maskSize, maskSize);
  const outData = outImg.data;

  const sceneCanvas = hasNeuralSeg ? document.createElement('canvas') : null;
  let scCtx = null;
  let sceneImg = null;
  let sceneData = null;
  if (sceneCanvas) {
    sceneCanvas.width = maskSize;
    sceneCanvas.height = maskSize;
    scCtx = sceneCanvas.getContext('2d');
    sceneImg = scCtx.createImageData(maskSize, maskSize);
    sceneData = sceneImg.data;
  }

  for (let y = 0; y < maskSize; y++) {
    const ny = y / (maskSize - 1);
    const sy = segClasses ? Math.min(segH - 1, Math.floor(ny * segH)) : 0;
    for (let x = 0; x < maskSize; x++) {
      const nx = x / (maskSize - 1);
      const idx = (y * maskSize + x) * 4;
      const polySkin = skinPixels[idx];
      const featCode = featPixels[idx];

      let skinVal = polySkin;
      let hairVal = 0;
      let fgProb = 0;
      let clothProb = 0;
      let solidProb = 0;

      if (segConfs) {
        const bgProb = sampleConfidenceBilinear(segConfs[0], segW, segH, nx, ny);
        const hairProb = sampleConfidenceBilinear(segConfs[1], segW, segH, nx, ny);
        const bodyProb = sampleConfidenceBilinear(segConfs[2], segW, segH, nx, ny);
        const faceProb = sampleConfidenceBilinear(segConfs[3], segW, segH, nx, ny);
        clothProb = sampleConfidenceBilinear(segConfs[4], segW, segH, nx, ny);
        const accProb = segConfs[5] ? sampleConfidenceBilinear(segConfs[5], segW, segH, nx, ny) : 0;

        const personSum = Math.min(1, hairProb + bodyProb + faceProb + clothProb + accProb);
        fgProb = Math.max(0, Math.min(1, 0.5 * (1 - bgProb) + 0.5 * personSum));
        solidProb = Math.max(0, Math.min(1, bodyProb + faceProb + clothProb + accProb));
        // Boost solidProb from fgProb ONLY for non-hair body/arm/hand regions so split
        // body/cloth softmax confidences keep solid torso/arms intact without masking hair gaps.
        if (fgProb > 0.55 && hairProb < 0.25 && solidProb < fgProb * 0.80) {
          solidProb = Math.max(solidProb, fgProb * 0.72);
        }
        hairVal = Math.round(Math.max(0, Math.min(1, hairProb)) * 255);

        if (bodyProb > 0.25 && ny > face.mouthY) {
          skinVal = Math.max(skinVal, Math.round(Math.min(1, bodyProb) * 235));
        }
        if (!pts && faceProb > 0.25) {
          skinVal = Math.max(skinVal, Math.round(Math.min(1, faceProb) * 240));
        }
        if (clothProb > 0.55 || bgProb > 0.65) {
          skinVal = Math.round(skinVal * (1 - Math.max(clothProb - 0.55, bgProb - 0.65) * 1.8));
          skinVal = Math.max(0, skinVal);
        } else if (hairProb > 0.65 && polySkin < 40) {
          skinVal = Math.min(skinVal, 15);
        }
        if (polySkin > 180) {
          fgProb = Math.max(fgProb, polySkin / 255);
          solidProb = Math.max(solidProb, polySkin / 255);
        }
      } else if (segClasses) {
        const sx = Math.min(segW - 1, Math.floor(nx * segW));
        const segClass = segClasses[sy * segW + sx];
        if (segClass === 2 && ny > face.mouthY) {
          skinVal = Math.max(skinVal, 215);
        } else if (segClass === 3 && !pts) {
          skinVal = Math.max(skinVal, 230);
        } else if (segClass === 0 || segClass === 1 || segClass === 4) {
          skinVal = Math.min(skinVal, segClass === 1 ? 15 : 0);
        }
        hairVal = segClass === 1 ? 255 : 0;
        fgProb = segClass === 0 ? 0 : 1;
        clothProb = segClass === 4 ? 1 : 0;
        solidProb = segClass >= 2 ? 1 : 0;
        if (polySkin > 180) {
          fgProb = Math.max(fgProb, polySkin / 255);
          solidProb = Math.max(solidProb, polySkin / 255);
        }
      } else {
        const headD2 = Math.pow((nx - face.centerX) / (face.faceRx * 1.5), 2) + Math.pow((ny - face.centerY) / (face.faceRy * 1.45), 2);
        if (headD2 >= 0.82 && headD2 < 1.65 && ny < face.mouthY + 0.05 && polySkin < 40) {
          hairVal = 255;
        }
      }

      if (featCode === 110 || featCode === 125 || featCode === 160 || featCode === 205 || featCode === 245) {
        skinVal = 0;
      }

      outData[idx] = skinVal;
      outData[idx + 1] = featCode;
      outData[idx + 2] = hairVal;
      outData[idx + 3] = 255;

      if (sceneData) {
        sceneData[idx] = Math.round(fgProb * 255);
        sceneData[idx + 1] = Math.round(clothProb * 255);
        sceneData[idx + 2] = Math.round(solidProb * 255);
        sceneData[idx + 3] = 255;
      }
    }
  }

  cCtx.putImageData(outImg, 0, 0);
  const maskBase64 = combinedCanvas.toDataURL('image/png');

  let sceneMaskBase64 = null;
  if (sceneCanvas && scCtx && sceneImg) {
    scCtx.putImageData(sceneImg, 0, 0);
    sceneMaskBase64 = sceneCanvas.toDataURL('image/png');
  }

  return {
    face,
    mesh478: pts,
    maskBase64,
    sceneMaskBase64,
    landmarksDetected,
  };
}
