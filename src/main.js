import { invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';
import { getCurrentWebview } from '@tauri-apps/api/webview';
import { analyzePhotoWithNeuralAI, MESH_INDICES } from './ai_masks.js';

export function createDefaultParams() {
  return {
    // MODULE 2: PORTRAIT RETOUCHING
    // 1. Blemish Removal
    freckleAcne: 0,
    faceMole: 0,
    faceShine: 0,
    darkCircles: 0,
    eyeBags: 0,
    foreheadWrinkles: 0,
    nasolabialFolds: 0,
    neckWrinkles: 0,
    glassesGlare: 0,
    doubleChin: 0,
    bodyBlemish: 0,

    // 2. Skin Retouching
    dodgeBurnEven: 0,
    dodgeBurnSculpt: 0,
    freqSepLow: 0,
    freqSepHigh: 80,
    facialSkinSmooth: 0,
    bodySkinSmooth: 0,
    skinRadiance: 0,
    skinRosy: 0,
    skinWarmth: 0,
    unifyComplexion: 0,

    // 3. Face Reshape (3D Liquify)
    faceWidth: 0,
    jawlineSlim: 0,
    cheekboneLift: 0,
    chinLength: 0,
    foreheadHeight: 0,
    noseWidth: 0,
    noseTipLift: 0,
    mouthSize: 0,
    lipPlump: 0,
    smileEnhance: 0,

    // 4. Eyes & Eyebrows
    eyeSize: 0,
    eyeBrighten: 0,
    redVeins: 0,
    catchlightBoost: 0,
    eyebrowThickness: 0,
    eyebrowLift: 0,

    // 5. Teeth
    teethWhiten: 0,
    teethDesaturate: 0,
    teethBrighten: 0,

    // 6. Makeup
    makeupHighlight: 0,
    makeupContour: 0,
    makeupBlush: 0,
    makeupEyeshadow: 0,
    makeupLipstick: 0,
    makeupLipTint: 'rose',

    // 7. Hair (Evoto HairBeauty)
    strayHair: 0,
    strayHairInner: 100,
    strayHairOuter: 100,
    hairVolume: 0,
    hairSideLeftVolume: 0,
    hairSideRightVolume: 0,
    hairlineAdjust: 0,
    hairPartLine: 0,
    hairSmooth: 0,
    hairFrizzy: 100,
    hairShine: 0,
    whiteHairBlacken: 0,
    hairColorUnify: 0,
    hairDyeStrength: 0,
    hairDyeDarken: 0,
    hairDyeColor: 'none',

    // 8. Body Reshape
    neckSlim: 0,
    shoulderSculpt: 0,
    waistSlim: 0,

    // MODULE 1: COLOR ADJUSTMENT (RAW / 8-CHANNEL HSL / 256-LUT CURVE / COLOR MATCH)
    temperature: 0,
    tint: 0,
    exposure: 0,
    contrast: 0,
    highlights: 0,
    shadows: 0,
    whites: 0,
    blacks: 0,
    clarity: 0,
    texture: 0,
    dehaze: 0,
    vibrance: 0,
    saturation: 0,

    curveShadows: 0,
    curveDarks: 0,
    curveLights: 0,
    curveHighlights: 0,
    curveLut: [],

    colorMatchStrength: 0,
    colorMatchStats: [],

    hslRedHue: 0,
    hslRedSat: 0,
    hslRedLum: 0,
    hslOrangeHue: 0,
    hslOrangeSat: 0,
    hslOrangeLum: 0,
    hslYellowHue: 0,
    hslYellowSat: 0,
    hslYellowLum: 0,
    hslGreenHue: 0,
    hslGreenSat: 0,
    hslGreenLum: 0,
    hslAquaHue: 0,
    hslAquaSat: 0,
    hslAquaLum: 0,
    hslBlueHue: 0,
    hslBlueSat: 0,
    hslBlueLum: 0,
    hslPurpleHue: 0,
    hslPurpleSat: 0,
    hslPurpleLum: 0,
    hslMagentaHue: 0,
    hslMagentaSat: 0,
    hslMagentaLum: 0,

    gradeShadowHue: 210,
    gradeShadowSat: 0,
    gradeHighHue: 38,
    gradeHighSat: 0,
    sharpenAmount: 0,
    noiseReduction: 0,
    filmGrain: 0,
    vignette: 0,
    lutPreset: 'none',

    // Lightroom-parity extras
    treatment: 'color',
    bwRed: 0, bwOrange: 0, bwYellow: 0, bwGreen: 0, bwAqua: 0, bwBlue: 0, bwPurple: 0, bwMagenta: 0,
    gradeMidHue: 120, gradeMidSat: 0, gradeGlobalHue: 45, gradeGlobalSat: 0,
    gradeShadowLum: 0, gradeMidLum: 0, gradeHighLum: 0, gradeGlobalLum: 0,
    gradeBlending: 50, gradeBalance: 0,
    sharpenRadius: 1, sharpenDetail: 25, sharpenMasking: 0, nrColor: 0, aiDenoise: 0, nrDetail: 50,
    grainSize: 25, grainRoughness: 50,
    vignetteMidpoint: 50, vignetteRoundness: 0, vignetteFeather: 50, vignetteHighlights: 0,
    opticsDistortion: 0, opticsVignetting: 0, opticsVignettingMidpoint: 50,
    opticsCaRed: 0, opticsCaBlue: 0, opticsDefringePurple: 0, opticsDefringeGreen: 0,

    // MODULE 3 & 3B: BACKGROUND & CLOTHING
    clothingTarget: 'all',
    clothingDewrinkle: 0,
    clothingSmallWrinkles: 100,
    clothingLargeWrinkles: 100,
    clothingLintClean: 0,
    clothingSweatStain: 0,
    clothingSweatEvenness: 50,
    clothingEdgeSmooth: 0,
    clothingEdgeRefine: 50,
    clothingMoireRemove: 0,
    backdropClean: 0,
    backdropBandingFix: 0,
    skyEnhance: 0,
    bgBlurBokeh: 0,
    bgReplaceMode: 'original',
    bgCustomColor: '#e8eef5',
    bgCustomImage: '',
    bgCleanDistractions: false,
    bgCleanDefects: false,
    bgEvenBrightness: false,
    bgRemoveBanding: false,
    bgEdgeMode: 'auto',
    bgKeepScope: 'subject_and_related',
    bgFillArea: 'full_image',
    bgFillMode: 'cover',
    bgBlendMode: 'normal',
    bgEdgeAdjust: 0,
    bgReplaceOpacity: 100,
    bgScale: 0,
    bgOffsetX: 0,
    bgOffsetY: 0,
    bgBlurStrength: 0,
    bgDecontam: false,
    bgGroundReflect: false,
    bgPreserveShadow: false,
    bgShadowType: 'none',
    bgShadowBlur: 25,
    bgShadowOpacity: 0,
    bgShadowShift: 0,
    bgShadowBlendMode: 'multiply',

    // MODULE 4: CROP & TRANSFORM
    cropX: 0,
    cropY: 0,
    cropW: 1,
    cropH: 1,
    cropPreset: 'original',
    cropAspectW: 0,
    cropAspectH: 0,
    cropCustomW: 0,
    cropCustomH: 0,
    cropDimUnit: 'px',
    cropDpi: 300,
    cropShowGrid: true,
    cropAiSubject: false,
    cropAlignVertical: false,
    cropAlignEye: false,
    rotateAngle: 0,
    keystoneVertical: 0,
    keystoneHorizontal: 0,
    geoScale: 100,
    geoOffsetX: 0,
    geoOffsetY: 0,
    flipHorizontal: false,
    flipVertical: false,

    blemishSpots: [],
    face: {
      centerX: 0.5,
      centerY: 0.42,
      faceRx: 0.18,
      faceRy: 0.24,
      leftEyeX: 0.43,
      leftEyeY: 0.38,
      rightEyeX: 0.57,
      rightEyeY: 0.38,
      mouthX: 0.5,
      mouthY: 0.51,
      noseX: 0.5,
      noseY: 0.44,
      chinX: 0.5,
      chinY: 0.65,
      foreheadX: 0.5,
      foreheadY: 0.2,
      jawLeftX: 0.35,
      jawLeftY: 0.55,
      jawRightX: 0.65,
      jawRightY: 0.55,
      cheekLeftX: 0.34,
      cheekLeftY: 0.43,
      cheekRightX: 0.66,
      cheekRightY: 0.43,
      mouthLeftX: 0.45,
      mouthLeftY: 0.51,
      mouthRightX: 0.55,
      mouthRightY: 0.51,
    },
  };
}

const RETOUCH_PRESETS = {
  clean_natural: {
    freckleAcne: 55,
    faceShine: 35,
    darkCircles: 45,
    dodgeBurnEven: 35,
    dodgeBurnSculpt: 22,
    freqSepLow: 42,
    freqSepHigh: 84,
    skinRadiance: 22,
    eyeBrighten: 38,
    catchlightBoost: 28,
    redVeins: 35,
    teethWhiten: 35,
    exposure: 4,
    contrast: 6,
    highlights: -8,
    shadows: 12,
    vibrance: 10,
    lutPreset: 'clean_portrait',
  },
  high_fashion: {
    freckleAcne: 85,
    faceMole: 60,
    faceShine: 65,
    darkCircles: 70,
    eyeBags: 55,
    nasolabialFolds: 45,
    dodgeBurnEven: 68,
    dodgeBurnSculpt: 65,
    freqSepLow: 68,
    freqSepHigh: 84,
    unifyComplexion: 55,
    skinRadiance: 35,
    jawlineSlim: 16,
    cheekboneLift: 18,
    eyeSize: 10,
    eyeBrighten: 60,
    catchlightBoost: 50,
    teethWhiten: 55,
    makeupHighlight: 42,
    makeupContour: 38,
    makeupLipstick: 30,
    strayHair: 55,
    hairShine: 48,
    contrast: 14,
    clarity: 14,
    sharpenAmount: 20,
    lutPreset: 'clean_portrait',
  },
  glamour_makeup: {
    freckleAcne: 70,
    freqSepLow: 58,
    freqSepHigh: 82,
    dodgeBurnSculpt: 50,
    skinRadiance: 38,
    skinRosy: 16,
    darkCircles: 60,
    eyeBrighten: 55,
    eyebrowThickness: 26,
    eyebrowLift: 14,
    lipPlump: 20,
    smileEnhance: 16,
    makeupHighlight: 50,
    makeupContour: 46,
    makeupBlush: 58,
    makeupEyeshadow: 45,
    makeupLipstick: 65,
    makeupLipTint: 'berry',
    hairVolume: 25,
    hairShine: 42,
  },
  wedding_luminous: {
    freckleAcne: 50,
    faceShine: 38,
    darkCircles: 48,
    freqSepLow: 48,
    freqSepHigh: 85,
    skinRadiance: 42,
    skinWarmth: 16,
    eyeBrighten: 42,
    teethWhiten: 48,
    smileEnhance: 14,
    exposure: 8,
    highlights: -14,
    shadows: 20,
    temperature: 12,
    vibrance: 14,
    lutPreset: 'warm_wedding',
  },
  corporate_exec: {
    freckleAcne: 60,
    faceShine: 70,
    glassesGlare: 45,
    darkCircles: 65,
    eyeBags: 50,
    doubleChin: 30,
    freqSepLow: 38,
    freqSepHigh: 88,
    dodgeBurnEven: 42,
    eyeBrighten: 45,
    redVeins: 50,
    teethWhiten: 45,
    strayHair: 65,
    clothingDewrinkle: 75,
    clothingLintClean: 55,
    backdropClean: 60,
    bgBlurBokeh: 35,
    contrast: 10,
    lutPreset: 'clean_portrait',
  },
  passport_id: {
    freckleAcne: 48,
    faceShine: 60,
    darkCircles: 50,
    freqSepLow: 32,
    strayHair: 80,
    clothingDewrinkle: 65,
    backdropClean: 85,
    bgReplaceMode: 'pure_white',
    exposure: 5,
    contrast: 6,
  },
  cinematic_film: {
    freckleAcne: 50,
    freqSepLow: 42,
    dodgeBurnSculpt: 50,
    eyeBrighten: 55,
    contrast: 20,
    highlights: -18,
    shadows: 12,
    clarity: 20,
    gradeShadowHue: 205,
    gradeShadowSat: 35,
    gradeHighHue: 36,
    gradeHighSat: 30,
    bgBlurBokeh: 50,
    vignette: 32,
    lutPreset: 'cinematic_teal',
  },
  bw_dramatic: {
    freckleAcne: 60,
    freqSepLow: 45,
    dodgeBurnSculpt: 70,
    eyeBrighten: 65,
    contrast: 28,
    highlights: -14,
    whites: 18,
    blacks: -18,
    clarity: 32,
    sharpenAmount: 22,
    filmGrain: 25,
    vignette: 38,
    lutPreset: 'bw_studio',
  },
};

const USER_PRESETS_STORAGE_KEY = 'retouch_studio_user_presets_v1';

const hasTauriRuntime = () =>
  typeof window !== 'undefined' && Boolean(window.__TAURI_INTERNALS__);

function loadSavedUserPresets() {
  try {
    const raw = localStorage.getItem(USER_PRESETS_STORAGE_KEY);
    if (!raw) return [];
    const parsed = JSON.parse(raw);
    return Array.isArray(parsed) ? parsed : [];
  } catch (_) {
    return [];
  }
}

function persistUserPresets(list) {
  try {
    localStorage.setItem(USER_PRESETS_STORAGE_KEY, JSON.stringify(list));
  } catch (_) {}
}

const state = {
  photos: [],
  activeIndex: 0,
  splitEnabled: true,
  splitRatio: 0.5,
  holdingBefore: false,
  spotHealMode: false,
  showFaceGuides: false,
  zoomScale: 1.0,
  panX: 0,
  panY: 0,
  realtimeColor: true,
  workspaceMode: 'develop', // 'develop' | 'grid'
  activeModule: 'mod-portrait',
  cropPreviewMode: false,
  isRestoringHistory: false,
  bgCategory: 'image', // 'image' | 'solid'
  bgLibTab: 'rec', // 'rec' | 'user'
  bgFilter: 'explore', // 'explore' | 'selected'
  bgUserFilter: 'personal', // 'personal' | 'fav'
  bgMoreExpanded: false,
  customBgGallery: [], // { id, name, dataUrl, fav }
  clothWrinkleLinked: true,
  hairSideLinked: true,
  presetScope: 'all', // 'all' | 'recommended' | 'mine'
  histOpen: true,
  copiedParams: null,
  userPresets: loadSavedUserPresets(),
  isProcessing: false,
  pendingProcess: false,
  draggingSplit: false,
  draggingLandmark: null,
  draggingCurveIdx: -1,
  draggingCrop: null,
  draggingRotateAngle: false,
  spacePressed: false,
  draggingPan: null,
};

const afterCanvas = document.getElementById('after-canvas');
const beforeCanvas = document.getElementById('before-canvas');
const overlayCanvas = document.getElementById('overlay-canvas');
const canvasWrap = document.getElementById('canvas-wrapper');
const splitLine = document.getElementById('split-line');
const histCanvas = document.getElementById('histogram-canvas');
const curveCanvas = document.getElementById('tone-curve-canvas');
const renderTimeLabel = document.getElementById('render-time-label');
const resolutionLabel = document.getElementById('resolution-label');
const filenameLabel = document.getElementById('active-filename-label');
const pixelReadout = document.getElementById('pixel-picker-readout');
const histRgbMean = document.getElementById('hist-rgb-mean');
const filmstripTrack = document.getElementById('filmstrip-track');
const photoCountBadge = document.getElementById('photo-count-badge');
const railBatchRing = document.getElementById('rail-batch-ring');
const spotBanner = document.getElementById('spot-mode-banner');
const zoomReadout = document.getElementById('zoom-readout');

function getActivePhoto() {
  return state.photos[state.activeIndex];
}

function loadImageElement(src) {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.crossOrigin = 'anonymous';
    img.onload = () => resolve(img);
    img.onerror = reject;
    img.src = src;
  });
}

async function fetchUrlAsDataUrl(url) {
  const resp = await fetch(url);
  const blob = await resp.blob();
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(reader.result);
    reader.onerror = reject;
    reader.readAsDataURL(blob);
  });
}

function formatSliderBadgeValue(val, key = '') {
  const num = Number(val);
  if (!Number.isFinite(num)) return '0';
  if (key === 'rotateAngle') return num.toFixed(2);
  return Number.isInteger(num) ? String(num) : num.toFixed(1);
}

function clampZoomPan() {
  if (state.zoomScale < 1) {
    state.panX = 0;
    state.panY = 0;
    return;
  }
  const vp = document.getElementById('canvas-viewport');
  const cw = canvasWrap;
  if (!vp || !cw) return;
  const scaledW = (cw.offsetWidth || 400) * state.zoomScale;
  const scaledH = (cw.offsetHeight || 400) * state.zoomScale;
  const maxX = Math.max(vp.clientWidth * 0.35, (scaledW - vp.clientWidth) * 0.5 + 80);
  const maxY = Math.max(vp.clientHeight * 0.35, (scaledH - vp.clientHeight) * 0.5 + 80);
  state.panX = Math.max(-maxX, Math.min(maxX, state.panX));
  state.panY = Math.max(-maxY, Math.min(maxY, state.panY));
}

let _lastZoomScheduleScale = 1;
let _zoomRefineTimer = null;

function updateZoomUI() {
  clampZoomPan();
  // Zoom + cursor-anchored pan via CSS transform so all canvas layers stay synced.
  if (canvasWrap) {
    canvasWrap.style.transform = `translate(${state.panX.toFixed(1)}px, ${state.panY.toFixed(1)}px) scale(${state.zoomScale})`;
  }
  if (zoomReadout) {
    zoomReadout.textContent =
      Math.abs(state.zoomScale - 1) < 0.02
        ? 'Vừa'
        : `${Math.round(state.zoomScale * 100)}%`;
  }
  const btnFit = document.getElementById('btn-zoom-fit');
  const btn100 = document.getElementById('btn-zoom-100');
  btnFit?.classList.toggle('active', Math.abs(state.zoomScale - 1) < 0.02);
  btn100?.classList.toggle('active', Math.abs(state.zoomScale - 2) < 0.05);

  if (Math.abs(state.zoomScale - _lastZoomScheduleScale) > 0.08) {
    _lastZoomScheduleScale = state.zoomScale;
    clearTimeout(_zoomRefineTimer);
    _zoomRefineTimer = setTimeout(() => {
      const photo = getActivePhoto();
      if (!photo) return;
      renderViewport();
      if (
        state.zoomScale >= 1.35 &&
        hasTauriRuntime() &&
        !photo._processedHQ &&
        !photo._isLiyingOutput &&
        isPhotoEdited(photo.params)
      ) {
        processActivePhoto(true);
      }
    }, 90);
  }
}

// =========================================================================
// PER-PHOTO UNDO / REDO HISTORY ENGINE (Ctrl+Z / Ctrl+Shift+Z)
// =========================================================================
const MAX_HISTORY_STEPS = 40;
const CROP_FRAME_PARAM_KEYS = new Set([
  'cropX',
  'cropY',
  'cropW',
  'cropH',
  'cropPreset',
  'cropAspectW',
  'cropAspectH',
  'cropCustomW',
  'cropCustomH',
  'cropDimUnit',
  'cropDpi',
  'cropShowGrid',
  'cropAiSubject',
  'cropAlignVertical',
  'cropAlignEye',
]);

function nonCropParamsFingerprint(params) {
  const out = {};
  for (const k of Object.keys(params || {})) {
    if (!CROP_FRAME_PARAM_KEYS.has(k)) out[k] = params[k];
  }
  return JSON.stringify(out);
}

function capturePhotoSnapshot(photo) {
  if (!photo) return null;
  const paramsCopy = JSON.parse(JSON.stringify(photo.params));
  const curveCopy = photo.curvePoints ? JSON.parse(JSON.stringify(photo.curvePoints)) : null;
  const sig = JSON.stringify({
    p: paramsCopy,
    c: curveCopy,
    cp: Boolean(state.cropPreviewMode),
    w: photo.width,
    h: photo.height,
    o: photo.originalDataUrl,
    ly: Boolean(photo._isLiyingOutput),
  });
  return {
    params: paramsCopy,
    curvePoints: curveCopy,
    cropPreviewMode: Boolean(state.cropPreviewMode),
    originalDataUrl: photo.originalDataUrl,
    processedDataUrl: photo.processedDataUrl,
    width: photo.width,
    height: photo.height,
    mesh478: photo.mesh478 || null,
    maskBase64: photo.maskBase64 || null,
    sceneMaskBase64: photo.sceneMaskBase64 || null,
    highresMatteBase64: photo.highresMatteBase64 || null,
    hairBounds: photo.hairBounds || null,
    _matteSourceRef: photo._matteSourceRef || null,
    landmarksDetected: Boolean(photo.landmarksDetected),
    isLiyingOutput: Boolean(photo._isLiyingOutput),
    _sig: sig,
  };
}

function updateHistoryButtonsUI(photo = getActivePhoto()) {
  const undoBtn = document.getElementById('btn-undo');
  const redoBtn = document.getElementById('btn-redo');
  const hist = photo ? ensurePhotoHistory(photo) : null;
  if (undoBtn) undoBtn.disabled = !hist || hist.undoStack.length === 0;
  if (redoBtn) redoBtn.disabled = !hist || hist.redoStack.length === 0;
  renderHistoryPanel(photo);
}

function initPhotoHistory(photo) {
  if (!photo) return;
  photo.history = {
    undoStack: [],
    redoStack: [],
    current: capturePhotoSnapshot(photo),
  };
  if (photo === getActivePhoto()) updateHistoryButtonsUI(photo);
}

function ensurePhotoHistory(photo) {
  if (!photo) return null;
  if (!photo.history || !photo.history.current) {
    initPhotoHistory(photo);
  }
  return photo.history;
}

function commitHistory(photo = getActivePhoto()) {
  if (!photo || state.isRestoringHistory) return;
  const hist = ensurePhotoHistory(photo);
  const next = capturePhotoSnapshot(photo);
  if (!next || next._sig === hist.current?._sig) return;
  hist.undoStack.push(hist.current);
  if (hist.undoStack.length > MAX_HISTORY_STEPS) {
    hist.undoStack.shift();
  }
  hist.current = next;
  hist.redoStack = [];
  if (photo === getActivePhoto()) updateHistoryButtonsUI(photo);
}

async function applyHistorySnapshot(photo, targetSnap, prevSnap) {
  if (!photo || !targetSnap) return;
  const nonCropChanged =
    !prevSnap ||
    targetSnap.originalDataUrl !== prevSnap.originalDataUrl ||
    nonCropParamsFingerprint(targetSnap.params) !== nonCropParamsFingerprint(prevSnap.params) ||
    JSON.stringify(targetSnap.curvePoints) !== JSON.stringify(prevSnap.curvePoints);

  state.isRestoringHistory = true;
  try {
    photo.params = JSON.parse(JSON.stringify(targetSnap.params));
    photo.curvePoints = targetSnap.curvePoints
      ? JSON.parse(JSON.stringify(targetSnap.curvePoints))
      : null;
    state.cropPreviewMode = Boolean(targetSnap.cropPreviewMode);
    photo.originalDataUrl = targetSnap.originalDataUrl;
    photo.width = targetSnap.width;
    photo.height = targetSnap.height;
    photo.mesh478 = targetSnap.mesh478 || null;
    photo.maskBase64 = targetSnap.maskBase64 || null;
    photo.sceneMaskBase64 = targetSnap.sceneMaskBase64 || null;
    photo.highresMatteBase64 = targetSnap.highresMatteBase64 || null;
    photo.hairBounds = targetSnap.hairBounds || null;
    photo._hairGuideCanvas = null;
    photo._hairGuideMaskRef = null;
    photo._matteSourceRef = targetSnap._matteSourceRef || null;
    photo.landmarksDetected = Boolean(targetSnap.landmarksDetected);
    photo._isLiyingOutput = Boolean(targetSnap.isLiyingOutput);
    photo._subjectBoundsCache = null;

    document.querySelectorAll('.retouch-preset-item').forEach((c) => c.classList.remove('active'));
    syncSlidersFromParams();

    if (targetSnap.isLiyingOutput || !nonCropChanged) {
      if (targetSnap.isLiyingOutput && targetSnap.processedDataUrl) {
        photo.processedDataUrl = targetSnap.processedDataUrl;
      }
      await renderViewport();
      renderFilmstrip();
      if (state.workspaceMode === 'grid') renderLibraryGrid();
    } else {
      await processActivePhoto(false);
    }
  } finally {
    state.isRestoringHistory = false;
  }
}

async function undoActivePhoto() {
  const photo = getActivePhoto();
  if (!photo) return;
  const hist = ensurePhotoHistory(photo);
  const liveSnap = capturePhotoSnapshot(photo);
  if (liveSnap && hist.current && liveSnap._sig !== hist.current._sig) {
    hist.undoStack.push(hist.current);
    hist.current = liveSnap;
  }
  if (hist.undoStack.length === 0) {
    renderTimeLabel.textContent = 'Không còn thao tác để hoàn tác';
    return;
  }
  const prevSnap = hist.current;
  const targetSnap = hist.undoStack.pop();
  hist.redoStack.push(prevSnap);
  hist.current = targetSnap;
  await applyHistorySnapshot(photo, targetSnap, prevSnap);
  updateHistoryButtonsUI(photo);
  renderTimeLabel.textContent = `Đã hoàn tác (${hist.undoStack.length} bước trước) ✓`;
}

async function redoActivePhoto() {
  const photo = getActivePhoto();
  if (!photo) return;
  const hist = ensurePhotoHistory(photo);
  if (hist.redoStack.length === 0) {
    renderTimeLabel.textContent = 'Không còn thao tác để làm lại';
    return;
  }
  const prevSnap = hist.current;
  const targetSnap = hist.redoStack.pop();
  hist.undoStack.push(prevSnap);
  hist.current = targetSnap;
  await applyHistorySnapshot(photo, targetSnap, prevSnap);
  updateHistoryButtonsUI(photo);
  renderTimeLabel.textContent = `Đã làm lại (${hist.redoStack.length} bước sau) ✓`;
}

const HISTORY_PARAM_META = {
  // Khuyết điểm
  freckleAcne: { cat: 'Xử lý khuyết điểm', label: 'Xóa mụn & tàn nhang' },
  faceMole: { cat: 'Xử lý khuyết điểm', label: 'Xóa nốt ruồi' },
  faceShine: { cat: 'Xử lý khuyết điểm', label: 'Giảm bóng dầu' },
  darkCircles: { cat: 'Xử lý khuyết điểm', label: 'Xóa quầng thâm' },
  eyeBags: { cat: 'Xử lý khuyết điểm', label: 'Xóa bọng mắt' },
  foreheadWrinkles: { cat: 'Xử lý khuyết điểm', label: 'Nếp nhăn trán' },
  nasolabialFolds: { cat: 'Xử lý khuyết điểm', label: 'Rãnh cười' },
  neckWrinkles: { cat: 'Xử lý khuyết điểm', label: 'Nếp nhăn cổ' },
  glassesGlare: { cat: 'Xử lý khuyết điểm', label: 'Loại bỏ lóa kính' },
  doubleChin: { cat: 'Xử lý khuyết điểm', label: 'Giảm nọng cằm' },
  bodyBlemish: { cat: 'Xử lý khuyết điểm', label: 'Mụn & vết thâm cơ thể' },
  blemishSpots: { cat: 'Xử lý khuyết điểm', label: 'Chấm xóa khuyết điểm' },
  // Da
  dodgeBurnEven: { cat: 'Làm mịn & Da', label: 'Làm đều màu da (D&B)' },
  dodgeBurnSculpt: { cat: 'Làm mịn & Da', label: 'Tạo khối sáng tối (D&B)' },
  freqSepLow: { cat: 'Làm mịn & Da', label: 'Làm mịn da (Tách tần số)' },
  freqSepHigh: { cat: 'Làm mịn & Da', label: 'Giữ chi tiết lỗ chân lông' },
  facialSkinSmooth: { cat: 'Làm mịn & Da', label: 'Làm mịn da mặt' },
  bodySkinSmooth: { cat: 'Làm mịn & Da', label: 'Làm mịn da cơ thể' },
  skinRadiance: { cat: 'Làm mịn & Da', label: 'Độ rạng rỡ da' },
  skinRosy: { cat: 'Làm mịn & Da', label: 'Độ hồng hào da' },
  skinWarmth: { cat: 'Làm mịn & Da', label: 'Tông da ấm / lạnh' },
  unifyComplexion: { cat: 'Làm mịn & Da', label: 'Đồng nhất sắc da' },
  // Tạo hình khuôn mặt
  faceWidth: { cat: 'Tạo hình khuôn mặt', label: 'Thon mặt' },
  jawlineSlim: { cat: 'Tạo hình khuôn mặt', label: 'Thon hàm' },
  cheekboneLift: { cat: 'Tạo hình khuôn mặt', label: 'Gò má' },
  chinLength: { cat: 'Tạo hình khuôn mặt', label: 'Độ dài cằm' },
  foreheadHeight: { cat: 'Tạo hình khuôn mặt', label: 'Độ cao trán' },
  noseWidth: { cat: 'Tạo hình khuôn mặt', label: 'Thu gọn cánh mũi' },
  noseTipLift: { cat: 'Tạo hình khuôn mặt', label: 'Nâng đầu mũi' },
  mouthSize: { cat: 'Tạo hình khuôn mặt', label: 'Cỡ miệng' },
  lipPlump: { cat: 'Tạo hình khuôn mặt', label: 'Độ đầy môi' },
  smileEnhance: { cat: 'Tạo hình khuôn mặt', label: 'Nâng khóe miệng cười' },
  // Mắt & Răng & Trang điểm
  eyeSize: { cat: 'Mắt & Lông mày', label: 'Mở to mắt' },
  eyeBrighten: { cat: 'Mắt & Lông mày', label: 'Sáng lòng mắt' },
  redVeins: { cat: 'Mắt & Lông mày', label: 'Xóa tia máu mắt' },
  catchlightBoost: { cat: 'Mắt & Lông mày', label: 'Tăng ánh sao mắt' },
  eyebrowThickness: { cat: 'Mắt & Lông mày', label: 'Độ đậm lông mày' },
  eyebrowLift: { cat: 'Mắt & Lông mày', label: 'Nâng cung mày' },
  teethWhiten: { cat: 'Răng & Nụ cười', label: 'Làm trắng răng' },
  teethDesaturate: { cat: 'Răng & Nụ cười', label: 'Khử ố vàng răng' },
  teethBrighten: { cat: 'Răng & Nụ cười', label: 'Làm sáng răng' },
  makeupHighlight: { cat: 'Trang điểm', label: 'Highlight bắt sáng' },
  makeupContour: { cat: 'Trang điểm', label: 'Tạo khối khối mặt' },
  makeupBlush: { cat: 'Trang điểm', label: 'Má hồng' },
  makeupEyeshadow: { cat: 'Trang điểm', label: 'Phấn mắt' },
  makeupLipstick: { cat: 'Trang điểm', label: 'Son môi' },
  makeupLipTint: { cat: 'Trang điểm', label: 'Màu son môi' },
  // Tóc (Evoto Hair)
  strayHair: { cat: 'Điều chỉnh tóc', label: 'Loại bỏ tóc gãy' },
  strayHairInner: { cat: 'Điều chỉnh tóc', label: 'Tóc bay bên trong' },
  strayHairOuter: { cat: 'Điều chỉnh tóc', label: 'Tóc bay bên ngoài' },
  hairVolume: { cat: 'Điều chỉnh tóc', label: 'Độ Dày Tóc Đỉnh Đầu' },
  hairSideLeftVolume: { cat: 'Điều chỉnh tóc', label: 'Tăng lượng tóc trái' },
  hairSideRightVolume: { cat: 'Điều chỉnh tóc', label: 'Tăng lượng tóc phải' },
  hairlineAdjust: { cat: 'Điều chỉnh tóc', label: 'Đường chân tóc' },
  hairPartLine: { cat: 'Điều chỉnh tóc', label: 'Làm đầy đường ngôi' },
  hairSmooth: { cat: 'Điều chỉnh tóc', label: 'Làm mượt tóc' },
  hairFrizzy: { cat: 'Điều chỉnh tóc', label: 'Giảm tóc rối bông xù' },
  hairShine: { cat: 'Điều chỉnh tóc', label: 'Tăng độ bóng cho tóc' },
  whiteHairBlacken: { cat: 'Điều chỉnh tóc', label: 'Tóc Đen Tóc Bạc' },
  hairColorUnify: { cat: 'Điều chỉnh tóc', label: 'Thống Nhất Màu Tóc' },
  hairDyeStrength: { cat: 'Điều chỉnh tóc', label: 'Cường độ màu nhuộm' },
  hairDyeDarken: { cat: 'Điều chỉnh tóc', label: 'Độ trầm màu tóc' },
  hairDyeColor: { cat: 'Điều chỉnh tóc', label: 'Màu Tóc' },
  // Dáng người
  neckSlim: { cat: 'Dáng người', label: 'Thon cổ' },
  shoulderSculpt: { cat: 'Dáng người', label: 'Cân & thon vai' },
  waistSlim: { cat: 'Dáng người', label: 'Thon eo' },
  // Trang phục & Phông nền
  clothingTarget: { cat: 'Trang phục', label: 'Vùng trang phục' },
  clothingDewrinkle: { cat: 'Trang phục', label: 'Là phẳng nếp nhăn áo' },
  clothingSmallWrinkles: { cat: 'Trang phục', label: 'Nếp nhăn nhỏ' },
  clothingLargeWrinkles: { cat: 'Trang phục', label: 'Nếp nhăn lớn' },
  clothingLintClean: { cat: 'Trang phục', label: 'Làm sạch bụi vải' },
  clothingSweatStain: { cat: 'Trang phục', label: 'Xóa vết ố mồ hôi' },
  clothingEdgeSmooth: { cat: 'Trang phục', label: 'Mượt viền trang phục' },
  clothingMoireRemove: { cat: 'Trang phục', label: 'Khử sọc Moiré' },
  backdropClean: { cat: 'Phông nền', label: 'Làm sạch phông nền' },
  backdropBandingFix: { cat: 'Phông nền', label: 'Khử vệt banding nền' },
  skyEnhance: { cat: 'Phông nền', label: 'Tăng chiều sâu bầu trời' },
  bgBlurBokeh: { cat: 'Phông nền', label: 'Xóa phông Bokeh' },
  bgReplaceMode: { cat: 'Phông nền', label: 'Thay phông nền' },
  bgCustomColor: { cat: 'Phông nền', label: 'Màu nền tùy chỉnh' },
  bgCustomImage: { cat: 'Phông nền', label: 'Ảnh nền tùy chỉnh' },
  // Màu sắc & Ống kính
  temperature: { cat: 'Chỉnh màu & Ánh sáng', label: 'Nhiệt độ màu' },
  tint: { cat: 'Chỉnh màu & Ánh sáng', label: 'Sắc thái Tint' },
  exposure: { cat: 'Chỉnh màu & Ánh sáng', label: 'Độ phơi sáng' },
  contrast: { cat: 'Chỉnh màu & Ánh sáng', label: 'Độ tương phản' },
  highlights: { cat: 'Chỉnh màu & Ánh sáng', label: 'Vùng sáng' },
  shadows: { cat: 'Chỉnh màu & Ánh sáng', label: 'Vùng tối' },
  whites: { cat: 'Chỉnh màu & Ánh sáng', label: 'Điểm trắng' },
  blacks: { cat: 'Chỉnh màu & Ánh sáng', label: 'Điểm đen' },
  clarity: { cat: 'Chỉnh màu & Ánh sáng', label: 'Độ trong (Clarity)' },
  texture: { cat: 'Chỉnh màu & Ánh sáng', label: 'Cấu trúc bề mặt' },
  dehaze: { cat: 'Chỉnh màu & Ánh sáng', label: 'Khử sương mù' },
  vibrance: { cat: 'Chỉnh màu & Ánh sáng', label: 'Độ tươi màu' },
  saturation: { cat: 'Chỉnh màu & Ánh sáng', label: 'Độ bão hòa' },
  lutPreset: { cat: 'Chỉnh màu & Ánh sáng', label: 'Bộ màu LUT' },
  sharpenAmount: { cat: 'Chi tiết & Độ nét', label: 'Làm nét ảnh' },
  noiseReduction: { cat: 'Chi tiết & Độ nét', label: 'Giảm nhiễu' },
  aiDenoise: { cat: 'Chi tiết & Độ nét', label: 'Khử nhiễu AI' },
  filmGrain: { cat: 'Hiệu ứng & Viền', label: 'Hạt phim' },
  vignette: { cat: 'Hiệu ứng & Viền', label: 'Tối góc (Vignette)' },
  opticsDistortion: { cat: 'Hiệu chỉnh ống kính', label: 'Hiệu chỉnh méo hình' },
  opticsVignetting: { cat: 'Hiệu chỉnh ống kính', label: 'Chống tối góc ống kính' },
  // Cắt & Xoay
  rotateAngle: { cat: 'Cắt & Xoay', label: 'Xoay góc ảnh' },
  keystoneVertical: { cat: 'Cắt & Xoay', label: 'Nghiêng dọc' },
  keystoneHorizontal: { cat: 'Cắt & Xoay', label: 'Nghiêng ngang' },
  flipHorizontal: { cat: 'Cắt & Xoay', label: 'Lật ngang' },
  flipVertical: { cat: 'Cắt & Xoay', label: 'Lật dọc' },
};

function resolveHistoryParamMeta(key) {
  if (HISTORY_PARAM_META[key]) return HISTORY_PARAM_META[key];
  if (key.startsWith('crop')) return { cat: 'Cắt', label: 'Khung cắt' };
  if (key.startsWith('geo')) return { cat: 'Cắt & Xoay', label: 'Tỉ lệ & Dịch khung' };
  if (key.startsWith('hsl')) return { cat: 'HSL & Phân màu', label: 'Kênh màu HSL' };
  if (key.startsWith('grade')) return { cat: 'Phân màu (Color Grading)', label: 'Bánh xe màu' };
  if (key.startsWith('curve')) return { cat: 'Đường cong Tone', label: 'Tone Curve' };
  if (key.startsWith('colorMatch')) return { cat: 'Khớp màu AI', label: 'Độ khớp màu mẫu' };
  if (key.startsWith('bg')) return { cat: 'Phông nền', label: 'Cấu hình tách nền' };
  if (key.startsWith('clothing')) return { cat: 'Trang phục', label: 'Xử lý trang phục' };
  if (key.startsWith('optics')) return { cat: 'Hiệu chỉnh ống kính', label: 'Quang học' };
  if (key.startsWith('bw') || key === 'treatment') return { cat: 'Chỉnh màu & Ánh sáng', label: 'Chế độ Đen Trắng' };
  return { cat: 'Tinh chỉnh thông số', label: key };
}

function formatHistoryValue(key, val, defaultVal) {
  if (val === 'none' || val === 'original') return 'Trống';
  if (typeof val === 'string' && key === 'hairDyeColor') return val.toUpperCase();
  if (typeof val === 'boolean') return val ? 'Bật' : 'Tắt';
  if (Array.isArray(val)) return `${val.length} điểm`;
  if (typeof val === 'number') {
    if (defaultVal !== undefined && val === defaultVal) return 'Reset';
    const rounded = Number.isInteger(val) ? val : Number(val.toFixed(1));
    return rounded > 0 ? `+${rounded}` : String(rounded);
  }
  return String(val ?? '');
}

const DEFAULT_PARAM_REF = createDefaultParams();

function describeHistoryStep(snap, prevSnap, index) {
  if (index === 0 || !prevSnap) {
    return { title: 'Ảnh gốc', param: 'Nhập ảnh ban đầu', value: '' };
  }
  if (snap.isLiyingOutput && !prevSnap.isLiyingOutput) {
    return { title: 'LiYing AI Studio', param: 'Tách nền / Cân vai AI', value: 'Hoàn tất' };
  }
  if (snap.originalDataUrl !== prevSnap.originalDataUrl) {
    return { title: 'Cập nhật ảnh nền / nguồn', param: 'Dữ liệu ảnh', value: 'Đã đổi' };
  }
  const changedKeys = [];
  const allKeys = new Set([
    ...Object.keys(snap.params || {}),
    ...Object.keys(prevSnap.params || {}),
  ]);
  for (const k of allKeys) {
    if (k === 'face') continue;
    if (JSON.stringify(snap.params?.[k]) !== JSON.stringify(prevSnap.params?.[k])) {
      changedKeys.push(k);
    }
  }
  if (JSON.stringify(snap.curvePoints) !== JSON.stringify(prevSnap.curvePoints)) {
    changedKeys.push('curveLut');
  }
  if (changedKeys.length === 0) {
    if (snap.cropPreviewMode !== prevSnap.cropPreviewMode) {
      return { title: 'Cắt', param: 'Xem trước khung cắt', value: snap.cropPreviewMode ? 'Bật' : 'Tắt' };
    }
    return { title: 'Cập nhật trạng thái', param: '', value: '' };
  }

  // Kiểm tra khôi phục mặc định toàn ảnh
  const nonCropDefaultCount = changedKeys.filter(
    (k) => !CROP_FRAME_PARAM_KEYS.has(k) && JSON.stringify(snap.params?.[k]) === JSON.stringify(DEFAULT_PARAM_REF[k])
  ).length;
  if (changedKeys.length >= 4 && nonCropDefaultCount >= Math.ceil(changedKeys.length * 0.75)) {
    return { title: 'Đặt lại thông số ảnh', param: 'Toàn bộ thông số', value: 'Reset' };
  }

  // Nhiều key thuộc cùng nhóm Cắt
  const allCrop = changedKeys.every((k) => CROP_FRAME_PARAM_KEYS.has(k));
  if (allCrop) {
    if (changedKeys.length === 1 && changedKeys[0] === 'cropPreset') {
      return { title: 'Cắt', param: 'Tỉ lệ khung', value: String(snap.params.cropPreset || 'Tùy chỉnh') };
    }
    return { title: 'Cắt', param: '', value: '' };
  }

  if (changedKeys.length === 1) {
    const k = changedKeys[0];
    const meta = resolveHistoryParamMeta(k);
    return {
      title: meta.cat,
      param: meta.label,
      value: formatHistoryValue(k, snap.params?.[k], DEFAULT_PARAM_REF[k]),
    };
  }

  const firstMeta = resolveHistoryParamMeta(changedKeys[0]);
  const sameCategory = changedKeys.every((k) => resolveHistoryParamMeta(k).cat === firstMeta.cat);
  if (sameCategory) {
    return {
      title: firstMeta.cat,
      param: firstMeta.label,
      value: `${changedKeys.length} mục`,
    };
  }
  return {
    title: 'Áp dụng Cài sẵn / Đồng bộ',
    param: 'Thay đổi hàng loạt',
    value: `${changedKeys.length} thông số`,
  };
}

function renderHistoryPanel(photo = getActivePhoto()) {
  const nameEl = document.getElementById('history-photo-name');
  const listEl = document.getElementById('history-list');
  if (!listEl) return;

  if (!photo) {
    if (nameEl) nameEl.textContent = 'Chưa chọn ảnh';
    listEl.innerHTML = '<div class="history-empty-note">Mở hoặc chọn một ảnh để xem lịch sử chỉnh sửa.</div>';
    return;
  }

  if (nameEl) nameEl.textContent = photo.name || 'Ảnh chưa đặt tên';
  const hist = ensurePhotoHistory(photo);
  const futureDesc = [...hist.redoStack].reverse();
  const flat = [...hist.undoStack, hist.current, ...futureDesc];
  const activeIdx = hist.undoStack.length;

  listEl.innerHTML = '';
  // Hiển thị mới nhất ở trên cùng giống Evoto
  for (let idx = flat.length - 1; idx >= 0; idx--) {
    const snap = flat[idx];
    const prevSnap = idx > 0 ? flat[idx - 1] : null;
    const desc = describeHistoryStep(snap, prevSnap, idx);

    const item = document.createElement('button');
    item.type = 'button';
    item.className = 'history-item';
    if (idx === activeIdx) item.classList.add('active');
    else if (idx > activeIdx) item.classList.add('future');
    item.dataset.histIdx = String(idx);

    const titleEl = document.createElement('div');
    titleEl.className = 'history-item-title';
    titleEl.textContent = desc.title;
    item.appendChild(titleEl);

    if (desc.param || desc.value) {
      const subEl = document.createElement('div');
      subEl.className = 'history-item-sub';

      const paramEl = document.createElement('span');
      paramEl.className = 'history-item-param';
      paramEl.textContent = desc.param || '';

      const valEl = document.createElement('span');
      valEl.className = 'history-item-val';
      valEl.textContent = desc.value || '';

      subEl.appendChild(paramEl);
      subEl.appendChild(valEl);
      item.appendChild(subEl);
    }

    item.addEventListener('click', () => {
      jumpToHistoryIndex(photo, idx);
    });

    listEl.appendChild(item);
  }
}

async function jumpToHistoryIndex(photo, targetIdx) {
  if (!photo || state.isRestoringHistory) return;
  const hist = ensurePhotoHistory(photo);
  const futureDesc = [...hist.redoStack].reverse();
  const flat = [...hist.undoStack, hist.current, ...futureDesc];
  if (targetIdx < 0 || targetIdx >= flat.length || targetIdx === hist.undoStack.length) {
    return;
  }

  const prevSnap = hist.current;
  const targetSnap = flat[targetIdx];
  hist.undoStack = flat.slice(0, targetIdx);
  hist.current = targetSnap;
  hist.redoStack = flat.slice(targetIdx + 1).reverse();

  await applyHistorySnapshot(photo, targetSnap, prevSnap);
  updateHistoryButtonsUI(photo);
  if (renderTimeLabel) {
    renderTimeLabel.textContent = `Đã chuyển tới bước lịch sử #${targetIdx + 1} ✓`;
  }
}

// =========================================================================
// INTERACTIVE 256-ENTRY TONE CURVE ENGINE (SPLINE INTERPOLATION + CANVAS UI)
// =========================================================================
function getPhotoCurvePoints(photo) {
  if (!photo.curvePoints || photo.curvePoints.length < 2) {
    photo.curvePoints = [
      { x: 0, y: 0 },
      { x: 0.25, y: 0.25 },
      { x: 0.5, y: 0.5 },
      { x: 0.75, y: 0.75 },
      { x: 1, y: 1 },
    ];
  }
  return photo.curvePoints;
}

function evaluateCurveSpline(points, x) {
  const sorted = [...points].sort((a, b) => a.x - b.x);
  if (x <= sorted[0].x) return sorted[0].y;
  if (x >= sorted[sorted.length - 1].x) return sorted[sorted.length - 1].y;

  let i = 0;
  while (i < sorted.length - 1 && sorted[i + 1].x < x) i++;
  const p0 = sorted[Math.max(0, i - 1)];
  const p1 = sorted[i];
  const p2 = sorted[i + 1];
  const p3 = sorted[Math.min(sorted.length - 1, i + 2)];

  const span = Math.max(1e-4, p2.x - p1.x);
  const t = (x - p1.x) / span;
  const t2 = t * t;
  const t3 = t2 * t;

  const y =
    0.5 *
    (2 * p1.y +
      (-p0.y + p2.y) * t +
      (2 * p0.y - 5 * p1.y + 4 * p2.y - p3.y) * t2 +
      (-p0.y + 3 * p1.y - 3 * p2.y + p3.y) * t3);
  return Math.max(0, Math.min(1, y));
}

function build256CurveLutFromPoints(points) {
  const isIdentity = points.every((pt) => Math.abs(pt.x - pt.y) < 0.004);
  if (isIdentity) return [];
  const lut = new Array(256);
  for (let i = 0; i < 256; i++) {
    const x = i / 255;
    const y = evaluateCurveSpline(points, x);
    lut[i] = Math.max(0, Math.min(255, Math.round(y * 255)));
  }
  return lut;
}

function renderToneCurveCanvas() {
  if (!curveCanvas) return;
  const ctx = curveCanvas.getContext('2d');
  const w = curveCanvas.width;
  const h = curveCanvas.height;
  ctx.clearRect(0, 0, w, h);

  ctx.strokeStyle = 'rgba(255, 255, 255, 0.07)';
  ctx.lineWidth = 1;
  for (let g = 1; g <= 3; g++) {
    const gx = (w * g) / 4;
    const gy = (h * g) / 4;
    ctx.beginPath();
    ctx.moveTo(gx, 0);
    ctx.lineTo(gx, h);
    ctx.moveTo(0, gy);
    ctx.lineTo(w, gy);
    ctx.stroke();
  }

  ctx.strokeStyle = 'rgba(255, 255, 255, 0.2)';
  ctx.setLineDash([4, 4]);
  ctx.beginPath();
  ctx.moveTo(0, h);
  ctx.lineTo(w, 0);
  ctx.stroke();
  ctx.setLineDash([]);

  const photo = getActivePhoto();
  if (!photo) return;
  const pts = getPhotoCurvePoints(photo);

  ctx.strokeStyle = '#f8d442';
  ctx.lineWidth = 2;
  ctx.beginPath();
  for (let px = 0; px <= w; px += 2) {
    const nx = px / w;
    const ny = evaluateCurveSpline(pts, nx);
    const py = (1 - ny) * h;
    if (px === 0) ctx.moveTo(px, py);
    else ctx.lineTo(px, py);
  }
  ctx.stroke();

  pts.forEach((pt, idx) => {
    const cx = pt.x * w;
    const cy = (1 - pt.y) * h;
    ctx.fillStyle = idx === state.draggingCurveIdx ? '#ffffff' : '#f8d442';
    ctx.strokeStyle = '#0c0d10';
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.arc(cx, cy, 4.5, 0, Math.PI * 2);
    ctx.fill();
    ctx.stroke();
  });
}

// =========================================================================
// EVOTO-GRADE CROP, ASPECT RATIO, ROTATION & AI POSITIONING ENGINE
// =========================================================================
function clamp01(v) {
  return Math.max(0, Math.min(1, v));
}

function getPhotoDimensions(photo) {
  return {
    imgW: Math.max(1, photo?.width || afterCanvas?.width || 1000),
    imgH: Math.max(1, photo?.height || afterCanvas?.height || 1000),
  };
}

function getCropLockedRatio(photo) {
  const p = photo?.params;
  if (!p) return 0;
  if (p.cropAspectW > 0.01 && p.cropAspectH > 0.01) {
    return p.cropAspectW / p.cropAspectH;
  }
  return 0;
}

function hasActiveCropRect(p) {
  if (!p) return false;
  return p.cropX > 0.001 || p.cropY > 0.001 || p.cropW < 0.999 || p.cropH < 0.999;
}

function isViewingCroppedViewport(photo) {
  if (!photo) return false;
  if (!hasActiveCropRect(photo.params)) return false;
  return state.activeModule !== 'mod-crop' || state.cropPreviewMode;
}

function fitCropBoxToAspect(photo, targetRatio, anchor = null) {
  const p = photo.params;
  if (!targetRatio || targetRatio <= 0) {
    p.cropX = 0;
    p.cropY = 0;
    p.cropW = 1;
    p.cropH = 1;
    return;
  }

  const { imgW, imgH } = getPhotoDimensions(photo);
  const srcRatio = imgW / imgH;
  let nw, nh;
  if (srcRatio > targetRatio) {
    nh = 1;
    nw = clamp01((imgH * targetRatio) / imgW);
  } else {
    nw = 1;
    nh = clamp01(imgW / (imgH * targetRatio));
  }

  const f = p.face;
  const hasFace = Boolean(f && f.faceRy > 0.01);
  const cx = anchor ? anchor.x : hasFace ? f.centerX : 0.5;
  const cy = anchor
    ? anchor.y
    : hasFace
    ? Math.max(0.25, Math.min(0.75, f.centerY + f.faceRy * 0.35))
    : 0.5;

  p.cropW = Math.max(0.04, nw);
  p.cropH = Math.max(0.04, nh);
  p.cropX = Math.max(0, Math.min(1 - p.cropW, cx - p.cropW * 0.5));
  p.cropY = Math.max(0, Math.min(1 - p.cropH, cy - p.cropH * 0.44));
}

function getSafeRotatedInsetScale(deg, imgAspect) {
  const a = Math.abs(deg || 0);
  if (a < 0.05) return 1;
  const rad = (a * Math.PI) / 180;
  const c = Math.cos(rad);
  const s = Math.sin(rad);
  const k = Math.max(imgAspect, 1 / imgAspect);
  return Math.max(0.35, Math.min(1, 1 / (c + s * k)));
}

function constrainCropBoxInsideRotatedBounds(photo, deg) {
  const p = photo.params;
  const { imgW, imgH } = getPhotoDimensions(photo);
  const safeScale = getSafeRotatedInsetScale(deg, imgW / imgH);
  if (safeScale >= 0.999) return;

  const minBound = (1 - safeScale) * 0.5;
  const maxSpan = safeScale;
  const curRatio = getCropLockedRatio(photo);

  let nw = Math.min(p.cropW, maxSpan);
  let nh = Math.min(p.cropH, maxSpan);
  if (curRatio > 0) {
    const normRatio = curRatio * (imgH / imgW);
    nh = nw / normRatio;
    if (nh > maxSpan) {
      nh = maxSpan;
      nw = nh * normRatio;
    }
    if (nw > maxSpan) {
      nw = maxSpan;
      nh = nw / normRatio;
    }
  }

  const midX = p.cropX + p.cropW * 0.5;
  const midY = p.cropY + p.cropH * 0.5;
  p.cropW = Math.max(0.04, nw);
  p.cropH = Math.max(0.04, nh);
  p.cropX = Math.max(minBound, Math.min(1 - minBound - p.cropW, midX - p.cropW * 0.5));
  p.cropY = Math.max(minBound, Math.min(1 - minBound - p.cropH, midY - p.cropH * 0.5));
}

async function measureSubjectBounds(photo) {
  if (photo._subjectBoundsCache) return photo._subjectBoundsCache;
  const f = photo.params.face || {};
  const fallback = {
    minX: clamp01((f.centerX ?? 0.5) - (f.faceRx ?? 0.18) * 2.4),
    maxX: clamp01((f.centerX ?? 0.5) + (f.faceRx ?? 0.18) * 2.4),
    minY: clamp01((f.centerY ?? 0.42) - (f.faceRy ?? 0.24) * 1.65),
    maxY: 0.98,
    centerX: f.centerX ?? 0.5,
  };

  const activeMaskUrl = getEffectiveSceneMask(photo);
  if (!activeMaskUrl) {
    photo._subjectBoundsCache = fallback;
    return fallback;
  }

  try {
    const maskImg = await loadImageElement(activeMaskUrl);
    const sz = 96;
    const c = document.createElement('canvas');
    c.width = sz;
    c.height = sz;
    const ctx = c.getContext('2d');
    ctx.drawImage(maskImg, 0, 0, sz, sz);
    const data = ctx.getImageData(0, 0, sz, sz).data;
    let minX = sz,
      minY = sz,
      maxX = -1,
      maxY = -1;
    let sumX = 0,
      count = 0;

    for (let y = 0; y < sz; y++) {
      for (let x = 0; x < sz; x++) {
        const fg = data[(y * sz + x) * 4];
        if (fg > 110) {
          if (x < minX) minX = x;
          if (x > maxX) maxX = x;
          if (y < minY) minY = y;
          if (y > maxY) maxY = y;
          sumX += x;
          count++;
        }
      }
    }

    if (count > 20 && maxX - minX >= 4 && maxY - minY >= 6) {
      const res = {
        minX: minX / (sz - 1),
        maxX: maxX / (sz - 1),
        minY: minY / (sz - 1),
        maxY: maxY / (sz - 1),
        centerX: sumX / count / (sz - 1),
      };
      photo._subjectBoundsCache = res;
      return res;
    }
  } catch (_) {}

  photo._subjectBoundsCache = fallback;
  return fallback;
}

function getEyeTiltDegrees(photo) {
  const { imgW, imgH } = getPhotoDimensions(photo);
  const f = photo?.params?.face;
  if (!f || !photo.landmarksDetected) {
    return 0;
  }
  const lEye = { x: f.leftEyeX, y: f.leftEyeY };
  const rEye = { x: f.rightEyeX, y: f.rightEyeY };
  const e1 = lEye.x <= rEye.x ? lEye : rEye;
  const e2 = lEye.x <= rEye.x ? rEye : lEye;
  const dx = (e2.x - e1.x) * imgW;
  const dy = (e2.y - e1.y) * imgH;
  if (Math.hypot(dx, dy) < 8) return 0;
  const deg = (-Math.atan2(dy, dx) * 180) / Math.PI;
  return Math.max(-45, Math.min(45, Math.round(deg * 10) / 10));
}

async function applyCropAlignAI(photo, mode) {
  if (!photo) return;
  const p = photo.params;
  const { imgW, imgH } = getPhotoDimensions(photo);
  const bounds = await measureSubjectBounds(photo);
  const f = p.face || {};
  const lockedRatio = getCropLockedRatio(photo);
  const effectiveRatio = lockedRatio > 0 ? lockedRatio : p.cropW < 0.999 || p.cropH < 0.999 ? (p.cropW * imgW) / (p.cropH * imgH) : imgW / imgH;
  const normRatio = effectiveRatio * (imgH / imgW);
  let needReprocess = false;

  if (mode === 'auto_straighten') {
    const tilt = getEyeTiltDegrees(photo);
    p.rotateAngle = tilt;
    constrainCropBoxInsideRotatedBounds(photo, tilt);
    needReprocess = true;
    renderTimeLabel.textContent =
      Math.abs(tilt) > 0.05
        ? `AI đã cân chỉnh góc nghiêng ${tilt.toFixed(2)}° ✓`
        : 'Đường chân trời & khuôn mặt đã cân bằng ✓';
  } else if (mode === 'eye') {
    p.cropAlignEye = true;
    p.cropAlignVertical = false;
    const tilt = getEyeTiltDegrees(photo);
    if (Math.abs(tilt - p.rotateAngle) > 0.05) {
      p.rotateAngle = tilt;
      needReprocess = true;
    }

    const eyeMidX = ((f.leftEyeX ?? 0.43) + (f.rightEyeX ?? 0.57)) * 0.5;
    const eyeMidY = ((f.leftEyeY ?? 0.38) + (f.rightEyeY ?? 0.38)) * 0.5;

    // Frame with eye-line near upper-third (~34%) while keeping crop centered around eyeMidX
    const safeScale = getSafeRotatedInsetScale(p.rotateAngle, imgW / imgH);
    const minB = (1 - safeScale) * 0.5;
    const maxDistX = Math.max(0.18, Math.min(eyeMidX - minB, 1 - minB - eyeMidX));
    let cw = Math.min(safeScale, maxDistX * 2, lockedRatio > 0 ? p.cropW : safeScale * 0.92);
    let ch = cw / normRatio;
    if (ch > safeScale) {
      ch = safeScale;
      cw = ch * normRatio;
    }
    cw = Math.max(0.2, Math.min(safeScale, cw));
    ch = Math.max(0.2, Math.min(safeScale, ch));

    p.cropW = cw;
    p.cropH = ch;
    p.cropX = Math.max(minB, Math.min(1 - minB - cw, eyeMidX - cw * 0.5));
    p.cropY = Math.max(minB, Math.min(1 - minB - ch, eyeMidY - ch * 0.34));
    renderTimeLabel.textContent = 'AI đã căn ngang theo tầm mắt ✓';
  } else if (mode === 'vertical' || mode === 'subject') {
    if (mode === 'vertical') {
      p.cropAlignVertical = true;
      p.cropAlignEye = false;
      // Compute slight vertical midline roll correction from forehead -> chin if 478 landmarks present
      if (photo.landmarksDetected && f.chinY - f.foreheadY > 0.08) {
        const vx = (f.chinX - f.foreheadX) * imgW;
        const vy = (f.chinY - f.foreheadY) * imgH;
        const vTilt = (-Math.atan2(vx, vy) * 180) / Math.PI;
        if (Math.abs(vTilt) <= 18) {
          p.rotateAngle = Math.round(vTilt * 10) / 10;
          needReprocess = true;
        }
      }
    }

    const subW = Math.max(0.15, bounds.maxX - bounds.minX);
    const subH = Math.max(0.2, bounds.maxY - bounds.minY);
    const padX = Math.max(0.06, subW * 0.18);
    const padTop = Math.max(0.05, subH * 0.08);
    const safeScale = getSafeRotatedInsetScale(p.rotateAngle, imgW / imgH);
    const minB = (1 - safeScale) * 0.5;

    let desiredH = Math.min(safeScale, subH + padTop * 1.6);
    let desiredW = desiredH * normRatio;
    if (desiredW < subW + padX * 2) {
      desiredW = Math.min(safeScale, subW + padX * 2);
      desiredH = desiredW / normRatio;
    }
    if (desiredH > safeScale) {
      desiredH = safeScale;
      desiredW = desiredH * normRatio;
    }
    if (desiredW > safeScale) {
      desiredW = safeScale;
      desiredH = desiredW / normRatio;
    }

    const subjCenterX = f.faceRx > 0.01 ? f.centerX * 0.65 + bounds.centerX * 0.35 : bounds.centerX;
    p.cropW = Math.max(0.18, desiredW);
    p.cropH = Math.max(0.18, desiredH);
    p.cropX = Math.max(minB, Math.min(1 - minB - p.cropW, subjCenterX - p.cropW * 0.5));
    const headTop = f.faceRy > 0.01 ? Math.min(bounds.minY, f.centerY - f.faceRy * 1.55) : bounds.minY;
    p.cropY = Math.max(minB, Math.min(1 - minB - p.cropH, headTop - padTop * 0.7));
    renderTimeLabel.textContent =
      mode === 'vertical' ? 'AI đã căn chỉnh dọc theo chủ thể ✓' : 'AI đã định vị đối tượng ✓';
  } else if (mode === 'headshot' || mode === 'waist' || mode === 'full') {
    const fy = f.faceRy || 0.16;
    const cx = f.centerX || bounds.centerX || 0.5;
    const headTop = Math.max(0, (f.centerY || 0.35) - fy * 1.65);
    const targetSpanH =
      mode === 'headshot' ? fy * 4.4 : mode === 'waist' ? fy * 7.6 : Math.max(0.75, bounds.maxY - headTop + 0.06);
    let ch = Math.max(0.22, Math.min(1, targetSpanH));
    let cw = ch * normRatio;
    if (cw > 1) {
      cw = 1;
      ch = cw / normRatio;
    }
    p.cropW = cw;
    p.cropH = ch;
    p.cropX = Math.max(0, Math.min(1 - cw, cx - cw * 0.5));
    p.cropY = Math.max(0, Math.min(1 - ch, headTop - ch * 0.08));
  }

  syncSlidersFromParams();
  updateCropControlsUI(photo);
  if (needReprocess) {
    await processActivePhoto(false);
  } else {
    await renderViewport();
  }
  commitHistory(photo);
}

function updateCropControlsUI(photo) {
  if (!photo) return;
  const p = photo.params;
  const { imgW, imgH } = getPhotoDimensions(photo);

  const labelMap = {
    original: 'Kích thước gốc',
    as_shot: 'Cài đặt ảnh gốc',
    fixed: 'Tỉ lệ cố định',
    custom_ratio: 'Kích thước tự chọn',
    custom_dim: 'Dài*Rộng*Độ phân giải',
    id_photo: 'Ảnh Thẻ',
    id_2x3: 'Ảnh Thẻ 2×3 cm',
    id_3x4: 'Ảnh Thẻ 3×4 cm',
    id_4x6: 'Ảnh Thẻ 4×6 cm',
    id_35x45: 'Ảnh Thẻ Visa 3.5×4.5',
    id_5x5: 'Ảnh Thẻ 5×5 cm',
    id_33x48: 'Ảnh Thẻ 3.3×4.8 cm',
    '1:1': '1 : 1',
    '4:5': '4 : 5',
    '5:7': '5 : 7',
    '2:3': '2 : 3',
    '16:9': '16 : 9',
    '1.91:1': '1.91 : 1',
    '2:1': '2 : 1',
    '3:1': '3 : 1',
    '4:3': '4 : 3',
  };

  const preset = p.cropPreset || 'original';
  const lbl = document.getElementById('crop-ratio-label');
  if (lbl) lbl.textContent = labelMap[preset] || preset;

  document.querySelectorAll('.crop-ratio-opt').forEach((opt) => {
    const op = opt.dataset.preset;
    const isIdActive = op === 'id_photo' && preset.startsWith('id_');
    const act = op === preset || isIdActive;
    opt.classList.toggle('active', act);
    const ck = opt.querySelector('.opt-check');
    if (ck) ck.textContent = act ? '✓' : '';
  });

  document.getElementById('crop-custom-ratio-box')?.classList.toggle('hidden', preset !== 'custom_ratio');
  document.getElementById('crop-custom-inputs')?.classList.toggle('hidden', preset !== 'custom_dim');
  document.getElementById('crop-id-presets-box')?.classList.toggle('hidden', !preset.startsWith('id_'));

  document.querySelectorAll('.crop-id-chip').forEach((chip) => {
    chip.classList.toggle('active', chip.dataset.idpreset === preset);
  });

  const gridChk = document.getElementById('crop-show-grid');
  if (gridChk) gridChk.checked = p.cropShowGrid !== false;

  const aiSw = document.getElementById('toggle-crop-ai-subject');
  if (aiSw) {
    aiSw.classList.toggle('on', Boolean(p.cropAiSubject));
    aiSw.setAttribute('aria-checked', String(Boolean(p.cropAiSubject)));
  }

  document.getElementById('btn-crop-align-v')?.classList.toggle('active', Boolean(p.cropAlignVertical));
  document.getElementById('btn-crop-align-eye')?.classList.toggle('active', Boolean(p.cropAlignEye));
  document.getElementById('btn-flip-h')?.classList.toggle('active', Boolean(p.flipHorizontal));
  document.getElementById('btn-flip-v')?.classList.toggle('active', Boolean(p.flipVertical));
  document.getElementById('btn-crop-preview-toggle')?.classList.toggle('active', Boolean(state.cropPreviewMode));
  const applyCropBtn = document.getElementById('btn-crop-apply');
  if (applyCropBtn) {
    const isAppliedView = Boolean(state.cropPreviewMode);
    applyCropBtn.classList.toggle('active', isAppliedView);
    applyCropBtn.textContent = isAppliedView ? 'Đang xem cắt ✓' : 'Áp dụng cắt';
  }

  const dpi = Math.max(72, Math.min(1200, Math.round(p.cropDpi || 300)));
  const unit = p.cropDimUnit || 'px';
  const dpiIn = document.getElementById('crop-dpi-in');
  if (dpiIn && document.activeElement !== dpiIn) dpiIn.value = String(dpi);
  const unitSel = document.getElementById('crop-dim-unit');
  if (unitSel && document.activeElement !== unitSel) unitSel.value = unit;

  const rwIn = document.getElementById('crop-ratio-w-in');
  const rhIn = document.getElementById('crop-ratio-h-in');
  if (preset === 'custom_ratio') {
    if (rwIn && document.activeElement !== rwIn && p.cropAspectW > 0) rwIn.value = String(p.cropAspectW);
    if (rhIn && document.activeElement !== rhIn && p.cropAspectH > 0) rhIn.value = String(p.cropAspectH);
  }

  const cwIn = document.getElementById('crop-custom-w-in');
  const chIn = document.getElementById('crop-custom-h-in');
  if (preset === 'custom_dim') {
    const pxToUnit = (px) => {
      if (!px || px <= 0) return '';
      if (unit === 'cm') return String(Number(((px / dpi) * 2.54).toFixed(2)));
      if (unit === 'mm') return String(Number(((px / dpi) * 25.4).toFixed(1)));
      return String(Math.round(px));
    };
    if (cwIn && document.activeElement !== cwIn) cwIn.value = p.cropCustomW > 0 ? pxToUnit(p.cropCustomW) : '';
    if (chIn && document.activeElement !== chIn) chIn.value = p.cropCustomH > 0 ? pxToUnit(p.cropCustomH) : '';
  }

  const idReadout = document.getElementById('crop-id-readout');
  if (idReadout && preset.startsWith('id_')) {
    const activeChip = document.querySelector(`.crop-id-chip[data-idpreset="${preset}"]`);
    const cmw = activeChip ? activeChip.dataset.cmw : '';
    const cmh = activeChip ? activeChip.dataset.cmh : '';
    idReadout.textContent =
      p.cropCustomW > 0 && p.cropCustomH > 0 && cmw && cmh
        ? `${dpi} DPI · ${cmw}×${cmh} cm (${p.cropCustomW} × ${p.cropCustomH} px)`
        : `${dpi} DPI · Tự động khóa đúng tỉ lệ ảnh thẻ`;
  }

  const cw = Math.max(1, Math.round((p.cropW ?? 1) * imgW));
  const ch = Math.max(1, Math.round((p.cropH ?? 1) * imgH));
  const readout = document.getElementById('crop-box-readout');
  if (readout) {
    if (p.cropCustomW > 0 && p.cropCustomH > 0) {
      readout.textContent = `${cw}×${ch} → ${p.cropCustomW}×${p.cropCustomH} px`;
    } else if (hasActiveCropRect(p)) {
      readout.textContent = `Khung: ${cw} × ${ch} px`;
    } else {
      readout.textContent = `Gốc: ${imgW} × ${imgH} px`;
    }
  }

  const dimHint = document.getElementById('crop-dim-computed');
  if (dimHint && preset === 'custom_dim') {
    if (p.cropCustomW > 0 && p.cropCustomH > 0) {
      dimHint.textContent = `Xuất ra: ${p.cropCustomW} × ${p.cropCustomH} px (${dpi} DPI)`;
    } else {
      dimHint.textContent = 'Nhập Rộng & Cao để tự khóa khung & xuất đúng kích thước';
    }
  }
}

let _renderViewportSeq = 0;

async function renderViewport() {
  const photo = getActivePhoto();
  if (!photo) return;

  const seq = ++_renderViewportSeq;
  const [beforeImg, afterImg] = await Promise.all([
    loadImageElement(photo.originalDataUrl),
    loadImageElement(photo.processedDataUrl || photo.originalDataUrl),
  ]);
  if (seq !== _renderViewportSeq) return;

  const bw = beforeImg.naturalWidth || photo.width;
  const bh = beforeImg.naturalHeight || photo.height;
  const aw = afterImg.naturalWidth || bw;
  const ah = afterImg.naturalHeight || bh;
  const p = photo.params;
  const showCropped = isViewingCroppedViewport(photo);

  // Logical image dimensions (in image pixels)
  const imgW = showCropped ? Math.max(1, Math.round(p.cropW * bw)) : bw;
  const imgH = showCropped ? Math.max(1, Math.round(p.cropH * bh)) : bh;

  const bsx = showCropped ? Math.max(0, Math.min(bw - 1, p.cropX * bw)) : 0;
  const bsy = showCropped ? Math.max(0, Math.min(bh - 1, p.cropY * bh)) : 0;
  const bsw = showCropped ? Math.max(1, Math.min(bw - bsx, p.cropW * bw)) : bw;
  const bsh = showCropped ? Math.max(1, Math.min(bh - bsy, p.cropH * bh)) : bh;

  const asx = showCropped ? Math.max(0, Math.min(aw - 1, p.cropX * aw)) : 0;
  const asy = showCropped ? Math.max(0, Math.min(ah - 1, p.cropY * ah)) : 0;
  const asw = showCropped ? Math.max(1, Math.min(aw - asx, p.cropW * aw)) : aw;
  const ash = showCropped ? Math.max(1, Math.min(ah - asy, p.cropH * ah)) : ah;

  // --- DPR-aware canvas sizing (zoom handled separately via CSS transform on canvasWrap) ---
  // Measure exact content box of #canvas-viewport so wrapper & all 3 canvases share one exact size.
  const dpr = Math.min(window.devicePixelRatio || 1, 3); // cap at 3× to avoid OOM
  const viewport = document.getElementById('canvas-viewport');
  const vpStyle = viewport ? getComputedStyle(viewport) : null;
  const padX = vpStyle ? (parseFloat(vpStyle.paddingLeft) || 0) + (parseFloat(vpStyle.paddingRight) || 0) : 32;
  const padY = vpStyle ? (parseFloat(vpStyle.paddingTop) || 0) + (parseFloat(vpStyle.paddingBottom) || 0) : 24;
  const rawVpW = viewport ? viewport.clientWidth - padX : window.innerWidth - 32;
  const rawVpH = viewport ? viewport.clientHeight - padY : window.innerHeight - 220;
  const maxCssW = Math.max(64, Math.floor(rawVpW > 0 ? rawVpW : window.innerWidth - 32));
  const maxCssH = Math.max(64, Math.floor(rawVpH > 0 ? rawVpH : window.innerHeight - 220));
  const fitScale = Math.min(1, maxCssW / imgW, maxCssH / imgH);
  const cssW = Math.max(1, Math.round(imgW * fitScale));
  const cssH = Math.max(1, Math.round(imgH * fitScale));
  // Physical pixels = CSS pixels × DPR × zoom boost (capped to source & 3600px) → crisp when zoomed in
  const zoomBoost = state.zoomScale > 1.05 ? Math.min(state.zoomScale, 3.5) : 1;
  const targetW = Math.round(cssW * dpr * zoomBoost);
  const targetH = Math.round(cssH * dpr * zoomBoost);
  const maxBacking = Math.min(3600, Math.max(imgW, aw, bw));
  const capScale = Math.min(1, maxBacking / Math.max(targetW, targetH, 1));
  const w = Math.max(Math.round(cssW * dpr), Math.round(targetW * capScale));
  const h = Math.max(Math.round(cssH * dpr), Math.round(targetH * capScale));

  afterCanvas.width  = w;  afterCanvas.height  = h;
  beforeCanvas.width = w;  beforeCanvas.height = h;
  overlayCanvas.width = w; overlayCanvas.height = h;

  // Lock #canvas-wrapper to exact cssW × cssH; canvases fill 100% via CSS position:absolute; inset:0
  if (canvasWrap) {
    canvasWrap.style.width = `${cssW}px`;
    canvasWrap.style.height = `${cssH}px`;
  }
  afterCanvas.style.width = '';
  afterCanvas.style.height = '';
  beforeCanvas.style.width = '';
  beforeCanvas.style.height = '';
  overlayCanvas.style.width = '';
  overlayCanvas.style.height = '';

  canvasWrap?.classList.toggle(
    'has-transparency',
    p.bgReplaceMode === 'transparent' && !state.holdingBefore
  );

  // Draw cropped afterImg region into physical-pixel canvas without stretching
  const afterCtx = afterCanvas.getContext('2d');
  afterCtx.imageSmoothingEnabled = true;
  afterCtx.imageSmoothingQuality = 'high';
  afterCtx.drawImage(afterImg, asx, asy, asw, ash, 0, 0, w, h);

  const beforeCtx = beforeCanvas.getContext('2d');
  beforeCtx.imageSmoothingEnabled = true;
  beforeCtx.imageSmoothingQuality = 'high';
  beforeCtx.clearRect(0, 0, w, h);

  if (state.holdingBefore) {
    beforeCtx.drawImage(beforeImg, bsx, bsy, bsw, bsh, 0, 0, w, h);
    splitLine.classList.add('hidden');
  } else if (state.splitEnabled) {
    const splitX = Math.round(w * state.splitRatio);
    beforeCtx.save();
    beforeCtx.beginPath();
    beforeCtx.rect(0, 0, splitX, h);
    beforeCtx.clip();
    beforeCtx.drawImage(beforeImg, bsx, bsy, bsw, bsh, 0, 0, w, h);
    beforeCtx.restore();

    splitLine.classList.remove('hidden');
    splitLine.style.left = `${(splitX / w) * 100}%`;
  } else {
    splitLine.classList.add('hidden');
  }

  filenameLabel.textContent = photo.name;
  const natW = Math.round((showCropped ? p.cropW : 1) * (photo.width || aw));
  const natH = Math.round((showCropped ? p.cropH : 1) * (photo.height || ah));
  if (p.cropCustomW > 0 && p.cropCustomH > 0) {
    resolutionLabel.textContent = `${natW} × ${natH} px → ${p.cropCustomW} × ${p.cropCustomH} px`;
  } else {
    resolutionLabel.textContent = `${natW} × ${natH} px`;
  }
  drawOverlayGuides(w, h, photo);
  updateHistogram(afterCtx, w, h);
  renderToneCurveCanvas();
}

function strokeMeshContour(ctx, pts, indices, w, h, color, lineWidth = 2, closed = true, mapPt = null) {
  if (!pts || indices.length < 2) return;
  ctx.save();
  ctx.strokeStyle = color;
  ctx.lineWidth = lineWidth;
  ctx.beginPath();
  for (let i = 0; i < indices.length; i++) {
    const raw = pts[indices[i]];
    if (!raw) continue;
    const p = mapPt ? mapPt(raw.x, raw.y) : raw;
    const x = p.x * w;
    const y = p.y * h;
    if (i === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  }
  if (closed) ctx.closePath();
  ctx.stroke();
  ctx.restore();
}

function drawCropOverlay(ctx, w, h, photo) {
  const p = photo.params;
  const cx = Math.round((p.cropX ?? 0) * w);
  const cy = Math.round((p.cropY ?? 0) * h);
  const cw = Math.max(4, Math.round((p.cropW ?? 1) * w));
  const ch = Math.max(4, Math.round((p.cropH ?? 1) * h));
  const { imgW, imgH } = getPhotoDimensions(photo);

  ctx.save();

  // Dim outside crop rectangle seamlessly with even-odd rule
  ctx.fillStyle = 'rgba(8, 9, 12, 0.64)';
  ctx.beginPath();
  ctx.rect(0, 0, w, h);
  ctx.rect(cx, cy, cw, ch);
  ctx.fill('evenodd');

  // Rule-of-thirds grid inside crop rect (and fine 9x9 alignment grid when rotating)
  if (state.draggingRotateAngle) {
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.28)';
    ctx.lineWidth = Math.max(1, w / 900);
    for (let i = 1; i < 9; i++) {
      const gx = cx + (cw * i) / 9;
      const gy = cy + (ch * i) / 9;
      ctx.beginPath();
      ctx.moveTo(gx, cy);
      ctx.lineTo(gx, cy + ch);
      ctx.moveTo(cx, gy);
      ctx.lineTo(cx + cw, gy);
      ctx.stroke();
    }
  } else if (p.cropShowGrid !== false || state.draggingCrop) {
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.42)';
    ctx.lineWidth = Math.max(1, w / 900);
    for (let i = 1; i <= 2; i++) {
      const gx = cx + (cw * i) / 3;
      const gy = cy + (ch * i) / 3;
      ctx.beginPath();
      ctx.moveTo(gx, cy);
      ctx.lineTo(gx, cy + ch);
      ctx.moveTo(cx, gy);
      ctx.lineTo(cx + cw, gy);
      ctx.stroke();
    }
  }

  // Main crop border
  ctx.strokeStyle = 'rgba(255, 255, 255, 0.92)';
  ctx.lineWidth = Math.max(1.5, w / 650);
  ctx.strokeRect(cx + 0.5, cy + 0.5, cw - 1, ch - 1);

  // Corner L-brackets & edge mid-bars (Evoto / Lightroom style)
  const scaleFactor = Math.max(1, w / 720);
  const arm = Math.min(20 * scaleFactor, cw * 0.22, ch * 0.22);
  const barLen = Math.min(22 * scaleFactor, cw * 0.18, ch * 0.18);
  ctx.strokeStyle = '#ffffff';
  ctx.lineWidth = Math.max(2.5, 3 * scaleFactor);
  ctx.lineCap = 'square';

  const drawCorner = (x0, y0, sx, sy) => {
    ctx.beginPath();
    ctx.moveTo(x0 + sx * arm, y0);
    ctx.lineTo(x0, y0);
    ctx.lineTo(x0, y0 + sy * arm);
    ctx.stroke();
  };
  drawCorner(cx, cy, 1, 1);
  drawCorner(cx + cw, cy, -1, 1);
  drawCorner(cx, cy + ch, 1, -1);
  drawCorner(cx + cw, cy + ch, -1, -1);

  // Edge center grab bars
  const mx = cx + cw * 0.5;
  const my = cy + ch * 0.5;
  ctx.beginPath();
  ctx.moveTo(mx - barLen * 0.5, cy);
  ctx.lineTo(mx + barLen * 0.5, cy);
  ctx.moveTo(mx - barLen * 0.5, cy + ch);
  ctx.lineTo(mx + barLen * 0.5, cy + ch);
  ctx.moveTo(cx, my - barLen * 0.5);
  ctx.lineTo(cx, my + barLen * 0.5);
  ctx.moveTo(cx + cw, my - barLen * 0.5);
  ctx.lineTo(cx + cw, my + barLen * 0.5);
  ctx.stroke();

  // Compact dimension badge inside top-left of crop frame
  const outW = p.cropCustomW > 0 ? p.cropCustomW : Math.max(1, Math.round((p.cropW ?? 1) * imgW));
  const outH = p.cropCustomH > 0 ? p.cropCustomH : Math.max(1, Math.round((p.cropH ?? 1) * imgH));
  const tagText = state.draggingRotateAngle
    ? `${Number(p.rotateAngle || 0).toFixed(2)}°`
    : `${outW} × ${outH} px`;
  const fontSize = Math.round(11 * scaleFactor);
  ctx.font = `600 ${fontSize}px -apple-system, BlinkMacSystemFont, sans-serif`;
  const padX = Math.round(7 * scaleFactor);
  const tagW = Math.ceil(ctx.measureText(tagText).width) + padX * 2;
  const tagH = Math.round(20 * scaleFactor);
  const tx = Math.min(w - tagW - 4, cx + 6 * scaleFactor);
  const ty = Math.min(h - tagH - 4, cy + 6 * scaleFactor);
  ctx.fillStyle = 'rgba(12, 13, 16, 0.84)';
  ctx.fillRect(tx, ty, tagW, tagH);
  ctx.fillStyle = '#f8d442';
  ctx.textBaseline = 'middle';
  ctx.fillText(tagText, tx + padX, ty + tagH * 0.5);

  ctx.restore();
}

function ensureHairGuideOverlayCanvas(photo, onReady) {
  if (!photo || !photo.maskBase64) return null;
  if (photo._hairGuideCanvas && photo._hairGuideMaskRef === photo.maskBase64) {
    return photo._hairGuideCanvas;
  }
  if (photo._hairGuideLoading) return null;
  photo._hairGuideLoading = true;
  const srcRef = photo.maskBase64;
  loadImageElement(srcRef)
    .then((img) => {
      const mw = img.naturalWidth || img.width || 512;
      const mh = img.naturalHeight || img.height || 512;
      const tmp = document.createElement('canvas');
      tmp.width = mw;
      tmp.height = mh;
      const tCtx = tmp.getContext('2d');
      tCtx.drawImage(img, 0, 0, mw, mh);
      const srcData = tCtx.getImageData(0, 0, mw, mh).data;

      const out = document.createElement('canvas');
      out.width = mw;
      out.height = mh;
      const oCtx = out.getContext('2d');
      const outImg = oCtx.createImageData(mw, mh);
      const oPx = outImg.data;

      const isHairAt = (x, y) => {
        const cx = Math.max(0, Math.min(mw - 1, x));
        const cy = Math.max(0, Math.min(mh - 1, y));
        const i = (cy * mw + cx) * 4;
        return srcData[i + 2] >= 90 && srcData[i] < 140;
      };

      for (let y = 0; y < mh; y++) {
        for (let x = 0; x < mw; x++) {
          const i = (y * mw + x) * 4;
          const hv = srcData[i + 2];
          const sv = srcData[i];
          if (hv < 72 || sv > 140) continue;
          const isEdge =
            !isHairAt(x - 2, y) ||
            !isHairAt(x + 2, y) ||
            !isHairAt(x, y - 2) ||
            !isHairAt(x, y + 2);
          if (isEdge && hv >= 90) {
            oPx[i] = 192;
            oPx[i + 1] = 132;
            oPx[i + 2] = 252;
            oPx[i + 3] = 215;
          } else {
            oPx[i] = 168;
            oPx[i + 1] = 85;
            oPx[i + 2] = 247;
            oPx[i + 3] = Math.round(((hv - 72) / 183) * 78);
          }
        }
      }
      oCtx.putImageData(outImg, 0, 0);
      photo._hairGuideCanvas = out;
      photo._hairGuideMaskRef = srcRef;
      photo._hairGuideLoading = false;
      onReady?.();
    })
    .catch(() => {
      photo._hairGuideLoading = false;
    });
  return null;
}

function drawOverlayGuides(w, h, photo) {
  const params = photo.params;
  const ctx = overlayCanvas.getContext('2d');
  ctx.clearRect(0, 0, w, h);

  const croppedView = isViewingCroppedViewport(photo);
  const mapPt = croppedView
    ? (nx, ny) => ({
        x: (nx - params.cropX) / Math.max(0.01, params.cropW),
        y: (ny - params.cropY) / Math.max(0.01, params.cropH),
      })
    : (nx, ny) => ({ x: nx, y: ny });

  if (state.spotHealMode && params.blemishSpots?.length) {
    for (const spot of params.blemishSpots) {
      const mp = mapPt(spot.x, spot.y);
      const cx = mp.x * w;
      const cy = mp.y * h;
      const r = (spot.radius / (croppedView ? Math.max(0.01, params.cropW) : 1)) * Math.min(w, h);
      ctx.strokeStyle = '#f8d442';
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.arc(cx, cy, r, 0, Math.PI * 2);
      ctx.stroke();
    }
  }

  if (state.showFaceGuides && params.face) {
    const f = params.face;

    // 0. AI Hair Segmentation region & contour overlay
    const hairOverlay = ensureHairGuideOverlayCanvas(photo, () => {
      if (state.showFaceGuides && photo === getActivePhoto()) {
        drawOverlayGuides(w, h, photo);
      }
    });
    if (hairOverlay) {
      const mw = hairOverlay.width;
      const mh = hairOverlay.height;
      const sx = croppedView ? params.cropX * mw : 0;
      const sy = croppedView ? params.cropY * mh : 0;
      const sw = croppedView ? Math.max(1, params.cropW * mw) : mw;
      const sh = croppedView ? Math.max(1, params.cropH * mh) : mh;
      ctx.drawImage(hairOverlay, sx, sy, sw, sh, 0, 0, w, h);
    }

    if (photo.hairBounds?.detected) {
      const hb = photo.hairBounds;
      const hairHandles = [
        { x: hb.crownX, y: hb.crownY, label: 'Đỉnh tóc', color: '#c084fc', align: 'right' },
        { x: hb.hairlineX, y: hb.hairlineY, label: 'Chân tóc', color: '#a855f7', align: 'right' },
        { x: hb.leftX, y: hb.leftY, label: 'Tóc T', color: '#c084fc', align: 'left' },
        { x: hb.rightX, y: hb.rightY, label: 'Tóc P', color: '#c084fc', align: 'right' },
      ];
      for (const hh of hairHandles) {
        const mp = mapPt(hh.x, hh.y);
        const px = mp.x * w;
        const py = mp.y * h;
        ctx.fillStyle = hh.color;
        ctx.strokeStyle = '#ffffff';
        ctx.lineWidth = 1.6;
        ctx.beginPath();
        ctx.arc(px, py, 4.5, 0, Math.PI * 2);
        ctx.fill();
        ctx.stroke();

        ctx.font = '600 9.5px sans-serif';
        const tw = Math.ceil(ctx.measureText(hh.label).width);
        const boxW = tw + 8;
        const bx = hh.align === 'left' ? Math.max(4, px - boxW - 8) : px + 8;
        ctx.fillStyle = 'rgba(18, 12, 28, 0.88)';
        ctx.fillRect(bx, py - 8, boxW, 15);
        ctx.fillStyle = '#e9d5ff';
        ctx.fillText(hh.label, bx + 4, py + 3);
      }
    }

    if (photo.mesh478 && photo.mesh478.length > 0) {
      const pts = photo.mesh478;
      ctx.fillStyle = 'rgba(248, 212, 66, 0.45)';
      for (let i = 0; i < pts.length; i++) {
        const mp = mapPt(pts[i].x, pts[i].y);
        ctx.fillRect(mp.x * w - 1, mp.y * h - 1, 2, 2);
      }

      strokeMeshContour(ctx, pts, MESH_INDICES.FACE_OVAL, w, h, 'rgba(248, 212, 66, 0.9)', 2, true, mapPt);
      strokeMeshContour(ctx, pts, MESH_INDICES.LEFT_EYE, w, h, '#38bdf8', 2.2, true, mapPt);
      strokeMeshContour(ctx, pts, MESH_INDICES.RIGHT_EYE, w, h, '#38bdf8', 2.2, true, mapPt);
      strokeMeshContour(ctx, pts, MESH_INDICES.LEFT_EYEBROW, w, h, '#34d399', 2, true, mapPt);
      strokeMeshContour(ctx, pts, MESH_INDICES.RIGHT_EYEBROW, w, h, '#34d399', 2, true, mapPt);
      strokeMeshContour(ctx, pts, MESH_INDICES.LIPS_OUTER, w, h, '#ec4899', 2.2, true, mapPt);
      strokeMeshContour(ctx, pts, MESH_INDICES.LIPS_INNER, w, h, '#f472b6', 1.6, true, mapPt);
      strokeMeshContour(ctx, pts, MESH_INDICES.LEFT_EYE_BAG_CRESCENT, w, h, 'rgba(167, 139, 250, 0.75)', 1.5, true, mapPt);
      strokeMeshContour(ctx, pts, MESH_INDICES.RIGHT_EYE_BAG_CRESCENT, w, h, 'rgba(167, 139, 250, 0.75)', 1.5, true, mapPt);
    } else {
      const mc = mapPt(f.centerX, f.centerY);
      const sxScale = croppedView ? 1 / Math.max(0.01, params.cropW) : 1;
      const syScale = croppedView ? 1 / Math.max(0.01, params.cropH) : 1;
      ctx.save();
      ctx.strokeStyle = 'rgba(248, 212, 66, 0.85)';
      ctx.lineWidth = 2;
      ctx.setLineDash([6, 4]);
      ctx.beginPath();
      ctx.ellipse(mc.x * w, mc.y * h, f.faceRx * sxScale * w, f.faceRy * syScale * h, 0, 0, Math.PI * 2);
      ctx.stroke();
      ctx.restore();
    }

    const handles = [
      { key: 'leftEye', x: f.leftEyeX, y: f.leftEyeY, label: 'Mắt T', color: '#38bdf8', align: 'right' },
      { key: 'rightEye', x: f.rightEyeX, y: f.rightEyeY, label: 'Mắt P', color: '#38bdf8', align: 'right' },
      { key: 'nose', x: f.noseX, y: f.noseY, label: 'Mũi', color: '#f8d442', align: 'left' },
      { key: 'mouth', x: f.mouthX, y: f.mouthY, label: 'Miệng', color: '#ec4899', align: 'right' },
      { key: 'center', x: f.centerX, y: f.centerY, label: 'Mặt', color: '#22c55e', align: 'right' },
    ];

    for (const p of handles) {
      const mp = mapPt(p.x, p.y);
      const px = mp.x * w;
      const py = mp.y * h;
      ctx.fillStyle = p.color;
      ctx.strokeStyle = '#ffffff';
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.arc(px, py, 6, 0, Math.PI * 2);
      ctx.fill();
      ctx.stroke();

      ctx.font = '600 10px sans-serif';
      const tw = Math.ceil(ctx.measureText(p.label).width);
      const boxW = Math.max(36, tw + 10);
      const bx = p.align === 'left' ? Math.max(4, px - boxW - 9) : px + 9;
      ctx.fillStyle = 'rgba(12, 13, 16, 0.88)';
      ctx.fillRect(bx, py - 8, boxW, 16);
      ctx.fillStyle = '#fff';
      ctx.fillText(p.label, bx + 5, py + 3);
    }
  }

  if (state.activeModule === 'mod-crop' && !state.cropPreviewMode) {
    drawCropOverlay(ctx, w, h, photo);
  }
}

function updateHistogram(sourceCtx, w, h) {
  if (!histCanvas) return;
  const hc = histCanvas.getContext('2d');
  const hw = histCanvas.width;
  const hh = histCanvas.height;
  hc.clearRect(0, 0, hw, hh);

  const sampleW = Math.min(w, 260);
  const sampleH = Math.min(h, 260);
  const imgData = sourceCtx.getImageData(
    Math.floor((w - sampleW) / 2),
    Math.floor((h - sampleH) / 2),
    sampleW,
    sampleH
  ).data;

  const rBins = new Uint32Array(64);
  const gBins = new Uint32Array(64);
  const bBins = new Uint32Array(64);
  let sumR = 0;
  let sumG = 0;
  let sumB = 0;
  let count = 0;

  for (let i = 0; i < imgData.length; i += 16) {
    const r = imgData[i];
    const g = imgData[i + 1];
    const b = imgData[i + 2];
    rBins[r >> 2]++;
    gBins[g >> 2]++;
    bBins[b >> 2]++;
    sumR += r;
    sumG += g;
    sumB += b;
    count++;
  }

  if (histRgbMean && count > 0) {
    histRgbMean.textContent = `R ${Math.round(sumR / count)} · G ${Math.round(
      sumG / count
    )} · B ${Math.round(sumB / count)}`;
  }

  let maxBin = 1;
  for (let i = 1; i < 63; i++) {
    maxBin = Math.max(maxBin, rBins[i], gBins[i], bBins[i]);
  }

  const drawChannel = (bins, color) => {
    hc.fillStyle = color;
    hc.beginPath();
    hc.moveTo(0, hh);
    for (let i = 0; i < 64; i++) {
      const x = (i / 63) * hw;
      const y = hh - Math.min(1, bins[i] / maxBin) * (hh - 5);
      hc.lineTo(x, y);
    }
    hc.lineTo(hw, hh);
    hc.closePath();
    hc.fill();
  };

  hc.globalCompositeOperation = 'screen';
  drawChannel(rBins, 'rgba(239, 68, 68, 0.58)');
  drawChannel(gBins, 'rgba(34, 197, 94, 0.58)');
  drawChannel(bBins, 'rgba(59, 130, 246, 0.62)');
  hc.globalCompositeOperation = 'source-over';
}

const BG_SWITCH_KEYS = [
  'bgCleanDistractions',
  'bgCleanDefects',
  'bgEvenBrightness',
  'bgRemoveBanding',
  'bgDecontam',
  'bgGroundReflect',
  'bgPreserveShadow',
];

const BG_SELECT_IDS = [
  'bgReplaceMode',
  'bgEdgeMode',
  'bgKeepScope',
  'bgFillArea',
  'bgFillMode',
  'bgBlendMode',
  'bgShadowBlendMode',
];

function renderBgUserGallery(params) {
  const gallery = document.getElementById('bg-user-gallery');
  if (!gallery) return;
  const activeImg = params?.bgCustomImage || '';
  const items = state.customBgGallery.filter((it) =>
    state.bgUserFilter === 'fav' ? Boolean(it.fav) : true
  );
  gallery.replaceChildren();
  gallery.classList.toggle('hidden', items.length === 0);

  for (const item of items) {
    const card = document.createElement('div');
    card.className = 'bg-user-thumb';
    if (params?.bgReplaceMode === 'custom_image' && activeImg === item.dataUrl) {
      card.classList.add('active');
    }
    card.title = item.name || 'Ảnh nền tùy chỉnh';

    const img = document.createElement('img');
    img.src = item.dataUrl;
    img.alt = item.name || 'Nền';
    card.appendChild(img);

    const actions = document.createElement('div');
    actions.className = 'bg-user-thumb-actions';

    const favBtn = document.createElement('button');
    favBtn.type = 'button';
    favBtn.className = `bg-thumb-btn${item.fav ? ' fav-on' : ''}`;
    favBtn.title = item.fav ? 'Bỏ yêu thích' : 'Yêu thích';
    favBtn.textContent = '★';
    favBtn.addEventListener('click', (e) => {
      e.stopPropagation();
      item.fav = !item.fav;
      const photo = getActivePhoto();
      renderBgUserGallery(photo?.params || params);
    });

    const delBtn = document.createElement('button');
    delBtn.type = 'button';
    delBtn.className = 'bg-thumb-btn';
    delBtn.title = 'Xóa khỏi thư viện';
    delBtn.textContent = '✕';
    delBtn.addEventListener('click', (e) => {
      e.stopPropagation();
      state.customBgGallery = state.customBgGallery.filter((x) => x.id !== item.id);
      const photo = getActivePhoto();
      if (photo && photo.params.bgCustomImage === item.dataUrl) {
        const nextItem = state.customBgGallery[0];
        photo.params.bgCustomImage = nextItem ? nextItem.dataUrl : '';
        if (!nextItem && photo.params.bgReplaceMode === 'custom_image') {
          photo.params.bgReplaceMode = 'original';
        }
        updateBgCustomControlsUI(photo.params);
        commitHistory(photo);
        processActivePhoto(false);
      } else {
        renderBgUserGallery(photo?.params || params);
      }
    });

    actions.appendChild(favBtn);
    actions.appendChild(delBtn);
    card.appendChild(actions);

    card.addEventListener('click', () => {
      const photo = getActivePhoto();
      if (!photo) return;
      photo.params.bgCustomImage = item.dataUrl;
      photo.params.bgReplaceMode = 'custom_image';
      updateBgCustomControlsUI(photo.params);
      commitHistory(photo);
      processActivePhoto(false);
    });

    gallery.appendChild(card);
  }
}

function updateBgCustomControlsUI(params) {
  const mode = params.bgReplaceMode || 'original';
  const hex = /^#[0-9a-fA-F]{6}$/.test(params.bgCustomColor || '') ? params.bgCustomColor : '#e8eef5';
  const hasCustomImg = Boolean(params.bgCustomImage && params.bgCustomImage.length > 0);

  document.querySelectorAll('.asset-swatch').forEach((sw) => {
    const swMode = sw.dataset.bgmode;
    sw.classList.toggle('active', Boolean(swMode) && swMode === mode);
  });
  document.getElementById('btn-bg-reset-original')?.classList.toggle('active', mode === 'original');

  for (const selId of BG_SELECT_IDS) {
    const sel = document.getElementById(selId);
    if (sel && params[selId] !== undefined) sel.value = params[selId];
  }

  for (const swId of BG_SWITCH_KEYS) {
    const sw = document.getElementById(swId);
    if (sw) {
      const on = Boolean(params[swId]);
      sw.classList.toggle('on', on);
      sw.setAttribute('aria-checked', String(on));
    }
  }

  const customColorSwatch = document.getElementById('swatch-custom-color');
  if (customColorSwatch) customColorSwatch.style.background = hex;

  const colorPicker = document.getElementById('bgCustomColor');
  if (colorPicker) colorPicker.value = hex;
  const colorHexLabel = document.getElementById('bg-custom-color-hex');
  if (colorHexLabel) colorHexLabel.textContent = hex;

  document.querySelectorAll('.bg-color-dot').forEach((dot) => {
    const dc = (dot.dataset.color || '').toLowerCase();
    dot.classList.toggle('active', mode === 'custom_color' && dc === hex.toLowerCase());
  });

  // Segmented Tabs (Hình ảnh / Màu đơn sắc) & Underline Tabs (Đề xuất / Nền của bạn)
  const isSolid = state.bgCategory === 'solid';
  const isUserTab = state.bgLibTab === 'user';
  document.getElementById('bg-cat-image')?.classList.toggle('active', !isSolid);
  document.getElementById('bg-cat-solid')?.classList.toggle('active', isSolid);
  document.getElementById('bg-lib-tab-rec')?.classList.toggle('active', !isUserTab);
  document.getElementById('bg-lib-tab-user')?.classList.toggle('active', isUserTab);

  const recPanel = document.getElementById('bg-lib-rec-panel');
  const userPanel = document.getElementById('bg-lib-user-panel');
  const userImgPane = document.getElementById('bg-user-image-pane');
  const userSolidPane = document.getElementById('bg-user-solid-pane');

  if (isSolid) {
    recPanel?.classList.add('hidden');
    userPanel?.classList.remove('hidden');
    userImgPane?.classList.add('hidden');
    userSolidPane?.classList.remove('hidden');
  } else {
    recPanel?.classList.toggle('hidden', isUserTab);
    userPanel?.classList.toggle('hidden', !isUserTab);
    userImgPane?.classList.toggle('hidden', !isUserTab);
    userSolidPane?.classList.add('hidden');
  }

  // Filter Pills (Khám phá ngay / Mục đã chọn) & More Swatches
  const isSelectedFilter = state.bgFilter === 'selected';
  document.getElementById('bg-filter-explore')?.classList.toggle('active', !isSelectedFilter);
  document.getElementById('bg-filter-selected')?.classList.toggle('active', isSelectedFilter);

  const btnMore = document.getElementById('btn-bg-more');
  const moreWrap = document.getElementById('bg-more-swatches');
  if (recPanel) {
    recPanel.querySelectorAll('.asset-swatch[data-bgmode]').forEach((sw) => {
      sw.classList.toggle('hidden', isSelectedFilter && sw.dataset.bgmode !== mode);
    });
    if (btnMore) {
      btnMore.classList.toggle('hidden', isSelectedFilter);
      btnMore.classList.toggle('active', !isSelectedFilter && state.bgMoreExpanded);
    }
    if (moreWrap) {
      const hasMatchInMore = Boolean(moreWrap.querySelector(`.asset-swatch[data-bgmode="${mode}"]`));
      moreWrap.classList.toggle(
        'hidden',
        isSelectedFilter ? !hasMatchInMore : !state.bgMoreExpanded
      );
    }
  }

  // User image filter & gallery
  document.getElementById('bg-user-filter-personal')?.classList.toggle('active', state.bgUserFilter === 'personal');
  document.getElementById('bg-user-filter-fav')?.classList.toggle('active', state.bgUserFilter === 'fav');
  renderBgUserGallery(params);

  const previewWrap = document.getElementById('custom-bg-preview');
  const previewImg = document.getElementById('custom-bg-preview-img');
  const clearBtn = document.getElementById('btn-clear-custom-bg');
  if (previewWrap) previewWrap.classList.toggle('hidden', !hasCustomImg);
  if (previewImg) previewImg.src = hasCustomImg ? params.bgCustomImage : '';
  if (clearBtn) clearBtn.classList.toggle('hidden', !hasCustomImg);

  // Lock advanced background controls when keeping original background
  document.getElementById('bg-replace-advanced-wrap')?.classList.toggle('is-locked', mode === 'original');

  // Shadow cards sync
  const shType = params.bgShadowType || 'none';
  document.querySelectorAll('.bg-shadow-card').forEach((card) => {
    card.classList.toggle('active', card.dataset.shadow === shType);
  });
}

let webPreviewSeq = 0;

function applyCanvasTransformDraw(ctx, img, w, h, p) {
  const sc = Math.max(0.1, (p?.geoScale ?? 100) / 100);
  const offX = ((p?.geoOffsetX ?? 0) / 100) * w * 0.5 * sc;
  const offY = ((p?.geoOffsetY ?? 0) / 100) * h * 0.5 * sc;
  const rad = ((p?.rotateAngle ?? 0) * Math.PI) / 180;
  const sx = (p?.flipHorizontal ? -1 : 1) * sc;
  const sy = (p?.flipVertical ? -1 : 1) * sc;

  ctx.save();
  ctx.translate(w * 0.5 + offX, h * 0.5 + offY);
  if (Math.abs(rad) > 1e-4) ctx.rotate(rad);
  ctx.scale(sx, sy);
  ctx.drawImage(img, -w * 0.5, -h * 0.5, w, h);
  ctx.restore();
}

function needsHighresMatte(params) {
  const p = params || {};
  return (
    (p.bgReplaceMode && p.bgReplaceMode !== 'original') ||
    (p.bgBlurBokeh || 0) > 0.5 ||
    (p.strayHair || 0) > 0.5 ||
    Math.abs(p.hairVolume || 0) > 0.3 ||
    Math.abs(p.hairSideLeftVolume || 0) > 0.3 ||
    Math.abs(p.hairSideRightVolume || 0) > 0.3 ||
    Math.abs(p.hairlineAdjust || 0) > 0.3 ||
    (p.hairPartLine || 0) > 0.5 ||
    (p.hairSmooth || 0) > 0.5 ||
    (p.whiteHairBlacken || 0) > 0.5 ||
    Math.abs(p.hairColorUnify || 0) > 0.5 ||
    ((p.hairDyeStrength || 0) > 0.5 && p.hairDyeColor && p.hairDyeColor !== 'none') ||
    (p.backdropClean || 0) > 0.5 ||
    (p.backdropBandingFix || 0) > 0.5 ||
    Boolean(p.bgCleanDistractions) ||
    Boolean(p.bgCleanDefects) ||
    Boolean(p.bgEvenBrightness) ||
    Boolean(p.bgRemoveBanding)
  );
}

function getEffectiveSceneMask(photo) {
  if (!photo) return null;
  const validHighres =
    photo.highresMatteBase64 && photo._matteSourceRef === photo.originalDataUrl
      ? photo.highresMatteBase64
      : null;
  if (needsHighresMatte(photo.params) && validHighres) {
    return validHighres;
  }
  return photo.sceneMaskBase64 || validHighres || null;
}

function isUsingHighresMatte(photo) {
  const eff = getEffectiveSceneMask(photo);
  return Boolean(eff && photo?.highresMatteBase64 && eff === photo.highresMatteBase64);
}

function buildInvokeParams(photo) {
  return {
    ...photo.params,
    isHighresMatte: isUsingHighresMatte(photo),
  };
}

async function ensureHighresMatteForPhoto(photo) {
  if (!photo || !photo.originalDataUrl || !hasTauriRuntime()) return null;
  if (photo.highresMatteBase64 && photo._matteSourceRef === photo.originalDataUrl) {
    return photo.highresMatteBase64;
  }
  if (photo.highresMattePromise) {
    return photo.highresMattePromise;
  }
  const sourceUrl = photo.originalDataUrl;
  photo._matteAttempted = true;
  photo.highresMattePromise = (async () => {
    try {
      const res = await invoke('extract_highres_matte_cmd', {
        imageBase64: sourceUrl,
      });
      if (res && res.matte_data_url) {
        photo.highresMatteBase64 = res.matte_data_url;
        photo._matteSourceRef = sourceUrl;
        photo._subjectBoundsCache = null;
        return res.matte_data_url;
      }
    } catch (err) {
      console.warn('High-res RMBG-1.4 matte fallback to MediaPipe:', err);
    } finally {
      photo.highresMattePromise = null;
    }
    return null;
  })();
  return photo.highresMattePromise;
}

function prewarmActiveHighresMatte() {
  if (!hasTauriRuntime()) return;
  const photo = getActivePhoto();
  if (!photo || photo._matteAttempted || photo.highresMattePromise) return;
  if (photo.highresMatteBase64 && photo._matteSourceRef === photo.originalDataUrl) return;
  ensureHighresMatteForPhoto(photo).then((matte) => {
    if (matte && photo === getActivePhoto() && needsHighresMatte(photo.params)) {
      processActivePhoto(false);
    }
  });
}

async function renderWebBgPreviewDataUrl(photo) {
  const p = photo.params || {};
  const mode = p.bgReplaceMode || 'original';
  const hasTransform =
    p.flipHorizontal ||
    p.flipVertical ||
    Math.abs(p.rotateAngle || 0) > 0.1 ||
    Math.abs((p.geoScale ?? 100) - 100) > 0.1 ||
    Math.abs(p.geoOffsetX || 0) > 0.1 ||
    Math.abs(p.geoOffsetY || 0) > 0.1;

  if (mode === 'original' && !hasTransform) return photo.originalDataUrl;
  const maskUrl = getEffectiveSceneMask(photo) || photo.maskBase64;
  if (mode !== 'original' && !maskUrl) return photo.originalDataUrl;

  const origImg = await loadImageElement(photo.originalDataUrl);
  const nw = origImg.naturalWidth || photo.width || 800;
  const nh = origImg.naturalHeight || photo.height || 1000;
  const scale = Math.min(1, 1920 / Math.max(nw, nh));
  const w = Math.max(1, Math.round(nw * scale));
  const h = Math.max(1, Math.round(nh * scale));

  const outCanvas = document.createElement('canvas');
  outCanvas.width = w;
  outCanvas.height = h;
  const ctx = outCanvas.getContext('2d');

  if (mode === 'original') {
    ctx.fillStyle = '#0c0d10';
    ctx.fillRect(0, 0, w, h);
    applyCanvasTransformDraw(ctx, origImg, w, h, p);
    return outCanvas.toDataURL('image/jpeg', 0.92);
  }

  const maskImg = await loadImageElement(maskUrl);
  const bgOpacity = Math.max(0, Math.min(1, (p.bgReplaceOpacity ?? 100) / 100));

  const drawCustomBgImage = async (targetCtx) => {
    const bgImg = await loadImageElement(p.bgCustomImage);
    const sw = bgImg.naturalWidth || 1;
    const sh = bgImg.naturalHeight || 1;
    const fillMode = p.bgFillMode || 'cover';
    const zoom = 1 + Math.max(0, Math.min(100, p.bgScale ?? 0)) / 100;
    const offX = ((p.bgOffsetX ?? 0) / 100) * w * 0.45;
    const offY = ((p.bgOffsetY ?? 0) / 100) * h * 0.45;
    const blurPx = Math.max(0, ((p.bgBlurStrength ?? 0) / 100) * 18);

    targetCtx.save();
    if (blurPx > 0.2) targetCtx.filter = `blur(${blurPx.toFixed(1)}px)`;

    if (fillMode === 'stretch') {
      const dw = w * zoom;
      const dh = h * zoom;
      targetCtx.drawImage(bgImg, (w - dw) * 0.5 + offX, (h - dh) * 0.5 + offY, dw, dh);
    } else if (fillMode === 'contain') {
      targetCtx.fillStyle = '#0c0d10';
      targetCtx.fillRect(0, 0, w, h);
      const fit = Math.min(w / sw, h / sh) * zoom;
      const dw = sw * fit;
      const dh = sh * fit;
      targetCtx.drawImage(bgImg, (w - dw) * 0.5 + offX, (h - dh) * 0.5 + offY, dw, dh);
    } else if (fillMode === 'tile') {
      const tileScale = Math.max(0.2, 0.5 * zoom);
      const tw = Math.max(16, Math.round(sw * tileScale));
      const th = Math.max(16, Math.round(sh * tileScale));
      const patCanvas = document.createElement('canvas');
      patCanvas.width = tw;
      patCanvas.height = th;
      patCanvas.getContext('2d').drawImage(bgImg, 0, 0, tw, th);
      const pat = targetCtx.createPattern(patCanvas, 'repeat');
      if (pat) {
        targetCtx.translate(offX, offY);
        targetCtx.fillStyle = pat;
        targetCtx.fillRect(-offX, -offY, w, h);
      }
    } else {
      const fit = Math.max(w / sw, h / sh) * zoom;
      const dw = sw * fit;
      const dh = sh * fit;
      targetCtx.drawImage(bgImg, (w - dw) * 0.5 + offX, (h - dh) * 0.5 + offY, dw, dh);
    }
    targetCtx.restore();
  };

  // 1. Draw chosen background onto output canvas
  if (mode === 'transparent') {
    ctx.clearRect(0, 0, w, h);
  } else {
    if (bgOpacity < 0.999 || p.bgFillArea === 'outer_only') {
      ctx.drawImage(origImg, 0, 0, w, h);
    }
    ctx.save();
    if (bgOpacity < 0.999) ctx.globalAlpha = bgOpacity;

    if (mode === 'custom_image' && p.bgCustomImage) {
      try {
        await drawCustomBgImage(ctx);
      } catch (_) {
        ctx.fillStyle = '#e8eef5';
        ctx.fillRect(0, 0, w, h);
      }
    } else if (mode === 'pure_white') {
      ctx.fillStyle = '#ffffff';
      ctx.fillRect(0, 0, w, h);
    } else if (mode === 'custom_color') {
      ctx.fillStyle = /^#[0-9a-fA-F]{6}$/.test(p.bgCustomColor || '') ? p.bgCustomColor : '#e8eef5';
      ctx.fillRect(0, 0, w, h);
    } else if (mode.startsWith('sky_')) {
      const grad = ctx.createLinearGradient(0, 0, 0, h);
      if (mode === 'sky_golden_sunset') {
        grad.addColorStop(0, '#f47b47');
        grad.addColorStop(1, '#f7c56e');
      } else if (mode === 'sky_alpine_blue') {
        grad.addColorStop(0, '#2a6fd8');
        grad.addColorStop(1, '#a7ccf8');
      } else if (mode === 'sky_dramatic_clouds') {
        grad.addColorStop(0, '#3d4758');
        grad.addColorStop(1, '#8490a3');
      } else {
        grad.addColorStop(0, '#0d142e');
        grad.addColorStop(1, '#22335c');
      }
      ctx.fillStyle = grad;
      ctx.fillRect(0, 0, w, h);
    } else {
      const rGrad = ctx.createRadialGradient(w * 0.5, h * 0.36, w * 0.05, w * 0.5, h * 0.4, Math.max(w, h) * 0.72);
      if (mode === 'warm_beige') {
        rGrad.addColorStop(0, '#ad9387');
        rGrad.addColorStop(1, '#8a7368');
      } else if (mode === 'studio_grey') {
        rGrad.addColorStop(0, '#b5b9c0');
        rGrad.addColorStop(1, '#6f737a');
      } else if (mode === 'slate_grey_03') {
        rGrad.addColorStop(0, '#7c7a82');
        rGrad.addColorStop(1, '#58565f');
      } else if (mode === 'fabric_02') {
        rGrad.addColorStop(0, '#555c68');
        rGrad.addColorStop(1, '#1c1f25');
      } else if (mode === 'passport_blue') {
        rGrad.addColorStop(0, '#2e7ad8');
        rGrad.addColorStop(1, '#2366bf');
      } else if (mode === 'pastel_pink') {
        rGrad.addColorStop(0, '#f7dce1');
        rGrad.addColorStop(1, '#e8bdc6');
      } else if (mode === 'sage_green') {
        rGrad.addColorStop(0, '#b2ccb9');
        rGrad.addColorStop(1, '#8aa694');
      } else if (mode === 'lavender_mist') {
        rGrad.addColorStop(0, '#ddd6f3');
        rGrad.addColorStop(1, '#beb3e0');
      } else {
        rGrad.addColorStop(0, '#252830');
        rGrad.addColorStop(1, '#08090c');
      }
      ctx.fillStyle = rGrad;
      ctx.fillRect(0, 0, w, h);
    }
    ctx.restore();
  }

  // 2. Build foreground canvas masked smoothly by continuous neural probability
  const fgCanvas = document.createElement('canvas');
  fgCanvas.width = w;
  fgCanvas.height = h;
  const fgCtx = fgCanvas.getContext('2d');
  fgCtx.drawImage(origImg, 0, 0, w, h);
  const fgData = fgCtx.getImageData(0, 0, w, h);

  const mCanvas = document.createElement('canvas');
  mCanvas.width = w;
  mCanvas.height = h;
  const mCtx = mCanvas.getContext('2d');
  mCtx.drawImage(maskImg, 0, 0, w, h);
  const mPixels = mCtx.getImageData(0, 0, w, h).data;
  const effectiveSceneMask = getEffectiveSceneMask(photo);
  const useSceneChannel = Boolean(effectiveSceneMask);
  const isHighresMatte = Boolean(
    effectiveSceneMask &&
      photo.highresMatteBase64 &&
      effectiveSceneMask === photo.highresMatteBase64
  );
  const edgeShift = -(p.bgEdgeAdjust ?? 0) / 360;

  // Also load the base mask (R=skin, G=feature, B=hair) to get neural hair channel
  // when sceneMask is available; this enables proper hair-strand alpha blending.
  let baseMaskPixels = null;
  if (useSceneChannel && photo.maskBase64) {
    try {
      const bmImg = await loadImageElement(photo.maskBase64);
      const bmCanvas = document.createElement('canvas');
      bmCanvas.width = w;
      bmCanvas.height = h;
      const bmCtx = bmCanvas.getContext('2d');
      bmCtx.drawImage(bmImg, 0, 0, w, h);
      baseMaskPixels = bmCtx.getImageData(0, 0, w, h).data;
    } catch (_) {
      baseMaskPixels = null;
    }
  }

  // Determine edge mode thresholds — mirrors Rust compute_guided_matte logic
  const edgeMode = p.bgEdgeMode || 'auto';
  const adjBias = -((p.bgEdgeAdjust ?? 0) / 100) * 0.14;
  let solidLo, solidHi, hairLo, hairHi, hairWeightScale;
  if (edgeMode === 'soft') {
    solidLo = Math.max(0.01, Math.min(0.72, 0.12 + adjBias));
    solidHi = Math.max(0.24, Math.min(0.99, 0.88 + adjBias));
    hairLo  = Math.max(0.01, Math.min(0.72, 0.09 + adjBias));
    hairHi  = Math.max(0.26, Math.min(0.99, 0.90 + adjBias));
    hairWeightScale = 1.15;
  } else if (edgeMode === 'crisp') {
    solidLo = Math.max(0.04, Math.min(0.80, 0.34 + adjBias));
    solidHi = Math.max(0.25, Math.min(0.98, 0.70 + adjBias));
    hairLo  = Math.max(0.04, Math.min(0.78, 0.24 + adjBias));
    hairHi  = Math.max(0.28, Math.min(0.98, 0.76 + adjBias));
    hairWeightScale = 0.70;
  } else if (edgeMode === 'hair_detail') {
    solidLo = Math.max(0.02, Math.min(0.74, 0.18 + adjBias));
    solidHi = Math.max(0.26, Math.min(0.98, 0.80 + adjBias));
    hairLo  = Math.max(0.02, Math.min(0.72, 0.10 + adjBias));
    hairHi  = Math.max(0.26, Math.min(0.98, 0.80 + adjBias));
    hairWeightScale = 1.40;
  } else {
    // 'auto' default
    solidLo = Math.max(0.02, Math.min(0.76, 0.24 + adjBias));
    solidHi = Math.max(0.26, Math.min(0.98, 0.78 + adjBias));
    hairLo  = Math.max(0.02, Math.min(0.76, 0.16 + adjBias));
    hairHi  = Math.max(0.28, Math.min(0.98, 0.80 + adjBias));
    hairWeightScale = 1.00;
  }

  function smoothstep(lo, hi, x) {
    const t = Math.max(0, Math.min(1, (x - lo) / (hi - lo)));
    return t * t * (3 - 2 * t);
  }

  const px = fgData.data;
  for (let i = 0; i < px.length; i += 4) {
    let alpha;
    if (isHighresMatte) {
      const rawMatte = mPixels[i] / 255;
      let a = edgeShift !== 0 ? Math.max(0, Math.min(1, rawMatte - edgeShift)) : rawMatte;
      if (edgeMode === 'crisp') {
        a = smoothstep(0.28, 0.72, a);
      } else if (edgeMode === 'soft') {
        a = smoothstep(0.01, 0.94, a);
      }
      alpha = a;
    } else if (useSceneChannel) {
      // sceneMask: R=fgProb, G=clothProb, B=solidProb (from ai_masks.js sceneData)
      const fgProb   = mPixels[i]     / 255;  // R
      const solidProb = mPixels[i + 2] / 255;  // B
      // Hair probability from base maskBase64 B-channel (neural hair segmentation)
      const hairProb = baseMaskPixels ? baseMaskPixels[i + 2] / 255 : 0;

      // Use solidProb as the primary fg signal (most reliable for arms/hands/clothing),
      // blended with hairProb-aware curve for hair strands — mirrors Rust solid_a/hair_a blend
      const fgSignal = Math.max(fgProb, solidProb * 0.95);
      const solidA = smoothstep(solidLo, solidHi, fgSignal);
      const hairA  = smoothstep(hairLo, hairHi, fgSignal);
      const hairW  = Math.min(1, smoothstep(0.15, 0.45, hairProb) * hairWeightScale);
      alpha = solidA * (1 - hairW) + hairA * hairW;
    } else {
      // Legacy: maskBase64 alpha channel
      const raw = mPixels[i + 3] / 255 - edgeShift;
      alpha = smoothstep(solidLo, solidHi, raw);
    }
    px[i + 3] = Math.round(Math.max(0, Math.min(1, alpha)) * 255);
  }
  fgCtx.putImageData(fgData, 0, 0);

  // 3. Drop shadow (web fallback preview) when enabled
  const shType = p.bgShadowType || 'none';
  const shOp = Math.max(0, Math.min(1, (p.bgShadowOpacity ?? 0) / 100));
  if (shType !== 'none' && shOp > 0.01) {
    const shBlur = Math.max(1, ((p.bgShadowBlur ?? 25) / 100) * Math.min(w, h) * 0.05);
    const shiftFrac = (p.bgShadowShift ?? 0) / 100;
    const offX = shType === 'cast_shadow' ? w * (0.036 + shiftFrac * 0.04) : w * 0.01 * (1 + shiftFrac);
    const offY = shType === 'cast_shadow' ? h * 0.014 : h * 0.018 * (1 + shiftFrac);
    ctx.save();
    if (mode !== 'transparent') {
      const shBlend = p.bgShadowBlendMode || 'multiply';
      ctx.globalCompositeOperation =
        shBlend === 'soft_light'
          ? 'soft-light'
          : shBlend === 'darken' || shBlend === 'multiply'
            ? shBlend
            : 'source-over';
    }
    ctx.shadowColor = `rgba(10, 12, 16, ${(shOp * 0.65).toFixed(3)})`;
    ctx.shadowBlur = shType === 'hard_shadow' ? shBlur * 0.35 : shBlur;
    ctx.shadowOffsetX = offX;
    ctx.shadowOffsetY = offY;
    applyCanvasTransformDraw(ctx, fgCanvas, w, h, p);
    ctx.restore();
  }

  applyCanvasTransformDraw(ctx, fgCanvas, w, h, p);

  return mode === 'transparent'
    ? outCanvas.toDataURL('image/png')
    : outCanvas.toDataURL('image/jpeg', 0.92);
}

async function processActivePhoto(highQuality = false) {
  const photo = getActivePhoto();
  if (!photo) return;
  if (!state.isRestoringHistory) {
    photo._isLiyingOutput = false;
  }

  // If no retouch/color/crop/spot edits are active, keep original full-resolution master directly
  // so newly imported or reset photos stay 100% as crisp as the source without preview re-compression.
  if (!isPhotoEdited(photo.params) && !photo._isLiyingOutput) {
    photo.processedDataUrl = photo.originalDataUrl;
    photo._processedHQ = true;
    const aiBadge = photo.landmarksDetected ? 'AI 478 điểm ✓' : 'Ảnh gốc 100% ✓';
    renderTimeLabel.textContent = `${aiBadge} • Gốc`;
    await renderViewport();
    renderFilmstrip();
    if (state.workspaceMode === 'grid') renderLibraryGrid();
    if (hasTauriRuntime() && !needsHighresMatte(photo.params) && !photo._matteAttempted) {
      prewarmActiveHighresMatte();
    }
    return;
  }

  if (!hasTauriRuntime()) {
    const seq = ++webPreviewSeq;
    photo.processedDataUrl = await renderWebBgPreviewDataUrl(photo);
    if (seq !== webPreviewSeq) return;
    renderTimeLabel.textContent = photo.landmarksDetected
      ? 'AI 478 điểm (Chế độ Xem web)'
      : 'Chế độ Xem web';
    await renderViewport();
    renderFilmstrip();
    if (state.workspaceMode === 'grid') renderLibraryGrid();
    return;
  }

  if (state.isProcessing) {
    state.pendingProcess = true;
    return;
  }

  state.isProcessing = true;
  renderTimeLabel.textContent = 'Đang xử lý Rust SIMD...';

  try {
    if (
      needsHighresMatte(photo.params) &&
      (!photo.highresMatteBase64 || photo._matteSourceRef !== photo.originalDataUrl)
    ) {
      renderTimeLabel.textContent = 'Đang tách nền AI HD (RMBG-1.4)...';
      await ensureHighresMatteForPhoto(photo);
    }

    const useHQ = Boolean(highQuality || state.zoomScale >= 1.35);
    const res = await invoke('process_photo_cmd', {
      imageBase64: photo.originalDataUrl,
      params: buildInvokeParams(photo),
      highQuality: useHQ,
      maskBase64: photo.maskBase64 || null,
      sceneMaskBase64: getEffectiveSceneMask(photo),
    });
    photo.processedDataUrl = res.dataUrl;
    photo._processedHQ = useHQ;
    const aiBadge = photo.landmarksDetected ? 'AI 478 điểm ✓' : 'Mặt nạ chân dung AI ✓';
    renderTimeLabel.textContent = `${aiBadge} • ${res.elapsedMs}ms`;
    await renderViewport();
    renderFilmstrip();
    if (state.workspaceMode === 'grid') renderLibraryGrid();
  } catch (err) {
    console.error('Lỗi xử lý Rust:', err);
    renderTimeLabel.textContent = 'AI 478 điểm ✓';
  } finally {
    state.isProcessing = false;
    if (state.pendingProcess) {
      state.pendingProcess = false;
      processActivePhoto(false);
    } else if (!needsHighresMatte(photo.params) && !photo._matteAttempted) {
      prewarmActiveHighresMatte();
    }
  }
}

async function rotateActivePhoto90(clockwise) {
  const photo = getActivePhoto();
  if (!photo) return;

  renderTimeLabel.textContent = clockwise
    ? 'Đang xoay phải 90° & quét lại AI...'
    : 'Đang xoay trái 90° & quét lại AI...';

  try {
    let fallbackFace = photo.params.face;
    if (hasTauriRuntime()) {
      const rotated = await invoke('rotate_photo_90_cmd', {
        imageBase64: photo.originalDataUrl,
        clockwise,
      });
      photo.originalDataUrl = rotated.dataUrl;
      photo.processedDataUrl = rotated.dataUrl;
      photo.width = rotated.width;
      photo.height = rotated.height;
      fallbackFace = rotated.detectedFace;
    } else {
      const origImg = await loadImageElement(photo.originalDataUrl);
      const sw = origImg.naturalWidth || photo.width || 800;
      const sh = origImg.naturalHeight || photo.height || 1000;
      const c = document.createElement('canvas');
      c.width = sh;
      c.height = sw;
      const ctx = c.getContext('2d');
      ctx.translate(c.width * 0.5, c.height * 0.5);
      ctx.rotate(clockwise ? Math.PI * 0.5 : -Math.PI * 0.5);
      ctx.drawImage(origImg, -sw * 0.5, -sh * 0.5, sw, sh);
      const dataUrl = c.toDataURL('image/jpeg', 0.94);
      photo.originalDataUrl = dataUrl;
      photo.processedDataUrl = dataUrl;
      photo.width = sh;
      photo.height = sw;
    }

    photo._subjectBoundsCache = null;
    photo.params.cropX = 0;
    photo.params.cropY = 0;
    photo.params.cropW = 1;
    photo.params.cropH = 1;

    const imgEl = await loadImageElement(photo.originalDataUrl);
    const aiResult = await analyzePhotoWithNeuralAI(imgEl, fallbackFace);
    photo.params.face = aiResult.face;
    photo.mesh478 = aiResult.mesh478;
    photo.maskBase64 = aiResult.maskBase64;
    photo.sceneMaskBase64 = aiResult.sceneMaskBase64;
    photo.hairBounds = aiResult.hairBounds || null;
    photo._hairGuideCanvas = null;
    photo._hairGuideMaskRef = null;
    photo.highresMatteBase64 = null;
    photo.highresMattePromise = null;
    photo._matteSourceRef = null;
    photo._matteAttempted = false;
    photo._subjectBoundsCache = null;
    photo.landmarksDetected = aiResult.landmarksDetected;
    photo.params.blemishSpots = [];

    syncSlidersFromParams();
    await processActivePhoto(false);
    commitHistory(photo);
  } catch (err) {
    console.error('Lỗi xoay 90°:', err);
  }
}

function syncSlidersFromParams() {
  const photo = getActivePhoto();
  if (!photo) return;
  const p = photo.params;
  const def = createDefaultParams();

  document.querySelectorAll('.retouch-slider').forEach((slider) => {
    const key = slider.id;
    if (key in p) {
      slider.value = p[key];
      const badge = document.querySelector(`.val[data-for="${key}"]`);
      if (badge) {
        badge.textContent = formatSliderBadgeValue(p[key], key);
        const isChanged = Math.abs(Number(p[key]) - Number(def[key])) > 1e-4;
        badge.classList.toggle('modified', isChanged);
      }
    }
  });

  for (const selId of [
    'lutPreset',
    'treatment',
    'bgReplaceMode',
    'bgEdgeMode',
    'bgKeepScope',
    'bgFillArea',
    'bgFillMode',
    'bgBlendMode',
    'bgShadowBlendMode',
    'makeupLipTint',
  ]) {
    const el = document.getElementById(selId);
    if (el && selId in p) el.value = p[selId];
  }

  updateBgCustomControlsUI(p);
  updateClothingControlsUI(p);
  updateHairControlsUI(p);
  updateCropControlsUI(photo);
  updateHistoryButtonsUI(photo);

  renderToneCurveCanvas();
}

function updateHairControlsUI(params) {
  const dyeColor = params?.hairDyeColor || 'none';
  document.querySelectorAll('.hair-color-card').forEach((card) => {
    card.classList.toggle('active', (card.dataset.haircolor || 'none') === dyeColor);
  });
  const dyeControls = document.getElementById('hair-dye-controls');
  if (dyeControls) {
    dyeControls.classList.toggle('disabled', dyeColor === 'none');
  }
  const sideLinkBtn = document.getElementById('btn-hair-side-link');
  if (sideLinkBtn) {
    sideLinkBtn.classList.toggle('active', Boolean(state.hairSideLinked));
    sideLinkBtn.setAttribute('aria-pressed', String(Boolean(state.hairSideLinked)));
  }
}

function updateClothingControlsUI(params) {
  const target = params?.clothingTarget || 'all';
  document.querySelectorAll('.cloth-target-pill').forEach((pill) => {
    pill.classList.toggle('active', pill.dataset.clothtarget === target);
  });
  const linkBtn = document.getElementById('btn-cloth-wrinkle-link');
  if (linkBtn) {
    linkBtn.classList.toggle('active', Boolean(state.clothWrinkleLinked));
    linkBtn.setAttribute('aria-pressed', String(Boolean(state.clothWrinkleLinked)));
  }
}

function isPhotoEdited(params) {
  const def = createDefaultParams();
  for (const k of Object.keys(def)) {
    if (k === 'face' || k === 'blemishSpots' || k === 'curveLut' || k === 'colorMatchStats') continue;
    if (params[k] !== def[k]) return true;
  }
  return (params.blemishSpots?.length || 0) > 0 || (params.curveLut?.length || 0) > 0;
}

function setWorkspaceMode(mode) {
  state.workspaceMode = mode;
  const gridView = document.getElementById('library-grid-view');
  const viewport = document.getElementById('canvas-viewport');
  const toolbar = document.getElementById('viewport-toolbar');
  const filmstrip = document.getElementById('retouch-filmstrip-panel');
  const btnLib = document.getElementById('btn-mode-library');
  const btnDev = document.getElementById('btn-mode-develop');

  const isGrid = mode === 'grid';
  btnLib?.classList.toggle('active', isGrid);
  btnDev?.classList.toggle('active', !isGrid);
  gridView?.classList.toggle('hidden', !isGrid);
  viewport?.classList.toggle('hidden', isGrid);
  toolbar?.classList.toggle('hidden', isGrid);
  filmstrip?.classList.toggle('hidden', isGrid);

  if (isGrid) {
    renderLibraryGrid();
  } else {
    requestAnimationFrame(() => {
      renderViewport();
      updateZoomUI();
    });
  }
}

function getEffectiveOutputDimensions(photo) {
  const { imgW, imgH } = getPhotoDimensions(photo);
  const p = photo?.params;
  if (!p) return { w: imgW, h: imgH };
  const isCropped = hasActiveCropRect(p);
  const cw = Math.max(1, Math.round((isCropped ? p.cropW : 1) * imgW));
  const ch = Math.max(1, Math.round((isCropped ? p.cropH : 1) * imgH));
  if (p.cropCustomW > 0 && p.cropCustomH > 0) {
    return { w: Math.round(p.cropCustomW), h: Math.round(p.cropCustomH) };
  }
  if (p.cropCustomW > 0) {
    return { w: Math.round(p.cropCustomW), h: Math.max(1, Math.round((p.cropCustomW * ch) / cw)) };
  }
  if (p.cropCustomH > 0) {
    return { w: Math.max(1, Math.round((p.cropCustomH * cw) / ch)), h: Math.round(p.cropCustomH) };
  }
  return { w: cw, h: ch };
}

function drawCroppedThumbCanvas(thumbCanvas, photo, tw, th) {
  loadImageElement(photo.processedDataUrl || photo.originalDataUrl)
    .then((img) => {
      const tCtx = thumbCanvas.getContext('2d');
      if (!tCtx) return;
      const sw = img.naturalWidth || tw;
      const sh = img.naturalHeight || th;
      const isCropped = hasActiveCropRect(photo.params);
      const sx = isCropped ? Math.max(0, Math.min(sw - 1, photo.params.cropX * sw)) : 0;
      const sy = isCropped ? Math.max(0, Math.min(sh - 1, photo.params.cropY * sh)) : 0;
      const cw = isCropped ? Math.max(1, Math.min(sw - sx, photo.params.cropW * sw)) : sw;
      const ch = isCropped ? Math.max(1, Math.min(sh - sy, photo.params.cropH * sh)) : sh;
      const scale = Math.max(tw / cw, th / ch);
      const dw = cw * scale;
      const dh = ch * scale;
      tCtx.clearRect(0, 0, tw, th);
      tCtx.drawImage(img, sx, sy, cw, ch, (tw - dw) * 0.5, (th - dh) * 0.5, dw, dh);
    })
    .catch(() => {});
}

function renderLibraryGrid() {
  const gridView = document.getElementById('library-grid-view');
  if (!gridView) return;
  gridView.innerHTML = '';

  state.photos.forEach((p, idx) => {
    const card = document.createElement('div');
    card.className = `library-grid-card ${idx === state.activeIndex ? 'active' : ''}`;

    const media = document.createElement('div');
    media.className = 'library-card-media';
    const thumbCanvas = document.createElement('canvas');
    thumbCanvas.width = 320;
    thumbCanvas.height = 240;
    drawCroppedThumbCanvas(thumbCanvas, p, 320, 240);
    media.appendChild(thumbCanvas);

    if (isPhotoEdited(p.params)) {
      const badge = document.createElement('span');
      badge.className = 'library-edited-badge';
      badge.textContent = 'Đã chỉnh';
      media.appendChild(badge);
    }

    const dims = getEffectiveOutputDimensions(p);
    const meta = document.createElement('div');
    meta.className = 'library-card-meta';
    const nameEl = document.createElement('div');
    nameEl.className = 'library-card-name';
    nameEl.title = p.name || '';
    nameEl.textContent = p.name || '';
    const dimEl = document.createElement('div');
    dimEl.className = 'library-card-dim';
    dimEl.textContent = `${dims.w || '?'} × ${dims.h || '?'} px`;
    meta.appendChild(nameEl);
    meta.appendChild(dimEl);

    card.appendChild(media);
    card.appendChild(meta);

    card.addEventListener('click', () => {
      state.activeIndex = idx;
      syncSlidersFromParams();
      renderFilmstrip();
      renderLibraryGrid();
    });

    card.addEventListener('dblclick', () => {
      state.activeIndex = idx;
      syncSlidersFromParams();
      setWorkspaceMode('develop');
      renderViewport();
      renderFilmstrip();
      prewarmActiveHighresMatte();
    });

    gridView.appendChild(card);
  });
}

function renderFilmstrip() {
  if (!filmstripTrack) return;
  filmstripTrack.innerHTML = '';
  if (photoCountBadge) photoCountBadge.textContent = String(state.photos.length);
  if (railBatchRing) railBatchRing.textContent = String(state.photos.length);

  state.photos.forEach((p, idx) => {
    const item = document.createElement('div');
    item.className = `film-thumb ${idx === state.activeIndex ? 'active' : ''}`;

    const thumbCanvas = document.createElement('canvas');
    thumbCanvas.width = 96;
    thumbCanvas.height = 96;
    item.appendChild(thumbCanvas);

    drawCroppedThumbCanvas(thumbCanvas, p, 96, 96);

    if (isPhotoEdited(p.params)) {
      const dot = document.createElement('div');
      dot.className = 'film-edited-dot';
      dot.title = 'Đã áp dụng chỉnh sửa';
      item.appendChild(dot);
    }

    const lbl = document.createElement('div');
    lbl.className = 'film-thumb-label';
    lbl.textContent = p.name;
    item.appendChild(lbl);

    item.addEventListener('click', () => {
      state.activeIndex = idx;
      syncSlidersFromParams();
      renderViewport();
      renderFilmstrip();
      prewarmActiveHighresMatte();
    });

    filmstripTrack.appendChild(item);
    if (idx === state.activeIndex) {
      item.scrollIntoView({ block: 'nearest', inline: 'nearest' });
    }
  });
}

function renderUserPresetsList() {
  const container = document.getElementById('user-presets-container');
  if (!container) return;
  container.innerHTML = '';

  if (state.userPresets.length === 0) {
    const empty = document.createElement('div');
    empty.className = 'preset-empty-note';
    empty.textContent = 'Chưa có cài sẵn cá nhân. Nhấn nút + hoặc "Lưu cài sẵn mới".';
    container.appendChild(empty);
    return;
  }

  state.userPresets.forEach((preset, idx) => {
    const btn = document.createElement('button');
    btn.className = 'retouch-preset-item user-preset-card';
    btn.type = 'button';
    btn.title = `${preset.name} (Cài sẵn của tôi)`;

    const thumb = document.createElement('span');
    thumb.className = 'preset-thumb preset-thumb--user_custom';
    thumb.setAttribute('aria-hidden', 'true');

    const badge = document.createElement('span');
    badge.className = 'preset-badge';
    badge.textContent = 'CÁ NHÂN';

    const meta = document.createElement('div');
    meta.className = 'preset-meta';
    const titleEl = document.createElement('div');
    titleEl.className = 'p-title';
    titleEl.textContent = preset.name;
    const subEl = document.createElement('div');
    subEl.className = 'p-sub';
    subEl.textContent = 'Đã lưu';
    meta.appendChild(titleEl);
    meta.appendChild(subEl);

    const del = document.createElement('span');
    del.className = 'preset-delete-x';
    del.title = 'Xóa cài sẵn này';
    del.textContent = '×';
    del.addEventListener('click', (e) => {
      e.stopPropagation();
      state.userPresets.splice(idx, 1);
      persistUserPresets(state.userPresets);
      renderUserPresetsList();
    });

    btn.appendChild(thumb);
    btn.appendChild(badge);
    btn.appendChild(meta);
    btn.appendChild(del);

    btn.addEventListener('click', () => {
      const curr = getActivePhoto();
      if (!curr) return;
      document.querySelectorAll('.retouch-preset-item').forEach((c) => c.classList.remove('active'));
      btn.classList.add('active');
      const ownFace = curr.params.face;
      curr.params = { ...createDefaultParams(), ...JSON.parse(JSON.stringify(preset.params)) };
      curr.params.face = ownFace;
      syncSlidersFromParams();
      commitHistory(curr);
      processActivePhoto(false);
    });

    container.appendChild(btn);
  });
}

function saveCurrentAsUserPreset() {
  const photo = getActivePhoto();
  if (!photo) return;
  let nextNum = state.userPresets.length + 1;
  const usedNames = new Set(state.userPresets.map((u) => u.name));
  while (usedNames.has(`Cài sẵn #${nextNum}`)) nextNum++;
  const name = `Cài sẵn #${nextNum}`;
  const saved = JSON.parse(JSON.stringify(photo.params));
  state.userPresets.push({ name, params: saved });
  persistUserPresets(state.userPresets);
  renderUserPresetsList();
  renderTimeLabel.textContent = `Đã lưu "${name}" ✓`;
}

async function openNativeImportDialog() {
  if (!hasTauriRuntime()) {
    document.getElementById('file-input')?.click();
    return;
  }
  try {
    const selected = await open({
      multiple: true,
      title: 'Nhập ảnh & Camera RAW (.CR2, .CR3, .NEF, .ARW, .DNG, .JPG)',
      filters: [
        {
          name: 'Tất cả ảnh & Camera RAW hỗ trợ',
          extensions: [
            'cr2', 'CR2', 'cr3', 'CR3', 'crw', 'CRW',
            'nef', 'NEF', 'arw', 'ARW', 'dng', 'DNG',
            'orf', 'ORF', 'rw2', 'RW2', 'raf', 'RAF',
            'pef', 'PEF', 'srw', 'SRW', 'heic', 'HEIC',
            'jpg', 'JPG', 'jpeg', 'JPEG', 'png', 'PNG',
            'webp', 'WEBP', 'bmp', 'BMP', 'tiff', 'TIFF', 'tif', 'TIF',
          ],
        },
      ],
    });

    if (!selected) return;
    const paths = Array.isArray(selected) ? selected : [selected];
    if (paths.length > 0) {
      await importPathsFromRust(paths);
    }
  } catch (err) {
    console.warn('Chuyển sang chọn file trình duyệt:', err);
    document.getElementById('file-input')?.click();
  }
}

async function importPathsFromRust(paths) {
  renderTimeLabel.textContent = 'Đang nhập & quét AI 478 điểm khuôn mặt...';
  let importedCount = 0;
  let lastError = null;
  for (const filePath of paths) {
    try {
      const payload = await invoke('load_photo_from_path_cmd', { filePath });
      const params = createDefaultParams();
      const imgEl = await loadImageElement(payload.dataUrl);
      const aiResult = await analyzePhotoWithNeuralAI(imgEl, payload.detectedFace);
      params.face = aiResult.face;

      state.photos.push({
        id: Date.now() + Math.random(),
        name: payload.name,
        originalDataUrl: payload.dataUrl,
        processedDataUrl: payload.dataUrl,
        width: payload.width,
        height: payload.height,
        mesh478: aiResult.mesh478,
        maskBase64: aiResult.maskBase64,
        sceneMaskBase64: aiResult.sceneMaskBase64,
        hairBounds: aiResult.hairBounds || null,
        landmarksDetected: aiResult.landmarksDetected,
        params,
      });
      initPhotoHistory(state.photos[state.photos.length - 1]);
      state.activeIndex = state.photos.length - 1;
      importedCount++;
    } catch (e) {
      lastError = e;
      console.error('Lỗi tải tệp qua Rust:', filePath, e);
    }
  }
  if (importedCount === 0) {
    const detail = lastError ? ` (${String(lastError)})` : '';
    renderTimeLabel.textContent = `Không thể mở tệp ảnh đã chọn${detail}`;
    return;
  }
  document.querySelectorAll('.retouch-preset-item').forEach((c) => c.classList.remove('active'));
  syncSlidersFromParams();
  await processActivePhoto(false);
}

async function importHtmlFileList(fileList) {
  if (!fileList || fileList.length === 0) return;
  renderTimeLabel.textContent = 'Đang nhập ảnh & quét AI 478 điểm...';
  let importedCount = 0;
  let failedName = '';

  for (const file of Array.from(fileList)) {
    try {
      const dataUrl = await new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => resolve(reader.result);
        reader.onerror = () => reject(reader.error || new Error('FileReader error'));
        reader.readAsDataURL(file);
      });

      const img = await loadImageElement(dataUrl);
      const params = createDefaultParams();

      let fallbackFace = params.face;
      if (hasTauriRuntime()) {
        try {
          const detected = await invoke('analyze_portrait_cmd', { imageBase64: dataUrl });
          if (detected) fallbackFace = detected;
        } catch (_) {}
      }

      const aiResult = await analyzePhotoWithNeuralAI(img, fallbackFace);
      params.face = aiResult.face;

      state.photos.push({
        id: Date.now() + Math.random(),
        name: file.name,
        originalDataUrl: dataUrl,
        processedDataUrl: dataUrl,
        width: img.naturalWidth,
        height: img.naturalHeight,
        mesh478: aiResult.mesh478,
        maskBase64: aiResult.maskBase64,
        sceneMaskBase64: aiResult.sceneMaskBase64,
        hairBounds: aiResult.hairBounds || null,
        landmarksDetected: aiResult.landmarksDetected,
        params,
      });
      initPhotoHistory(state.photos[state.photos.length - 1]);
      state.activeIndex = state.photos.length - 1;
      importedCount++;
    } catch (err) {
      failedName = file?.name || '';
      console.error('Lỗi đọc tệp ảnh trên trình duyệt:', failedName, err);
    }
  }

  if (importedCount === 0) {
    const isRaw = /\.(dng|cr2|cr3|crw|nef|arw|orf|rw2|raf|pef|srw|heic)$/i.test(failedName);
    renderTimeLabel.textContent = isRaw && !hasTauriRuntime()
      ? `Trình duyệt không giải mã được RAW (${failedName}) — hãy chạy bằng app Desktop (tauri dev)`
      : `Không thể đọc tệp ảnh ${failedName}`;
    return;
  }

  document.querySelectorAll('.retouch-preset-item').forEach((c) => c.classList.remove('active'));
  syncSlidersFromParams();
  await processActivePhoto(false);
}

function computeColorMatchStatsFromImage(imgEl) {
  const c = document.createElement('canvas');
  c.width = 128;
  c.height = 128;
  const ctx = c.getContext('2d');
  ctx.drawImage(imgEl, 0, 0, 128, 128);
  const data = ctx.getImageData(0, 0, 128, 128).data;
  const n = 128 * 128;
  let sr = 0, sg = 0, sb = 0;
  for (let i = 0; i < data.length; i += 4) {
    sr += data[i] / 255;
    sg += data[i + 1] / 255;
    sb += data[i + 2] / 255;
  }
  const mr = sr / n;
  const mg = sg / n;
  const mb = sb / n;
  let vr = 0, vg = 0, vb = 0;
  for (let i = 0; i < data.length; i += 4) {
    vr += Math.pow(data[i] / 255 - mr, 2);
    vg += Math.pow(data[i + 1] / 255 - mg, 2);
    vb += Math.pow(data[i + 2] / 255 - mb, 2);
  }
  return [
    mr,
    Math.max(0.02, Math.sqrt(vr / n)),
    mg,
    Math.max(0.02, Math.sqrt(vg / n)),
    mb,
    Math.max(0.02, Math.sqrt(vb / n)),
  ];
}

function applyResetActivePhoto() {
  const photo = getActivePhoto();
  if (!photo) return;
  const faceBackup = photo.params.face;
  photo.params = createDefaultParams();
  photo.params.face = faceBackup;
  photo.curvePoints = null;
  state.cropPreviewMode = false;
  document.querySelectorAll('.retouch-preset-item').forEach((c) => c.classList.remove('active'));
  syncSlidersFromParams();
  commitHistory(photo);
  processActivePhoto(false);
}

async function applySyncToAllPhotos() {
  const source = getActivePhoto();
  if (!source) return;
  if (!hasTauriRuntime()) {
    for (let i = 0; i < state.photos.length; i++) {
      if (i === state.activeIndex) continue;
      const target = state.photos[i];
      const ownFace = target.params.face;
      target.params = JSON.parse(JSON.stringify(source.params));
      target.params.face = ownFace;
      target.params.blemishSpots = [];
      commitHistory(target);
    }
    renderFilmstrip();
    if (state.workspaceMode === 'grid') renderLibraryGrid();
    renderTimeLabel.textContent = `Đã đồng bộ thông số cho ${state.photos.length} ảnh ✓`;
    return;
  }

  renderTimeLabel.textContent = `Đang đồng bộ ${state.photos.length} ảnh...`;

  for (let i = 0; i < state.photos.length; i++) {
    if (i === state.activeIndex) continue;
    const target = state.photos[i];
    const ownFace = target.params.face;
    target.params = JSON.parse(JSON.stringify(source.params));
    target.params.face = ownFace;
    target.params.blemishSpots = [];
    try {
      if (needsHighresMatte(target.params)) {
        await ensureHighresMatteForPhoto(target);
      }
      const res = await invoke('process_photo_cmd', {
        imageBase64: target.originalDataUrl,
        params: buildInvokeParams(target),
        highQuality: false,
        maskBase64: target.maskBase64 || null,
        sceneMaskBase64: getEffectiveSceneMask(target),
      });
      target.processedDataUrl = res.dataUrl;
      commitHistory(target);
    } catch (err) {
      console.error('Lỗi đồng bộ ảnh:', target.name, err);
    }
  }
  renderFilmstrip();
  if (state.workspaceMode === 'grid') renderLibraryGrid();
  renderTimeLabel.textContent = `Đã đồng bộ xong ${state.photos.length} ảnh ✓`;
}

function setupRetouchEvents() {
  const retouchBody = document.getElementById('retouch-body');

  // Chế độ Thư viện ảnh | Tinh chỉnh
  document.getElementById('btn-mode-library')?.addEventListener('click', () => setWorkspaceMode('grid'));
  document.getElementById('btn-mode-develop')?.addEventListener('click', () => setWorkspaceMode('develop'));

  // Thanh icon bên phải (Evoto Right Rail)
  const rightHeading = document.getElementById('right-module-title');
  document.querySelectorAll('.rail-icon-btn').forEach((btn) => {
    btn.addEventListener('click', () => {
      document.querySelectorAll('.rail-icon-btn').forEach((b) => b.classList.remove('active'));
      document.querySelectorAll('.retouch-module-pane').forEach((p) => p.classList.remove('active'));
      btn.classList.add('active');
      const nextMod = btn.dataset.mod || 'mod-portrait';
      document.getElementById(nextMod)?.classList.add('active');
      state.activeModule = nextMod;
      state.cropPreviewMode = false;
      document.getElementById('crop-ratio-menu')?.classList.add('hidden');
      document.getElementById('crop-ratio-btn')?.setAttribute('aria-expanded', 'false');
      if (rightHeading && btn.dataset.title) {
        rightHeading.textContent = btn.dataset.title;
      }
      updateCropControlsUI(getActivePhoto());
      renderViewport();
    });
  });

  // Thanh icon & bảng nổi bên trái (có thể thu gọn / mở rộng)
  const leftHeading = document.getElementById('left-panel-heading');
  const savePresetHeaderBtn = document.getElementById('btn-save-user-preset');
  const clearHistoryHeaderBtn = document.getElementById('btn-clear-history');

  const syncLeftHeaderActions = (tabId) => {
    savePresetHeaderBtn?.classList.toggle('hidden', tabId !== 'ltab-presets');
    clearHistoryHeaderBtn?.classList.toggle('hidden', tabId !== 'ltab-history');
  };

  document.querySelectorAll('.left-rail-btn').forEach((btn) => {
    btn.addEventListener('click', () => {
      const targetTab = btn.dataset.ltab;
      const isAlreadyOpen =
        btn.classList.contains('active') && !retouchBody?.classList.contains('left-collapsed');
      if (isAlreadyOpen) {
        retouchBody?.classList.add('left-collapsed');
        btn.classList.remove('active');
        return;
      }
      retouchBody?.classList.remove('left-collapsed');
      document.querySelectorAll('.left-rail-btn').forEach((t) => t.classList.remove('active'));
      document.querySelectorAll('.left-tab-content').forEach((c) => c.classList.remove('active'));
      btn.classList.add('active');
      document.getElementById(targetTab)?.classList.add('active');
      if (leftHeading) {
        const headingMap = {
          'ltab-presets': 'Cài sẵn',
          'ltab-history': 'Lịch sử',
          'ltab-project': 'Dự án & AI',
          'ltab-info': 'Thông tin & Hướng dẫn',
        };
        leftHeading.textContent = headingMap[targetTab] || 'Dự án & AI';
      }
      syncLeftHeaderActions(targetTab);
      if (targetTab === 'ltab-history') {
        renderHistoryPanel(getActivePhoto());
      }
    });
  });

  clearHistoryHeaderBtn?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo) return;
    initPhotoHistory(photo);
    if (renderTimeLabel) {
      renderTimeLabel.textContent = 'Đã xóa lịch sử chỉnh sửa (giữ trạng thái hiện tại) ✓';
    }
  });

  document.querySelectorAll('#ltab-presets .preset-cat-header').forEach((header) => {
    header.addEventListener('click', () => {
      const cat = header.closest('.preset-category');
      if (!cat) return;
      const collapsed = cat.classList.toggle('collapsed');
      const labelSpan = header.querySelector('span') || header;
      const raw = labelSpan.textContent.replace(/^[▾▸]\s*/, '');
      labelSpan.textContent = `${collapsed ? '▸' : '▾'} ${raw}`;
    });
  });

  document.getElementById('btn-close-left')?.addEventListener('click', () => {
    retouchBody?.classList.add('left-collapsed');
    document.querySelectorAll('.left-rail-btn').forEach((t) => t.classList.remove('active'));
  });

  // Lọc phạm vi Preset: Tất cả | Đề xuất | Của tôi
  const recWrap = document.getElementById('recommended-presets-wrap');
  const mineWrap = document.getElementById('user-presets-wrap');
  document.querySelectorAll('.preset-scope-btn').forEach((btn) => {
    btn.addEventListener('click', () => {
      state.presetScope = btn.dataset.scope || 'all';
      document.querySelectorAll('.preset-scope-btn').forEach((b) => b.classList.remove('active'));
      btn.classList.add('active');
      if (recWrap) recWrap.style.display = state.presetScope === 'mine' ? 'none' : '';
      if (mineWrap) mineWrap.style.display = state.presetScope === 'recommended' ? 'none' : '';
    });
  });

  // Đóng / Mở Biểu đồ & Công tắc Chế độ chỉnh màu thời gian thực
  document.getElementById('btn-toggle-histogram')?.addEventListener('click', () => {
    state.histOpen = !state.histOpen;
    const body = document.getElementById('hist-body-wrap');
    const title = document.getElementById('btn-toggle-histogram');
    if (body) body.style.display = state.histOpen ? '' : 'none';
    const firstSpan = title?.querySelector('span');
    if (firstSpan) firstSpan.textContent = state.histOpen ? '▾ Biểu đồ' : '▸ Biểu đồ';
  });

  const realtimeSwitch = document.getElementById('toggle-realtime-color');
  realtimeSwitch?.addEventListener('click', () => {
    state.realtimeColor = !state.realtimeColor;
    realtimeSwitch.classList.toggle('on', state.realtimeColor);
    realtimeSwitch.setAttribute('aria-checked', String(state.realtimeColor));
  });

  // Accordion đóng / mở mượt mà
  document.querySelectorAll('.retouch-acc-title').forEach((header) => {
    header.addEventListener('click', () => {
      header.parentElement.classList.toggle('collapsed');
    });
  });

  // HSL 8 dải màu
  document.querySelectorAll('.hsl-dot').forEach((dot) => {
    dot.addEventListener('click', () => {
      const channel = dot.dataset.hsl;
      document.querySelectorAll('.hsl-dot').forEach((d) => d.classList.remove('active'));
      document.querySelectorAll('.hsl-band-pane').forEach((p) => p.classList.remove('active'));
      dot.classList.add('active');
      document.querySelector(`.hsl-band-pane[data-hsl-pane="${channel}"]`)?.classList.add('active');
    });
  });

  // Ô chọn nhanh Bầu trời & Phông Studio
  document.querySelectorAll('.asset-swatch').forEach((swatch) => {
    swatch.addEventListener('click', () => {
      const mode = swatch.dataset.bgmode;
      if (!mode) return;
      const photo = getActivePhoto();
      if (!photo) return;
      if (mode === 'custom_image' && !photo.params.bgCustomImage) {
        if (state.customBgGallery.length > 0) {
          photo.params.bgCustomImage = state.customBgGallery[0].dataUrl;
        } else {
          state.bgCategory = 'image';
          state.bgLibTab = 'user';
          updateBgCustomControlsUI(photo.params);
          document.getElementById('bg-custom-file-input')?.click();
          return;
        }
      }
      photo.params.bgReplaceMode = mode;
      updateBgCustomControlsUI(photo.params);
      commitHistory(photo);
      processActivePhoto(false);
    });
  });

  // Tương tác Đường cong tông 256-LUT
  if (curveCanvas) {
    curveCanvas.addEventListener('mousedown', (e) => {
      const photo = getActivePhoto();
      if (!photo) return;
      const rect = curveCanvas.getBoundingClientRect();
      const nx = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
      const ny = Math.max(0, Math.min(1, 1 - (e.clientY - rect.top) / rect.height));
      const pts = getPhotoCurvePoints(photo);

      let hitIdx = -1;
      for (let i = 0; i < pts.length; i++) {
        if (Math.hypot(pts[i].x - nx, pts[i].y - ny) < 0.06) {
          hitIdx = i;
          break;
        }
      }

      if (hitIdx === -1) {
        pts.push({ x: nx, y: ny });
        pts.sort((a, b) => a.x - b.x);
        hitIdx = pts.findIndex((p) => Math.abs(p.x - nx) < 1e-4 && Math.abs(p.y - ny) < 1e-4);
      }
      state.draggingCurveIdx = hitIdx;
      photo.params.curveLut = build256CurveLutFromPoints(pts);
      renderToneCurveCanvas();
      processActivePhoto(false);
    });

    curveCanvas.addEventListener('dblclick', (e) => {
      const photo = getActivePhoto();
      if (!photo) return;
      const rect = curveCanvas.getBoundingClientRect();
      const nx = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
      const ny = Math.max(0, Math.min(1, 1 - (e.clientY - rect.top) / rect.height));
      const pts = getPhotoCurvePoints(photo);
      for (let i = 1; i < pts.length - 1; i++) {
        if (Math.hypot(pts[i].x - nx, pts[i].y - ny) < 0.07) {
          pts.splice(i, 1);
          photo.params.curveLut = build256CurveLutFromPoints(pts);
          renderToneCurveCanvas();
          commitHistory(photo);
          processActivePhoto(false);
          break;
        }
      }
    });
  }

  document.getElementById('btn-reset-curve')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo) return;
    photo.curvePoints = [
      { x: 0, y: 0 },
      { x: 0.25, y: 0.25 },
      { x: 0.5, y: 0.5 },
      { x: 0.75, y: 0.75 },
      { x: 1, y: 1 },
    ];
    photo.params.curveLut = [];
    renderToneCurveCanvas();
    commitHistory(photo);
    processActivePhoto(false);
  });

  // AI Khớp màu tham chiếu
  const refInput = document.getElementById('ref-color-input');
  document.getElementById('btn-load-color-ref')?.addEventListener('click', () => refInput?.click());
  refInput?.addEventListener('change', async (e) => {
    const file = e.target.files?.[0];
    const photo = getActivePhoto();
    if (!file || !photo) return;
    const dataUrl = await new Promise((resolve) => {
      const reader = new FileReader();
      reader.onload = () => resolve(reader.result);
      reader.readAsDataURL(file);
    });
    const imgEl = await loadImageElement(dataUrl);
    photo.params.colorMatchStats = computeColorMatchStatsFromImage(imgEl);
    if (photo.params.colorMatchStrength === 0) {
      photo.params.colorMatchStrength = 65;
    }
    syncSlidersFromParams();
    commitHistory(photo);
    processActivePhoto(false);
    refInput.value = '';
  });

  document.getElementById('btn-clear-color-ref')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo) return;
    photo.params.colorMatchStats = [];
    photo.params.colorMatchStrength = 0;
    syncSlidersFromParams();
    commitHistory(photo);
    processActivePhoto(false);
  });

  // Tìm kiếm Cài sẵn
  document.getElementById('preset-search')?.addEventListener('input', (e) => {
    const q = e.target.value.toLowerCase();
    document.querySelectorAll('.retouch-preset-item').forEach((item) => {
      const txt = item.textContent.toLowerCase();
      item.style.display = txt.includes(q) ? 'flex' : 'none';
    });
  });

  // Toàn bộ 152 thanh trượt + reset bằng double-click hoặc click vào giá trị số
  const defaultsSnapshot = createDefaultParams();
  document.querySelectorAll('.retouch-slider').forEach((slider) => {
    const key = slider.id;
    const resetOneSlider = () => {
      const photo = getActivePhoto();
      if (!photo) return;
      const def = defaultsSnapshot[key];
      slider.value = def;
      photo.params[key] = def;
      const badge = document.querySelector(`.val[data-for="${key}"]`);
      if (badge) {
        badge.textContent = formatSliderBadgeValue(def, key);
        badge.classList.remove('modified');
      }
      if (key === 'rotateAngle') {
        constrainCropBoxInsideRotatedBounds(photo, def);
        updateCropControlsUI(photo);
      }
      commitHistory(photo);
      processActivePhoto(false);
    };

    if (key === 'rotateAngle') {
      const startRotDrag = () => {
        state.draggingRotateAngle = true;
        const photo = getActivePhoto();
        if (photo && overlayCanvas.width && overlayCanvas.height) {
          drawOverlayGuides(overlayCanvas.width, overlayCanvas.height, photo);
        }
      };
      slider.addEventListener('mousedown', startRotDrag);
      slider.addEventListener('touchstart', startRotDrag, { passive: true });
    }

    slider.addEventListener('input', () => {
      const photo = getActivePhoto();
      if (!photo) return;
      const val = parseFloat(slider.value);
      photo.params[key] = val;
      const badge = document.querySelector(`.val[data-for="${key}"]`);
      if (badge) {
        badge.textContent = formatSliderBadgeValue(val, key);
        badge.classList.toggle('modified', Math.abs(val - Number(defaultsSnapshot[key])) > 1e-4);
      }
      if (
        state.clothWrinkleLinked &&
        (key === 'clothingSmallWrinkles' || key === 'clothingLargeWrinkles')
      ) {
        const pairedKey =
          key === 'clothingSmallWrinkles' ? 'clothingLargeWrinkles' : 'clothingSmallWrinkles';
        photo.params[pairedKey] = val;
        const pairedEl = document.getElementById(pairedKey);
        const pairedBadge = document.querySelector(`.val[data-for="${pairedKey}"]`);
        if (pairedEl) pairedEl.value = String(val);
        if (pairedBadge) {
          pairedBadge.textContent = formatSliderBadgeValue(val, pairedKey);
          pairedBadge.classList.toggle(
            'modified',
            Math.abs(val - Number(defaultsSnapshot[pairedKey])) > 1e-4
          );
        }
      }
      if (
        state.hairSideLinked &&
        (key === 'hairSideLeftVolume' || key === 'hairSideRightVolume')
      ) {
        const pairedKey =
          key === 'hairSideLeftVolume' ? 'hairSideRightVolume' : 'hairSideLeftVolume';
        photo.params[pairedKey] = val;
        const pairedEl = document.getElementById(pairedKey);
        const pairedBadge = document.querySelector(`.val[data-for="${pairedKey}"]`);
        if (pairedEl) pairedEl.value = String(val);
        if (pairedBadge) {
          pairedBadge.textContent = formatSliderBadgeValue(val, pairedKey);
          pairedBadge.classList.toggle(
            'modified',
            Math.abs(val - Number(defaultsSnapshot[pairedKey])) > 1e-4
          );
        }
      }
      if (key === 'rotateAngle') {
        constrainCropBoxInsideRotatedBounds(photo, val);
        updateCropControlsUI(photo);
        if (overlayCanvas.width && overlayCanvas.height) {
          drawOverlayGuides(overlayCanvas.width, overlayCanvas.height, photo);
        }
      }
      if (state.realtimeColor) {
        processActivePhoto(false);
      }
    });

    slider.addEventListener('change', () => {
      const photo = getActivePhoto();
      if (key === 'rotateAngle' && state.draggingRotateAngle) {
        state.draggingRotateAngle = false;
        if (photo && overlayCanvas.width && overlayCanvas.height) {
          drawOverlayGuides(overlayCanvas.width, overlayCanvas.height, photo);
        }
      }
      if (photo) commitHistory(photo);
      if (!state.realtimeColor) {
        processActivePhoto(false);
      }
    });

    slider.addEventListener('dblclick', resetOneSlider);
    const badge = document.querySelector(`.val[data-for="${key}"]`);
    if (badge) {
      badge.title = 'Nhấp vào số để đặt lại mặc định';
      badge.addEventListener('click', resetOneSlider);
    }
  });

  // Select Dropdowns
  for (const selId of ['lutPreset', 'treatment', 'makeupLipTint', ...BG_SELECT_IDS]) {
    document.getElementById(selId)?.addEventListener('change', (e) => {
      const photo = getActivePhoto();
      if (!photo) return;
      photo.params[selId] = e.target.value;
      if (BG_SELECT_IDS.includes(selId)) {
        updateBgCustomControlsUI(photo.params);
      }
      commitHistory(photo);
      processActivePhoto(false);
    });
  }

  // Tùy chỉnh màu nền, thư viện nền & điều khiển phông nền Evoto
  const handleCustomColorPick = (e) => {
    const photo = getActivePhoto();
    if (!photo) return;
    photo.params.bgCustomColor = e.target.value;
    photo.params.bgReplaceMode = 'custom_color';
    updateBgCustomControlsUI(photo.params);
    if (e.type === 'change') commitHistory(photo);
    processActivePhoto(false);
  };
  document.getElementById('bgCustomColor')?.addEventListener('input', handleCustomColorPick);
  document.getElementById('bgCustomColor')?.addEventListener('change', handleCustomColorPick);

  document.querySelectorAll('.bg-color-dot').forEach((dot) => {
    dot.addEventListener('click', () => {
      const photo = getActivePhoto();
      const hex = dot.dataset.color;
      if (!photo || !hex) return;
      photo.params.bgCustomColor = hex;
      photo.params.bgReplaceMode = 'custom_color';
      updateBgCustomControlsUI(photo.params);
      commitHistory(photo);
      processActivePhoto(false);
    });
  });

  const customBgInput = document.getElementById('bg-custom-file-input');
  document.getElementById('btn-upload-custom-bg')?.addEventListener('click', () => customBgInput?.click());
  customBgInput?.addEventListener('change', async (e) => {
    const files = Array.from(e.target.files || []);
    const photo = getActivePhoto();
    if (files.length === 0 || !photo) return;
    const uploaded = [];
    for (const file of files) {
      const rawUrl = await new Promise((resolve) => {
        const reader = new FileReader();
        reader.onload = () => resolve(reader.result);
        reader.readAsDataURL(file);
      });
      const bgImg = await loadImageElement(rawUrl);
      const maxDim = 2048;
      const scale = Math.min(1, maxDim / Math.max(bgImg.naturalWidth || 1, bgImg.naturalHeight || 1));
      const bw = Math.max(1, Math.round((bgImg.naturalWidth || 1) * scale));
      const bh = Math.max(1, Math.round((bgImg.naturalHeight || 1) * scale));
      const c = document.createElement('canvas');
      c.width = bw;
      c.height = bh;
      c.getContext('2d').drawImage(bgImg, 0, 0, bw, bh);
      const dataUrl = file.type === 'image/png' ? c.toDataURL('image/png') : c.toDataURL('image/jpeg', 0.92);
      const item = { id: `${Date.now()}-${Math.random().toString(36).slice(2, 7)}`, name: file.name, dataUrl, fav: false };
      state.customBgGallery.unshift(item);
      uploaded.push(item);
    }
    if (uploaded.length > 0) {
      photo.params.bgCustomImage = uploaded[0].dataUrl;
      photo.params.bgReplaceMode = 'custom_image';
    }
    state.bgCategory = 'image';
    state.bgLibTab = 'user';
    customBgInput.value = '';
    updateBgCustomControlsUI(photo.params);
    commitHistory(photo);
    processActivePhoto(false);
  });

  document.getElementById('btn-clear-custom-bg')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo) return;
    photo.params.bgCustomImage = '';
    if (photo.params.bgReplaceMode === 'custom_image') {
      photo.params.bgReplaceMode = 'original';
    }
    updateBgCustomControlsUI(photo.params);
    commitHistory(photo);
    processActivePhoto(false);
  });

  document.getElementById('bg-cat-image')?.addEventListener('click', () => {
    state.bgCategory = 'image';
    updateBgCustomControlsUI(getActivePhoto()?.params || createDefaultParams());
  });
  document.getElementById('bg-cat-solid')?.addEventListener('click', () => {
    state.bgCategory = 'solid';
    updateBgCustomControlsUI(getActivePhoto()?.params || createDefaultParams());
  });
  document.getElementById('bg-lib-tab-rec')?.addEventListener('click', () => {
    state.bgLibTab = 'rec';
    updateBgCustomControlsUI(getActivePhoto()?.params || createDefaultParams());
  });
  document.getElementById('bg-lib-tab-user')?.addEventListener('click', () => {
    state.bgLibTab = 'user';
    updateBgCustomControlsUI(getActivePhoto()?.params || createDefaultParams());
  });
  document.getElementById('bg-filter-explore')?.addEventListener('click', () => {
    state.bgFilter = 'explore';
    updateBgCustomControlsUI(getActivePhoto()?.params || createDefaultParams());
  });
  document.getElementById('bg-filter-selected')?.addEventListener('click', () => {
    state.bgFilter = 'selected';
    updateBgCustomControlsUI(getActivePhoto()?.params || createDefaultParams());
  });
  document.getElementById('bg-user-filter-personal')?.addEventListener('click', () => {
    state.bgUserFilter = 'personal';
    updateBgCustomControlsUI(getActivePhoto()?.params || createDefaultParams());
  });
  document.getElementById('bg-user-filter-fav')?.addEventListener('click', () => {
    state.bgUserFilter = 'fav';
    updateBgCustomControlsUI(getActivePhoto()?.params || createDefaultParams());
  });
  document.getElementById('btn-bg-more')?.addEventListener('click', () => {
    state.bgMoreExpanded = !state.bgMoreExpanded;
    updateBgCustomControlsUI(getActivePhoto()?.params || createDefaultParams());
  });
  document.getElementById('btn-bg-reset-original')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo) return;
    photo.params.bgReplaceMode = 'original';
    updateBgCustomControlsUI(photo.params);
    commitHistory(photo);
    processActivePhoto(false);
  });

  for (const swId of BG_SWITCH_KEYS) {
    document.getElementById(swId)?.addEventListener('click', () => {
      const photo = getActivePhoto();
      if (!photo) return;
      photo.params[swId] = !Boolean(photo.params[swId]);
      updateBgCustomControlsUI(photo.params);
      commitHistory(photo);
      processActivePhoto(false);
    });
  }

  document.querySelectorAll('.bg-shadow-card').forEach((card) => {
    card.addEventListener('click', () => {
      const photo = getActivePhoto();
      if (!photo) return;
      const sh = card.dataset.shadow || 'none';
      photo.params.bgShadowType = sh;
      if (sh === 'none') {
        photo.params.bgShadowOpacity = 0;
      } else if (!(photo.params.bgShadowOpacity > 0)) {
        photo.params.bgShadowOpacity = sh === 'hard_shadow' ? 65 : sh === 'cast_shadow' ? 55 : 45;
      }
      const opSlider = document.getElementById('bgShadowOpacity');
      const opBadge = document.querySelector('.val[data-for="bgShadowOpacity"]');
      if (opSlider) opSlider.value = String(photo.params.bgShadowOpacity);
      if (opBadge) opBadge.textContent = formatSliderBadgeValue(photo.params.bgShadowOpacity, 'bgShadowOpacity');
      updateBgCustomControlsUI(photo.params);
      commitHistory(photo);
      processActivePhoto(false);
    });
  });

  // LiYing AI: Tách & Thay nền chuẩn ảnh thẻ / Chân dung (Offline)
  let selectedLiyingColor = 'blue';
  let liyingRunSeq = 0;
  let liyingInFlight = false;

  document.querySelectorAll('.liying-color-chip').forEach((chip) => {
    chip.addEventListener('click', () => {
      document.querySelectorAll('.liying-color-chip').forEach((c) => c.classList.remove('active'));
      chip.classList.add('active');
      selectedLiyingColor = chip.dataset.color || 'blue';
    });
  });

  async function executeLiyingBackgroundReplacement(levelOnly = false) {
    const photo = getActivePhoto();
    if (!photo || !photo.originalDataUrl) return;

    if (!hasTauriRuntime()) {
      alert('Tính năng LiYing AI yêu cầu chạy trong ứng dụng Collabs-Retouch-Pro (Tauri).');
      return;
    }

    const reqSeq = ++liyingRunSeq;
    liyingInFlight = true;

    const straighten = levelOnly ? true : Boolean(document.getElementById('liying-straighten-cb')?.checked);
    const color = selectedLiyingColor || 'blue';
    const transparent = color === 'transparent';

    // Capture pre-LiYing source state once so "Hủy" can cleanly revert both background & shoulder-level changes
    if (!photo._liyingBackup) {
      photo._liyingBackup = {
        originalDataUrl: photo.originalDataUrl,
        processedDataUrl: photo.processedDataUrl,
        width: photo.width,
        height: photo.height,
        face: photo.params?.face ? JSON.parse(JSON.stringify(photo.params.face)) : null,
        mesh478: photo.mesh478 || null,
        maskBase64: photo.maskBase64 || null,
        sceneMaskBase64: photo.sceneMaskBase64 || null,
        highresMatteBase64: photo.highresMatteBase64 || null,
        _matteSourceRef: photo._matteSourceRef || null,
        landmarksDetected: Boolean(photo.landmarksDetected),
        bgReplaceMode: photo.params?.bgReplaceMode || 'original',
        bgCustomColor: photo.params?.bgCustomColor || '#628bce',
      };
    }

    renderTimeLabel.textContent = levelOnly
      ? 'Đang cân bằng vai & bố cục LiYing AI...'
      : 'Đang tách & thay nền LiYing AI (RMBG + Pose)...';

    try {
      const res = await invoke('liying_change_background_cmd', {
        imageBase64: photo.originalDataUrl,
        color,
        transparent,
        straighten,
        levelOnly,
      });

      if (reqSeq !== liyingRunSeq) {
        return;
      }

      if (res && res.data_url) {
        if (levelOnly || straighten) {
          photo.originalDataUrl = res.data_url;
          photo.processedDataUrl = res.data_url;
          photo.width = res.width;
          photo.height = res.height;
          photo._subjectBoundsCache = null;

          try {
            const imgEl = await loadImageElement(photo.originalDataUrl);
            const aiResult = await analyzePhotoWithNeuralAI(imgEl, photo.params.face);
            if (reqSeq !== liyingRunSeq) return;
            photo.params.face = aiResult.face;
            photo.mesh478 = aiResult.mesh478;
            photo.maskBase64 = aiResult.maskBase64;
            photo.sceneMaskBase64 = aiResult.sceneMaskBase64;
            photo.hairBounds = aiResult.hairBounds || null;
            photo._hairGuideCanvas = null;
            photo._hairGuideMaskRef = null;
            photo.landmarksDetected = aiResult.landmarksDetected;
          } catch (_) {}
        } else {
          photo.processedDataUrl = res.data_url;
        }

        if (res.matte_data_url) {
          photo.highresMatteBase64 = res.matte_data_url;
          photo._matteSourceRef = photo.originalDataUrl;
        }

        if (!levelOnly) {
          if (transparent) {
            photo.params.bgReplaceMode = 'transparent';
          } else if (color === 'white') {
            photo.params.bgReplaceMode = 'pure_white';
          } else if (color === 'blue' || color === 'passport_blue') {
            photo.params.bgReplaceMode = 'passport_blue';
          } else {
            photo.params.bgReplaceMode = 'custom_color';
            const hexMap = {
              red: '#d74532',
              dark_blue: '#0047ab',
              light_blue: '#b4c8e6',
              gray: '#808080',
              beige: '#f5f5dc',
            };
            photo.params.bgCustomColor = hexMap[color] || '#628bce';
          }
          updateBgCustomControlsUI(photo.params);
        }

        const statusTag = levelOnly
          ? 'LiYing Cân bằng vai ✓'
          : res.straightened
          ? 'LiYing AI (Cân vai + Nền) ✓'
          : 'LiYing AI Thay nền ✓';
        renderTimeLabel.textContent = `${statusTag} • ${res.elapsed_ms}ms`;

        photo._isLiyingOutput = true;
        commitHistory(photo);

        await renderViewport();
        renderFilmstrip();
        if (state.workspaceMode === 'grid') renderLibraryGrid();
      }
    } catch (err) {
      if (reqSeq !== liyingRunSeq) return;
      console.error('Lỗi chạy LiYing AI:', err);
      alert('Lỗi xử lý LiYing AI: ' + (err?.message || err));
      renderTimeLabel.textContent = 'Lỗi LiYing AI';
    } finally {
      if (reqSeq === liyingRunSeq) {
        liyingInFlight = false;
      }
    }
  }

  async function cancelLiyingBackgroundReplacement() {
    const wasRunning = liyingInFlight;
    liyingRunSeq++;
    liyingInFlight = false;

    const photo = getActivePhoto();
    if (!photo) return;

    if (wasRunning) {
      renderTimeLabel.textContent = 'Đã hủy xử lý LiYing AI';
      return;
    }

    const backup = photo._liyingBackup;
    const hadBgChange =
      Boolean(backup) ||
      Boolean(photo._isLiyingOutput) ||
      (photo.params?.bgReplaceMode && photo.params.bgReplaceMode !== 'original');

    if (!hadBgChange) {
      renderTimeLabel.textContent = 'Ảnh đang ở trạng thái phông nền gốc';
      return;
    }

    if (backup) {
      photo.originalDataUrl = backup.originalDataUrl;
      photo.processedDataUrl = backup.processedDataUrl;
      photo.width = backup.width;
      photo.height = backup.height;
      if (backup.face) photo.params.face = JSON.parse(JSON.stringify(backup.face));
      photo.mesh478 = backup.mesh478;
      photo.maskBase64 = backup.maskBase64;
      photo.sceneMaskBase64 = backup.sceneMaskBase64;
      photo.highresMatteBase64 = backup.highresMatteBase64;
      photo._matteSourceRef = backup._matteSourceRef;
      photo.landmarksDetected = backup.landmarksDetected;
      photo.params.bgCustomColor = backup.bgCustomColor;
      photo._subjectBoundsCache = null;
      photo._liyingBackup = null;
    }

    photo._isLiyingOutput = false;
    photo.params.bgReplaceMode = 'original';
    updateBgCustomControlsUI(photo.params);
    commitHistory(photo);

    if (isPhotoEdited(photo.params)) {
      await processActivePhoto(false);
    } else {
      photo.processedDataUrl = photo.originalDataUrl;
      await renderViewport();
      renderFilmstrip();
      if (state.workspaceMode === 'grid') renderLibraryGrid();
    }
    renderTimeLabel.textContent = 'Đã hủy thay nền LiYing AI';
  }

  document.getElementById('btn-liying-apply')?.addEventListener('click', () => {
    executeLiyingBackgroundReplacement(false);
  });

  document.getElementById('btn-liying-level-only')?.addEventListener('click', () => {
    executeLiyingBackgroundReplacement(true);
  });

  document.getElementById('btn-liying-cancel')?.addEventListener('click', () => {
    cancelLiyingBackgroundReplacement();
  });

  // Evoto Module 3B: Làm đẹp trang phục
  document.querySelectorAll('.cloth-target-pill').forEach((pill) => {
    pill.addEventListener('click', () => {
      const photo = getActivePhoto();
      if (!photo) return;
      photo.params.clothingTarget = pill.dataset.clothtarget || 'all';
      updateClothingControlsUI(photo.params);
      commitHistory(photo);
      processActivePhoto(false);
    });
  });

  document.getElementById('btn-cloth-wrinkle-link')?.addEventListener('click', () => {
    state.clothWrinkleLinked = !state.clothWrinkleLinked;
    const photo = getActivePhoto();
    updateClothingControlsUI(photo?.params || createDefaultParams());
  });

  document.getElementById('btn-clothing-reset')?.addEventListener('click', (e) => {
    e.stopPropagation();
    const photo = getActivePhoto();
    if (!photo) return;
    const def = createDefaultParams();
    for (const k of [
      'clothingTarget',
      'clothingDewrinkle',
      'clothingSmallWrinkles',
      'clothingLargeWrinkles',
      'clothingLintClean',
      'clothingSweatStain',
      'clothingSweatEvenness',
      'clothingEdgeSmooth',
      'clothingEdgeRefine',
      'clothingMoireRemove',
    ]) {
      photo.params[k] = def[k];
    }
    syncSlidersFromParams(photo);
    commitHistory(photo);
    processActivePhoto(false);
  });

  const bindDrawerToggle = (btnId, drawerId) => {
    const btn = document.getElementById(btnId);
    const drawer = document.getElementById(drawerId);
    btn?.addEventListener('click', () => {
      if (!drawer) return;
      const willOpen = drawer.classList.contains('hidden');
      drawer.classList.toggle('hidden', !willOpen);
      btn.setAttribute('aria-expanded', String(willOpen));
    });
  };
  bindDrawerToggle('btn-cloth-sweat-expand', 'cloth-sweat-drawer');
  bindDrawerToggle('btn-cloth-edge-expand', 'cloth-edge-drawer');
  bindDrawerToggle('btn-cloth-ailab-moire', 'cloth-ailab-moire-drawer');
  bindDrawerToggle('btn-hair-stray-expand', 'hair-stray-drawer');
  bindDrawerToggle('btn-hair-smooth-expand', 'hair-smooth-drawer');

  // Evoto Module: Điều chỉnh tóc
  document.getElementById('btn-hair-side-link')?.addEventListener('click', () => {
    state.hairSideLinked = !state.hairSideLinked;
    const photo = getActivePhoto();
    updateHairControlsUI(photo?.params || createDefaultParams());
  });

  document.querySelectorAll('.hair-color-card').forEach((card) => {
    card.addEventListener('click', () => {
      const photo = getActivePhoto();
      if (!photo) return;
      const colorId = card.dataset.haircolor || 'none';
      const defaultDeg = Number(card.dataset.deg || 0);
      photo.params.hairDyeColor = colorId;
      photo.params.hairDyeStrength = colorId === 'none' ? 0 : defaultDeg;
      if (colorId === 'none') {
        photo.params.hairDyeDarken = 0;
      }
      syncSlidersFromParams();
      commitHistory(photo);
      processActivePhoto(false);
    });
  });

  document.getElementById('btn-hair-reset')?.addEventListener('click', (e) => {
    e.stopPropagation();
    const photo = getActivePhoto();
    if (!photo) return;
    const def = createDefaultParams();
    for (const k of [
      'strayHair',
      'strayHairInner',
      'strayHairOuter',
      'hairVolume',
      'hairSideLeftVolume',
      'hairSideRightVolume',
      'hairlineAdjust',
      'hairPartLine',
      'hairSmooth',
      'hairFrizzy',
      'hairShine',
      'whiteHairBlacken',
      'hairColorUnify',
      'hairDyeStrength',
      'hairDyeDarken',
      'hairDyeColor',
    ]) {
      photo.params[k] = def[k];
    }
    syncSlidersFromParams();
    commitHistory(photo);
    processActivePhoto(false);
  });

  document.getElementById('btn-cloth-try-moire')?.addEventListener('click', () => {
    const aiBtn = document.getElementById('btn-cloth-ailab-moire');
    const drawer = document.getElementById('cloth-ailab-moire-drawer');
    if (drawer) {
      drawer.classList.remove('hidden');
      aiBtn?.setAttribute('aria-expanded', 'true');
    }
    const photo = getActivePhoto();
    if (photo) {
      if (!(photo.params.clothingMoireRemove > 0)) {
        photo.params.clothingMoireRemove = 45;
        const sl = document.getElementById('clothingMoireRemove');
        const badge = document.querySelector('.val[data-for="clothingMoireRemove"]');
        if (sl) sl.value = '45';
        if (badge) {
          badge.textContent = '45';
          badge.classList.add('modified');
        }
        commitHistory(photo);
        processActivePhoto(false);
      }
      document.getElementById('clothingMoireRemove')?.focus();
    }
  });

  // Xoay 90° & Lật ảnh
  document.getElementById('btn-rot-left')?.addEventListener('click', () => rotateActivePhoto90(false));
  document.getElementById('btn-rot-right')?.addEventListener('click', () => rotateActivePhoto90(true));
  document.getElementById('btn-crop-rot-left')?.addEventListener('click', () => rotateActivePhoto90(false));
  document.getElementById('btn-crop-rot-right')?.addEventListener('click', () => rotateActivePhoto90(true));

  document.getElementById('btn-flip-v')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo) return;
    photo.params.flipVertical = !photo.params.flipVertical;
    updateCropControlsUI(photo);
    commitHistory(photo);
    processActivePhoto(false);
  });

  document.getElementById('btn-flip-h')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo) return;
    photo.params.flipHorizontal = !photo.params.flipHorizontal;
    updateCropControlsUI(photo);
    commitHistory(photo);
    processActivePhoto(false);
  });

  // Evoto Cắt & Xoay: Sub-tabs (Cắt & Xoay | Phối cảnh & Mở rộng)
  document.querySelectorAll('.crop-mode-tab').forEach((tab) => {
    tab.addEventListener('click', () => {
      document.querySelectorAll('.crop-mode-tab').forEach((t) => t.classList.remove('active'));
      document.querySelectorAll('.crop-subtab-pane').forEach((pane) => pane.classList.remove('active'));
      tab.classList.add('active');
      if (tab.dataset.croptab) {
        document.getElementById(tab.dataset.croptab)?.classList.add('active');
      }
    });
  });

  // Dropdown tỉ lệ cắt ảnh
  const cropRatioBtn = document.getElementById('crop-ratio-btn');
  const cropRatioMenu = document.getElementById('crop-ratio-menu');
  const closeCropRatioMenu = () => {
    cropRatioMenu?.classList.add('hidden');
    cropRatioBtn?.setAttribute('aria-expanded', 'false');
  };

  cropRatioBtn?.addEventListener('click', (e) => {
    e.stopPropagation();
    if (!cropRatioMenu) return;
    const isNowOpen = !cropRatioMenu.classList.toggle('hidden');
    cropRatioBtn.setAttribute('aria-expanded', String(isNowOpen));
  });

  document.addEventListener('click', (e) => {
    if (!cropRatioMenu || cropRatioMenu.classList.contains('hidden')) return;
    if (!e.target?.closest?.('.crop-ratio-wrap')) {
      closeCropRatioMenu();
    }
  });

  const unitToPixels = (val, unit, dpi) => {
    if (!(val > 0)) return 0;
    if (unit === 'cm') return Math.max(1, Math.round((val / 2.54) * dpi));
    if (unit === 'mm') return Math.max(1, Math.round((val / 25.4) * dpi));
    return Math.max(1, Math.round(val));
  };

  const applyIdPhotoPreset = (photo, idPreset, cmW, cmH, refit = true) => {
    if (!photo || !(cmW > 0) || !(cmH > 0)) return;
    const p = photo.params;
    const dpiInVal = parseFloat(document.getElementById('crop-dpi-in')?.value || '');
    const dpi = Math.max(72, Math.min(1200, Math.round(dpiInVal || p.cropDpi || 300)));
    const pxW = Math.max(1, Math.round((cmW / 2.54) * dpi));
    const pxH = Math.max(1, Math.round((cmH / 2.54) * dpi));
    p.cropPreset = idPreset;
    p.cropDpi = dpi;
    p.cropCustomW = pxW;
    p.cropCustomH = pxH;
    p.cropAspectW = pxW;
    p.cropAspectH = pxH;
    if (refit) {
      fitCropBoxToAspect(photo, pxW / pxH);
    }
  };

  const applyCustomDimensionInputs = (photo, refit = true) => {
    if (!photo) return;
    const p = photo.params;
    const unit = document.getElementById('crop-dim-unit')?.value || p.cropDimUnit || 'px';
    const rawDpi = parseFloat(document.getElementById('crop-dpi-in')?.value || '');
    const dpi = Math.max(72, Math.min(1200, Math.round(rawDpi || p.cropDpi || 300)));
    const rawW = parseFloat(document.getElementById('crop-custom-w-in')?.value || '');
    const rawH = parseFloat(document.getElementById('crop-custom-h-in')?.value || '');
    p.cropDimUnit = unit;
    p.cropDpi = dpi;

    const pxW = unitToPixels(rawW, unit, dpi);
    const pxH = unitToPixels(rawH, unit, dpi);
    if (pxW > 0 && pxH > 0) {
      p.cropCustomW = pxW;
      p.cropCustomH = pxH;
      p.cropAspectW = pxW;
      p.cropAspectH = pxH;
      if (refit) {
        fitCropBoxToAspect(photo, pxW / pxH);
      }
    } else {
      p.cropCustomW = 0;
      p.cropCustomH = 0;
      p.cropAspectW = 0;
      p.cropAspectH = 0;
    }
  };

  document.querySelectorAll('.crop-ratio-opt').forEach((opt) => {
    opt.addEventListener('click', () => {
      const photo = getActivePhoto();
      closeCropRatioMenu();
      if (!photo) return;
      const p = photo.params;
      const preset = opt.dataset.preset || 'original';
      const aw = parseFloat(opt.dataset.aw || '0');
      const ah = parseFloat(opt.dataset.ah || '0');
      const { imgW, imgH } = getPhotoDimensions(photo);

      if (preset === 'original') {
        p.cropPreset = 'original';
        p.cropAspectW = 0;
        p.cropAspectH = 0;
        p.cropCustomW = 0;
        p.cropCustomH = 0;
        fitCropBoxToAspect(photo, 0);
      } else if (preset === 'as_shot') {
        p.cropPreset = 'as_shot';
        p.cropAspectW = imgW;
        p.cropAspectH = imgH;
        p.cropCustomW = 0;
        p.cropCustomH = 0;
        fitCropBoxToAspect(photo, imgW / imgH);
      } else if (preset === 'fixed') {
        const curW = Math.max(1, Math.round((p.cropW ?? 1) * imgW));
        const curH = Math.max(1, Math.round((p.cropH ?? 1) * imgH));
        p.cropPreset = 'fixed';
        p.cropAspectW = curW;
        p.cropAspectH = curH;
        p.cropCustomW = 0;
        p.cropCustomH = 0;
      } else if (preset === 'custom_ratio') {
        const rwIn = document.getElementById('crop-ratio-w-in');
        const rhIn = document.getElementById('crop-ratio-h-in');
        const rw = parseFloat(rwIn?.value || '') || (p.cropAspectW > 0 ? p.cropAspectW : 4);
        const rh = parseFloat(rhIn?.value || '') || (p.cropAspectH > 0 ? p.cropAspectH : 5);
        if (rwIn && !(parseFloat(rwIn.value) > 0)) rwIn.value = String(rw);
        if (rhIn && !(parseFloat(rhIn.value) > 0)) rhIn.value = String(rh);
        p.cropPreset = 'custom_ratio';
        p.cropAspectW = rw;
        p.cropAspectH = rh;
        p.cropCustomW = 0;
        p.cropCustomH = 0;
        fitCropBoxToAspect(photo, rw / rh);
      } else if (preset === 'custom_dim') {
        p.cropPreset = 'custom_dim';
        applyCustomDimensionInputs(photo, true);
      } else if (preset === 'id_photo') {
        const curChip = p.cropPreset?.startsWith('id_')
          ? document.querySelector(`.crop-id-chip[data-idpreset="${p.cropPreset}"]`)
          : document.querySelector('.crop-id-chip[data-idpreset="id_3x4"]');
        const idKey = curChip?.dataset.idpreset || 'id_3x4';
        const cmW = parseFloat(curChip?.dataset.cmw || '3');
        const cmH = parseFloat(curChip?.dataset.cmh || '4');
        applyIdPhotoPreset(photo, idKey, cmW, cmH, true);
      } else if (aw > 0 && ah > 0) {
        p.cropPreset = preset;
        p.cropAspectW = aw;
        p.cropAspectH = ah;
        p.cropCustomW = 0;
        p.cropCustomH = 0;
        fitCropBoxToAspect(photo, aw / ah);
      }

      state.cropPreviewMode = false;
      updateCropControlsUI(photo);
      commitHistory(photo);
      renderViewport();
    });
  });

  // Kích thước tự chọn (Tỉ lệ W : H)
  const handleCustomRatioChange = () => {
    const photo = getActivePhoto();
    if (!photo) return;
    const rw = parseFloat(document.getElementById('crop-ratio-w-in')?.value || '');
    const rh = parseFloat(document.getElementById('crop-ratio-h-in')?.value || '');
    if (rw > 0 && rh > 0) {
      photo.params.cropPreset = 'custom_ratio';
      photo.params.cropAspectW = rw;
      photo.params.cropAspectH = rh;
      photo.params.cropCustomW = 0;
      photo.params.cropCustomH = 0;
      fitCropBoxToAspect(photo, rw / rh);
      state.cropPreviewMode = false;
      updateCropControlsUI(photo);
      renderViewport();
    }
  };

  document.getElementById('crop-ratio-w-in')?.addEventListener('input', handleCustomRatioChange);
  document.getElementById('crop-ratio-h-in')?.addEventListener('input', handleCustomRatioChange);
  document.getElementById('crop-ratio-w-in')?.addEventListener('change', () => commitHistory(getActivePhoto()));
  document.getElementById('crop-ratio-h-in')?.addEventListener('change', () => commitHistory(getActivePhoto()));
  document.getElementById('btn-crop-swap-ratio')?.addEventListener('click', () => {
    const rwIn = document.getElementById('crop-ratio-w-in');
    const rhIn = document.getElementById('crop-ratio-h-in');
    if (rwIn && rhIn) {
      const tmp = rwIn.value;
      rwIn.value = rhIn.value;
      rhIn.value = tmp;
    }
    handleCustomRatioChange();
    commitHistory(getActivePhoto());
  });

  // Dài*Rộng*Độ phân giải (px / cm / mm + DPI)
  const handleCustomDimOrDpiChange = (refit = true) => {
    const photo = getActivePhoto();
    if (!photo) return;
    const p = photo.params;
    if (p.cropPreset?.startsWith('id_')) {
      const activeChip = document.querySelector(`.crop-id-chip[data-idpreset="${p.cropPreset}"]`);
      if (activeChip) {
        applyIdPhotoPreset(
          photo,
          activeChip.dataset.idpreset,
          parseFloat(activeChip.dataset.cmw || '3'),
          parseFloat(activeChip.dataset.cmh || '4'),
          false
        );
      }
    } else if (p.cropPreset === 'custom_dim') {
      applyCustomDimensionInputs(photo, refit);
    } else {
      const dpi = parseFloat(document.getElementById('crop-dpi-in')?.value || '');
      if (dpi >= 72 && dpi <= 1200) p.cropDpi = Math.round(dpi);
      const unit = document.getElementById('crop-dim-unit')?.value;
      if (unit) p.cropDimUnit = unit;
    }
    updateCropControlsUI(photo);
    renderViewport();
  };

  document.getElementById('crop-custom-w-in')?.addEventListener('input', () => handleCustomDimOrDpiChange(true));
  document.getElementById('crop-custom-h-in')?.addEventListener('input', () => handleCustomDimOrDpiChange(true));
  document.getElementById('crop-custom-w-in')?.addEventListener('change', () => commitHistory(getActivePhoto()));
  document.getElementById('crop-custom-h-in')?.addEventListener('change', () => commitHistory(getActivePhoto()));
  document.getElementById('crop-dim-unit')?.addEventListener('change', () => {
    handleCustomDimOrDpiChange(true);
    commitHistory(getActivePhoto());
  });
  document.getElementById('crop-dpi-in')?.addEventListener('input', () => handleCustomDimOrDpiChange(false));
  document.getElementById('crop-dpi-in')?.addEventListener('change', () => commitHistory(getActivePhoto()));
  document.getElementById('btn-crop-swap-dim')?.addEventListener('click', () => {
    const cwIn = document.getElementById('crop-custom-w-in');
    const chIn = document.getElementById('crop-custom-h-in');
    if (cwIn && chIn) {
      const tmp = cwIn.value;
      cwIn.value = chIn.value;
      chIn.value = tmp;
    }
    handleCustomDimOrDpiChange(true);
    commitHistory(getActivePhoto());
  });

  // Preset Ảnh Thẻ (2×3, 3×4, 4×6, Visa, 5×5, 3.3×4.8 cm)
  document.querySelectorAll('.crop-id-chip').forEach((chip) => {
    chip.addEventListener('click', () => {
      const photo = getActivePhoto();
      if (!photo) return;
      applyIdPhotoPreset(
        photo,
        chip.dataset.idpreset || 'id_3x4',
        parseFloat(chip.dataset.cmw || '3'),
        parseFloat(chip.dataset.cmh || '4'),
        true
      );
      state.cropPreviewMode = false;
      updateCropControlsUI(photo);
      commitHistory(photo);
      renderViewport();
    });
  });

  // Nút thao tác nhanh: Đảo chiều khung, Xem thử đã cắt, Đặt lại khung, Hiển thị lưới
  document.getElementById('btn-crop-orient')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo) return;
    const p = photo.params;
    const { imgW, imgH } = getPhotoDimensions(photo);

    if (p.cropAspectW > 0 && p.cropAspectH > 0) {
      const tmpA = p.cropAspectW;
      p.cropAspectW = p.cropAspectH;
      p.cropAspectH = tmpA;
    }
    if (p.cropCustomW > 0 && p.cropCustomH > 0) {
      const tmpC = p.cropCustomW;
      p.cropCustomW = p.cropCustomH;
      p.cropCustomH = tmpC;
    }

    const curPixelRatio = (Math.max(1, p.cropH * imgH)) / Math.max(1, p.cropW * imgW);
    const nextRatio = p.cropAspectW > 0 && p.cropAspectH > 0 ? p.cropAspectW / p.cropAspectH : curPixelRatio;
    const anchor = {
      x: (p.cropX ?? 0) + (p.cropW ?? 1) * 0.5,
      y: (p.cropY ?? 0) + (p.cropH ?? 1) * 0.44,
    };
    fitCropBoxToAspect(photo, nextRatio, anchor);
    updateCropControlsUI(photo);
    commitHistory(photo);
    renderViewport();
  });

  document.getElementById('btn-crop-preview-toggle')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo) return;
    state.cropPreviewMode = !state.cropPreviewMode;
    updateCropControlsUI(photo);
    commitHistory(photo);
    renderViewport();
  });

  document.getElementById('btn-crop-apply')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo) return;
    state.cropPreviewMode = true;
    updateCropControlsUI(photo);
    commitHistory(photo);
    renderViewport();
    renderFilmstrip();
    if (state.workspaceMode === 'grid') renderLibraryGrid();
    const p = photo.params;
    const { imgW, imgH } = getPhotoDimensions(photo);
    const cw = Math.max(1, Math.round((p.cropW ?? 1) * imgW));
    const ch = Math.max(1, Math.round((p.cropH ?? 1) * imgH));
    renderTimeLabel.textContent = `Đã áp dụng cắt (${cw} × ${ch} px) ✓`;
  });

  const handleResetCrop = () => {
    const photo = getActivePhoto();
    if (!photo) return;
    const p = photo.params;
    p.cropX = 0;
    p.cropY = 0;
    p.cropW = 1;
    p.cropH = 1;
    p.cropPreset = 'original';
    p.cropAspectW = 0;
    p.cropAspectH = 0;
    p.cropCustomW = 0;
    p.cropCustomH = 0;
    p.cropAiSubject = false;
    p.cropAlignVertical = false;
    p.cropAlignEye = false;
    const hadRotate = Math.abs(p.rotateAngle || 0) > 1e-4;
    p.rotateAngle = 0;
    state.cropPreviewMode = false;
    document.querySelectorAll('.crop-frame-chip').forEach((c) => c.classList.remove('active'));
    syncSlidersFromParams();
    updateCropControlsUI(photo);
    renderFilmstrip();
    commitHistory(photo);
    if (hadRotate) {
      processActivePhoto(false);
    } else {
      renderViewport();
    }
    renderTimeLabel.textContent = 'Đã đặt lại khung cắt ✓';
  };
  document.getElementById('btn-crop-reset')?.addEventListener('click', handleResetCrop);
  document.getElementById('btn-crop-reset-btn')?.addEventListener('click', handleResetCrop);

  document.getElementById('crop-show-grid')?.addEventListener('change', (e) => {
    const photo = getActivePhoto();
    if (!photo) return;
    photo.params.cropShowGrid = Boolean(e.target.checked);
    commitHistory(photo);
    if (overlayCanvas.width && overlayCanvas.height) {
      drawOverlayGuides(overlayCanvas.width, overlayCanvas.height, photo);
    }
  });

  // Định vị bằng AI: Tab Chủ thể / Khung chân dung + Nút căn chỉnh + Tự động cân góc
  document.getElementById('crop-ai-tab-subject')?.addEventListener('click', () => {
    document.getElementById('crop-ai-tab-subject')?.classList.add('active');
    document.getElementById('crop-ai-tab-head')?.classList.remove('active');
    document.getElementById('crop-ai-subject-panel')?.classList.remove('hidden');
    document.getElementById('crop-ai-head-panel')?.classList.add('hidden');
  });

  document.getElementById('crop-ai-tab-head')?.addEventListener('click', () => {
    document.getElementById('crop-ai-tab-head')?.classList.add('active');
    document.getElementById('crop-ai-tab-subject')?.classList.remove('active');
    document.getElementById('crop-ai-head-panel')?.classList.remove('hidden');
    document.getElementById('crop-ai-subject-panel')?.classList.add('hidden');
  });

  document.getElementById('toggle-crop-ai-subject')?.addEventListener('click', async () => {
    const photo = getActivePhoto();
    if (!photo) return;
    photo.params.cropAiSubject = !photo.params.cropAiSubject;
    if (photo.params.cropAiSubject) {
      await applyCropAlignAI(photo, 'subject');
    } else {
      updateCropControlsUI(photo);
      await renderViewport();
    }
  });

  document.getElementById('btn-crop-align-v')?.addEventListener('click', async () => {
    const photo = getActivePhoto();
    if (!photo) return;
    await applyCropAlignAI(photo, 'vertical');
  });

  document.getElementById('btn-crop-align-eye')?.addEventListener('click', async () => {
    const photo = getActivePhoto();
    if (!photo) return;
    await applyCropAlignAI(photo, 'eye');
  });

  document.querySelectorAll('.crop-frame-chip').forEach((chip) => {
    chip.addEventListener('click', async () => {
      const photo = getActivePhoto();
      if (!photo) return;
      document.querySelectorAll('.crop-frame-chip').forEach((c) => c.classList.remove('active'));
      chip.classList.add('active');
      await applyCropAlignAI(photo, chip.dataset.frame || 'headshot');
    });
  });

  document.getElementById('btn-ai-auto-straighten')?.addEventListener('click', async () => {
    const photo = getActivePhoto();
    if (!photo) return;
    await applyCropAlignAI(photo, 'auto_straighten');
  });

  // Chọn Preset có sẵn
  document.querySelectorAll('.retouch-preset-item[data-preset]').forEach((card) => {
    card.addEventListener('click', () => {
      const presetKey = card.dataset.preset;
      const presetValues = RETOUCH_PRESETS[presetKey];
      const photo = getActivePhoto();
      if (!photo || !presetValues) return;

      document.querySelectorAll('.retouch-preset-item').forEach((c) => c.classList.remove('active'));
      card.classList.add('active');

      const fresh = createDefaultParams();
      fresh.face = photo.params.face;
      fresh.blemishSpots = photo.params.blemishSpots;
      photo.params = { ...fresh, ...presetValues };
      syncSlidersFromParams();
      commitHistory(photo);
      processActivePhoto(false);
    });
  });

  // Nhập ảnh & Kéo thả tệp / RAW
  const fileInput = document.getElementById('file-input');
  ['btn-import-top', 'btn-import-side', 'btn-import-strip'].forEach((id) => {
    document.getElementById(id)?.addEventListener('click', () => openNativeImportDialog());
  });

  fileInput?.addEventListener('change', async (e) => {
    await importHtmlFileList(e.target.files);
    fileInput.value = '';
  });

  try {
    if (hasTauriRuntime()) {
      getCurrentWebview()
        .onDragDropEvent(async (event) => {
          if (event.payload.type === 'drop' && event.payload.paths?.length) {
            await importPathsFromRust(event.payload.paths);
          }
        })
        .catch(() => {});
    }
  } catch (_) {}

  window.addEventListener('dragover', (e) => e.preventDefault());
  window.addEventListener('drop', async (e) => {
    e.preventDefault();
    if (e.dataTransfer?.files?.length) {
      await importHtmlFileList(e.dataTransfer.files);
    }
  });

  // Thu phóng (Fit / 1:1 / +/- và Alt + Lăn chuột trong preview)
  document.getElementById('btn-zoom-fit')?.addEventListener('click', () => {
    state.zoomScale = 1;
    state.panX = 0;
    state.panY = 0;
    updateZoomUI();
  });
  document.getElementById('btn-zoom-100')?.addEventListener('click', () => {
    state.zoomScale = 2;
    updateZoomUI();
  });
  document.getElementById('btn-zoom-in')?.addEventListener('click', () => {
    state.zoomScale = Math.min(4.0, +(state.zoomScale + 0.25).toFixed(2));
    updateZoomUI();
  });
  document.getElementById('btn-zoom-out')?.addEventListener('click', () => {
    state.zoomScale = Math.max(0.5, +(state.zoomScale - 0.25).toFixed(2));
    updateZoomUI();
  });

  const viewportEl = document.getElementById('canvas-viewport');
  viewportEl?.addEventListener(
    'wheel',
    (e) => {
      if (!e.altKey || !getActivePhoto()) return;
      e.preventDefault();
      if (!e.deltaY) return;

      const oldScale = state.zoomScale;
      const factor = e.deltaY < 0 ? 1.15 : 1 / 1.15;
      let nextScale = Math.max(0.5, Math.min(4.0, +(oldScale * factor).toFixed(3)));
      if (
        Math.abs(nextScale - 1) < 0.06 &&
        ((oldScale > 1 && e.deltaY > 0) || (oldScale < 1 && e.deltaY < 0))
      ) {
        nextScale = 1;
      }
      if (Math.abs(nextScale - oldScale) < 1e-4) return;

      if (nextScale <= 1) {
        state.panX = 0;
        state.panY = 0;
      } else if (canvasWrap) {
        const rect = canvasWrap.getBoundingClientRect();
        if (rect.width > 0 && rect.height > 0) {
          const cx = rect.left + rect.width * 0.5;
          const cy = rect.top + rect.height * 0.5;
          const k = nextScale / oldScale;
          state.panX = state.panX - (e.clientX - cx) * (k - 1);
          state.panY = state.panY - (e.clientY - cy) * (k - 1);
        }
      }

      state.zoomScale = nextScale;
      updateZoomUI();
    },
    { passive: false }
  );

  const mapViewportToFullNorm = (photo, nx, ny) => {
    if (!photo || !isViewingCroppedViewport(photo)) return { x: nx, y: ny };
    const p = photo.params;
    return {
      x: (p.cropX ?? 0) + nx * (p.cropW ?? 1),
      y: (p.cropY ?? 0) + ny * (p.cropH ?? 1),
    };
  };

  const hitTestCropHandle = (photo, nx, ny, rect) => {
    if (!photo || !rect || rect.width <= 0 || rect.height <= 0) return null;
    const p = photo.params;
    const cx = p.cropX ?? 0;
    const cy = p.cropY ?? 0;
    const cw = p.cropW ?? 1;
    const ch = p.cropH ?? 1;
    const px = nx * rect.width;
    const py = ny * rect.height;
    const x1 = cx * rect.width;
    const y1 = cy * rect.height;
    const x2 = (cx + cw) * rect.width;
    const y2 = (cy + ch) * rect.height;

    const cornerTol = 14;
    if (Math.hypot(px - x1, py - y1) <= cornerTol) return 'nw';
    if (Math.hypot(px - x2, py - y1) <= cornerTol) return 'ne';
    if (Math.hypot(px - x1, py - y2) <= cornerTol) return 'sw';
    if (Math.hypot(px - x2, py - y2) <= cornerTol) return 'se';

    const edgeTol = 9;
    if (Math.abs(py - y1) <= edgeTol && px >= x1 + 10 && px <= x2 - 10) return 'n';
    if (Math.abs(py - y2) <= edgeTol && px >= x1 + 10 && px <= x2 - 10) return 's';
    if (Math.abs(px - x1) <= edgeTol && py >= y1 + 10 && py <= y2 - 10) return 'w';
    if (Math.abs(px - x2) <= edgeTol && py >= y1 + 10 && py <= y2 - 10) return 'e';

    if (px >= x1 && px <= x2 && py >= y1 && py <= y2) return 'move';
    return null;
  };

  const getCropHandleCursor = (handle) => {
    if (handle === 'nw' || handle === 'se') return 'nwse-resize';
    if (handle === 'ne' || handle === 'sw') return 'nesw-resize';
    if (handle === 'n' || handle === 's') return 'ns-resize';
    if (handle === 'w' || handle === 'e') return 'ew-resize';
    if (handle === 'move') return 'move';
    return 'default';
  };

  const applyCropDragDelta = (photo, drag, nx, ny) => {
    const p = photo.params;
    const { imgW, imgH } = getPhotoDimensions(photo);
    const imgAspect = imgW / imgH;
    const safeScale = getSafeRotatedInsetScale(p.rotateAngle, imgAspect);
    const minB = (1 - safeScale) * 0.5;
    const maxB = 1 - minB;
    const minSize = 0.05;
    const handle = drag.handle;

    if (handle === 'move') {
      const dx = nx - drag.startNx;
      const dy = ny - drag.startNy;
      p.cropW = drag.origW;
      p.cropH = drag.origH;
      p.cropX = Math.max(minB, Math.min(maxB - drag.origW, drag.origX + dx));
      p.cropY = Math.max(minB, Math.min(maxB - drag.origH, drag.origY + dy));
      return;
    }

    const lockedRatio = getCropLockedRatio(photo);
    const normRatio = lockedRatio > 0 ? lockedRatio / imgAspect : 0;
    const ox1 = drag.origX;
    const oy1 = drag.origY;
    const ox2 = drag.origX + drag.origW;
    const oy2 = drag.origY + drag.origH;

    if (!(normRatio > 0)) {
      const dx = nx - drag.startNx;
      const dy = ny - drag.startNy;
      let x1 = ox1;
      let y1 = oy1;
      let x2 = ox2;
      let y2 = oy2;
      if (handle.includes('w')) x1 = Math.max(minB, Math.min(ox2 - minSize, ox1 + dx));
      if (handle.includes('e')) x2 = Math.min(maxB, Math.max(ox1 + minSize, ox2 + dx));
      if (handle.includes('n')) y1 = Math.max(minB, Math.min(oy2 - minSize, oy1 + dy));
      if (handle.includes('s')) y2 = Math.min(maxB, Math.max(oy1 + minSize, oy2 + dy));
      p.cropX = x1;
      p.cropY = y1;
      p.cropW = Math.max(minSize, x2 - x1);
      p.cropH = Math.max(minSize, y2 - y1);
      return;
    }

    if (handle.length === 2) {
      const ax = handle.includes('w') ? ox2 : ox1;
      const ay = handle.includes('n') ? oy2 : oy1;
      const availMaxW = handle.includes('w') ? ax - minB : maxB - ax;
      const availMaxH = handle.includes('n') ? ay - minB : maxB - ay;
      const maxW = Math.max(minSize, Math.min(availMaxW, availMaxH * normRatio));
      const minW = Math.min(maxW, Math.max(minSize, minSize * normRatio));
      const candW = handle.includes('w') ? ax - nx : nx - ax;
      const candH = handle.includes('n') ? ay - ny : ny - ay;
      const nw = Math.max(minW, Math.min(maxW, Math.max(candW, candH * normRatio)));
      const nh = nw / normRatio;
      p.cropW = nw;
      p.cropH = nh;
      p.cropX = handle.includes('w') ? ax - nw : ax;
      p.cropY = handle.includes('n') ? ay - nh : ay;
      return;
    }

    const ocx = (ox1 + ox2) * 0.5;
    const ocy = (oy1 + oy2) * 0.5;
    if (handle === 'w' || handle === 'e') {
      const ax = handle === 'w' ? ox2 : ox1;
      const availMaxW = handle === 'w' ? ax - minB : maxB - ax;
      const availHalfH = Math.max(minSize * 0.5, Math.min(ocy - minB, maxB - ocy));
      const maxW = Math.max(minSize, Math.min(availMaxW, availHalfH * 2 * normRatio));
      const minW = Math.min(maxW, Math.max(minSize, minSize * normRatio));
      const candW = handle === 'w' ? ax - nx : nx - ax;
      const nw = Math.max(minW, Math.min(maxW, candW));
      const nh = nw / normRatio;
      p.cropW = nw;
      p.cropH = nh;
      p.cropX = handle === 'w' ? ax - nw : ax;
      p.cropY = ocy - nh * 0.5;
    } else {
      const ay = handle === 'n' ? oy2 : oy1;
      const availMaxH = handle === 'n' ? ay - minB : maxB - ay;
      const availHalfW = Math.max(minSize * 0.5, Math.min(ocx - minB, maxB - ocx));
      const maxH = Math.max(minSize, Math.min(availMaxH, (availHalfW * 2) / normRatio));
      const minH = Math.min(maxH, Math.max(minSize, minSize / normRatio));
      const candH = handle === 'n' ? ay - ny : ny - ay;
      const nh = Math.max(minH, Math.min(maxH, candH));
      const nw = nh * normRatio;
      p.cropW = nw;
      p.cropH = nh;
      p.cropX = ocx - nw * 0.5;
      p.cropY = handle === 'n' ? ay - nh : ay;
    }
  };

  viewportEl?.addEventListener(
    'mousedown',
    (e) => {
      if (!state.spacePressed || e.button !== 0 || !getActivePhoto()) return;
      e.preventDefault();
      e.stopPropagation();
      state.draggingPan = {
        startX: e.clientX,
        startY: e.clientY,
        origPanX: state.panX,
        origPanY: state.panY,
      };
      viewportEl.classList.add('hand-pan-dragging');
    },
    true
  );

  // Kéo thanh chia đôi, Đường cong, Mốc mặt, Khung cắt
  splitLine.addEventListener('mousedown', (e) => {
    e.preventDefault();
    state.draggingSplit = true;
  });

  window.addEventListener('mousemove', (e) => {
    if (state.draggingPan) {
      state.panX = state.draggingPan.origPanX + (e.clientX - state.draggingPan.startX);
      state.panY = state.draggingPan.origPanY + (e.clientY - state.draggingPan.startY);
      updateZoomUI();
      return;
    }

    if (state.draggingCrop) {
      const photo = getActivePhoto();
      if (!photo) return;
      const rect = overlayCanvas.getBoundingClientRect();
      if (rect.width <= 0 || rect.height <= 0) return;
      const nx = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
      const ny = Math.max(0, Math.min(1, (e.clientY - rect.top) / rect.height));
      applyCropDragDelta(photo, state.draggingCrop, nx, ny);
      updateCropControlsUI(photo);
      if (overlayCanvas.width && overlayCanvas.height) {
        drawOverlayGuides(overlayCanvas.width, overlayCanvas.height, photo);
      }
      return;
    }

    if (state.draggingCurveIdx !== -1 && curveCanvas) {
      const photo = getActivePhoto();
      if (!photo) return;
      const rect = curveCanvas.getBoundingClientRect();
      const nx = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
      const ny = Math.max(0, Math.min(1, 1 - (e.clientY - rect.top) / rect.height));
      const pts = getPhotoCurvePoints(photo);
      const idx = state.draggingCurveIdx;
      if (idx === 0) {
        pts[0].x = 0;
        pts[0].y = ny;
      } else if (idx === pts.length - 1) {
        pts[idx].x = 1;
        pts[idx].y = ny;
      } else {
        const minX = pts[idx - 1].x + 0.02;
        const maxX = pts[idx + 1].x - 0.02;
        pts[idx].x = Math.max(minX, Math.min(maxX, nx));
        pts[idx].y = ny;
      }
      photo.params.curveLut = build256CurveLutFromPoints(pts);
      renderToneCurveCanvas();
      if (state.realtimeColor) processActivePhoto(false);
    } else if (state.draggingSplit) {
      const rect = canvasWrap.getBoundingClientRect();
      state.splitRatio = Math.max(0.02, Math.min(0.98, (e.clientX - rect.left) / rect.width));
      renderViewport();
    } else if (state.draggingLandmark) {
      const photo = getActivePhoto();
      if (!photo) return;
      const rect = overlayCanvas.getBoundingClientRect();
      const vnx = Math.max(0.02, Math.min(0.98, (e.clientX - rect.left) / rect.width));
      const vny = Math.max(0.02, Math.min(0.98, (e.clientY - rect.top) / rect.height));
      const mapped = mapViewportToFullNorm(photo, vnx, vny);
      const nx = Math.max(0.05, Math.min(0.95, mapped.x));
      const ny = Math.max(0.05, Math.min(0.95, mapped.y));
      const f = photo.params.face;

      if (state.draggingLandmark === 'leftEye') {
        f.leftEyeX = nx;
        f.leftEyeY = ny;
      } else if (state.draggingLandmark === 'rightEye') {
        f.rightEyeX = nx;
        f.rightEyeY = ny;
      } else if (state.draggingLandmark === 'nose') {
        f.noseX = nx;
        f.noseY = ny;
      } else if (state.draggingLandmark === 'mouth') {
        f.mouthX = nx;
        f.mouthY = ny;
      } else if (state.draggingLandmark === 'center') {
        const dx = nx - f.centerX;
        const dy = ny - f.centerY;
        f.centerX = nx;
        f.centerY = ny;
        f.leftEyeX += dx;
        f.leftEyeY += dy;
        f.rightEyeX += dx;
        f.rightEyeY += dy;
        f.noseX += dx;
        f.noseY += dy;
        f.mouthX += dx;
        f.mouthY += dy;
        f.chinX += dx;
        f.chinY += dy;
        f.foreheadX += dx;
        f.foreheadY += dy;
        f.jawLeftX += dx;
        f.jawLeftY += dy;
        f.jawRightX += dx;
        f.jawRightY += dy;
        f.cheekLeftX += dx;
        f.cheekLeftY += dy;
        f.cheekRightX += dx;
        f.cheekRightY += dy;
        f.mouthLeftX += dx;
        f.mouthLeftY += dy;
        f.mouthRightX += dx;
        f.mouthRightY += dy;
      }
      renderViewport();
    }
  });

  window.addEventListener('mouseup', () => {
    if (state.draggingPan) {
      state.draggingPan = null;
      viewportEl?.classList.remove('hand-pan-dragging');
    }
    if (state.draggingCrop) {
      state.draggingCrop = null;
      const photo = getActivePhoto();
      if (photo && overlayCanvas.width && overlayCanvas.height) {
        drawOverlayGuides(overlayCanvas.width, overlayCanvas.height, photo);
        updateCropControlsUI(photo);
      }
      if (photo) commitHistory(photo);
    }
    if (state.draggingRotateAngle) {
      state.draggingRotateAngle = false;
      const photo = getActivePhoto();
      if (photo && overlayCanvas.width && overlayCanvas.height) {
        drawOverlayGuides(overlayCanvas.width, overlayCanvas.height, photo);
      }
      if (photo) commitHistory(photo);
    }
    if (state.draggingCurveIdx !== -1) {
      state.draggingCurveIdx = -1;
      renderToneCurveCanvas();
      commitHistory(getActivePhoto());
      if (!state.realtimeColor) processActivePhoto(false);
    }
    if (state.draggingLandmark) {
      state.draggingLandmark = null;
      const photo = getActivePhoto();
      if (photo && !photo.landmarksDetected) {
        photo.maskBase64 = null;
      }
      commitHistory(photo);
      processActivePhoto(false);
    }
    state.draggingSplit = false;
  });

  overlayCanvas.addEventListener('mousemove', (e) => {
    const rect = overlayCanvas.getBoundingClientRect();
    const photo = getActivePhoto();
    if (rect.width > 0 && rect.height > 0 && photo && !state.draggingCrop) {
      if (state.spotHealMode) {
        overlayCanvas.style.cursor = 'crosshair';
      } else if (state.activeModule === 'mod-crop' && !state.cropPreviewMode) {
        const nx = (e.clientX - rect.left) / rect.width;
        const ny = (e.clientY - rect.top) / rect.height;
        const handle = hitTestCropHandle(photo, nx, ny, rect);
        overlayCanvas.style.cursor = getCropHandleCursor(handle);
      } else {
        overlayCanvas.style.cursor = 'default';
      }
    }

    if (!afterCanvas.width || !afterCanvas.height || !pixelReadout) return;
    const px = Math.floor(((e.clientX - rect.left) / rect.width) * afterCanvas.width);
    const py = Math.floor(((e.clientY - rect.top) / rect.height) * afterCanvas.height);
    if (px >= 0 && px < afterCanvas.width && py >= 0 && py < afterCanvas.height) {
      const ctx = afterCanvas.getContext('2d');
      const d = ctx.getImageData(px, py, 1, 1).data;
      pixelReadout.textContent = `RGB(${d[0]}, ${d[1]}, ${d[2]})`;
    }
  });

  overlayCanvas.addEventListener('mousedown', (e) => {
    const photo = getActivePhoto();
    if (!photo) return;
    const rect = overlayCanvas.getBoundingClientRect();
    const nx = (e.clientX - rect.left) / rect.width;
    const ny = (e.clientY - rect.top) / rect.height;

    if (state.activeModule === 'mod-crop' && !state.cropPreviewMode && !state.spotHealMode) {
      const handle = hitTestCropHandle(photo, nx, ny, rect);
      if (handle) {
        e.preventDefault();
        const p = photo.params;
        state.draggingCrop = {
          handle,
          startNx: nx,
          startNy: ny,
          origX: p.cropX ?? 0,
          origY: p.cropY ?? 0,
          origW: p.cropW ?? 1,
          origH: p.cropH ?? 1,
        };
        return;
      }
    }

    if (state.showFaceGuides) {
      const f = photo.params.face;
      const p = photo.params;
      const croppedView = isViewingCroppedViewport(photo);
      const toViewX = (fx) => (croppedView ? (fx - p.cropX) / Math.max(0.01, p.cropW) : fx);
      const toViewY = (fy) => (croppedView ? (fy - p.cropY) / Math.max(0.01, p.cropH) : fy);
      for (const c of [
        { key: 'leftEye', x: toViewX(f.leftEyeX), y: toViewY(f.leftEyeY) },
        { key: 'rightEye', x: toViewX(f.rightEyeX), y: toViewY(f.rightEyeY) },
        { key: 'nose', x: toViewX(f.noseX), y: toViewY(f.noseY) },
        { key: 'mouth', x: toViewX(f.mouthX), y: toViewY(f.mouthY) },
        { key: 'center', x: toViewX(f.centerX), y: toViewY(f.centerY) },
      ]) {
        if (Math.hypot(nx - c.x, ny - c.y) < 0.035) {
          state.draggingLandmark = c.key;
          return;
        }
      }
    }

    if (state.spotHealMode) {
      const mapped = mapViewportToFullNorm(photo, nx, ny);
      photo.params.blemishSpots.push({
        x: Math.max(0, Math.min(1, mapped.x)),
        y: Math.max(0, Math.min(1, mapped.y)),
        radius: 0.016,
      });
      commitHistory(photo);
      processActivePhoto(false);
    }
  });

  const btnHand = document.getElementById('btn-tool-hand');
  const btnSpot = document.getElementById('btn-spot-heal');
  const activateHandTool = () => {
    state.spotHealMode = false;
    btnHand?.classList.add('active');
    btnSpot?.classList.remove('active');
    spotBanner?.classList.add('hidden');
    overlayCanvas.style.cursor = 'default';
    renderViewport();
  };
  const toggleSpotTool = () => {
    state.spotHealMode = !state.spotHealMode;
    btnSpot?.classList.toggle('active', state.spotHealMode);
    btnHand?.classList.toggle('active', !state.spotHealMode);
    spotBanner?.classList.toggle('hidden', !state.spotHealMode);
    overlayCanvas.style.cursor = state.spotHealMode ? 'crosshair' : 'default';
    renderViewport();
  };

  btnHand?.addEventListener('click', activateHandTool);
  btnSpot?.addEventListener('click', toggleSpotTool);

  document.getElementById('btn-undo-spot')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (photo && photo.params.blemishSpots.length > 0) {
      photo.params.blemishSpots.pop();
      commitHistory(photo);
      processActivePhoto(false);
    }
  });

  document.getElementById('btn-clear-spots')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (photo) {
      photo.params.blemishSpots = [];
      commitHistory(photo);
      processActivePhoto(false);
    }
  });

  const btnGuides = document.getElementById('btn-face-guides');
  btnGuides?.addEventListener('click', () => {
    state.showFaceGuides = !state.showFaceGuides;
    btnGuides.classList.toggle('active', state.showFaceGuides);
    renderViewport();
  });

  const btnSplit = document.getElementById('btn-split-view');
  btnSplit?.addEventListener('click', () => {
    state.splitEnabled = !state.splitEnabled;
    btnSplit.classList.toggle('active', state.splitEnabled);
    renderViewport();
  });

  const btnHoldBefore = document.getElementById('btn-hold-before');
  btnHoldBefore?.addEventListener('mousedown', () => {
    state.holdingBefore = true;
    renderViewport();
  });
  window.addEventListener('mouseup', () => {
    if (state.holdingBefore) {
      state.holdingBefore = false;
      renderViewport();
    }
  });

  const btnPaste = document.getElementById('btn-paste-settings');
  const copySettingsAction = () => {
    const photo = getActivePhoto();
    if (!photo) return;
    state.copiedParams = JSON.parse(JSON.stringify(photo.params));
    if (btnPaste) btnPaste.disabled = false;
    renderTimeLabel.textContent = 'Đã sao chép thông số ✓';
  };
  const pasteSettingsAction = () => {
    const photo = getActivePhoto();
    if (!photo || !state.copiedParams) return;
    const ownFace = photo.params.face;
    photo.params = JSON.parse(JSON.stringify(state.copiedParams));
    photo.params.face = ownFace;
    syncSlidersFromParams();
    commitHistory(photo);
    processActivePhoto(false);
  };

  window.addEventListener('keydown', (e) => {
    const activeEl = document.activeElement;
    const tag = activeEl?.tagName;
    const isTextEntry =
      tag === 'TEXTAREA' ||
      (tag === 'INPUT' &&
        !['range', 'checkbox', 'radio', 'color', 'button', 'submit'].includes(
          (activeEl.type || 'text').toLowerCase()
        ));

    if ((e.ctrlKey || e.metaKey) && !e.altKey) {
      const k = e.key.toLowerCase();
      if (k === 'z' && !isTextEntry) {
        e.preventDefault();
        if (e.shiftKey) {
          redoActivePhoto();
        } else {
          undoActivePhoto();
        }
        return;
      }
      if (k === 'y' && !e.shiftKey && !isTextEntry) {
        e.preventDefault();
        redoActivePhoto();
        return;
      }
    }

    if (e.code === 'Space' && !isTextEntry && state.workspaceMode === 'develop') {
      e.preventDefault();
      if (!state.spacePressed) {
        state.spacePressed = true;
        viewportEl?.classList.add('hand-pan-ready');
      }
      return;
    }

    if (tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT') return;

    if (e.key === '\\' && !state.holdingBefore) {
      e.preventDefault();
      state.holdingBefore = true;
      renderViewport();
      return;
    }
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'c') {
      e.preventDefault();
      copySettingsAction();
      return;
    }
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'v') {
      e.preventDefault();
      pasteSettingsAction();
      return;
    }
    if (e.key.toLowerCase() === 'h') {
      activateHandTool();
    } else if (e.key.toLowerCase() === 'b') {
      toggleSpotTool();
    } else if (e.key.toLowerCase() === 'g') {
      setWorkspaceMode('grid');
    } else if (e.key.toLowerCase() === 'd') {
      setWorkspaceMode('develop');
    } else if (e.key === 'ArrowRight' && state.activeIndex + 1 < state.photos.length) {
      state.activeIndex++;
      syncSlidersFromParams();
      renderViewport();
      renderFilmstrip();
      if (state.workspaceMode === 'grid') renderLibraryGrid();
      prewarmActiveHighresMatte();
    } else if (e.key === 'ArrowLeft' && state.activeIndex > 0) {
      state.activeIndex--;
      syncSlidersFromParams();
      renderViewport();
      renderFilmstrip();
      if (state.workspaceMode === 'grid') renderLibraryGrid();
      prewarmActiveHighresMatte();
    }
  });

  window.addEventListener('keyup', (e) => {
    if (e.code === 'Space') {
      state.spacePressed = false;
      viewportEl?.classList.remove('hand-pan-ready');
    }
    if (e.key === '\\' && state.holdingBefore) {
      state.holdingBefore = false;
      renderViewport();
    }
  });

  window.addEventListener('blur', () => {
    const needViewport = state.holdingBefore;
    state.holdingBefore = false;
    state.spacePressed = false;
    state.draggingPan = null;
    viewportEl?.classList.remove('hand-pan-ready', 'hand-pan-dragging');
    state.draggingSplit = false;
    state.draggingCrop = null;
    state.draggingCurveIdx = -1;
    state.draggingLandmark = null;
    state.draggingRotateAngle = false;
    if (needViewport) renderViewport();
  });

  document.getElementById('btn-undo')?.addEventListener('click', undoActivePhoto);
  document.getElementById('btn-redo')?.addEventListener('click', redoActivePhoto);
  updateHistoryButtonsUI();

  document.getElementById('btn-reset-all')?.addEventListener('click', applyResetActivePhoto);
  document.getElementById('btn-reset-all-footer')?.addEventListener('click', applyResetActivePhoto);

  document.getElementById('btn-copy-settings')?.addEventListener('click', copySettingsAction);
  btnPaste?.addEventListener('click', pasteSettingsAction);

  document.getElementById('btn-sync-all')?.addEventListener('click', applySyncToAllPhotos);
  document.getElementById('btn-sync-all-footer')?.addEventListener('click', applySyncToAllPhotos);

  document.getElementById('btn-save-user-preset')?.addEventListener('click', saveCurrentAsUserPreset);
  document.getElementById('btn-save-user-preset-footer')?.addEventListener('click', saveCurrentAsUserPreset);

  document.getElementById('btn-export')?.addEventListener('click', async () => {
    const photo = getActivePhoto();
    if (!photo) return;
    renderTimeLabel.textContent = 'Đang kết xuất ảnh chất lượng gốc...';
    const base = photo.name.replace(/\.[^.]+$/, '');
    const isTransparent = photo.params.bgReplaceMode === 'transparent';
    const defaultName = isTransparent
      ? `RetouchStudio_${base}_TachNen.png`
      : `RetouchStudio_${base}_DaChinh.jpg`;

    if (hasTauriRuntime()) {
      try {
        const filters = isTransparent
          ? [
              { name: 'Ảnh PNG (Trong suốt)', extensions: ['png'] },
              { name: 'Ảnh JPEG', extensions: ['jpg', 'jpeg'] },
            ]
          : [
              { name: 'Ảnh JPEG', extensions: ['jpg', 'jpeg'] },
              { name: 'Ảnh PNG', extensions: ['png'] },
            ];
        const savePath = await save({
          title: 'Xuất ảnh đã chỉnh sửa — Retouch Studio Pro',
          defaultPath: defaultName,
          filters,
        });

        if (savePath) {
          if (needsHighresMatte(photo.params)) {
            await ensureHighresMatteForPhoto(photo);
          }
          await invoke('export_photo_to_disk_cmd', {
            imageBase64: photo.originalDataUrl,
            params: buildInvokeParams(photo),
            outputPath: savePath,
            maskBase64: photo.maskBase64 || null,
            sceneMaskBase64: getEffectiveSceneMask(photo),
          });
          renderTimeLabel.textContent = 'Đã xuất ảnh ra ổ đĩa ✓';
          return;
        }
        renderTimeLabel.textContent = 'Đã hủy xuất ảnh';
        return;
      } catch (err) {
        console.warn('Chuyển sang tải xuống trực tiếp:', err);
      }

      if (needsHighresMatte(photo.params)) {
        await ensureHighresMatteForPhoto(photo);
      }
      const res = await invoke('process_photo_cmd', {
        imageBase64: photo.originalDataUrl,
        params: buildInvokeParams(photo),
        highQuality: true,
        maskBase64: photo.maskBase64 || null,
        sceneMaskBase64: getEffectiveSceneMask(photo),
      });
      photo.processedDataUrl = res.dataUrl;
      await renderViewport();
    }

    let exportHref = photo.processedDataUrl || photo.originalDataUrl;
    const p = photo.params;
    if (hasActiveCropRect(p) || p.cropCustomW > 0 || p.cropCustomH > 0) {
      const srcImg = await loadImageElement(exportHref);
      const sw = srcImg.naturalWidth || photo.width || 800;
      const sh = srcImg.naturalHeight || photo.height || 1000;
      const sx = Math.max(0, Math.min(sw - 1, Math.round((p.cropX ?? 0) * sw)));
      const sy = Math.max(0, Math.min(sh - 1, Math.round((p.cropY ?? 0) * sh)));
      const cw = Math.max(1, Math.min(sw - sx, Math.round((p.cropW ?? 1) * sw)));
      const ch = Math.max(1, Math.min(sh - sy, Math.round((p.cropH ?? 1) * sh)));
      let outW = cw;
      let outH = ch;
      if (p.cropCustomW > 0 && p.cropCustomH > 0) {
        outW = Math.round(p.cropCustomW);
        outH = Math.round(p.cropCustomH);
      } else if (p.cropCustomW > 0) {
        outW = Math.round(p.cropCustomW);
        outH = Math.max(1, Math.round((outW * ch) / cw));
      } else if (p.cropCustomH > 0) {
        outH = Math.round(p.cropCustomH);
        outW = Math.max(1, Math.round((outH * cw) / ch));
      }
      const expCanvas = document.createElement('canvas');
      expCanvas.width = Math.max(1, outW);
      expCanvas.height = Math.max(1, outH);
      const expCtx = expCanvas.getContext('2d');
      expCtx.imageSmoothingEnabled = true;
      expCtx.imageSmoothingQuality = 'high';
      expCtx.drawImage(srcImg, sx, sy, cw, ch, 0, 0, expCanvas.width, expCanvas.height);
      exportHref = isTransparent
        ? expCanvas.toDataURL('image/png')
        : expCanvas.toDataURL('image/jpeg', 0.95);
    }

    const a = document.createElement('a');
    a.href = exportHref;
    a.download = defaultName;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    renderTimeLabel.textContent = 'Đã tải ảnh chất lượng cao ✓';
  });
}

async function initRetouchStudio() {
  setupRetouchEvents();
  renderUserPresetsList();
  updateZoomUI();

  const viewportEl = document.getElementById('canvas-viewport');
  let _resizeRaf = 0;
  const scheduleViewportSync = () => {
    cancelAnimationFrame(_resizeRaf);
    _resizeRaf = requestAnimationFrame(() => {
      if (state.workspaceMode !== 'develop' || !getActivePhoto()) return;
      renderViewport();
      updateZoomUI();
    });
  };
  if (viewportEl && typeof ResizeObserver !== 'undefined') {
    const ro = new ResizeObserver(() => scheduleViewportSync());
    ro.observe(viewportEl);
  } else {
    window.addEventListener('resize', scheduleViewportSync);
  }

  const realDemoSamples = [
    { url: '/samples/portrait_3.jpg', name: 'Chan_dung_Ao_do.jpg' },
    { url: '/samples/portrait_4.jpg', name: 'Chan_dung_Studio_Neon.jpg' },
    { url: '/samples/portrait_1.jpg', name: 'Chan_dung_Mua_thu.jpg' },
    { url: '/samples/portrait_2.jpg', name: 'Chan_dung_Ngoai_troi_85mm.jpg' },
  ];

  for (let i = 0; i < realDemoSamples.length; i++) {
    try {
      const dataUrl = await fetchUrlAsDataUrl(realDemoSamples[i].url);
      const imgEl = await loadImageElement(dataUrl);
      const params = createDefaultParams();

      let fallbackFace = params.face;
      if (hasTauriRuntime()) {
        try {
          const detected = await invoke('analyze_portrait_cmd', { imageBase64: dataUrl });
          if (detected) fallbackFace = detected;
        } catch (_) {}
      }

      const aiResult = await analyzePhotoWithNeuralAI(imgEl, fallbackFace);
      params.face = aiResult.face;

      state.photos.push({
        id: i + 1,
        name: realDemoSamples[i].name,
        originalDataUrl: dataUrl,
        processedDataUrl: dataUrl,
        width: imgEl.naturalWidth || 1024,
        height: imgEl.naturalHeight || 1280,
        mesh478: aiResult.mesh478,
        maskBase64: aiResult.maskBase64,
        sceneMaskBase64: aiResult.sceneMaskBase64,
        hairBounds: aiResult.hairBounds || null,
        landmarksDetected: aiResult.landmarksDetected,
        params,
      });
      initPhotoHistory(state.photos[state.photos.length - 1]);
    } catch (err) {
      console.error('Lỗi tải ảnh mẫu:', realDemoSamples[i].url, err);
    }
  }

  syncSlidersFromParams();
  await processActivePhoto(false);
}

initRetouchStudio();
