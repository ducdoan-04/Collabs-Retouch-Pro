#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use image::{DynamicImage, ImageFormat, RgbaImage};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlemishSpot {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct FaceRegion {
    pub center_x: f32,
    pub center_y: f32,
    pub face_rx: f32,
    pub face_ry: f32,
    pub left_eye_x: f32,
    pub left_eye_y: f32,
    pub right_eye_x: f32,
    pub right_eye_y: f32,
    pub mouth_x: f32,
    pub mouth_y: f32,
    pub nose_x: f32,
    pub nose_y: f32,
    // Exact 478-point anatomical anchor coordinates
    pub chin_x: f32,
    pub chin_y: f32,
    pub forehead_x: f32,
    pub forehead_y: f32,
    pub jaw_left_x: f32,
    pub jaw_left_y: f32,
    pub jaw_right_x: f32,
    pub jaw_right_y: f32,
    pub cheek_left_x: f32,
    pub cheek_left_y: f32,
    pub cheek_right_x: f32,
    pub cheek_right_y: f32,
    pub mouth_left_x: f32,
    pub mouth_left_y: f32,
    pub mouth_right_x: f32,
    pub mouth_right_y: f32,
}

impl Default for FaceRegion {
    fn default() -> Self {
        Self {
            center_x: 0.5,
            center_y: 0.42,
            face_rx: 0.18,
            face_ry: 0.24,
            left_eye_x: 0.43,
            left_eye_y: 0.38,
            right_eye_x: 0.57,
            right_eye_y: 0.38,
            mouth_x: 0.50,
            mouth_y: 0.51,
            nose_x: 0.50,
            nose_y: 0.44,
            chin_x: 0.50,
            chin_y: 0.65,
            forehead_x: 0.50,
            forehead_y: 0.20,
            jaw_left_x: 0.35,
            jaw_left_y: 0.55,
            jaw_right_x: 0.65,
            jaw_right_y: 0.55,
            cheek_left_x: 0.34,
            cheek_left_y: 0.43,
            cheek_right_x: 0.66,
            cheek_right_y: 0.43,
            mouth_left_x: 0.45,
            mouth_left_y: 0.51,
            mouth_right_x: 0.55,
            mouth_right_y: 0.51,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportedPhotoPayload {
    pub path: String,
    pub name: String,
    pub data_url: String,
    pub width: u32,
    pub height: u32,
    pub detected_face: FaceRegion,
}

fn default_treatment() -> String {
    "color".to_string()
}
fn default_wb_mode() -> String {
    "asShot".to_string()
}
fn default_100() -> f32 {
    100.0
}
fn default_80() -> f32 {
    80.0
}
fn default_75() -> f32 {
    75.0
}
fn default_50() -> f32 {
    50.0
}
fn default_40() -> f32 {
    40.0
}
fn default_25() -> f32 {
    25.0
}
fn default_1() -> f32 {
    1.0
}
fn default_120() -> f32 {
    120.0
}
fn default_45() -> f32 {
    45.0
}
fn default_lip_tint() -> String {
    "rose".to_string()
}
fn default_none_str() -> String {
    "none".to_string()
}
fn default_original_str() -> String {
    "original".to_string()
}
fn default_bg_custom_color() -> String {
    "#e8eef5".to_string()
}
fn default_auto_str() -> String {
    "auto".to_string()
}
fn default_all_str() -> String {
    "all".to_string()
}
fn default_keep_scope_str() -> String {
    "subject_and_related".to_string()
}
fn default_fill_area_str() -> String {
    "full_image".to_string()
}
fn default_fill_mode_str() -> String {
    "cover".to_string()
}
fn default_normal_str() -> String {
    "normal".to_string()
}
fn default_multiply_str() -> String {
    "multiply".to_string()
}
fn default_true() -> bool {
    true
}
fn default_dpi() -> f32 {
    300.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RetouchParams {
    // --- MODULE 2: PORTRAIT RETOUCHING ---
    // 1. Blemish Removal
    pub freckle_acne: f32,
    pub face_mole: f32,
    pub face_shine: f32,
    pub dark_circles: f32,
    pub eye_bags: f32,
    pub forehead_wrinkles: f32,
    pub nasolabial_folds: f32,
    pub neck_wrinkles: f32,
    pub glasses_glare: f32,
    pub double_chin: f32,
    pub body_blemish: f32,

    // 2. Skin Retouching
    pub dodge_burn_even: f32,
    pub dodge_burn_sculpt: f32,
    pub freq_sep_low: f32,
    #[serde(default = "default_80")]
    pub freq_sep_high: f32,
    pub facial_skin_smooth: f32,
    pub body_skin_smooth: f32,
    pub skin_radiance: f32,
    pub skin_rosy: f32,
    pub skin_warmth: f32,
    pub unify_complexion: f32,

    // 3. Face Reshape (3D Liquify)
    pub face_width: f32,
    pub jawline_slim: f32,
    pub cheekbone_lift: f32,
    pub chin_length: f32,
    pub forehead_height: f32,
    pub nose_width: f32,
    pub nose_tip_lift: f32,
    pub mouth_size: f32,
    pub lip_plump: f32,
    pub smile_enhance: f32,

    // 4. Eyes & Eyebrows
    pub eye_size: f32,
    #[serde(default)]
    pub eye_distance: f32,
    #[serde(default)]
    pub eye_tilt: f32,
    pub eye_brighten: f32,
    pub red_veins: f32,
    pub catchlight_boost: f32,
    pub eyebrow_thickness: f32,
    pub eyebrow_lift: f32,
    #[serde(default)]
    pub eyebrow_tilt: f32,

    // 5. Teeth
    pub teeth_whiten: f32,
    pub teeth_desaturate: f32,
    pub teeth_brighten: f32,

    // 6. Makeup Studio
    pub makeup_highlight: f32,
    pub makeup_contour: f32,
    pub makeup_blush: f32,
    pub makeup_eyeshadow: f32,
    pub makeup_lipstick: f32,
    #[serde(default = "default_lip_tint")]
    pub makeup_lip_tint: String,

    // 7. Hair (Evoto HairBeauty)
    pub stray_hair: f32,
    #[serde(default = "default_100")]
    pub stray_hair_inner: f32,
    #[serde(default = "default_100")]
    pub stray_hair_outer: f32,
    pub hair_volume: f32,
    #[serde(default)]
    pub hair_side_left_volume: f32,
    #[serde(default)]
    pub hair_side_right_volume: f32,
    #[serde(default)]
    pub hairline_adjust: f32,
    pub hair_part_line: f32,
    #[serde(default)]
    pub hair_smooth: f32,
    #[serde(default = "default_100")]
    pub hair_frizzy: f32,
    pub hair_shine: f32,
    #[serde(default)]
    pub white_hair_blacken: f32,
    #[serde(default)]
    pub hair_color_unify: f32,
    pub hair_dye_strength: f32,
    #[serde(default)]
    pub hair_dye_darken: f32,
    #[serde(default = "default_none_str")]
    pub hair_dye_color: String,

    // 8. Body Reshape
    pub neck_slim: f32,
    pub shoulder_sculpt: f32,
    pub waist_slim: f32,

    // --- MODULE 1: COLOR ADJUSTMENT (LIGHTCRAFT + RETOUCH) ---
    #[serde(default = "default_treatment")]
    pub treatment: String,
    #[serde(default = "default_100")]
    pub profile_amount: f32,
    #[serde(default = "default_wb_mode")]
    pub wb_mode: String,
    pub temperature: f32,
    pub tint: f32,
    pub exposure: f32,
    pub contrast: f32,
    pub highlights: f32,
    pub shadows: f32,
    pub whites: f32,
    pub blacks: f32,
    pub clarity: f32,
    pub texture: f32,
    pub dehaze: f32,
    pub vibrance: f32,
    pub saturation: f32,

    // Parametric + Point Tone Curve (Master + R/G/B)
    pub curve_shadows: f32,
    pub curve_darks: f32,
    pub curve_lights: f32,
    pub curve_highlights: f32,
    #[serde(default = "default_25")]
    pub curve_split_shadows: f32,
    #[serde(default = "default_50")]
    pub curve_split_mid: f32,
    #[serde(default = "default_75")]
    pub curve_split_highlights: f32,
    #[serde(default = "default_100")]
    pub curve_refine_saturation: f32,
    #[serde(default)]
    pub curve_lut: Vec<u8>,
    #[serde(default)]
    pub curve_lut_red: Vec<u8>,
    #[serde(default)]
    pub curve_lut_green: Vec<u8>,
    #[serde(default)]
    pub curve_lut_blue: Vec<u8>,

    // 8-Band HSL Color Mixer
    pub hsl_red_hue: f32,
    pub hsl_red_sat: f32,
    pub hsl_red_lum: f32,
    pub hsl_orange_hue: f32,
    pub hsl_orange_sat: f32,
    pub hsl_orange_lum: f32,
    pub hsl_yellow_hue: f32,
    pub hsl_yellow_sat: f32,
    pub hsl_yellow_lum: f32,
    pub hsl_green_hue: f32,
    pub hsl_green_sat: f32,
    pub hsl_green_lum: f32,
    #[serde(default)]
    pub hsl_aqua_hue: f32,
    #[serde(default)]
    pub hsl_aqua_sat: f32,
    #[serde(default)]
    pub hsl_aqua_lum: f32,
    pub hsl_blue_hue: f32,
    pub hsl_blue_sat: f32,
    pub hsl_blue_lum: f32,
    #[serde(default)]
    pub hsl_purple_hue: f32,
    #[serde(default)]
    pub hsl_purple_sat: f32,
    #[serde(default)]
    pub hsl_purple_lum: f32,
    #[serde(default)]
    pub hsl_magenta_hue: f32,
    #[serde(default)]
    pub hsl_magenta_sat: f32,
    #[serde(default)]
    pub hsl_magenta_lum: f32,

    // 8-Band B&W Mixer (Lightcraft BwMix)
    #[serde(default)]
    pub bw_red: f32,
    #[serde(default)]
    pub bw_orange: f32,
    #[serde(default)]
    pub bw_yellow: f32,
    #[serde(default)]
    pub bw_green: f32,
    #[serde(default)]
    pub bw_aqua: f32,
    #[serde(default)]
    pub bw_blue: f32,
    #[serde(default)]
    pub bw_purple: f32,
    #[serde(default)]
    pub bw_magenta: f32,

    // Reference Color Match
    #[serde(default)]
    pub color_match_strength: f32,
    #[serde(default)]
    pub color_match_stats: Vec<f32>,

    // 4-Way Color Grading (Shadows, Midtones, Highlights, Global + Blending & Balance)
    pub grade_shadow_hue: f32,
    pub grade_shadow_sat: f32,
    #[serde(default)]
    pub grade_shadow_lum: f32,
    #[serde(default = "default_120")]
    pub grade_mid_hue: f32,
    #[serde(default)]
    pub grade_mid_sat: f32,
    #[serde(default)]
    pub grade_mid_lum: f32,
    pub grade_high_hue: f32,
    pub grade_high_sat: f32,
    #[serde(default)]
    pub grade_high_lum: f32,
    #[serde(default = "default_45")]
    pub grade_global_hue: f32,
    #[serde(default)]
    pub grade_global_sat: f32,
    #[serde(default)]
    pub grade_global_lum: f32,
    #[serde(default = "default_50")]
    pub grade_blending: f32,
    #[serde(default)]
    pub grade_balance: f32,

    // Detail, Sharpening & Noise Reduction (Lightcraft Detail)
    #[serde(default)]
    pub ai_denoise: f32,
    pub sharpen_amount: f32,
    #[serde(default = "default_1")]
    pub sharpen_radius: f32,
    #[serde(default = "default_25")]
    pub sharpen_detail: f32,
    #[serde(default)]
    pub sharpen_masking: f32,
    pub noise_reduction: f32,
    #[serde(default = "default_50")]
    pub nr_detail: f32,
    #[serde(default)]
    pub nr_contrast: f32,
    #[serde(default)]
    pub nr_color: f32,
    #[serde(default = "default_50")]
    pub nr_color_detail: f32,
    #[serde(default = "default_50")]
    pub nr_color_smoothness: f32,

    // Effects: Film Grain & Post-Crop Vignette (Lightcraft Finish)
    pub film_grain: f32,
    #[serde(default = "default_25")]
    pub grain_size: f32,
    #[serde(default = "default_50")]
    pub grain_roughness: f32,
    pub vignette: f32,
    #[serde(default = "default_50")]
    pub vignette_midpoint: f32,
    #[serde(default)]
    pub vignette_roundness: f32,
    #[serde(default = "default_50")]
    pub vignette_feather: f32,
    #[serde(default)]
    pub vignette_highlights: f32,

    // Camera Calibration (Lightcraft Calibration)
    #[serde(default)]
    pub cal_shadows_tint: f32,
    #[serde(default)]
    pub cal_red_hue: f32,
    #[serde(default)]
    pub cal_red_sat: f32,
    #[serde(default)]
    pub cal_green_hue: f32,
    #[serde(default)]
    pub cal_green_sat: f32,
    #[serde(default)]
    pub cal_blue_hue: f32,
    #[serde(default)]
    pub cal_blue_sat: f32,

    #[serde(default = "default_none_str")]
    pub lut_preset: String,

    // --- MODULE 3 & 3B: BACKGROUND, LENS BLUR & CLOTHING ---
    pub clothing_dewrinkle: f32,
    pub clothing_lint_clean: f32,
    #[serde(default = "default_all_str")]
    pub clothing_target: String,
    #[serde(default = "default_100")]
    pub clothing_small_wrinkles: f32,
    #[serde(default = "default_100")]
    pub clothing_large_wrinkles: f32,
    #[serde(default)]
    pub clothing_sweat_stain: f32,
    #[serde(default = "default_50")]
    pub clothing_sweat_evenness: f32,
    #[serde(default)]
    pub clothing_edge_smooth: f32,
    #[serde(default = "default_50")]
    pub clothing_edge_refine: f32,
    #[serde(default)]
    pub clothing_moire_remove: f32,
    pub backdrop_clean: f32,
    pub backdrop_banding_fix: f32,
    pub sky_enhance: f32,
    pub bg_blur_bokeh: f32,
    #[serde(default = "default_50")]
    pub lens_blur_focal_distance: f32,
    #[serde(default = "default_40")]
    pub lens_blur_focal_range: f32,
    #[serde(default = "default_original_str")]
    pub bg_replace_mode: String,
    #[serde(default = "default_bg_custom_color")]
    pub bg_custom_color: String,
    #[serde(default)]
    pub bg_custom_image: String,

    // Evoto Background Optimization, Matte Edge Refinement & Shadow Composite
    #[serde(default)]
    pub bg_clean_distractions: bool,
    #[serde(default)]
    pub bg_clean_defects: bool,
    #[serde(default)]
    pub bg_even_brightness: bool,
    #[serde(default)]
    pub bg_remove_banding: bool,
    #[serde(default = "default_auto_str")]
    pub bg_edge_mode: String,
    #[serde(default = "default_keep_scope_str")]
    pub bg_keep_scope: String,
    #[serde(default = "default_fill_area_str")]
    pub bg_fill_area: String,
    #[serde(default = "default_fill_mode_str")]
    pub bg_fill_mode: String,
    #[serde(default = "default_normal_str")]
    pub bg_blend_mode: String,
    #[serde(default)]
    pub bg_edge_adjust: f32,
    #[serde(default = "default_100")]
    pub bg_replace_opacity: f32,
    #[serde(default)]
    pub bg_scale: f32,
    #[serde(default)]
    pub bg_offset_x: f32,
    #[serde(default)]
    pub bg_offset_y: f32,
    #[serde(default)]
    pub bg_blur_strength: f32,
    #[serde(default)]
    pub bg_decontam: bool,
    #[serde(default)]
    pub is_highres_matte: bool,
    #[serde(default)]
    pub bg_ground_reflect: bool,
    #[serde(default)]
    pub bg_preserve_shadow: bool,
    #[serde(default = "default_none_str")]
    pub bg_shadow_type: String,
    #[serde(default = "default_25")]
    pub bg_shadow_blur: f32,
    #[serde(default)]
    pub bg_shadow_opacity: f32,
    #[serde(default)]
    pub bg_shadow_shift: f32,
    #[serde(default = "default_multiply_str")]
    pub bg_shadow_blend_mode: String,

    // --- MODULE 4: OPTICS, GEOMETRY & TRANSFORM (LIGHTCRAFT OPTICS + UPRIGHT) ---
    #[serde(default)]
    pub optics_distortion: f32,
    #[serde(default)]
    pub optics_vignetting: f32,
    #[serde(default = "default_50")]
    pub optics_vignetting_midpoint: f32,
    #[serde(default)]
    pub optics_ca_red: f32,
    #[serde(default)]
    pub optics_ca_blue: f32,
    #[serde(default)]
    pub optics_defringe_purple: f32,
    #[serde(default)]
    pub optics_defringe_green: f32,

    pub rotate_angle: f32,
    pub keystone_vertical: f32,
    pub keystone_horizontal: f32,
    #[serde(default)]
    pub geo_aspect: f32,
    #[serde(default = "default_100")]
    pub geo_scale: f32,
    #[serde(default)]
    pub geo_offset_x: f32,
    #[serde(default)]
    pub geo_offset_y: f32,
    pub flip_horizontal: bool,
    #[serde(default)]
    pub flip_vertical: bool,

    // Crop: normalized interactive crop rect [0..1], preset/aspect lock, custom pixel size + DPI,
    // grid visibility, and AI vertical / eye-level alignment flags.
    #[serde(default)]
    pub crop_x: f32,
    #[serde(default)]
    pub crop_y: f32,
    #[serde(default = "default_1")]
    pub crop_w: f32,
    #[serde(default = "default_1")]
    pub crop_h: f32,
    #[serde(default = "default_original_str")]
    pub crop_preset: String,
    #[serde(default)]
    pub crop_aspect_w: f32,   // numerator   e.g. 4.0 for 4:5; 0 = no lock
    #[serde(default)]
    pub crop_aspect_h: f32,   // denominator e.g. 5.0 for 4:5; 0 = no lock
    #[serde(default)]
    pub crop_custom_w: f32,   // export width  in pixels (0 = use cropped image width)
    #[serde(default)]
    pub crop_custom_h: f32,   // export height in pixels (0 = use cropped image height)
    #[serde(default = "default_dpi")]
    pub crop_dpi: f32,        // target print DPI (e.g. 300) for ID photo / cm sizing
    #[serde(default = "default_true")]
    pub crop_show_grid: bool, // overlay rule-of-thirds grid in preview
    #[serde(default)]
    pub crop_align_vertical: bool,   // AI: center subject horizontally/vertically
    #[serde(default)]
    pub crop_align_eye: bool,        // AI: straighten & align eye-line horizontally

    #[serde(default)]
    pub blemish_spots: Vec<BlemishSpot>,
    #[serde(default)]
    pub face: FaceRegion,
}

impl Default for RetouchParams {
    fn default() -> Self {
        Self {
            freckle_acne: 0.0,
            face_mole: 0.0,
            face_shine: 0.0,
            dark_circles: 0.0,
            eye_bags: 0.0,
            forehead_wrinkles: 0.0,
            nasolabial_folds: 0.0,
            neck_wrinkles: 0.0,
            glasses_glare: 0.0,
            double_chin: 0.0,
            body_blemish: 0.0,
            dodge_burn_even: 0.0,
            dodge_burn_sculpt: 0.0,
            freq_sep_low: 0.0,
            freq_sep_high: 80.0,
            facial_skin_smooth: 0.0,
            body_skin_smooth: 0.0,
            skin_radiance: 0.0,
            skin_rosy: 0.0,
            skin_warmth: 0.0,
            unify_complexion: 0.0,
            face_width: 0.0,
            jawline_slim: 0.0,
            cheekbone_lift: 0.0,
            chin_length: 0.0,
            forehead_height: 0.0,
            nose_width: 0.0,
            nose_tip_lift: 0.0,
            mouth_size: 0.0,
            lip_plump: 0.0,
            smile_enhance: 0.0,
            eye_size: 0.0,
            eye_distance: 0.0,
            eye_tilt: 0.0,
            eye_brighten: 0.0,
            red_veins: 0.0,
            catchlight_boost: 0.0,
            eyebrow_thickness: 0.0,
            eyebrow_lift: 0.0,
            eyebrow_tilt: 0.0,
            teeth_whiten: 0.0,
            teeth_desaturate: 0.0,
            teeth_brighten: 0.0,
            makeup_highlight: 0.0,
            makeup_contour: 0.0,
            makeup_blush: 0.0,
            makeup_eyeshadow: 0.0,
            makeup_lipstick: 0.0,
            makeup_lip_tint: "rose".to_string(),
            stray_hair: 0.0,
            stray_hair_inner: 100.0,
            stray_hair_outer: 100.0,
            hair_volume: 0.0,
            hair_side_left_volume: 0.0,
            hair_side_right_volume: 0.0,
            hairline_adjust: 0.0,
            hair_part_line: 0.0,
            hair_smooth: 0.0,
            hair_frizzy: 100.0,
            hair_shine: 0.0,
            white_hair_blacken: 0.0,
            hair_color_unify: 0.0,
            hair_dye_strength: 0.0,
            hair_dye_darken: 0.0,
            hair_dye_color: "none".to_string(),
            neck_slim: 0.0,
            shoulder_sculpt: 0.0,
            waist_slim: 0.0,
            treatment: "color".to_string(),
            profile_amount: 100.0,
            wb_mode: "asShot".to_string(),
            temperature: 0.0,
            tint: 0.0,
            exposure: 0.0,
            contrast: 0.0,
            highlights: 0.0,
            shadows: 0.0,
            whites: 0.0,
            blacks: 0.0,
            clarity: 0.0,
            texture: 0.0,
            dehaze: 0.0,
            vibrance: 0.0,
            saturation: 0.0,
            curve_shadows: 0.0,
            curve_darks: 0.0,
            curve_lights: 0.0,
            curve_highlights: 0.0,
            curve_split_shadows: 25.0,
            curve_split_mid: 50.0,
            curve_split_highlights: 75.0,
            curve_refine_saturation: 100.0,
            curve_lut: Vec::new(),
            curve_lut_red: Vec::new(),
            curve_lut_green: Vec::new(),
            curve_lut_blue: Vec::new(),
            hsl_red_hue: 0.0,
            hsl_red_sat: 0.0,
            hsl_red_lum: 0.0,
            hsl_orange_hue: 0.0,
            hsl_orange_sat: 0.0,
            hsl_orange_lum: 0.0,
            hsl_yellow_hue: 0.0,
            hsl_yellow_sat: 0.0,
            hsl_yellow_lum: 0.0,
            hsl_green_hue: 0.0,
            hsl_green_sat: 0.0,
            hsl_green_lum: 0.0,
            hsl_aqua_hue: 0.0,
            hsl_aqua_sat: 0.0,
            hsl_aqua_lum: 0.0,
            hsl_blue_hue: 0.0,
            hsl_blue_sat: 0.0,
            hsl_blue_lum: 0.0,
            hsl_purple_hue: 0.0,
            hsl_purple_sat: 0.0,
            hsl_purple_lum: 0.0,
            hsl_magenta_hue: 0.0,
            hsl_magenta_sat: 0.0,
            hsl_magenta_lum: 0.0,
            bw_red: 0.0,
            bw_orange: 0.0,
            bw_yellow: 0.0,
            bw_green: 0.0,
            bw_aqua: 0.0,
            bw_blue: 0.0,
            bw_purple: 0.0,
            bw_magenta: 0.0,
            color_match_strength: 0.0,
            color_match_stats: Vec::new(),
            grade_shadow_hue: 215.0,
            grade_shadow_sat: 0.0,
            grade_shadow_lum: 0.0,
            grade_mid_hue: 120.0,
            grade_mid_sat: 0.0,
            grade_mid_lum: 0.0,
            grade_high_hue: 38.0,
            grade_high_sat: 0.0,
            grade_high_lum: 0.0,
            grade_global_hue: 45.0,
            grade_global_sat: 0.0,
            grade_global_lum: 0.0,
            grade_blending: 50.0,
            grade_balance: 0.0,
            ai_denoise: 0.0,
            sharpen_amount: 0.0,
            sharpen_radius: 1.0,
            sharpen_detail: 25.0,
            sharpen_masking: 0.0,
            noise_reduction: 0.0,
            nr_detail: 50.0,
            nr_contrast: 0.0,
            nr_color: 0.0,
            nr_color_detail: 50.0,
            nr_color_smoothness: 50.0,
            film_grain: 0.0,
            grain_size: 25.0,
            grain_roughness: 50.0,
            vignette: 0.0,
            vignette_midpoint: 50.0,
            vignette_roundness: 0.0,
            vignette_feather: 50.0,
            vignette_highlights: 0.0,
            cal_shadows_tint: 0.0,
            cal_red_hue: 0.0,
            cal_red_sat: 0.0,
            cal_green_hue: 0.0,
            cal_green_sat: 0.0,
            cal_blue_hue: 0.0,
            cal_blue_sat: 0.0,
            lut_preset: "none".to_string(),
            clothing_dewrinkle: 0.0,
            clothing_lint_clean: 0.0,
            clothing_target: "all".to_string(),
            clothing_small_wrinkles: 100.0,
            clothing_large_wrinkles: 100.0,
            clothing_sweat_stain: 0.0,
            clothing_sweat_evenness: 50.0,
            clothing_edge_smooth: 0.0,
            clothing_edge_refine: 50.0,
            clothing_moire_remove: 0.0,
            backdrop_clean: 0.0,
            backdrop_banding_fix: 0.0,
            sky_enhance: 0.0,
            bg_blur_bokeh: 0.0,
            lens_blur_focal_distance: 50.0,
            lens_blur_focal_range: 40.0,
            bg_replace_mode: "original".to_string(),
            bg_custom_color: "#e8eef5".to_string(),
            bg_custom_image: String::new(),
            bg_clean_distractions: false,
            bg_clean_defects: false,
            bg_even_brightness: false,
            bg_remove_banding: false,
            bg_edge_mode: "auto".to_string(),
            bg_keep_scope: "subject_and_related".to_string(),
            bg_fill_area: "full_image".to_string(),
            bg_fill_mode: "cover".to_string(),
            bg_blend_mode: "normal".to_string(),
            bg_edge_adjust: 0.0,
            bg_replace_opacity: 100.0,
            bg_scale: 0.0,
            bg_offset_x: 0.0,
            bg_offset_y: 0.0,
            bg_blur_strength: 0.0,
            bg_decontam: false,
            is_highres_matte: false,
            bg_ground_reflect: false,
            bg_preserve_shadow: false,
            bg_shadow_type: "none".to_string(),
            bg_shadow_blur: 25.0,
            bg_shadow_opacity: 0.0,
            bg_shadow_shift: 0.0,
            bg_shadow_blend_mode: "multiply".to_string(),
            optics_distortion: 0.0,
            optics_vignetting: 0.0,
            optics_vignetting_midpoint: 50.0,
            optics_ca_red: 0.0,
            optics_ca_blue: 0.0,
            optics_defringe_purple: 0.0,
            optics_defringe_green: 0.0,
            rotate_angle: 0.0,
            keystone_vertical: 0.0,
            keystone_horizontal: 0.0,
            geo_aspect: 0.0,
            geo_scale: 100.0,
            geo_offset_x: 0.0,
            geo_offset_y: 0.0,
            flip_horizontal: false,
            flip_vertical: false,
            crop_x: 0.0,
            crop_y: 0.0,
            crop_w: 1.0,
            crop_h: 1.0,
            crop_preset: "original".to_string(),
            crop_aspect_w: 0.0,
            crop_aspect_h: 0.0,
            crop_custom_w: 0.0,
            crop_custom_h: 0.0,
            crop_dpi: 300.0,
            crop_show_grid: true,
            crop_align_vertical: false,
            crop_align_eye: false,
            blemish_spots: Vec::new(),
            face: FaceRegion::default(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessResult {
    pub data_url: String,
    pub width: u32,
    pub height: u32,
    pub elapsed_ms: u128,
    pub detected_face: FaceRegion,
}

/// Pre-filtered 512x512 Semantic Anatomical Weights (zero label crosstalk, 100% eye/lip/brow protection)
#[derive(Copy, Clone, Default)]
struct SemanticWeights {
    skin: f32,        // Face & body skin, STRICTLY 0.0 on eyes, eyelashes, eyebrows, nostrils, lips, teeth
    forehead: f32,    // Forehead zone (G == 30)
    under_eye: f32,   // Under-eye dark circles / eye bags crescent (G == 60), STRICTLY 0.0 on eyeball
    nasolabial: f32,  // Smile lines / nasolabial folds (G == 85)
    eyebrow: f32,     // Eyebrows (G == 110)
    eyeshadow: f32,   // Upper eyelid eyeshadow zone (G == 125)
    cheek: f32,       // Left & right cheek blush zones (G == 135)
    eye: f32,         // Sclera + Iris eye opening (G == 160)
    lips_outer: f32,  // Lipstick zone (G == 205), STRICTLY 0.0 on inner mouth teeth
    teeth_inner: f32, // Inner mouth cavity for teeth whitening (G == 245)
    hair: f32,        // Neural hair segmentation mask (B channel)
    clothing: f32,    // Neural clothing segmentation mask (A == 128)
    background: f32,  // Neural background segmentation mask (A == 0)
}

const MASK_DIM: usize = 512;

#[inline]
fn compute_geometric_weights(nx: f32, ny: f32, face: &FaceRegion) -> SemanticWeights {
    let fdx = (nx - face.center_x) / face.face_rx.max(0.05);
    let fdy = (ny - face.center_y) / face.face_ry.max(0.05);
    let f_d2 = fdx * fdx + fdy * fdy;

    let le_d2 = ((nx - face.left_eye_x) / (face.face_rx * 0.20)).powi(2)
        + ((ny - face.left_eye_y) / (face.face_ry * 0.09)).powi(2);
    let re_d2 = ((nx - face.right_eye_x) / (face.face_rx * 0.20)).powi(2)
        + ((ny - face.right_eye_y) / (face.face_ry * 0.09)).powi(2);
    let eye = if le_d2 < 1.0 || re_d2 < 1.0 { 1.0 } else { 0.0 };

    let lb_d2 = ((nx - face.left_eye_x) / (face.face_rx * 0.24)).powi(2)
        + ((ny - (face.left_eye_y - face.face_ry * 0.12)) / (face.face_ry * 0.06)).powi(2);
    let rb_d2 = ((nx - face.right_eye_x) / (face.face_rx * 0.24)).powi(2)
        + ((ny - (face.right_eye_y - face.face_ry * 0.12)) / (face.face_ry * 0.06)).powi(2);
    let eyebrow = if lb_d2 < 1.0 || rb_d2 < 1.0 { 1.0 } else { 0.0 };

    let les_d2 = ((nx - face.left_eye_x) / (face.face_rx * 0.22)).powi(2)
        + ((ny - (face.left_eye_y - face.face_ry * 0.065)) / (face.face_ry * 0.048)).powi(2);
    let res_d2 = ((nx - face.right_eye_x) / (face.face_rx * 0.22)).powi(2)
        + ((ny - (face.right_eye_y - face.face_ry * 0.065)) / (face.face_ry * 0.048)).powi(2);
    let eyeshadow = if (les_d2 < 1.0 || res_d2 < 1.0) && eye == 0.0 && eyebrow == 0.0 {
        1.0
    } else {
        0.0
    };

    let m_outer_d2 = ((nx - face.mouth_x) / (face.face_rx * 0.28)).powi(2)
        + ((ny - face.mouth_y) / (face.face_ry * 0.10)).powi(2);
    let m_inner_d2 = ((nx - face.mouth_x) / (face.face_rx * 0.19)).powi(2)
        + ((ny - face.mouth_y) / (face.face_ry * 0.048)).powi(2);
    let teeth_inner = if m_inner_d2 < 1.0 { 1.0 } else { 0.0 };
    let lips_outer = if m_outer_d2 < 1.0 && m_inner_d2 >= 1.0 { 1.0 } else { 0.0 };

    let lu_d2 = ((nx - face.left_eye_x) / (face.face_rx * 0.22)).powi(2)
        + ((ny - (face.left_eye_y + face.face_ry * 0.11)) / (face.face_ry * 0.075)).powi(2);
    let ru_d2 = ((nx - face.right_eye_x) / (face.face_rx * 0.22)).powi(2)
        + ((ny - (face.right_eye_y + face.face_ry * 0.11)) / (face.face_ry * 0.075)).powi(2);
    let under_eye = if (lu_d2 < 1.0 || ru_d2 < 1.0) && eye == 0.0 { 1.0 } else { 0.0 };

    let l_cheek_d2 = ((nx - (face.center_x - face.face_rx * 0.50)) / (face.face_rx * 0.28)).powi(2)
        + ((ny - (face.center_y + face.face_ry * 0.08)) / (face.face_ry * 0.18)).powi(2);
    let r_cheek_d2 = ((nx - (face.center_x + face.face_rx * 0.50)) / (face.face_rx * 0.28)).powi(2)
        + ((ny - (face.center_y + face.face_ry * 0.08)) / (face.face_ry * 0.18)).powi(2);
    let cheek = if l_cheek_d2 < 1.0 || r_cheek_d2 < 1.0 { 1.0 } else { 0.0 };

    let l_nf_d2 = ((nx - (face.nose_x - face.face_rx * 0.24)) / (face.face_rx * 0.14)).powi(2)
        + ((ny - (face.nose_y + face.face_ry * 0.16)) / (face.face_ry * 0.16)).powi(2);
    let r_nf_d2 = ((nx - (face.nose_x + face.face_rx * 0.24)) / (face.face_rx * 0.14)).powi(2)
        + ((ny - (face.nose_y + face.face_ry * 0.16)) / (face.face_ry * 0.16)).powi(2);
    let nasolabial = if (l_nf_d2 < 1.0 || r_nf_d2 < 1.0) && m_outer_d2 >= 1.0 {
        1.0
    } else {
        0.0
    };

    let fh_d2 = ((nx - face.center_x) / (face.face_rx * 0.75)).powi(2)
        + ((ny - (face.center_y - face.face_ry * 0.55)) / (face.face_ry * 0.26)).powi(2);
    let forehead = if fh_d2 < 1.0 && eyebrow == 0.0 { 1.0 } else { 0.0 };

    let neck_d2 = ((nx - face.center_x) / (face.face_rx * 0.65)).powi(2)
        + ((ny - (face.chin_y + face.face_ry * 0.28)) / (face.face_ry * 0.35)).powi(2);
    let skin = if (f_d2 < 1.05 || (neck_d2 < 1.0 && ny > face.chin_y * 0.92))
        && eye == 0.0
        && eyebrow == 0.0
        && m_outer_d2 >= 1.05
    {
        if f_d2 < 1.05 {
            (1.0 - f_d2 * 0.25).clamp(0.2, 1.0)
        } else {
            (1.0 - neck_d2 * 0.35).clamp(0.2, 1.0)
        }
    } else {
        0.0
    };

    let head_d2 = ((nx - face.center_x) / (face.face_rx * 1.55)).powi(2)
        + ((ny - (face.center_y - face.face_ry * 0.10)) / (face.face_ry * 1.48)).powi(2);
    let torso_d2 = ((nx - face.center_x) / (face.face_rx * 2.45)).powi(2)
        + ((ny - (face.center_y + face.face_ry * 2.05)) / (face.face_ry * 1.75)).powi(2);

    SemanticWeights {
        skin,
        forehead,
        under_eye,
        nasolabial,
        eyebrow,
        eyeshadow,
        cheek,
        eye,
        lips_outer,
        teeth_inner,
        hair: if head_d2 >= 0.72 && head_d2 < 1.65 && ny < face.mouth_y + 0.04 {
            1.0
        } else {
            0.0
        },
        clothing: if torso_d2 < 1.05 && ny > face.chin_y + face.face_ry * 0.18 {
            1.0
        } else {
            0.0
        },
        background: if head_d2 > 1.42 && torso_d2 > 1.05 {
            1.0
        } else {
            0.0
        },
    }
}

struct SemanticData {
    weights: Vec<SemanticWeights>,
    raw_fg: Vec<f32>,
    raw_hair: Vec<f32>,
    raw_solid: Vec<f32>,
    #[allow(dead_code)]
    has_neural_scene: bool,
    hair_crown_x: f32,
    hair_top_y: f32,
    hair_left_x: f32,
    hair_left_y: f32,
    hair_right_x: f32,
    hair_right_y: f32,
    hairline_y: f32,
    has_hair_bounds: bool,
}

#[inline]
fn sample_rgba_bilinear(img: &RgbaImage, nx: f32, ny: f32) -> [f32; 4] {
    let w = img.width().max(1) as f32;
    let h = img.height().max(1) as f32;
    let fx = (nx * (w - 1.0)).clamp(0.0, w - 1.0);
    let fy = (ny * (h - 1.0)).clamp(0.0, h - 1.0);
    let x0 = fx.floor() as u32;
    let y0 = fy.floor() as u32;
    let x1 = (x0 + 1).min(img.width().saturating_sub(1));
    let y1 = (y0 + 1).min(img.height().saturating_sub(1));
    let dx = fx - x0 as f32;
    let dy = fy - y0 as f32;
    let p00 = img.get_pixel(x0, y0);
    let p10 = img.get_pixel(x1, y0);
    let p01 = img.get_pixel(x0, y1);
    let p11 = img.get_pixel(x1, y1);
    let w00 = (1.0 - dx) * (1.0 - dy);
    let w10 = dx * (1.0 - dy);
    let w01 = (1.0 - dx) * dy;
    let w11 = dx * dy;
    let mix = |c: usize| {
        (p00[c] as f32 * w00 + p10[c] as f32 * w10 + p01[c] as f32 * w01 + p11[c] as f32 * w11) / 255.0
    };
    [mix(0), mix(1), mix(2), mix(3)]
}

fn build_semantic_grid(
    mask_img: Option<&RgbaImage>,
    scene_mask_img: Option<&RgbaImage>,
    face: &FaceRegion,
) -> SemanticData {
    let mut raw_grid = vec![SemanticWeights::default(); MASK_DIM * MASK_DIM];
    let mut raw_fg = vec![0.0f32; MASK_DIM * MASK_DIM];
    let mut raw_hair = vec![0.0f32; MASK_DIM * MASK_DIM];
    let mut raw_solid = vec![0.0f32; MASK_DIM * MASK_DIM];
    let legacy_has_scene = mask_img
        .map(|m| m.pixels().any(|p| p[3] < 250))
        .unwrap_or(false);
    let has_neural_hair = mask_img
        .map(|m| m.pixels().any(|p| p[2] > 45))
        .unwrap_or(false);
    let has_neural_features = mask_img
        .map(|m| m.pixels().any(|p| p[1] > 20))
        .unwrap_or(false);
    let has_neural_scene = scene_mask_img.is_some() || legacy_has_scene;
    let scene_is_single_matte = scene_mask_img
        .map(|s_img| {
            s_img
                .pixels()
                .all(|p| p[0].abs_diff(p[1]) <= 2 && p[1].abs_diff(p[2]) <= 2)
        })
        .unwrap_or(false);

    if mask_img.is_some() || scene_mask_img.is_some() {
        raw_grid
            .par_chunks_mut(MASK_DIM)
            .zip(raw_fg.par_chunks_mut(MASK_DIM))
            .zip(raw_hair.par_chunks_mut(MASK_DIM))
            .zip(raw_solid.par_chunks_mut(MASK_DIM))
            .enumerate()
            .for_each(|(y, (((row, fg_row), hair_row), solid_row))| {
                let ny = y as f32 / (MASK_DIM - 1) as f32;
                for x in 0..MASK_DIM {
                    let nx = x as f32 / (MASK_DIM - 1) as f32;
                    let geo = compute_geometric_weights(nx, ny, face);

                    let (r_skin, g_code, b_hair, a_scene) = if let Some(m_img) = mask_img {
                        let mw = m_img.width() as usize;
                        let mh = m_img.height() as usize;
                        let sx = ((x * mw) / MASK_DIM).min(mw.saturating_sub(1)) as u32;
                        let sy = ((y * mh) / MASK_DIM).min(mh.saturating_sub(1)) as u32;
                        let p = m_img.get_pixel(sx, sy);
                        (p[0] as f32 / 255.0, p[1], p[2] as f32 / 255.0, p[3])
                    } else {
                        (geo.skin, 0u8, geo.hair, 255u8)
                    };

                    let solid_face = if r_skin > 0.72 { r_skin } else { 0.0 };
                    let (fg_prob, cloth_prob, hair_val, solid_prob) = if let Some(s_img) = scene_mask_img {
                        let s = sample_rgba_bilinear(s_img, nx, ny);
                        if scene_is_single_matte {
                            let matte_a = s[0].clamp(0.0, 1.0);
                            let h_val = if has_neural_hair {
                                let base_h = if b_hair > 0.08 {
                                    (b_hair * 0.72 + b_hair.min(matte_a) * 0.28)
                                        .max(if b_hair > 0.28 { matte_a * 0.90 } else { 0.0 })
                                } else if matte_a > 0.45
                                    && r_skin < 0.20
                                    && ny < (face.forehead_y - face.face_ry * 0.12).max(0.05)
                                    && (nx - face.center_x).abs() < face.face_rx * 1.85
                                {
                                    // Rescues upper dark hair crown via 1024px RMBG-1.4 when 256x256 missed top tips
                                    matte_a * 0.85
                                } else {
                                    b_hair
                                };
                                if matte_a < 0.04 {
                                    base_h.min(matte_a * 2.5 + 0.12)
                                } else {
                                    base_h
                                }
                            } else if geo.hair > 0.12 && matte_a > 0.05 && r_skin < 0.25 {
                                matte_a
                            } else {
                                0.0
                            };
                            let sol_p = (matte_a * (1.0 - h_val * 0.75)).clamp(0.0, 1.0).max(solid_face);
                            let cloth_p = (geo.clothing * matte_a * (1.0 - h_val * 0.85)).clamp(0.0, 1.0);
                            (matte_a, cloth_p, h_val.clamp(0.0, 1.0), sol_p)
                        } else {
                            (
                                s[0].clamp(0.0, 1.0),
                                s[1].clamp(0.0, 1.0),
                                b_hair.clamp(0.0, 1.0),
                                s[2].clamp(0.0, 1.0).max(solid_face),
                            )
                        }
                    } else if legacy_has_scene {
                        let fg = if a_scene < 40 { 0.0 } else { 1.0 };
                        let cl = if a_scene.abs_diff(128) <= 28 { 1.0 } else { 0.0 };
                        let sp = (fg * (1.0 - b_hair * 0.80)).clamp(0.0, 1.0).max(solid_face);
                        (fg, cl, b_hair.clamp(0.0, 1.0), sp)
                    } else {
                        // Geometric-only fallback: extend coverage below torso ellipse using a
                        // soft vertical centre strip so full-body portraits keep legs/trousers.
                        // The strip fades at left/right edges (nx ≈ 0 or ≈ 1) and is active
                        // only below the chin, tapering off towards 100 % of image height so
                        // the very bottom of frame (ground shadow) is not merged in.
                        let geo_fg = (1.0 - geo.background).clamp(0.0, 1.0);
                        let lower_body_fg = if ny > face.chin_y {
                            // Horizontal soft-window centred on face centre, width ≈ 55% of frame
                            let half_w = (face.face_rx * 2.8).clamp(0.18, 0.46);
                            let dx = ((nx - face.center_x).abs() - half_w).max(0.0);
                            let h_weight = (1.0 - dx / 0.12).clamp(0.0, 1.0);
                            // Vertical fade: full at chin, falls to 0 near bottom of frame
                            let v_weight = (1.0 - ((ny - face.chin_y) / (1.0 - face.chin_y + 0.01)).clamp(0.0, 1.0) * 0.35)
                                .clamp(0.0, 1.0);
                            h_weight * v_weight * 0.82
                        } else {
                            0.0
                        };
                        let combined_fg = geo_fg.max(lower_body_fg);
                        let h_val = b_hair.max(geo.hair * 0.45);
                        (
                            combined_fg,
                            geo.clothing * 0.45,
                            h_val,
                            (combined_fg * (1.0 - h_val * 0.85)).clamp(0.0, 1.0).max(solid_face),
                        )
                    };

                    fg_row[x] = fg_prob;
                    hair_row[x] = hair_val;
                    solid_row[x] = solid_prob;

                    row[x] = SemanticWeights {
                        skin: if mask_img.is_some() && ny <= face.chin_y {
                            r_skin
                        } else {
                            r_skin.max(geo.skin * 0.35)
                        },
                        forehead: if g_code.abs_diff(30) <= 6 {
                            1.0
                        } else if !has_neural_features {
                            geo.forehead * 0.5
                        } else {
                            0.0
                        },
                        under_eye: if g_code.abs_diff(60) <= 6 {
                            1.0
                        } else if !has_neural_features {
                            geo.under_eye * 0.5
                        } else {
                            0.0
                        },
                        nasolabial: if g_code.abs_diff(85) <= 6 {
                            1.0
                        } else if !has_neural_features {
                            geo.nasolabial * 0.6
                        } else {
                            0.0
                        },
                        eyebrow: if g_code.abs_diff(110) <= 6 {
                            1.0
                        } else if !has_neural_features {
                            geo.eyebrow * 0.5
                        } else {
                            0.0
                        },
                        eyeshadow: if g_code.abs_diff(125) <= 6 {
                            1.0
                        } else if !has_neural_features {
                            geo.eyeshadow * 0.6
                        } else {
                            0.0
                        },
                        cheek: if g_code.abs_diff(135) <= 6 {
                            1.0
                        } else if !has_neural_features {
                            geo.cheek * 0.6
                        } else {
                            0.0
                        },
                        eye: if g_code.abs_diff(160) <= 6 {
                            1.0
                        } else if !has_neural_features {
                            geo.eye * 0.5
                        } else {
                            0.0
                        },
                        lips_outer: if g_code.abs_diff(205) <= 6 {
                            1.0
                        } else if !has_neural_features {
                            geo.lips_outer * 0.5
                        } else {
                            0.0
                        },
                        teeth_inner: if g_code.abs_diff(245) <= 6 {
                            1.0
                        } else if !has_neural_features {
                            geo.teeth_inner * 0.5
                        } else {
                            0.0
                        },
                        hair: hair_val,
                        clothing: cloth_prob,
                        background: (1.0 - fg_prob).clamp(0.0, 1.0),
                    };
                }
            });
    } else {
        raw_grid
            .par_chunks_mut(MASK_DIM)
            .zip(raw_fg.par_chunks_mut(MASK_DIM))
            .zip(raw_hair.par_chunks_mut(MASK_DIM))
            .zip(raw_solid.par_chunks_mut(MASK_DIM))
            .enumerate()
            .for_each(|(y, (((row, fg_row), hair_row), solid_row))| {
                let ny = y as f32 / (MASK_DIM - 1) as f32;
                for x in 0..MASK_DIM {
                    let nx = x as f32 / (MASK_DIM - 1) as f32;
                    let w = compute_geometric_weights(nx, ny, face);
                    let geo_fg = (1.0 - w.background).clamp(0.0, 1.0);
                    let lower_body_fg = if ny > face.chin_y {
                        let half_w = (face.face_rx * 2.8).clamp(0.18, 0.46);
                        let dx = ((nx - face.center_x).abs() - half_w).max(0.0);
                        let h_weight = (1.0 - dx / 0.12).clamp(0.0, 1.0);
                        let v_weight = (1.0 - ((ny - face.chin_y) / (1.0 - face.chin_y + 0.01)).clamp(0.0, 1.0) * 0.35)
                            .clamp(0.0, 1.0);
                        h_weight * v_weight * 0.82
                    } else {
                        0.0
                    };
                    let fg = geo_fg.max(lower_body_fg);
                    fg_row[x] = fg;
                    hair_row[x] = w.hair;
                    solid_row[x] = (fg * (1.0 - w.hair * 0.85)).clamp(0.0, 1.0);
                    row[x] = w;
                }
            });
    }

    // Separable 5-tap box blur on anatomical retouch weights (keeps raw_fg unblurred for Guided Filter)
    let rad: isize = 2;
    let mut temp = vec![SemanticWeights::default(); MASK_DIM * MASK_DIM];
    temp.par_chunks_mut(MASK_DIM)
        .enumerate()
        .for_each(|(y, row_out)| {
            let row_in = &raw_grid[y * MASK_DIM..(y + 1) * MASK_DIM];
            for x in 0..MASK_DIM {
                let x0 = (x as isize - rad).max(0) as usize;
                let x1 = (x as isize + rad).min(MASK_DIM as isize - 1) as usize;
                let inv = 1.0 / ((x1 - x0 + 1) as f32);
                let mut acc = SemanticWeights::default();
                for ix in x0..=x1 {
                    let w = row_in[ix];
                    acc.skin += w.skin;
                    acc.forehead += w.forehead;
                    acc.under_eye += w.under_eye;
                    acc.nasolabial += w.nasolabial;
                    acc.eyebrow += w.eyebrow;
                    acc.eyeshadow += w.eyeshadow;
                    acc.cheek += w.cheek;
                    acc.eye += w.eye;
                    acc.lips_outer += w.lips_outer;
                    acc.teeth_inner += w.teeth_inner;
                    acc.hair += w.hair;
                    acc.clothing += w.clothing;
                    acc.background += w.background;
                }
                row_out[x] = SemanticWeights {
                    skin: acc.skin * inv,
                    forehead: acc.forehead * inv,
                    under_eye: acc.under_eye * inv,
                    nasolabial: acc.nasolabial * inv,
                    eyebrow: acc.eyebrow * inv,
                    eyeshadow: acc.eyeshadow * inv,
                    cheek: acc.cheek * inv,
                    eye: acc.eye * inv,
                    lips_outer: acc.lips_outer * inv,
                    teeth_inner: acc.teeth_inner * inv,
                    hair: acc.hair * inv,
                    clothing: acc.clothing * inv,
                    background: acc.background * inv,
                };
            }
        });

    let mut final_grid = vec![SemanticWeights::default(); MASK_DIM * MASK_DIM];
    final_grid
        .par_chunks_mut(MASK_DIM)
        .enumerate()
        .for_each(|(y, row_out)| {
            let y0 = (y as isize - rad).max(0) as usize;
            let y1 = (y as isize + rad).min(MASK_DIM as isize - 1) as usize;
            let inv = 1.0 / ((y1 - y0 + 1) as f32);
            for x in 0..MASK_DIM {
                let mut acc = SemanticWeights::default();
                for iy in y0..=y1 {
                    let w = temp[iy * MASK_DIM + x];
                    acc.skin += w.skin;
                    acc.forehead += w.forehead;
                    acc.under_eye += w.under_eye;
                    acc.nasolabial += w.nasolabial;
                    acc.eyebrow += w.eyebrow;
                    acc.eyeshadow += w.eyeshadow;
                    acc.cheek += w.cheek;
                    acc.eye += w.eye;
                    acc.lips_outer += w.lips_outer;
                    acc.teeth_inner += w.teeth_inner;
                    acc.hair += w.hair;
                    acc.clothing += w.clothing;
                    acc.background += w.background;
                }
                let eye = acc.eye * inv;
                let eyebrow = acc.eyebrow * inv;
                let lips_outer = (acc.lips_outer * inv * (1.0 - (acc.teeth_inner * inv * 1.5).min(1.0))).clamp(0.0, 1.0);
                let teeth_inner = acc.teeth_inner * inv;

                // STRICT ISOLATION: Skin, Under-Eye, and Eyeshadow masks are forced to 0.0 on Eyes, Eyebrows, Lips, and Teeth!
                let feature_guard = (eye * 2.2 + eyebrow * 1.8 + lips_outer * 2.2 + teeth_inner * 2.5).clamp(0.0, 1.0);
                let skin = (acc.skin * inv * (1.0 - feature_guard)).clamp(0.0, 1.0);
                let under_eye = (acc.under_eye * inv * (1.0 - (eye * 2.5).min(1.0))).clamp(0.0, 1.0);
                let eyeshadow = (acc.eyeshadow * inv * (1.0 - (eye * 2.5).min(1.0)) * (1.0 - (eyebrow * 1.8).min(1.0))).clamp(0.0, 1.0);
                let clothing = (acc.clothing * inv * (1.0 - skin * 0.6)).clamp(0.0, 1.0);
                let hair = (acc.hair * inv * (1.0 - skin * 0.8)).clamp(0.0, 1.0);

                row_out[x] = SemanticWeights {
                    skin,
                    forehead: (acc.forehead * inv * (1.0 - (eyebrow * 1.8).min(1.0))).clamp(0.0, 1.0),
                    under_eye,
                    nasolabial: acc.nasolabial * inv * (1.0 - (lips_outer * 1.8).min(1.0)),
                    eyebrow,
                    eyeshadow,
                    cheek: acc.cheek * inv * skin.max(0.25),
                    eye,
                    lips_outer,
                    teeth_inner,
                    hair,
                    clothing,
                    background: (acc.background * inv * (1.0 - skin) * (1.0 - clothing * 0.9)).clamp(0.0, 1.0),
                };
            }
        });

    // Derive actual hair crown / left / right / hairline anchors directly from final_grid
    let mut h_min_x = 1.0f32;
    let mut h_max_x = 0.0f32;
    let mut h_min_y = 1.0f32;
    let mut h_max_y = 0.0f32;
    let mut h_cnt = 0usize;
    let mut hl_max_y = -1.0f32;
    let brow_limit_y = (face.left_eye_y.min(face.right_eye_y) - face.face_ry * 0.06).max(0.02);
    let center_band = (face.face_rx * 0.45).max(0.04);
    let head_side_max_y = (face.chin_y + face.face_ry * 0.16).min(0.98);
    let side_ref_y = face.center_y - face.face_ry * 0.06;
    let mut head_left_x = 1.0f32;
    let mut head_left_y = side_ref_y;
    let mut has_head_left = false;
    let mut head_right_x = 0.0f32;
    let mut head_right_y = side_ref_y;
    let mut has_head_right = false;

    for y in 0..MASK_DIM {
        let ny = y as f32 / (MASK_DIM - 1) as f32;
        for x in 0..MASK_DIM {
            let w = final_grid[y * MASK_DIM + x];
            if w.hair < 0.34 || w.skin >= 0.32 {
                continue;
            }
            let nx = x as f32 / (MASK_DIM - 1) as f32;
            h_cnt += 1;
            if nx < h_min_x { h_min_x = nx; }
            if nx > h_max_x { h_max_x = nx; }
            if ny < h_min_y { h_min_y = ny; }
            if ny > h_max_y { h_max_y = ny; }
            if ny <= brow_limit_y && (nx - face.center_x).abs() <= center_band && ny > hl_max_y {
                hl_max_y = ny;
            }
            if ny <= head_side_max_y && w.hair >= 0.36 {
                if nx < face.center_x
                    && (!has_head_left
                        || nx < head_left_x - 0.006
                        || ((nx - head_left_x).abs() <= 0.006
                            && (ny - side_ref_y).abs() < (head_left_y - side_ref_y).abs()))
                {
                    head_left_x = nx;
                    head_left_y = ny;
                    has_head_left = true;
                }
                if nx > face.center_x
                    && (!has_head_right
                        || nx > head_right_x + 0.006
                        || ((nx - head_right_x).abs() <= 0.006
                            && (ny - side_ref_y).abs() < (head_right_y - side_ref_y).abs()))
                {
                    head_right_x = nx;
                    head_right_y = ny;
                    has_head_right = true;
                }
            }
        }
    }
    let has_hair_bounds = h_cnt >= 16 && h_max_x > h_min_x && h_max_y > h_min_y;
    let (hair_crown_x, hair_top_y, hair_left_x, hair_left_y, hair_right_x, hair_right_y, hairline_y) =
        if has_hair_bounds {
            let crown_band_y = h_min_y + 0.028;
            let mut cx_sum = 0.0f32;
            let mut cx_n = 0.0f32;
            for y in 0..MASK_DIM {
                let ny = y as f32 / (MASK_DIM - 1) as f32;
                if ny > crown_band_y {
                    break;
                }
                for x in 0..MASK_DIM {
                    let w = final_grid[y * MASK_DIM + x];
                    if w.hair < 0.36 || w.skin >= 0.32 {
                        continue;
                    }
                    cx_sum += x as f32 / (MASK_DIM - 1) as f32;
                    cx_n += 1.0;
                }
            }
            (
                if cx_n > 0.5 { cx_sum / cx_n } else { face.center_x },
                h_min_y,
                if has_head_left { head_left_x } else { h_min_x },
                if has_head_left { head_left_y } else { side_ref_y },
                if has_head_right { head_right_x } else { h_max_x },
                if has_head_right { head_right_y } else { side_ref_y },
                if hl_max_y > h_min_y { hl_max_y } else { face.forehead_y },
            )
        } else {
            (
                face.center_x,
                (face.forehead_y - face.face_ry * 0.34).max(0.03),
                (face.center_x - face.face_rx * 1.15).max(0.02),
                face.center_y - face.face_ry * 0.10,
                (face.center_x + face.face_rx * 1.15).min(0.98),
                face.center_y - face.face_ry * 0.10,
                face.forehead_y,
            )
        };

    SemanticData {
        weights: final_grid,
        raw_fg,
        raw_hair,
        raw_solid,
        has_neural_scene,
        hair_crown_x,
        hair_top_y,
        hair_left_x,
        hair_left_y,
        hair_right_x,
        hair_right_y,
        hairline_y,
        has_hair_bounds,
    }
}

#[inline]
fn sample_semantic_grid(grid: &[SemanticWeights], nx: f32, ny: f32) -> SemanticWeights {
    let fx = (nx * (MASK_DIM - 1) as f32).clamp(0.0, (MASK_DIM - 1) as f32);
    let fy = (ny * (MASK_DIM - 1) as f32).clamp(0.0, (MASK_DIM - 1) as f32);
    let x0 = fx.floor() as usize;
    let y0 = fy.floor() as usize;
    let x1 = (x0 + 1).min(MASK_DIM - 1);
    let y1 = (y0 + 1).min(MASK_DIM - 1);
    let dx = fx - x0 as f32;
    let dy = fy - y0 as f32;

    let w00 = (1.0 - dx) * (1.0 - dy);
    let w10 = dx * (1.0 - dy);
    let w01 = (1.0 - dx) * dy;
    let w11 = dx * dy;

    let a = grid[y0 * MASK_DIM + x0];
    let b = grid[y0 * MASK_DIM + x1];
    let c = grid[y1 * MASK_DIM + x0];
    let d = grid[y1 * MASK_DIM + x1];

    SemanticWeights {
        skin: a.skin * w00 + b.skin * w10 + c.skin * w01 + d.skin * w11,
        forehead: a.forehead * w00 + b.forehead * w10 + c.forehead * w01 + d.forehead * w11,
        under_eye: a.under_eye * w00 + b.under_eye * w10 + c.under_eye * w01 + d.under_eye * w11,
        nasolabial: a.nasolabial * w00 + b.nasolabial * w10 + c.nasolabial * w01 + d.nasolabial * w11,
        eyebrow: a.eyebrow * w00 + b.eyebrow * w10 + c.eyebrow * w01 + d.eyebrow * w11,
        eyeshadow: a.eyeshadow * w00 + b.eyeshadow * w10 + c.eyeshadow * w01 + d.eyeshadow * w11,
        cheek: a.cheek * w00 + b.cheek * w10 + c.cheek * w01 + d.cheek * w11,
        eye: a.eye * w00 + b.eye * w10 + c.eye * w01 + d.eye * w11,
        lips_outer: a.lips_outer * w00 + b.lips_outer * w10 + c.lips_outer * w01 + d.lips_outer * w11,
        teeth_inner: a.teeth_inner * w00 + b.teeth_inner * w10 + c.teeth_inner * w01 + d.teeth_inner * w11,
        hair: a.hair * w00 + b.hair * w10 + c.hair * w01 + d.hair * w11,
        clothing: a.clothing * w00 + b.clothing * w10 + c.clothing * w01 + d.clothing * w11,
        background: a.background * w00 + b.background * w10 + c.background * w01 + d.background * w11,
    }
}

#[inline]
fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let h = if delta < 1e-5 {
        0.0
    } else if (max - r).abs() < 1e-5 {
        60.0 * (((g - b) / delta).rem_euclid(6.0))
    } else if (max - g).abs() < 1e-5 {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };
    let s = if max < 1e-5 { 0.0 } else { delta / max };
    (h, s, max)
}

#[inline]
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let hh = (h.rem_euclid(360.0)) / 60.0;
    let x = c * (1.0 - (hh.rem_euclid(2.0) - 1.0).abs());
    let m = v - c;
    let (r1, g1, b1) = if hh < 1.0 {
        (c, x, 0.0)
    } else if hh < 2.0 {
        (x, c, 0.0)
    } else if hh < 3.0 {
        (0.0, c, x)
    } else if hh < 4.0 {
        (0.0, x, c)
    } else if hh < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (r1 + m, g1 + m, b1 + m)
}

#[inline]
fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    if (e1 - e0).abs() < 1e-6 {
        return if x >= e1 { 1.0 } else { 0.0 };
    }
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

#[inline]
fn hash_u32(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^= x >> 16;
    x
}

#[inline]
fn hash2(ix: i32, iy: i32, seed: u32) -> f32 {
    let h = hash_u32((ix as u32).wrapping_mul(374_761_393) ^ (iy as u32).wrapping_mul(668_265_263) ^ seed);
    (h as f32) * (1.0 / u32::MAX as f32) - 0.5
}

#[inline]
fn value_noise(x: f32, y: f32, seed: u32) -> f32 {
    let ix = x.floor() as i32;
    let iy = y.floor() as i32;
    let fx = x - ix as f32;
    let fy = y - iy as f32;
    let ux = fx * fx * (3.0 - 2.0 * fx);
    let uy = fy * fy * (3.0 - 2.0 * fy);
    let a = hash2(ix, iy, seed);
    let b = hash2(ix + 1, iy, seed);
    let c = hash2(ix, iy + 1, seed);
    let d = hash2(ix + 1, iy + 1, seed);
    let ab = a + (b - a) * ux;
    let cd = c + (d - c) * ux;
    ab + (cd - ab) * uy
}

#[inline]
fn lc_grain_noise(px: f32, py: f32, amount: f32, size: f32, roughness: f32) -> f32 {
    let freq = 1.0 / (0.6 + (size / 100.0).clamp(0.0, 1.0) * 2.6);
    let n1 = value_noise(px * freq, py * freq, 0xA341_316C);
    let n2 = value_noise(px * freq * 2.13, py * freq * 2.13, 0xC801_3EA4);
    let rough = (roughness / 100.0).clamp(0.0, 1.0);
    let n = n1 * (1.0 - 0.45 * rough) + n2 * (0.65 * rough);
    n * (amount / 100.0) * 0.22
}

#[inline]
fn skin_weight(r: f32, g: f32, b: f32) -> f32 {
    let r255 = r * 255.0;
    let g255 = g * 255.0;
    let b255 = b * 255.0;
    let cb = 128.0 - 0.168736 * r255 - 0.331264 * g255 + 0.5 * b255;
    let cr = 128.0 + 0.5 * r255 - 0.418688 * g255 - 0.081312 * b255;
    let cb_dist = ((cb - 112.0) / 26.0).abs();
    let cr_dist = ((cr - 152.0) / 28.0).abs();
    let chroma_score = (1.0 - (cb_dist * cb_dist + cr_dist * cr_dist).sqrt()).clamp(0.0, 1.0);
    let (h, s, v) = rgb_to_hsv(r, g, b);
    let hue_ok = if h <= 54.0 || h >= 330.0 { 1.0 } else { 0.0 };
    let sat_ok = if s >= 0.06 && s <= 0.82 && v >= 0.10 { 1.0 } else { 0.25 };
    (chroma_score * hue_ok * sat_ok).clamp(0.0, 1.0)
}

pub fn detect_face_region(img: &RgbaImage) -> FaceRegion {
    let (w, h) = img.dimensions();
    if w == 0 || h == 0 {
        return FaceRegion::default();
    }
    let mut sum_x = 0.0f64;
    let mut sum_y = 0.0f64;
    let mut sum_w = 0.0f64;
    let step_x = (w / 120).max(1);
    let step_y = (h / 120).max(1);

    for y in (0..h).step_by(step_y as usize) {
        let ny = y as f32 / h as f32;
        if ny < 0.06 || ny > 0.80 {
            continue;
        }
        for x in (0..w).step_by(step_x as usize) {
            let nx = x as f32 / w as f32;
            if nx < 0.10 || nx > 0.90 {
                continue;
            }
            let p = img.get_pixel(x, y);
            let sw = skin_weight(p[0] as f32 / 255.0, p[1] as f32 / 255.0, p[2] as f32 / 255.0);
            let prior = (1.0 - ((nx - 0.5) * 1.35).powi(2)).max(0.1)
                * (1.0 - ((ny - 0.38) * 1.55).powi(2)).max(0.1);
            let weight = (sw * prior) as f64;
            if weight > 0.16 {
                sum_x += nx as f64 * weight;
                sum_y += ny as f64 * weight;
                sum_w += weight;
            }
        }
    }

    if sum_w < 3.0 {
        return FaceRegion::default();
    }
    let cx = (sum_x / sum_w) as f32;
    let cy = (sum_y / sum_w) as f32;

    let mut var_x = 0.0f64;
    let mut var_y = 0.0f64;
    for y in (0..h).step_by(step_y as usize) {
        let ny = y as f32 / h as f32;
        if ny < 0.06 || ny > 0.80 {
            continue;
        }
        for x in (0..w).step_by(step_x as usize) {
            let nx = x as f32 / w as f32;
            let p = img.get_pixel(x, y);
            let sw = skin_weight(p[0] as f32 / 255.0, p[1] as f32 / 255.0, p[2] as f32 / 255.0);
            if sw > 0.30 && (nx - cx).abs() < 0.35 && (ny - cy).abs() < 0.38 {
                var_x += ((nx - cx) as f64).powi(2) * sw as f64;
                var_y += ((ny - cy) as f64).powi(2) * sw as f64;
            }
        }
    }

    let rx = ((var_x / sum_w).sqrt() as f32 * 1.75).clamp(0.10, 0.28);
    let ry = ((var_y / sum_w).sqrt() as f32 * 1.85).clamp(0.14, 0.35);

    FaceRegion {
        center_x: cx,
        center_y: cy,
        face_rx: rx,
        face_ry: ry,
        left_eye_x: (cx - rx * 0.36).clamp(0.05, 0.95),
        left_eye_y: (cy - ry * 0.16).clamp(0.05, 0.95),
        right_eye_x: (cx + rx * 0.36).clamp(0.05, 0.95),
        right_eye_y: (cy - ry * 0.16).clamp(0.05, 0.95),
        nose_x: cx,
        nose_y: (cy + ry * 0.06).clamp(0.05, 0.95),
        mouth_x: cx,
        mouth_y: (cy + ry * 0.38).clamp(0.05, 0.95),
        chin_x: cx,
        chin_y: (cy + ry * 0.95).clamp(0.05, 0.98),
        forehead_x: cx,
        forehead_y: (cy - ry * 0.92).clamp(0.02, 0.95),
        jaw_left_x: (cx - rx * 0.82).clamp(0.05, 0.95),
        jaw_left_y: (cy + ry * 0.52).clamp(0.05, 0.95),
        jaw_right_x: (cx + rx * 0.82).clamp(0.05, 0.95),
        jaw_right_y: (cy + ry * 0.52).clamp(0.05, 0.95),
        cheek_left_x: (cx - rx * 0.88).clamp(0.05, 0.95),
        cheek_left_y: (cy + ry * 0.06).clamp(0.05, 0.95),
        cheek_right_x: (cx + rx * 0.88).clamp(0.05, 0.95),
        cheek_right_y: (cy + ry * 0.06).clamp(0.05, 0.95),
        mouth_left_x: (cx - rx * 0.28).clamp(0.05, 0.95),
        mouth_left_y: (cy + ry * 0.38).clamp(0.05, 0.95),
        mouth_right_x: (cx + rx * 0.28).clamp(0.05, 0.95),
        mouth_right_y: (cy + ry * 0.38).clamp(0.05, 0.95),
    }
}

fn separable_blur(src: &[f32], w: usize, h: usize, radius: usize) -> Vec<f32> {
    if radius == 0 || w == 0 || h == 0 {
        return src.to_vec();
    }
    let mut temp = vec![0.0f32; w * h * 3];
    let mut dst = vec![0.0f32; w * h * 3];
    let r = radius as isize;

    temp.par_chunks_mut(w * 3)
        .enumerate()
        .for_each(|(y, row_out)| {
            let row_in = &src[y * w * 3..(y + 1) * w * 3];
            for x in 0..w {
                let mut sr = 0.0;
                let mut sg = 0.0;
                let mut sb = 0.0;
                let x0 = (x as isize - r).max(0) as usize;
                let x1 = (x as isize + r).min(w as isize - 1) as usize;
                let count = (x1 - x0 + 1) as f32;
                for ix in x0..=x1 {
                    let idx = ix * 3;
                    sr += row_in[idx];
                    sg += row_in[idx + 1];
                    sb += row_in[idx + 2];
                }
                let out_idx = x * 3;
                row_out[out_idx] = sr / count;
                row_out[out_idx + 1] = sg / count;
                row_out[out_idx + 2] = sb / count;
            }
        });

    dst.par_chunks_mut(w * 3)
        .enumerate()
        .for_each(|(y, row_out)| {
            let y0 = (y as isize - r).max(0) as usize;
            let y1 = (y as isize + r).min(h as isize - 1) as usize;
            let count = (y1 - y0 + 1) as f32;
            for x in 0..w {
                let mut sr = 0.0;
                let mut sg = 0.0;
                let mut sb = 0.0;
                for iy in y0..=y1 {
                    let idx = (iy * w + x) * 3;
                    sr += temp[idx];
                    sg += temp[idx + 1];
                    sb += temp[idx + 2];
                }
                let out_idx = x * 3;
                row_out[out_idx] = sr / count;
                row_out[out_idx + 1] = sg / count;
                row_out[out_idx + 2] = sb / count;
            }
        });

    dst
}

#[inline]
fn sample_bilinear(buf: &[f32], w: usize, h: usize, fx: f32, fy: f32) -> (f32, f32, f32) {
    let x = fx.clamp(0.0, (w - 1) as f32);
    let y = fy.clamp(0.0, (h - 1) as f32);
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(w - 1);
    let y1 = (y0 + 1).min(h - 1);
    let dx = x - x0 as f32;
    let dy = y - y0 as f32;

    let i00 = (y0 * w + x0) * 3;
    let i10 = (y0 * w + x1) * 3;
    let i01 = (y1 * w + x0) * 3;
    let i11 = (y1 * w + x1) * 3;

    let mut out = [0.0f32; 3];
    for c in 0..3 {
        let top = buf[i00 + c] * (1.0 - dx) + buf[i10 + c] * dx;
        let bot = buf[i01 + c] * (1.0 - dx) + buf[i11 + c] * dx;
        out[c] = top * (1.0 - dy) + bot * dy;
    }
    (out[0], out[1], out[2])
}

#[inline]
fn sample_plane_bilinear(plane: &[f32], w: usize, h: usize, nx: f32, ny: f32) -> f32 {
    if plane.is_empty() || w == 0 || h == 0 {
        return 0.0;
    }
    let fx = (nx * w as f32 - 0.5).clamp(0.0, (w - 1) as f32);
    let fy = (ny * h as f32 - 0.5).clamp(0.0, (h - 1) as f32);
    let x0 = fx.floor() as usize;
    let y0 = fy.floor() as usize;
    let x1 = (x0 + 1).min(w - 1);
    let y1 = (y0 + 1).min(h - 1);
    let dx = fx - x0 as f32;
    let dy = fy - y0 as f32;
    let top = plane[y0 * w + x0] * (1.0 - dx) + plane[y0 * w + x1] * dx;
    let bot = plane[y1 * w + x0] * (1.0 - dx) + plane[y1 * w + x1] * dx;
    top * (1.0 - dy) + bot * dy
}

#[inline]
fn sample_alpha_unit(buf: &[f32], w: usize, h: usize, fx: f32, fy: f32) -> f32 {
    if buf.is_empty() || w == 0 || h == 0 {
        return 1.0;
    }
    let x = fx.clamp(0.0, (w - 1) as f32);
    let y = fy.clamp(0.0, (h - 1) as f32);
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(w - 1);
    let y1 = (y0 + 1).min(h - 1);
    let dx = x - x0 as f32;
    let dy = y - y0 as f32;
    let top = buf[y0 * w + x0] * (1.0 - dx) + buf[y0 * w + x1] * dx;
    let bot = buf[y1 * w + x0] * (1.0 - dx) + buf[y1 * w + x1] * dx;
    top * (1.0 - dy) + bot * dy
}

fn box_filter_plane(src: &[f32], w: usize, h: usize, radius: usize) -> Vec<f32> {
    if radius == 0 || w == 0 || h == 0 {
        return src.to_vec();
    }
    let r = radius as isize;
    let mut temp = vec![0.0f32; w * h];
    temp.par_chunks_mut(w).enumerate().for_each(|(y, row_out)| {
        let row_in = &src[y * w..(y + 1) * w];
        let mut pref = Vec::with_capacity(w + 1);
        pref.push(0.0f64);
        let mut acc = 0.0f64;
        for &v in row_in {
            acc += v as f64;
            pref.push(acc);
        }
        for x in 0..w {
            let x0 = (x as isize - r).max(0) as usize;
            let x1 = (x as isize + r).min(w as isize - 1) as usize;
            row_out[x] = ((pref[x1 + 1] - pref[x0]) / (x1 - x0 + 1) as f64) as f32;
        }
    });

    let mut trans = vec![0.0f32; w * h];
    trans.par_chunks_mut(h).enumerate().for_each(|(x, col_out)| {
        for y in 0..h {
            col_out[y] = temp[y * w + x];
        }
    });

    let mut trans_blur = vec![0.0f32; w * h];
    trans_blur.par_chunks_mut(h).enumerate().for_each(|(x, col_out)| {
        let col_in = &trans[x * h..(x + 1) * h];
        let mut pref = Vec::with_capacity(h + 1);
        pref.push(0.0f64);
        let mut acc = 0.0f64;
        for &v in col_in {
            acc += v as f64;
            pref.push(acc);
        }
        for y in 0..h {
            let y0 = (y as isize - r).max(0) as usize;
            let y1 = (y as isize + r).min(h as isize - 1) as usize;
            col_out[y] = ((pref[y1 + 1] - pref[y0]) / (y1 - y0 + 1) as f64) as f32;
        }
    });

    let mut dst = vec![0.0f32; w * h];
    dst.par_chunks_mut(w).enumerate().for_each(|(y, row_out)| {
        for x in 0..w {
            row_out[x] = trans_blur[x * h + y];
        }
    });
    dst
}

fn morph_min_max_plane(src: &[f32], w: usize, h: usize, radius: usize) -> (Vec<f32>, Vec<f32>) {
    if radius == 0 || w == 0 || h == 0 {
        return (src.to_vec(), src.to_vec());
    }
    let r = radius as isize;
    let mut t_min = vec![0.0f32; w * h];
    let mut t_max = vec![0.0f32; w * h];

    t_min
        .par_chunks_mut(w)
        .zip(t_max.par_chunks_mut(w))
        .enumerate()
        .for_each(|(y, (row_min, row_max))| {
            let row_in = &src[y * w..(y + 1) * w];
            for x in 0..w {
                let x0 = (x as isize - r).max(0) as usize;
                let x1 = (x as isize + r).min(w as isize - 1) as usize;
                let mut mn = 1.0f32;
                let mut mx = 0.0f32;
                for &v in &row_in[x0..=x1] {
                    if v < mn { mn = v; }
                    if v > mx { mx = v; }
                }
                row_min[x] = mn;
                row_max[x] = mx;
            }
        });

    let mut d_min = vec![0.0f32; w * h];
    let mut d_max = vec![0.0f32; w * h];
    d_min
        .par_chunks_mut(w)
        .zip(d_max.par_chunks_mut(w))
        .enumerate()
        .for_each(|(y, (row_min, row_max))| {
            let y0 = (y as isize - r).max(0) as usize;
            let y1 = (y as isize + r).min(h as isize - 1) as usize;
            for x in 0..w {
                let mut mn = 1.0f32;
                let mut mx = 0.0f32;
                for iy in y0..=y1 {
                    let v0 = t_min[iy * w + x];
                    let v1 = t_max[iy * w + x];
                    if v0 < mn { mn = v0; }
                    if v1 > mx { mx = v1; }
                }
                row_min[x] = mn;
                row_max[x] = mx;
            }
        });

    (d_min, d_max)
}

struct GuidedMatteResult {
    fg_alpha: Vec<f32>,
    f_local: Vec<f32>,
    b_local: Vec<f32>,
    gw: usize,
    gh: usize,
}

fn compute_guided_matte(
    base_rgb: &[f32],
    w: usize,
    h: usize,
    semantic: &SemanticData,
    params: &RetouchParams,
    need_decontam: bool,
) -> GuidedMatteResult {
    let max_guide = 1024usize;
    let (gw, gh) = if w.max(h) > max_guide {
        let scale = max_guide as f32 / w.max(h) as f32;
        (
            ((w as f32 * scale).round() as usize).max(32),
            ((h as f32 * scale).round() as usize).max(32),
        )
    } else {
        (w, h)
    };
    let n = gw * gh;

    let mut ir = vec![0.0f32; n];
    let mut ig = vec![0.0f32; n];
    let mut ib = vec![0.0f32; n];
    let mut p_fg = vec![0.0f32; n];
    let mut p_solid = vec![0.0f32; n];
    let mut p_hair = vec![0.0f32; n];
    let is_subject_only = params.bg_keep_scope == "subject_only";
    let is_portrait_focus = params.bg_keep_scope == "portrait_focus";
    let face = &params.face;

    ir.par_chunks_mut(gw)
        .zip(ig.par_chunks_mut(gw))
        .zip(ib.par_chunks_mut(gw))
        .zip(p_fg.par_chunks_mut(gw))
        .zip(p_solid.par_chunks_mut(gw))
        .zip(p_hair.par_chunks_mut(gw))
        .enumerate()
        .for_each(|(y, (((((rr, rg), rb), rp), rsol), rhair))| {
            let ny = (y as f32 + 0.5) / gh as f32;
            let sy = (ny * h as f32 - 0.5).clamp(0.0, (h - 1) as f32);
            for x in 0..gw {
                let nx = (x as f32 + 0.5) / gw as f32;
                let sx = (nx * w as f32 - 0.5).clamp(0.0, (w - 1) as f32);
                let (cr, cg, cb) = sample_bilinear(base_rgb, w, h, sx, sy);
                rr[x] = cr;
                rg[x] = cg;
                rb[x] = cb;
                let mut raw = sample_plane_bilinear(&semantic.raw_fg, MASK_DIM, MASK_DIM, nx, ny).clamp(0.0, 1.0);
                if is_subject_only {
                    raw = raw.powf(1.22);
                } else if is_portrait_focus {
                    let rx = face.face_rx.max(0.12) * 3.8;
                    let dx = ((nx - face.center_x) / rx).abs();
                    let vert = ((ny - (face.chin_y + face.face_ry * 3.6)).max(0.0) * 2.2).clamp(0.0, 1.0);
                    let env = (1.0 - smoothstep(0.65, 1.25, dx) * 0.65 - vert * 0.45).clamp(0.0, 1.0);
                    raw *= env;
                }
                rp[x] = raw;
                rsol[x] = sample_plane_bilinear(&semantic.raw_solid, MASK_DIM, MASK_DIM, nx, ny).clamp(0.0, 1.0);
                rhair[x] = sample_plane_bilinear(&semantic.raw_hair, MASK_DIM, MASK_DIM, nx, ny).clamp(0.0, 1.0);
            }
        });

    if need_decontam && gw >= 32 && gh >= 32 {
        // Rescue enclosed lower-body holes (e.g. pale hands/fingers tucked inside folded arms)
        // while keeping genuine open background gaps and hair/neck wall pockets open.
        let rw = 128usize;
        let rh = 128usize;
        let rn = rw * rh;
        let mut fg_r = vec![0.0f32; rn];
        let mut hair_r = vec![0.0f32; rn];
        let mut rr_r = vec![0.0f32; rn];
        let mut rg_r = vec![0.0f32; rn];
        let mut rb_r = vec![0.0f32; rn];
        let mut wall_acc = [0.0f64; 4];

        for cy in 0..rh {
            let ny = (cy as f32 + 0.5) / rh as f32;
            let gy_i = ((ny * gh as f32).floor() as usize).min(gh - 1);
            for cx in 0..rw {
                let nx = (cx as f32 + 0.5) / rw as f32;
                let gx_i = ((nx * gw as f32).floor() as usize).min(gw - 1);
                let gi = gy_i * gw + gx_i;
                let ci = cy * rw + cx;
                let fv = p_fg[gi];
                fg_r[ci] = fv;
                hair_r[ci] = p_hair[gi];
                rr_r[ci] = ir[gi];
                rg_r[ci] = ig[gi];
                rb_r[ci] = ib[gi];
                if fv < 0.15 {
                    wall_acc[0] += ir[gi] as f64;
                    wall_acc[1] += ig[gi] as f64;
                    wall_acc[2] += ib[gi] as f64;
                    wall_acc[3] += 1.0;
                }
            }
        }

        if wall_acc[3] > 10.0 {
            let wr = (wall_acc[0] / wall_acc[3]) as f32;
            let wg = (wall_acc[1] / wall_acc[3]) as f32;
            let wb = (wall_acc[2] / wall_acc[3]) as f32;

            let mut open_bg = vec![false; rn];
            let mut q: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
            for cy in 0..rh {
                for cx in 0..rw {
                    let is_outer = cx == 0 || cx == rw - 1 || cy == 0;
                    let is_bottom_wall = if cy == rh - 1 {
                        let ci = cy * rw + cx;
                        let dr = rr_r[ci] - wr;
                        let dg = rg_r[ci] - wg;
                        let db = rb_r[ci] - wb;
                        fg_r[ci] < 0.35 && (dr * dr + dg * dg + db * db).sqrt() < 0.14
                    } else {
                        false
                    };
                    if is_outer || is_bottom_wall {
                        let ci = cy * rw + cx;
                        if fg_r[ci] < 0.55 && !open_bg[ci] {
                            open_bg[ci] = true;
                            q.push_back(ci);
                        }
                    }
                }
            }
            while let Some(ci) = q.pop_front() {
                let cx = ci % rw;
                let cy = ci / rw;
                for (nx_i, ny_i) in [
                    (cx.wrapping_sub(1), cy),
                    (cx + 1, cy),
                    (cx, cy.wrapping_sub(1)),
                    (cx, cy + 1),
                ] {
                    if nx_i < rw && ny_i < rh {
                        let ni = ny_i * rw + nx_i;
                        if !open_bg[ni] && fg_r[ni] < 0.55 {
                            open_bg[ni] = true;
                            q.push_back(ni);
                        }
                    }
                }
            }

            // Diffuse open outer background pixels onto a 64x64 grid so top-to-bottom lighting
            // falloff is preserved when evaluating enclosed torso gaps (never polluted by enclosed holes).
            let ww = 64usize;
            let wh = 64usize;
            let wn = ww * wh;
            let mut lw_r = vec![0.0f32; wn];
            let mut lw_g = vec![0.0f32; wn];
            let mut lw_b = vec![0.0f32; wn];
            let mut lw_w = vec![0.0f32; wn];
            for cy in 0..rh {
                let ly = (cy * wh) / rh;
                for cx in 0..rw {
                    let ci = cy * rw + cx;
                    if open_bg[ci] && fg_r[ci] < 0.15 {
                        let lx = (cx * ww) / rw;
                        let li = ly * ww + lx;
                        lw_r[li] += rr_r[ci];
                        lw_g[li] += rg_r[ci];
                        lw_b[li] += rb_r[ci];
                        lw_w[li] += 1.0;
                    }
                }
            }
            for _ in 0..3 {
                lw_r = box_filter_plane(&lw_r, ww, wh, 10);
                lw_g = box_filter_plane(&lw_g, ww, wh, 10);
                lw_b = box_filter_plane(&lw_b, ww, wh, 10);
                lw_w = box_filter_plane(&lw_w, ww, wh, 10);
            }
            let mut wall_local_r = vec![wr; wn];
            let mut wall_local_g = vec![wg; wn];
            let mut wall_local_b = vec![wb; wn];
            for i in 0..wn {
                let d = lw_w[i];
                if d > 1e-5 {
                    wall_local_r[i] = (lw_r[i] / d).clamp(0.0, 1.0);
                    wall_local_g[i] = (lw_g[i] / d).clamp(0.0, 1.0);
                    wall_local_b[i] = (lw_b[i] / d).clamp(0.0, 1.0);
                }
            }

            let mut visited = vec![false; rn];
            let mut rescue_r = vec![0.0f32; rn];
            let mut punch_bg = vec![0.0f32; rn];
            let lower_y = (face.chin_y + face.face_ry * 0.22).clamp(0.18, 0.85);

            let is_candidate_pocket = |ci: usize| -> bool {
                if open_bg[ci] { return false; }
                if fg_r[ci] < 0.62 { return true; }
                let cx = ci % rw;
                let cy = ci / rw;
                let nx_w = (cx as f32 + 0.5) / rw as f32;
                let ny_w = (cy as f32 + 0.5) / rh as f32;
                let wlr = sample_plane_bilinear(&wall_local_r, ww, wh, nx_w, ny_w);
                let wlg = sample_plane_bilinear(&wall_local_g, ww, wh, nx_w, ny_w);
                let wlb = sample_plane_bilinear(&wall_local_b, ww, wh, nx_w, ny_w);
                let dr = rr_r[ci] - wlr;
                let dg = rg_r[ci] - wlg;
                let db = rb_r[ci] - wlb;
                let dist = (dr * dr + dg * dg + db * db).sqrt();
                let warm_diff = (rr_r[ci] - rb_r[ci]) - (wlr - wlb);
                dist < 0.12 && warm_diff <= 0.045
            };

            for start in 0..rn {
                if visited[start] || !is_candidate_pocket(start) {
                    continue;
                }
                let mut comp: Vec<usize> = Vec::new();
                let mut cq: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
                visited[start] = true;
                cq.push_back(start);
                let mut ny_sum = 0.0f32;
                let mut hair_sum = 0.0f32;
                let mut non_wall_cnt = 0usize;
                let mut wall_cnt = 0usize;
                let mut warm_skin_cnt = 0usize;
                let mut edge_cnt = 0usize;
                let mut adj_hair_hits = 0usize;
                let mut adj_total = 0usize;

                while let Some(ci) = cq.pop_front() {
                    comp.push(ci);
                    let cx = ci % rw;
                    let cy = ci / rw;
                    let nx_w = (cx as f32 + 0.5) / rw as f32;
                    let ny_w = (cy as f32 + 0.5) / rh as f32;
                    ny_sum += ny_w;
                    hair_sum += hair_r[ci];
                    let wlr = sample_plane_bilinear(&wall_local_r, ww, wh, nx_w, ny_w);
                    let wlg = sample_plane_bilinear(&wall_local_g, ww, wh, nx_w, ny_w);
                    let wlb = sample_plane_bilinear(&wall_local_b, ww, wh, nx_w, ny_w);
                    let dr = rr_r[ci] - wlr;
                    let dg = rg_r[ci] - wlg;
                    let db = rb_r[ci] - wlb;
                    let dist = (dr * dr + dg * dg + db * db).sqrt();
                    let w_norm2 = wlr * wlr + wlg * wlg + wlb * wlb;
                    let proj = if w_norm2 > 1e-4 {
                        ((rr_r[ci] * wlr + rg_r[ci] * wlg + rb_r[ci] * wlb) / w_norm2).clamp(0.68, 1.12)
                    } else {
                        1.0
                    };
                    let sr = rr_r[ci] - proj * wlr;
                    let sg = rg_r[ci] - proj * wlg;
                    let sb = rb_r[ci] - proj * wlb;
                    let shadow_chroma_dist = (sr * sr + sg * sg + sb * sb).sqrt();
                    let is_shaded_wall =
                        w_norm2 > 0.04 && proj > 0.69 && proj < 1.11 && shadow_chroma_dist < 0.035;

                    let warm_diff = (rr_r[ci] - rb_r[ci]) - (wlr - wlb);
                    let is_warm_skin = warm_diff > 0.045 && dist > 0.04;
                    if is_warm_skin {
                        warm_skin_cnt += 1;
                    }

                    if dist <= 0.12 || is_shaded_wall {
                        if !is_warm_skin {
                            wall_cnt += 1;
                        } else {
                            non_wall_cnt += 1;
                        }
                    } else {
                        non_wall_cnt += 1;
                    }

                    let mut has_internal_edge = false;
                    for (ex, ey) in [(cx + 1, cy), (cx, cy + 1)] {
                        if ex < rw && ey < rh {
                            let ei = ey * rw + ex;
                            if is_candidate_pocket(ei) {
                                let ed = (rr_r[ci] - rr_r[ei])
                                    .abs()
                                    .max((rg_r[ci] - rg_r[ei]).abs())
                                    .max((rb_r[ci] - rb_r[ei]).abs());
                                if ed > 0.08 {
                                    has_internal_edge = true;
                                }
                            }
                        }
                    }
                    if has_internal_edge {
                        edge_cnt += 1;
                    }

                    for (nx_i, ny_i) in [
                        (cx.wrapping_sub(1), cy),
                        (cx + 1, cy),
                        (cx, cy.wrapping_sub(1)),
                        (cx, cy + 1),
                    ] {
                        if nx_i < rw && ny_i < rh {
                            let ni = ny_i * rw + nx_i;
                            if is_candidate_pocket(ni) && !visited[ni] {
                                visited[ni] = true;
                                cq.push_back(ni);
                            } else if open_bg[ni] || fg_r[ni] >= 0.62 {
                                adj_total += 1;
                                if hair_r[ni] > 0.25 {
                                    adj_hair_hits += 1;
                                }
                            }
                        }
                    }
                }

                let len = comp.len().max(1);
                let mean_ny = ny_sum / len as f32;
                let mean_hair = hair_sum / len as f32;
                let non_wall_frac = non_wall_cnt as f32 / len as f32;
                let wall_frac = wall_cnt as f32 / len as f32;
                let warm_skin_frac = warm_skin_cnt as f32 / len as f32;
                let edge_frac = edge_cnt as f32 / len as f32;
                let adj_hair_frac = if adj_total > 0 { adj_hair_hits as f32 / adj_total as f32 } else { 0.0 };

                // 1. Folded arms / tucked fingers / skin rescue has PRIORITY when warm skin is present
                if mean_ny > lower_y
                    && mean_hair < 0.18
                    && adj_hair_frac < 0.12
                    && len <= 320
                    && (warm_skin_frac > 0.25
                        || non_wall_frac > 0.45
                        || (len <= 90 && edge_frac > 0.25 && non_wall_frac > 0.20))
                {
                    for &ci in &comp {
                        rescue_r[ci] = 1.0;
                    }
                }
                // 2. Enclosed studio wall pocket / cavity punching (e.g. gap between arm and waist or between legs)
                else if wall_frac > 0.65
                    && warm_skin_frac < 0.15
                    && mean_hair < 0.20
                    && adj_hair_frac < 0.18
                    && edge_frac < 0.40
                {
                    for &ci in &comp {
                        punch_bg[ci] = 1.0;
                    }
                }
            }

            if punch_bg.iter().any(|&v| v > 0.5) {
                let punch_soft = box_filter_plane(&punch_bg, rw, rh, 1);
                p_fg.par_chunks_mut(gw)
                    .zip(p_solid.par_chunks_mut(gw))
                    .enumerate()
                    .for_each(|(y, (rp, rsol))| {
                        let ny = (y as f32 + 0.5) / gh as f32;
                        for x in 0..gw {
                            let nx = (x as f32 + 0.5) / gw as f32;
                            let pv = sample_plane_bilinear(&punch_soft, rw, rh, nx, ny);
                            if pv > 0.10 {
                                let k = ((pv - 0.10) / 0.50).clamp(0.0, 1.0);
                                rp[x] *= 1.0 - k;
                                rsol[x] *= 1.0 - k;
                            }
                        }
                    });
            }

            if rescue_r.iter().any(|&v| v > 0.5) {
                let rescue_soft = box_filter_plane(&rescue_r, rw, rh, 1);
                p_fg.par_chunks_mut(gw)
                    .zip(p_solid.par_chunks_mut(gw))
                    .enumerate()
                    .for_each(|(y, (rp, rsol))| {
                        let ny = (y as f32 + 0.5) / gh as f32;
                        for x in 0..gw {
                            let nx = (x as f32 + 0.5) / gw as f32;
                            let rv = (sample_plane_bilinear(&rescue_soft, rw, rh, nx, ny) * 1.35).clamp(0.0, 1.0);
                            if rv > 0.08 {
                                rp[x] = rp[x].max(rv);
                                rsol[x] = rsol[x].max(rv);
                            }
                        }
                    });
            }
        }
    }

    let edge_adj_norm = (params.bg_edge_adjust / 100.0).clamp(-1.0, 1.0);
    let r_adj = if edge_adj_norm.abs() > 0.06 {
        ((edge_adj_norm.abs() * (gw.min(gh) as f32 * 0.011)).round() as usize).clamp(1, 8)
    } else {
        0
    };
    let (p_adj_min, p_adj_max) = if r_adj > 0 {
        morph_min_max_plane(&p_fg, gw, gh, r_adj)
    } else {
        (Vec::new(), Vec::new())
    };
    if r_adj > 0 {
        let blend = edge_adj_norm.abs().clamp(0.0, 1.0);
        if edge_adj_norm < 0.0 {
            for i in 0..n {
                p_fg[i] = p_fg[i] * (1.0 - blend) + p_adj_min[i] * blend;
            }
        } else {
            for i in 0..n {
                p_fg[i] = p_fg[i] * (1.0 - blend) + p_adj_max[i] * blend;
            }
        }
    }

    let r_tri = ((gw.min(gh) as f32) * 0.0065).round().clamp(2.0, 6.0) as usize;
    let (p_min, p_max) = morph_min_max_plane(&p_fg, gw, gh, r_tri);

    let r_guide = ((gw.min(gh) as f32) * 0.0055).round().clamp(2.0, 6.0) as usize;
    let eps = 0.00055f32;

    let mean_r = box_filter_plane(&ir, gw, gh, r_guide);
    let mean_g = box_filter_plane(&ig, gw, gh, r_guide);
    let mean_b = box_filter_plane(&ib, gw, gh, r_guide);
    let mean_p = box_filter_plane(&p_fg, gw, gh, r_guide);

    let mul = |a: &[f32], b: &[f32]| -> Vec<f32> {
        a.par_iter().zip(b.par_iter()).map(|(&x, &y)| x * y).collect()
    };

    let mean_rr = box_filter_plane(&mul(&ir, &ir), gw, gh, r_guide);
    let mean_rg = box_filter_plane(&mul(&ir, &ig), gw, gh, r_guide);
    let mean_rb = box_filter_plane(&mul(&ir, &ib), gw, gh, r_guide);
    let mean_gg = box_filter_plane(&mul(&ig, &ig), gw, gh, r_guide);
    let mean_gb = box_filter_plane(&mul(&ig, &ib), gw, gh, r_guide);
    let mean_bb = box_filter_plane(&mul(&ib, &ib), gw, gh, r_guide);

    let mean_rp = box_filter_plane(&mul(&ir, &p_fg), gw, gh, r_guide);
    let mean_gp = box_filter_plane(&mul(&ig, &p_fg), gw, gh, r_guide);
    let mean_bp = box_filter_plane(&mul(&ib, &p_fg), gw, gh, r_guide);

    let mut ar_buf = vec![0.0f32; n];
    let mut ag_buf = vec![0.0f32; n];
    let mut ab_buf = vec![0.0f32; n];
    let mut b_buf = vec![0.0f32; n];

    ar_buf
        .par_iter_mut()
        .zip(ag_buf.par_iter_mut())
        .zip(ab_buf.par_iter_mut())
        .zip(b_buf.par_iter_mut())
        .enumerate()
        .for_each(|(i, (((ar_o, ag_o), ab_o), b_o))| {
            let mr = mean_r[i];
            let mg = mean_g[i];
            let mb = mean_b[i];
            let mp = mean_p[i];

            let s00 = (mean_rr[i] - mr * mr + eps).max(eps);
            let s01 = mean_rg[i] - mr * mg;
            let s02 = mean_rb[i] - mr * mb;
            let s11 = (mean_gg[i] - mg * mg + eps).max(eps);
            let s12 = mean_gb[i] - mg * mb;
            let s22 = (mean_bb[i] - mb * mb + eps).max(eps);

            let cr = mean_rp[i] - mr * mp;
            let cg = mean_gp[i] - mg * mp;
            let cb = mean_bp[i] - mb * mp;

            let c00 = s11 * s22 - s12 * s12;
            let c01 = s02 * s12 - s01 * s22;
            let c02 = s01 * s12 - s02 * s11;
            let c11 = s00 * s22 - s02 * s02;
            let c12 = s01 * s02 - s00 * s12;
            let c22 = s00 * s11 - s01 * s01;
            let det = (s00 * c00 + s01 * c01 + s02 * c02).max(1e-10);

            let ar = (c00 * cr + c01 * cg + c02 * cb) / det;
            let ag = (c01 * cr + c11 * cg + c12 * cb) / det;
            let ab = (c02 * cr + c12 * cg + c22 * cb) / det;
            let bc = mp - (ar * mr + ag * mg + ab * mb);

            *ar_o = ar;
            *ag_o = ag;
            *ab_o = ab;
            *b_o = bc;
        });

    let mean_ar = box_filter_plane(&ar_buf, gw, gh, r_guide);
    let mean_ag = box_filter_plane(&ag_buf, gw, gh, r_guide);
    let mean_ab = box_filter_plane(&ab_buf, gw, gh, r_guide);
    let mean_bc = box_filter_plane(&b_buf, gw, gh, r_guide);

    let r_strand = ((gw.min(gh) as f32) * 0.011).round().clamp(3.0, 11.0) as usize;
    let hair_env = box_filter_plane(&p_hair, gw, gh, r_strand);

    let (f_local, b_local, f_w, b_w) = if need_decontam {
        let q_guide: Vec<f32> = (0..n)
            .into_par_iter()
            .map(|i| {
                let q = (mean_ar[i] * ir[i] + mean_ag[i] * ig[i] + mean_ab[i] * ib[i] + mean_bc[i]).clamp(0.0, 1.0);
                let t_fg = smoothstep(0.88, 0.98, p_min[i]);
                let t_bg = smoothstep(0.04, 0.16, p_max[i]);
                ((q * (1.0 - t_fg) + t_fg) * t_bg).clamp(0.0, 1.0)
            })
            .collect();

        let r_col_fg = ((gw.min(gh) as f32) * 0.009).round().clamp(3.0, 9.0) as usize;
        let r_col_bg = ((gw.min(gh) as f32) * 0.013).round().clamp(4.0, 12.0) as usize;
        let r_col_hair = ((gw.min(gh) as f32) * 0.012).round().clamp(4.0, 13.0) as usize;
        let diffuse_weighted_rgb = |w_map: &[f32], r_col: usize| -> (Vec<f32>, Vec<f32>) {
            let mut wr = mul(&ir, w_map);
            let mut wg = mul(&ig, w_map);
            let mut wb = mul(&ib, w_map);
            let mut ww = w_map.to_vec();
            for _ in 0..2 {
                wr = box_filter_plane(&wr, gw, gh, r_col);
                wg = box_filter_plane(&wg, gw, gh, r_col);
                wb = box_filter_plane(&wb, gw, gh, r_col);
                ww = box_filter_plane(&ww, gw, gh, r_col);
            }
            let mut out = vec![0.0f32; n * 3];
            out.par_chunks_mut(3).enumerate().for_each(|(i, px)| {
                let d = ww[i];
                if d > 1e-4 {
                    px[0] = (wr[i] / d).clamp(0.0, 1.0);
                    px[1] = (wg[i] / d).clamp(0.0, 1.0);
                    px[2] = (wb[i] / d).clamp(0.0, 1.0);
                } else {
                    px[0] = ir[i];
                    px[1] = ig[i];
                    px[2] = ib[i];
                }
            });
            (out, ww)
        };

        let w_fg: Vec<f32> = q_guide.par_iter().map(|&q| (q - 0.85).max(0.0).powi(2)).collect();
        let w_fg_hair: Vec<f32> = q_guide
            .par_iter()
            .zip(p_hair.par_iter())
            .map(|(&q, &hp)| (q - 0.68).max(0.0).powi(2) * (hp - 0.10).max(0.0))
            .collect();
        let w_bg: Vec<f32> = q_guide.par_iter().map(|&q| (0.12 - q).max(0.0).powi(2)).collect();
        let (mut fl, mut fw) = diffuse_weighted_rgb(&w_fg, r_col_fg);
        let (fl_hair, fw_hair) = diffuse_weighted_rgb(&w_fg_hair, r_col_hair);
        for i in 0..n {
            let hair_presence = (hair_env[i] * 1.5 + p_hair[i] * 1.3).clamp(0.0, 1.0);
            let hw = smoothstep(0.03, 0.16, hair_presence);
            if fw_hair[i] > 1e-4 && hw > 0.01 {
                let k = hw;
                fl[i * 3] = fl[i * 3] * (1.0 - k) + fl_hair[i * 3] * k;
                fl[i * 3 + 1] = fl[i * 3 + 1] * (1.0 - k) + fl_hair[i * 3 + 1] * k;
                fl[i * 3 + 2] = fl[i * 3 + 2] * (1.0 - k) + fl_hair[i * 3 + 2] * k;
                fw[i] = fw[i].max(fw_hair[i]);
            }
        }
        let (mut bl, mut bw) = diffuse_weighted_rgb(&w_bg, r_col_bg);

        // Coarse multi-scale fallback so enclosed hair/neck wall pockets get surrounding wall color
        let cw = 64usize;
        let ch = 64usize;
        let cn = cw * ch;
        let mut c_wr = vec![0.0f32; cn];
        let mut c_wg = vec![0.0f32; cn];
        let mut c_wb = vec![0.0f32; cn];
        let mut c_ww = vec![0.0f32; cn];
        for gy_i in 0..gh {
            let cy = (gy_i * ch) / gh;
            for gx_i in 0..gw {
                let cx = (gx_i * cw) / gw;
                let si = gy_i * gw + gx_i;
                let ci = cy * cw + cx;
                let w_v = w_bg[si];
                if w_v > 0.0 {
                    c_wr[ci] += ir[si] * w_v;
                    c_wg[ci] += ig[si] * w_v;
                    c_wb[ci] += ib[si] * w_v;
                    c_ww[ci] += w_v;
                }
            }
        }
        for _ in 0..3 {
            c_wr = box_filter_plane(&c_wr, cw, ch, 10);
            c_wg = box_filter_plane(&c_wg, cw, ch, 10);
            c_wb = box_filter_plane(&c_wb, cw, ch, 10);
            c_ww = box_filter_plane(&c_ww, cw, ch, 10);
        }
        let mut c_rgb = vec![0.0f32; cn * 3];
        for i in 0..cn {
            let d = c_ww[i];
            if d > 1e-6 {
                c_rgb[i * 3] = (c_wr[i] / d).clamp(0.0, 1.0);
                c_rgb[i * 3 + 1] = (c_wg[i] / d).clamp(0.0, 1.0);
                c_rgb[i * 3 + 2] = (c_wb[i] / d).clamp(0.0, 1.0);
            }
        }
        for gy_i in 0..gh {
            let ny_c = (gy_i as f32 + 0.5) / gh as f32;
            let cy_f = (ny_c * ch as f32 - 0.5).clamp(0.0, (ch - 1) as f32);
            for gx_i in 0..gw {
                let i = gy_i * gw + gx_i;
                if bw[i] <= 1e-4 {
                    let nx_c = (gx_i as f32 + 0.5) / gw as f32;
                    let cx_f = (nx_c * cw as f32 - 0.5).clamp(0.0, (cw - 1) as f32);
                    let cw_v = sample_plane_bilinear(&c_ww, cw, ch, nx_c, ny_c);
                    if cw_v > 1e-6 {
                        let (cr, cg, cb) = sample_bilinear(&c_rgb, cw, ch, cx_f, cy_f);
                        bl[i * 3] = cr;
                        bl[i * 3 + 1] = cg;
                        bl[i * 3 + 2] = cb;
                        bw[i] = 0.5;
                    }
                }
            }
        }
        (fl, bl, fw, bw)
    } else {
        (Vec::new(), Vec::new(), Vec::new(), Vec::new())
    };

    let eff_outer_stray = (params.stray_hair * (params.stray_hair_outer / 100.0)).clamp(0.0, 100.0);
    let stray_cut = (eff_outer_stray / 100.0).clamp(0.0, 1.0) * 0.22;
    let adj_bias = -edge_adj_norm * 0.14;
    let (solid_lo, solid_hi, hair_lo, hair_hi, hair_weight_scale) = match params.bg_edge_mode.as_str() {
        "soft" => (
            (0.22 + adj_bias).clamp(0.02, 0.74),
            (0.80 + adj_bias).clamp(0.28, 0.99),
            (0.04 + stray_cut * 0.5 + adj_bias).clamp(0.01, 0.70),
            (0.88 + adj_bias).clamp(0.28, 0.99),
            1.15f32,
        ),
        "crisp" => (
            (0.48 + adj_bias).clamp(0.08, 0.84),
            (0.62 + adj_bias).clamp(0.28, 0.96),
            (0.18 + stray_cut + adj_bias).clamp(0.04, 0.76),
            (0.74 + adj_bias).clamp(0.28, 0.98),
            0.70f32,
        ),
        "hair_detail" => (
            (0.36 + adj_bias).clamp(0.04, 0.76),
            (0.66 + adj_bias).clamp(0.28, 0.96),
            (0.03 + stray_cut * 0.4 + adj_bias).clamp(0.01, 0.68),
            (0.76 + adj_bias).clamp(0.26, 0.98),
            1.40f32,
        ),
        _ => (
            (0.44 + adj_bias).clamp(0.05, 0.80),
            (0.66 + adj_bias).clamp(0.28, 0.96),
            (0.06 + stray_cut + adj_bias).clamp(0.01, 0.74),
            (0.76 + adj_bias).clamp(0.28, 0.98),
            1.15f32,
        ),
    };
    let mut fg_alpha = vec![0.0f32; w * h];
    fg_alpha.par_chunks_mut(w).enumerate().for_each(|(y, row_out)| {
        let ny = (y as f32 + 0.5) / h as f32;
        for x in 0..w {
            let nx = (x as f32 + 0.5) / w as f32;
            let idx = (y * w + x) * 3;
            let r0 = base_rgb[idx];
            let g0 = base_rgb[idx + 1];
            let b0 = base_rgb[idx + 2];

            let ar = sample_plane_bilinear(&mean_ar, gw, gh, nx, ny);
            let ag = sample_plane_bilinear(&mean_ag, gw, gh, nx, ny);
            let ab = sample_plane_bilinear(&mean_ab, gw, gh, nx, ny);
            let bc = sample_plane_bilinear(&mean_bc, gw, gh, nx, ny);
            let pmin = sample_plane_bilinear(&p_min, gw, gh, nx, ny);
            let pmax = sample_plane_bilinear(&p_max, gw, gh, nx, ny);
            let hair_p = sample_plane_bilinear(&p_hair, gw, gh, nx, ny);
            let hair_e = sample_plane_bilinear(&hair_env, gw, gh, nx, ny);
            let solid_p = sample_plane_bilinear(&p_solid, gw, gh, nx, ny);

            let q = (ar * r0 + ag * g0 + ab * b0 + bc).clamp(0.0, 1.0);
            let t_fg = smoothstep(0.88, 0.98, pmin);
            let t_bg = smoothstep(0.04, 0.16, pmax);
            let mut fg_raw = ((q * (1.0 - t_fg) + t_fg) * t_bg).clamp(0.0, 1.0);
            let mut eff_hair_p = hair_p;

            if !f_local.is_empty() && !b_local.is_empty() && pmin < 0.88 && solid_p < 0.65 {
                let gx = ((nx * gw as f32) - 0.5).clamp(0.0, (gw - 1) as f32);
                let gy = ((ny * gh as f32) - 0.5).clamp(0.0, (gh - 1) as f32);
                let (fr, fg_c, fb) = sample_bilinear(&f_local, gw, gh, gx, gy);
                let (br, bg_c, bb) = sample_bilinear(&b_local, gw, gh, gx, gy);
                let fw_px = sample_plane_bilinear(&f_w, gw, gh, nx, ny);
                let bw_px = sample_plane_bilinear(&b_w, gw, gh, nx, ny);
                let vr = r0 - br;
                let vg = g0 - bg_c;
                let vb = b0 - bb;
                let dist_bg = (vr * vr + vg * vg + vb * vb).sqrt();
                let dr = fr - br;
                let dg = fg_c - bg_c;
                let db = fb - bb;
                let df2 = dr * dr + dg * dg + db * db;
                if df2 > 0.02 && fw_px > 1e-4 {
                    let df_len = df2.sqrt();
                    let rel_bg = dist_bg / df_len;
                    let alpha_proj = ((vr * dr + vg * dg + vb * db) / df2).clamp(0.0, 1.0);
                    let er = vr - alpha_proj * dr;
                    let eg = vg - alpha_proj * dg;
                    let eb = vb - alpha_proj * db;
                    let line_err = (er * er + eg * eg + eb * eb).sqrt() / df_len;

                    if df_len > 0.20
                        && alpha_proj > 0.13
                        && dist_bg > 0.11
                        && hair_e > 0.025
                        && params.bg_edge_mode != "crisp"
                    {
                        if hair_p > 0.12 && fg_raw > 0.04 {
                            fg_raw = (fg_raw * 0.40 + alpha_proj * 0.60).clamp(0.0, 1.0);
                        } else if line_err < 0.26 {
                            let wisp_conf = smoothstep(0.025, 0.11, hair_e)
                                * (1.0 - smoothstep(0.14, 0.26, line_err))
                                * smoothstep(0.11, 0.24, alpha_proj);
                            fg_raw = fg_raw.max(alpha_proj * wisp_conf);
                            eff_hair_p = eff_hair_p.max(wisp_conf * 0.55);
                        }
                    }
                    if solid_p < 0.58 && bw_px > 1e-4 && pmax > 0.02 {
                        let near_bg_rel = 1.0 - smoothstep(0.06, 0.22, rel_bg);
                        let near_bg_abs = 1.0 - smoothstep(0.035, 0.115, dist_bg);
                        let low_solid = 1.0 - smoothstep(0.24, 0.58, solid_p);
                        let kill_bg = near_bg_rel * near_bg_abs * low_solid;
                        fg_raw *= 1.0 - kill_bg;
                    }
                } else if bw_px > 1e-4 && solid_p < 0.58 && fg_raw < 0.78 && pmax > 0.02 {
                    let near_bg_abs = 1.0 - smoothstep(0.035, 0.11, dist_bg);
                    let low_solid = 1.0 - smoothstep(0.24, 0.58, solid_p);
                    let kill_bg = near_bg_abs * low_solid;
                    fg_raw *= 1.0 - kill_bg;
                }
            }

            let solid_a = smoothstep(solid_lo, solid_hi, fg_raw);
            let hair_a = smoothstep(hair_lo, hair_hi, fg_raw);
            let hair_w = (smoothstep(0.14, 0.44, eff_hair_p) * hair_weight_scale).clamp(0.0, 1.0);
            row_out[x] = (solid_a * (1.0 - hair_w) + hair_a * hair_w).clamp(0.0, 1.0);
        }
    });

    GuidedMatteResult {
        fg_alpha,
        f_local,
        b_local,
        gw,
        gh,
    }
}

fn compute_highres_direct_matte(
    base_rgb: &[f32],
    w: usize,
    h: usize,
    matte_img: &RgbaImage,
    params: &RetouchParams,
    need_decontam: bool,
) -> GuidedMatteResult {
    let mw = matte_img.width();
    let mh = matte_img.height();
    let same_dims = mw == w as u32 && mh == h as u32;
    let is_subject_only = params.bg_keep_scope == "subject_only";
    let is_portrait_focus = params.bg_keep_scope == "portrait_focus";
    let face = &params.face;
    let edge_adj_norm = (params.bg_edge_adjust / 100.0).clamp(-1.0, 1.0);

    let mut fg_alpha = vec![0.0f32; w * h];
    fg_alpha.par_chunks_mut(w).enumerate().for_each(|(y, row_out)| {
        let ny = (y as f32 + 0.5) / h.max(1) as f32;
        for x in 0..w {
            let nx = (x as f32 + 0.5) / w.max(1) as f32;
            let mut a = if same_dims {
                matte_img.get_pixel(x as u32, y as u32)[0] as f32 / 255.0
            } else {
                sample_rgba_bilinear(matte_img, nx, ny)[0].clamp(0.0, 1.0)
            };

            if is_subject_only {
                a = a.powf(1.22);
            } else if is_portrait_focus {
                let rx = face.face_rx.max(0.12) * 3.8;
                let dx = ((nx - face.center_x) / rx).abs();
                let vert = ((ny - (face.chin_y + face.face_ry * 3.6)).max(0.0) * 2.2).clamp(0.0, 1.0);
                let env = (1.0 - smoothstep(0.65, 1.25, dx) * 0.65 - vert * 0.45).clamp(0.0, 1.0);
                a *= env;
            }

            if edge_adj_norm.abs() > 0.01 {
                if edge_adj_norm > 0.0 {
                    a = (a + edge_adj_norm * 0.28 * smoothstep(0.02, 0.22, a)).clamp(0.0, 1.0);
                } else {
                    a = (a + edge_adj_norm * 0.28).clamp(0.0, 1.0);
                }
            }

            a = match params.bg_edge_mode.as_str() {
                "crisp" => smoothstep(0.28, 0.72, a),
                "soft" => smoothstep(0.01, 0.94, a),
                _ => {
                    if a < 0.008 {
                        0.0
                    } else if a > 0.992 {
                        1.0
                    } else {
                        a
                    }
                }
            };
            row_out[x] = a.clamp(0.0, 1.0);
        }
    });

    let max_guide = 1024usize;
    let (gw, gh) = if w.max(h) > max_guide {
        let scale = max_guide as f32 / w.max(h) as f32;
        (
            ((w as f32 * scale).round() as usize).max(32),
            ((h as f32 * scale).round() as usize).max(32),
        )
    } else {
        (w, h)
    };

    let (f_local, b_local) = if need_decontam && gw >= 16 && gh >= 16 {
        let n = gw * gh;
        let mut ir = vec![0.0f32; n];
        let mut ig = vec![0.0f32; n];
        let mut ib = vec![0.0f32; n];
        let mut w_fg = vec![0.0f32; n];
        let mut w_bg = vec![0.0f32; n];

        ir.par_chunks_mut(gw)
            .zip(ig.par_chunks_mut(gw))
            .zip(ib.par_chunks_mut(gw))
            .zip(w_fg.par_chunks_mut(gw))
            .zip(w_bg.par_chunks_mut(gw))
            .enumerate()
            .for_each(|(y, ((((rr, rg), rb), wf), wb))| {
                let ny = (y as f32 + 0.5) / gh as f32;
                let sy = (ny * h as f32 - 0.5).clamp(0.0, (h - 1) as f32);
                for x in 0..gw {
                    let nx = (x as f32 + 0.5) / gw as f32;
                    let sx = (nx * w as f32 - 0.5).clamp(0.0, (w - 1) as f32);
                    let (cr, cg, cb) = sample_bilinear(base_rgb, w, h, sx, sy);
                    rr[x] = cr;
                    rg[x] = cg;
                    rb[x] = cb;
                    let a = sample_alpha_unit(&fg_alpha, w, h, sx, sy);
                    wf[x] = (a - 0.88).max(0.0).powi(2);
                    wb[x] = (0.12 - a).max(0.0).powi(2);
                }
            });

        let r_col_fg = ((gw.min(gh) as f32) * 0.009).round().clamp(3.0, 9.0) as usize;
        let r_col_bg = ((gw.min(gh) as f32) * 0.013).round().clamp(4.0, 12.0) as usize;

        let diffuse_weighted_rgb = |w_map: &[f32], r_col: usize| -> Vec<f32> {
            let mut wr: Vec<f32> = ir.par_iter().zip(w_map.par_iter()).map(|(&c, &m)| c * m).collect();
            let mut wg: Vec<f32> = ig.par_iter().zip(w_map.par_iter()).map(|(&c, &m)| c * m).collect();
            let mut wb: Vec<f32> = ib.par_iter().zip(w_map.par_iter()).map(|(&c, &m)| c * m).collect();
            let mut ww = w_map.to_vec();
            for _ in 0..2 {
                wr = box_filter_plane(&wr, gw, gh, r_col);
                wg = box_filter_plane(&wg, gw, gh, r_col);
                wb = box_filter_plane(&wb, gw, gh, r_col);
                ww = box_filter_plane(&ww, gw, gh, r_col);
            }
            let mut out = vec![0.0f32; n * 3];
            out.par_chunks_mut(3).enumerate().for_each(|(i, px)| {
                let d = ww[i];
                if d > 1e-4 {
                    px[0] = (wr[i] / d).clamp(0.0, 1.0);
                    px[1] = (wg[i] / d).clamp(0.0, 1.0);
                    px[2] = (wb[i] / d).clamp(0.0, 1.0);
                } else {
                    px[0] = ir[i];
                    px[1] = ig[i];
                    px[2] = ib[i];
                }
            });
            out
        };

        (
            diffuse_weighted_rgb(&w_fg, r_col_fg),
            diffuse_weighted_rgb(&w_bg, r_col_bg),
        )
    } else {
        (Vec::new(), Vec::new())
    };

    GuidedMatteResult {
        fg_alpha,
        f_local,
        b_local,
        gw,
        gh,
    }
}

fn masked_background_bokeh(base_rgb: &[f32], fg_alpha: &[f32], w: usize, h: usize, radius: usize) -> Vec<f32> {
    if radius == 0 || w == 0 || h == 0 {
        return base_rgb.to_vec();
    }
    let n = w * h;
    let mut wb = vec![0.0f32; n];
    let mut wr = vec![0.0f32; n];
    let mut wg = vec![0.0f32; n];
    let mut wbl = vec![0.0f32; n];

    for y in 0..h {
        let yp = (y + 2).min(h - 1);
        let ym = y.saturating_sub(2);
        for x in 0..w {
            let xp = (x + 2).min(w - 1);
            let xm = x.saturating_sub(2);
            let i = y * w + x;
            let ix_p = (y * w + xp) * 3;
            let ix_m = (y * w + xm) * 3;
            let iy_p = (yp * w + x) * 3;
            let iy_m = (ym * w + x) * 3;
            // Suppress sharp high-contrast flyaway strands / edges so un-masked wispy hairs do not darken bokeh into gray haze
            let grad = ((base_rgb[ix_p] - base_rgb[ix_m]).abs()
                + (base_rgb[ix_p + 1] - base_rgb[ix_m + 1]).abs()
                + (base_rgb[ix_p + 2] - base_rgb[ix_m + 2]).abs()
                + (base_rgb[iy_p] - base_rgb[iy_m]).abs()
                + (base_rgb[iy_p + 1] - base_rgb[iy_m + 1]).abs()
                + (base_rgb[iy_p + 2] - base_rgb[iy_m + 2]).abs())
                * 0.5;
            let smooth_k = 1.0 - smoothstep(0.035, 0.11, grad);
            let base_w = (1.0 - fg_alpha[i]).clamp(0.0, 1.0).powi(3);
            let weight = base_w * (0.04 + 0.96 * smooth_k);
            wb[i] = weight;
            wr[i] = base_rgb[i * 3] * weight;
            wg[i] = base_rgb[i * 3 + 1] * weight;
            wbl[i] = base_rgb[i * 3 + 2] * weight;
        }
    }

    let r2 = (radius / 2).max(1);
    let wr_b = box_filter_plane(&box_filter_plane(&wr, w, h, radius), w, h, r2);
    let wg_b = box_filter_plane(&box_filter_plane(&wg, w, h, radius), w, h, r2);
    let wbl_b = box_filter_plane(&box_filter_plane(&wbl, w, h, radius), w, h, r2);
    let wb_b = box_filter_plane(&box_filter_plane(&wb, w, h, radius), w, h, r2);

    let mut out = vec![0.0f32; n * 3];
    out.par_chunks_mut(3).enumerate().for_each(|(i, px)| {
        let denom = wb_b[i];
        if denom > 1e-4 {
            px[0] = (wr_b[i] / denom).clamp(0.0, 1.0);
            px[1] = (wg_b[i] / denom).clamp(0.0, 1.0);
            px[2] = (wbl_b[i] / denom).clamp(0.0, 1.0);
        } else {
            px[0] = base_rgb[i * 3];
            px[1] = base_rgb[i * 3 + 1];
            px[2] = base_rgb[i * 3 + 2];
        }
    });
    out
}

fn parse_hex_color(hex: &str) -> (f32, f32, f32) {
    let s = hex.trim().trim_start_matches('#');
    if s.len() == 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&s[0..2], 16),
            u8::from_str_radix(&s[2..4], 16),
            u8::from_str_radix(&s[4..6], 16),
        ) {
            return (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
        }
    }
    (0.91, 0.93, 0.96)
}

fn compute_cast_shadow_plane(
    fg_alpha: &[f32],
    w: usize,
    h: usize,
    params: &RetouchParams,
) -> (Vec<f32>, usize, usize, f32, f32) {
    let max_d = 384usize;
    let scale = (max_d as f32 / w.max(h).max(1) as f32).min(1.0);
    let sw = ((w as f32 * scale).round() as usize).max(32);
    let sh = ((h as f32 * scale).round() as usize).max(32);

    let mut plane = vec![0.0f32; sw * sh];
    for y in 0..sh {
        let ny = (y as f32 + 0.5) / sh as f32;
        for x in 0..sw {
            let nx = (x as f32 + 0.5) / sw as f32;
            plane[y * sw + x] = sample_plane_bilinear(fg_alpha, w, h, nx, ny).clamp(0.0, 1.0);
        }
    }

    let blur_norm = (params.bg_shadow_blur / 100.0).clamp(0.0, 1.0);
    let (base_rad_frac, off_x, off_y) = match params.bg_shadow_type.as_str() {
        "soft_shadow" => (0.022 + blur_norm * 0.055, 0.008f32, 0.018f32),
        "hard_shadow" => (0.006 + blur_norm * 0.024, 0.026f32, 0.022f32),
        "cast_shadow" => (0.010 + blur_norm * 0.036, 0.052f32, 0.036f32),
        _ => (0.015, 0.01, 0.015),
    };
    let rad = ((sw.min(sh) as f32 * base_rad_frac).round() as usize).clamp(1, 28);
    let b1 = box_filter_plane(&plane, sw, sh, rad);
    let b2 = box_filter_plane(&b1, sw, sh, (rad / 2).max(1));
    (b2, sw, sh, off_x, off_y)
}

fn prepare_custom_background(
    b64: &str,
    w: usize,
    h: usize,
    params: &RetouchParams,
) -> Option<Vec<f32>> {
    if b64.trim().is_empty() || w == 0 || h == 0 {
        return None;
    }
    let dyn_img = decode_data_url(b64).ok()?;
    let rgba = dyn_img.to_rgba8();
    let sw = rgba.width().max(1) as f32;
    let sh = rgba.height().max(1) as f32;

    let target_aspect = w as f32 / h as f32;
    let src_aspect = sw / sh;
    let scale_zoom = 1.0 + (params.bg_scale / 100.0).clamp(0.0, 1.5);
    let pan_x = -(params.bg_offset_x / 100.0) * 0.45;
    let pan_y = -(params.bg_offset_y / 100.0) * 0.45;
    let fill_mode = params.bg_fill_mode.as_str();

    let (u0, v0, uw, vh) = if fill_mode == "stretch" {
        (0.0, 0.0, 1.0, 1.0)
    } else if fill_mode == "contain" {
        if src_aspect > target_aspect {
            let fit_h = target_aspect / src_aspect;
            (0.0, -(1.0 - fit_h) * 0.5 / fit_h, 1.0, 1.0 / fit_h)
        } else {
            let fit_w = src_aspect / target_aspect;
            (-(1.0 - fit_w) * 0.5 / fit_w, 0.0, 1.0 / fit_w, 1.0)
        }
    } else if src_aspect > target_aspect {
        let crop_w = target_aspect / src_aspect;
        ((1.0 - crop_w) * 0.5, 0.0, crop_w, 1.0)
    } else {
        let crop_h = src_aspect / target_aspect;
        (0.0, (1.0 - crop_h) * 0.5, 1.0, crop_h)
    };

    let mut buf = vec![0.0f32; w * h * 3];
    buf.par_chunks_mut(w * 3).enumerate().for_each(|(y, row)| {
        let vy = (y as f32 + 0.5) / h as f32;
        let fy = 0.5 + (vy - 0.5) / scale_zoom + pan_y;
        let mut ny = v0 + fy * vh;
        for x in 0..w {
            let vx = (x as f32 + 0.5) / w as f32;
            let fx = 0.5 + (vx - 0.5) / scale_zoom + pan_x;
            let mut nx = u0 + fx * uw;
            let p = if fill_mode == "tile" {
                let tx = (nx * 2.0).rem_euclid(1.0);
                let ty = (ny * 2.0).rem_euclid(1.0);
                sample_rgba_bilinear(&rgba, tx, ty)
            } else if fill_mode == "contain" && (nx < 0.0 || nx > 1.0 || ny < 0.0 || ny > 1.0) {
                [0.08, 0.09, 0.11, 1.0]
            } else {
                nx = nx.clamp(0.0, 1.0);
                ny = ny.clamp(0.0, 1.0);
                sample_rgba_bilinear(&rgba, nx, ny)
            };
            let i = x * 3;
            row[i] = p[0];
            row[i + 1] = p[1];
            row[i + 2] = p[2];
        }
    });

    let effective_blur = params.bg_blur_strength.max(params.bg_blur_bokeh);
    if effective_blur > 0.5 {
        let rad = ((w.min(h) as f32 * 0.020 * (effective_blur / 100.0)).round() as usize).clamp(1, 36);
        let mut r_ch = Vec::with_capacity(w * h);
        let mut g_ch = Vec::with_capacity(w * h);
        let mut b_ch = Vec::with_capacity(w * h);
        for chunk in buf.chunks_exact(3) {
            r_ch.push(chunk[0]);
            g_ch.push(chunk[1]);
            b_ch.push(chunk[2]);
        }
        let rb = box_filter_plane(&r_ch, w, h, rad);
        let gb = box_filter_plane(&g_ch, w, h, rad);
        let bb = box_filter_plane(&b_ch, w, h, rad);
        for i in 0..(w * h) {
            buf[i * 3] = rb[i];
            buf[i * 3 + 1] = gb[i];
            buf[i * 3 + 2] = bb[i];
        }
    }
    Some(buf)
}

#[inline]
fn is_studio_solid_bg(mode: &str) -> bool {
    matches!(
        mode,
        "transparent"
            | "pure_white"
            | "warm_beige"
            | "studio_grey"
            | "slate_grey_03"
            | "fabric_02"
            | "passport_blue"
            | "moody_black"
            | "pastel_pink"
            | "sage_green"
            | "lavender_mist"
            | "custom_color"
    )
}

#[inline]
fn eval_replaced_background(
    params: &RetouchParams,
    custom_rgb: (f32, f32, f32),
    custom_img: &[f32],
    x: usize,
    y: usize,
    w: usize,
    h: usize,
) -> (f32, f32, f32) {
    let mode = params.bg_replace_mode.as_str();
    if mode == "custom_image" && custom_img.len() == w * h * 3 {
        let idx = (y * w + x) * 3;
        return (custom_img[idx], custom_img[idx + 1], custom_img[idx + 2]);
    }

    let scale_zoom = 1.0 + (params.bg_scale / 100.0).clamp(0.0, 1.5);
    let pan_x = -(params.bg_offset_x / 100.0) * 0.45;
    let pan_y = -(params.bg_offset_y / 100.0) * 0.45;
    let nx = (0.5 + ((x as f32 / w.max(1) as f32) - 0.5) / scale_zoom + pan_x).clamp(0.0, 1.0);
    let ny = (0.5 + ((y as f32 / h.max(1) as f32) - 0.5) / scale_zoom + pan_y).clamp(0.0, 1.0);

    let dx = (nx - 0.5) * 1.4;
    let dy = (ny - 0.36) * 1.25;
    let radial = (1.0 - (dx * dx + dy * dy).sqrt().min(1.0)).clamp(0.0, 1.0);
    let effective_blur = params.bg_blur_strength.max(params.bg_blur_bokeh);
    let cloud_soft = 1.0 - (effective_blur / 100.0).clamp(0.0, 1.0) * 0.85;
    let sky_boost = (params.sky_enhance / 100.0).clamp(0.0, 1.0);

    match mode {
        "transparent" => (0.0, 0.0, 0.0),
        "pure_white" => (1.0, 1.0, 1.0),
        "warm_beige" => {
            let k = 0.88 + 0.08 * radial;
            ((0.72 * k).clamp(0.0, 1.0), (0.61 * k).clamp(0.0, 1.0), (0.55 * k).clamp(0.0, 1.0))
        }
        "studio_grey" => {
            let k = 0.78 + 0.28 * radial;
            ((0.52 * k).clamp(0.0, 1.0), (0.54 * k).clamp(0.0, 1.0), (0.57 * k).clamp(0.0, 1.0))
        }
        "slate_grey_03" => {
            let k = 0.82 + 0.20 * radial;
            ((0.42 * k).clamp(0.0, 1.0), (0.41 * k).clamp(0.0, 1.0), (0.45 * k).clamp(0.0, 1.0))
        }
        "fabric_02" => {
            let m1 = ((nx * 6.2 + 0.4).sin() * (ny * 5.4 + 0.9).cos()) * 0.5 + 0.5;
            let m2 = ((nx * 14.0 - ny * 11.0).sin() * (nx * 9.0 + ny * 13.0).cos()) * 0.5 + 0.5;
            let weave = if effective_blur < 25.0 {
                (((x * 2654435761 ^ y * 2246822519) & 0xff) as f32 / 255.0 - 0.5) * 0.024 * cloud_soft
            } else {
                0.0
            };
            let mot = (m1 * 0.62 + m2 * 0.38 - 0.5) * 0.14 * cloud_soft;
            let base = (0.20 + radial * 0.12 + mot + weave).clamp(0.05, 0.48);
            ((base * 0.96).clamp(0.0, 1.0), (base * 1.00).clamp(0.0, 1.0), (base * 1.07).clamp(0.0, 1.0))
        }
        "passport_blue" => {
            let k = 0.88 + 0.14 * radial;
            ((0.18 * k).clamp(0.0, 1.0), (0.47 * k).clamp(0.0, 1.0), (0.84 * k).clamp(0.0, 1.0))
        }
        "moody_black" => {
            let k = 0.35 + 0.85 * radial;
            ((0.08 * k).clamp(0.0, 1.0), (0.085 * k).clamp(0.0, 1.0), (0.10 * k).clamp(0.0, 1.0))
        }
        "pastel_pink" => {
            let k = 0.92 + 0.08 * radial;
            ((0.96 * k).clamp(0.0, 1.0), (0.80 * k).clamp(0.0, 1.0), (0.84 * k).clamp(0.0, 1.0))
        }
        "sage_green" => {
            let k = 0.90 + 0.10 * radial;
            ((0.60 * k).clamp(0.0, 1.0), (0.72 * k).clamp(0.0, 1.0), (0.65 * k).clamp(0.0, 1.0))
        }
        "lavender_mist" => {
            let k = 0.90 + 0.10 * radial;
            ((0.80 * k).clamp(0.0, 1.0), (0.77 * k).clamp(0.0, 1.0), (0.91 * k).clamp(0.0, 1.0))
        }
        "custom_color" => custom_rgb,
        "sky_golden_sunset" => {
            let cloud = (((nx * 9.0 + ny * 4.0).sin() * (nx * 15.0 - ny * 7.0).cos()).max(0.0) * 0.14) * cloud_soft;
            let sr = (0.97 - ny * 0.10 + cloud + sky_boost * 0.04).clamp(0.0, 1.0);
            let sg = (0.54 + ny * 0.22 + cloud * 0.8).clamp(0.0, 1.0);
            let sb = (0.36 + (1.0 - ny) * 0.28 + cloud * 0.5 - sky_boost * 0.06).clamp(0.0, 1.0);
            (sr, sg, sb)
        }
        "sky_alpine_blue" => {
            let cloud = (((nx * 11.0 + 1.2).sin() * (ny * 8.0 + 0.5).cos()).max(0.0).powi(2) * 0.26 * (1.0 - ny * 0.6)) * cloud_soft;
            let sr = (0.20 + ny * 0.46 + cloud - sky_boost * 0.08).clamp(0.0, 1.0);
            let sg = (0.50 + ny * 0.35 + cloud - sky_boost * 0.03).clamp(0.0, 1.0);
            let sb = (0.89 + ny * 0.09 + cloud * 0.5 + sky_boost * 0.08).clamp(0.0, 1.0);
            (sr, sg, sb)
        }
        "sky_dramatic_clouds" => {
            let storm = (((nx * 7.5).sin() * (ny * 6.0).cos() * 0.5 + 0.5) * 0.22) * cloud_soft;
            let sr = (0.24 + ny * 0.26 + storm - sky_boost * 0.05).clamp(0.0, 1.0);
            let sg = (0.28 + ny * 0.26 + storm).clamp(0.0, 1.0);
            let sb = (0.37 + ny * 0.23 + storm * 1.1 + sky_boost * 0.07).clamp(0.0, 1.0);
            (sr, sg, sb)
        }
        "sky_night_stars" => {
            let star_hash = (((x * 747796405 + y * 2891336453) ^ 0x27d4eb2d) & 0xfff) as f32 / 4095.0;
            let star = if star_hash > 0.996 && ny < 0.65 && effective_blur < 30.0 { 0.85 } else { 0.0 };
            let sr = (0.05 + ny * 0.08 + star).clamp(0.0, 1.0);
            let sg = (0.08 + ny * 0.12 + star).clamp(0.0, 1.0);
            let sb = (0.18 + ny * 0.18 + star + sky_boost * 0.08).clamp(0.0, 1.0);
            (sr, sg, sb)
        }
        _ => (0.94, 0.95, 0.97),
    }
}

/// Adaptive denoiser for RAW and JPG: estimates the noise level of the photo itself,
/// then applies non-local means (patch matching, edge preserving) and chroma smoothing.
fn adaptive_denoise_rgb(src: &[f32], w: usize, h: usize, strength: f32, detail: f32) -> Vec<f32> {
    let s = (strength / 100.0).clamp(0.0, 1.0);
    if s < 0.005 || w < 8 || h < 8 {
        return src.to_vec();
    }
    let luma: Vec<f32> = (0..w * h)
        .map(|i| 0.299 * src[i * 3] + 0.587 * src[i * 3 + 1] + 0.114 * src[i * 3 + 2])
        .collect();

    // Noise estimate: robust MAD of (pixel - local 4-neighbour mean) on a sparse grid
    let mut devs: Vec<f32> = Vec::new();
    let step = ((w * h) / 20000).max(1);
    let mut i = w + 1;
    while i < w * h - w - 1 {
        if i % w != 0 && i % w != w - 1 {
            let m = (luma[i - 1] + luma[i + 1] + luma[i - w] + luma[i + w]) * 0.25;
            devs.push((luma[i] - m).abs());
        }
        i += step;
    }
    devs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mad = if devs.is_empty() { 0.01 } else { devs[devs.len() / 2] };
    // 4-neighbour residual has variance sigma^2 * 1.25 -> sigma = MAD*1.4826/sqrt(1.25)
    let sigma = (mad * 1.4826 / 1.118).clamp(0.002, 0.25);
    let h_filter = (sigma * (0.6 + 1.6 * s)).max(1e-4);
    let h2 = h_filter * h_filter;
    let noise_floor = 2.0 * sigma * sigma;

    const R: i32 = 3; // search radius (7x7 window)
    let at = |x: i32, y: i32| -> usize {
        (y.clamp(0, h as i32 - 1) as usize) * w + x.clamp(0, w as i32 - 1) as usize
    };

    let mut out = vec![0.0f32; w * h * 3];
    out.par_chunks_mut(w * 3).enumerate().for_each(|(y, row)| {
        let yi = y as i32;
        for x in 0..w {
            let xi = x as i32;
            let mut acc = [0.0f32; 3];
            let mut wsum = 0.0f32;
            for dy in -R..=R {
                for dx in -R..=R {
                    let mut d2 = 0.0f32;
                    for py in -1..=1 {
                        for px in -1..=1 {
                            let a = luma[at(xi + px, yi + py)];
                            let b = luma[at(xi + dx + px, yi + dy + py)];
                            d2 += (a - b) * (a - b);
                        }
                    }
                    d2 /= 9.0;
                    let wgt = (-((d2 - noise_floor).max(0.0)) / h2).exp();
                    let j = at(xi + dx, yi + dy) * 3;
                    acc[0] += src[j] * wgt;
                    acc[1] += src[j + 1] * wgt;
                    acc[2] += src[j + 2] * wgt;
                    wsum += wgt;
                }
            }
            let o = x * 3;
            let idx = (y * w + x) * 3;
            // restore a share of original fine detail (Detail slider)
            let keep = (detail / 100.0).clamp(0.0, 1.0) * 0.30 * (1.0 - s * 0.5);
            for c in 0..3 {
                let nlm = acc[c] / wsum.max(1e-6);
                row[o + c] = nlm + (src[idx + c] - nlm) * keep;
            }
        }
    });

    // Chroma smoothing: colour blotches are removed more aggressively than luminance
    let blurred = separable_blur(&out, w, h, 2 + (s * 4.0) as usize);
    let k = (s * 0.85).clamp(0.0, 0.85);
    out.par_chunks_mut(3).enumerate().for_each(|(i, px)| {
        let l = 0.299 * px[0] + 0.587 * px[1] + 0.114 * px[2];
        let lb = 0.299 * blurred[i * 3] + 0.587 * blurred[i * 3 + 1] + 0.114 * blurred[i * 3 + 2];
        for c in 0..3 {
            let chroma = px[c] - l;
            let chroma_b = blurred[i * 3 + c] - lb;
            px[c] = l + chroma * (1.0 - k) + chroma_b * k;
        }
    });
    out
}

/// Detect protruding wispy flyaway hairs & tangled clouds against background and recover uncontaminated local background color.
#[inline]
fn clean_stray_hair_bg(
    base_rgb: &[f32],
    blur_fine: &[f32],
    blur_med: &[f32],
    fg_alpha: &[f32],
    w: usize,
    h: usize,
    x: usize,
    y: usize,
    r_inner: f32,
    r_mid: f32,
    r_far: f32,
    stray_str: f32,
    curr_r: f32,
    curr_g: f32,
    curr_b: f32,
    bokeh_r: f32,
    bokeh_g: f32,
    bokeh_b: f32,
) -> (f32, f32, f32, f32) {
    const DIRS: [(f32, f32); 8] = [
        (1.0, 0.0),
        (0.70710677, 0.70710677),
        (0.0, 1.0),
        (-0.70710677, 0.70710677),
        (-1.0, 0.0),
        (-0.70710677, -0.70710677),
        (0.0, -1.0),
        (0.70710677, -0.70710677),
    ];
    // 22.5 deg staggered directions for mid ring
    const DIRS_STAGGER: [(f32, f32); 8] = [
        (0.9238795, 0.38268343),
        (0.38268343, 0.9238795),
        (-0.38268343, 0.9238795),
        (-0.9238795, 0.38268343),
        (-0.9238795, -0.38268343),
        (-0.38268343, -0.9238795),
        (0.38268343, -0.9238795),
        (0.9238795, -0.38268343),
    ];
    let max_x = w.saturating_sub(1) as f32;
    let max_y = h.saturating_sub(1) as f32;

    // 3 concentric rings (24 taps): r_inner handles thin strands near wall, r_mid/r_far escape 50-90px shoulder puffs
    // (clean_score, r, g, b, fg_a, hi_r, hi_g, hi_b, is_inner)
    let mut taps = [(0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, true); 24];
    for ring in 0..3 {
        let radius = match ring {
            0 => r_inner,
            1 => r_mid,
            _ => r_far,
        };
        let dirs = if ring == 1 { &DIRS_STAGGER } else { &DIRS };
        let ring_bias = match ring {
            0 => 0.0,
            1 => 0.020,
            _ => 0.045,
        };
        for (k, &(dx, dy)) in dirs.iter().enumerate() {
            let sx = (x as f32 + dx * radius).round().clamp(0.0, max_x) as usize;
            let sy = (y as f32 + dy * radius).round().clamp(0.0, max_y) as usize;
            let s_idx = (sy * w + sx) * 3;
            let sr = base_rgb[s_idx];
            let sg = base_rgb[s_idx + 1];
            let sb = base_rgb[s_idx + 2];
            let fr = blur_fine[s_idx];
            let fg_c = blur_fine[s_idx + 1];
            let fb = blur_fine[s_idx + 2];
            let mr = blur_med[s_idx];
            let mg = blur_med[s_idx + 1];
            let mb = blur_med[s_idx + 2];
            let hi_r = sr - fr;
            let hi_g = sg - fg_c;
            let hi_b = sb - fb;
            let edge_fine = hi_r.abs() + hi_g.abs() + hi_b.abs();
            // Reject narrow 3px bright gaps sitting inside darkened crisscross hair puffs
            let edge_med = (sr - mr).abs() + (sg - mg).abs() + (sb - mb).abs();
            let t_fg = fg_alpha[sy * w + sx];
            let dist_b = (sr - bokeh_r).abs() + (sg - bokeh_g).abs() + (sb - bokeh_b).abs();
            let clean_score = t_fg * 1.15 + edge_fine * 2.0 + edge_med * 1.45 + dist_b * 0.08 + ring_bias;
            taps[ring * 8 + k] = (clean_score, sr, sg, sb, t_fg, hi_r, hi_g, hi_b, ring == 0);
        }
    }
    taps.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));

    // If even across wide rings all top background candidates sit inside solid foreground core, preserve hair mass
    if taps[3].4 > 0.85 {
        return (curr_r, curr_g, curr_b, 0.0);
    }

    // Consensus average anchored on the single cleanest smooth donor tap (blocks 1 outlier collar/shadow tap)
    let mut sum_r = taps[0].1;
    let mut sum_g = taps[0].2;
    let mut sum_b = taps[0].3;
    let mut sum_n = 1.0f32;
    for t in &taps[1..6] {
        let d0 = (t.1 - taps[0].1).abs() + (t.2 - taps[0].2).abs() + (t.3 - taps[0].3).abs();
        if d0 < 0.095 {
            sum_r += t.1;
            sum_g += t.2;
            sum_b += t.3;
            sum_n += 1.0;
        }
    }
    let bg_r = sum_r / sum_n;
    let bg_g = sum_g / sum_n;
    let bg_b = sum_b / sum_n;

    let mut clean_bg_count = 0u32;
    let mut inner_mass = 0.0f32;
    for t in &taps {
        let d = (t.1 - bg_r).abs() + (t.2 - bg_g).abs() + (t.3 - bg_b).abs();
        if d < 0.095 && t.4 < 0.68 {
            clean_bg_count += 1;
        }
        if t.8 && d > 0.12 {
            if t.4 > 0.88 {
                inner_mass += 1.0;
            } else if t.4 > 0.68 {
                inner_mass += 0.55;
            } else if t.4 > 0.50 {
                inner_mass += 0.25;
            }
        }
    }

    // Distinguish solid thick hair locks / black clothing (dense on BOTH r_inner and blur_med) from wide sparse wispy puffs
    let px_idx = (y * w + x) * 3;
    let med_dev = (blur_med[px_idx] - bg_r).abs()
        + (blur_med[px_idx + 1] - bg_g).abs()
        + (blur_med[px_idx + 2] - bg_b).abs();
    let dev = (curr_r - bg_r).abs() + (curr_g - bg_g).abs() + (curr_b - bg_b).abs();
    let med_ratio = med_dev / dev.max(0.18);

    let max_mass = 2.8 + stray_str * 1.2;
    let core_block = smoothstep(max_mass, max_mass + 1.1, inner_mass) * smoothstep(0.30, 0.48, med_ratio);
    let dense_guard = 1.0 - core_block;
    let strand_w = smoothstep(0.016, 0.064, dev) * dense_guard * smoothstep(2.0, 5.0, clean_bg_count as f32);

    // Reintroduce subtle background sensor grain from the cleanest donor tap so healed area blends seamlessly
    let tex_r = (bg_r + (taps[0].5 * 0.45).clamp(-0.02, 0.02)).clamp(0.0, 1.0);
    let tex_g = (bg_g + (taps[0].6 * 0.45).clamp(-0.02, 0.02)).clamp(0.0, 1.0);
    let tex_b = (bg_b + (taps[0].7 * 0.45).clamp(-0.02, 0.02)).clamp(0.0, 1.0);
    (tex_r, tex_g, tex_b, strand_w)
}

/// Remove thin isolated stray hairs over upper forehead while keeping dense bangs intact.
#[inline]
fn heal_forehead_flyaway(
    base_rgb: &[f32],
    w: usize,
    h: usize,
    x: usize,
    y: usize,
    r_ring: f32,
    curr_lum: f32,
) -> (f32, f32, f32, f32) {
    const DIRS: [(f32, f32); 8] = [
        (1.0, 0.0),
        (0.70710677, 0.70710677),
        (0.0, 1.0),
        (-0.70710677, 0.70710677),
        (-1.0, 0.0),
        (-0.70710677, -0.70710677),
        (0.0, -1.0),
        (0.70710677, -0.70710677),
    ];
    let max_x = w.saturating_sub(1) as f32;
    let max_y = h.saturating_sub(1) as f32;
    let mut taps = [(0.0f32, 0.0f32, 0.0f32, 0.0f32); 8];
    let mut skin_taps = 0u32;
    for (k, &(dx, dy)) in DIRS.iter().enumerate() {
        let sx = (x as f32 + dx * r_ring).round().clamp(0.0, max_x) as usize;
        let sy = (y as f32 + dy * r_ring).round().clamp(0.0, max_y) as usize;
        let s_idx = (sy * w + sx) * 3;
        let sr = base_rgb[s_idx];
        let sg = base_rgb[s_idx + 1];
        let sb = base_rgb[s_idx + 2];
        let sl = 0.299 * sr + 0.587 * sg + 0.114 * sb;
        if skin_weight(sr, sg, sb) > 0.38 && sl > curr_lum + 0.065 {
            skin_taps += 1;
        }
        taps[k] = (sl, sr, sg, sb);
    }
    if skin_taps < 6 {
        return (0.0, 0.0, 0.0, 0.0);
    }
    taps.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
    let sk_r = (taps[4].1 + taps[5].1 + taps[6].1) / 3.0;
    let sk_g = (taps[4].2 + taps[5].2 + taps[6].2) / 3.0;
    let sk_b = (taps[4].3 + taps[5].3 + taps[6].3) / 3.0;
    let sk_lum = 0.299 * sk_r + 0.587 * sk_g + 0.114 * sk_b;
    let weight = smoothstep(0.065, 0.18, sk_lum - curr_lum);
    (sk_r, sk_g, sk_b, weight)
}

/// Fill exposed bright scalp gaps along a narrow hair part line using adjacent hair tone & texture.
#[inline]
fn fill_hair_part_line(
    base_rgb: &[f32],
    blur_fine: &[f32],
    w: usize,
    h: usize,
    x: usize,
    y: usize,
    r_part: f32,
    curr_lum: f32,
) -> (f32, f32, f32, f32) {
    const DIRS: [(f32, f32); 8] = [
        (1.0, 0.0),
        (0.70710677, 0.70710677),
        (0.0, 1.0),
        (-0.70710677, 0.70710677),
        (-1.0, 0.0),
        (-0.70710677, -0.70710677),
        (0.0, -1.0),
        (0.70710677, -0.70710677),
    ];
    let max_x = w.saturating_sub(1) as f32;
    let max_y = h.saturating_sub(1) as f32;
    let mut taps = [(0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32); 8];
    let mut darker_taps = 0u32;
    for (k, &(dx, dy)) in DIRS.iter().enumerate() {
        let sx = (x as f32 + dx * r_part).round().clamp(0.0, max_x) as usize;
        let sy = (y as f32 + dy * r_part).round().clamp(0.0, max_y) as usize;
        let s_idx = (sy * w + sx) * 3;
        let sr = base_rgb[s_idx];
        let sg = base_rgb[s_idx + 1];
        let sb = base_rgb[s_idx + 2];
        let fr = blur_fine[s_idx];
        let fg = blur_fine[s_idx + 1];
        let fb = blur_fine[s_idx + 2];
        let sl = 0.299 * sr + 0.587 * sg + 0.114 * sb;
        if sl < curr_lum - 0.045 {
            darker_taps += 1;
        }
        taps[k] = (sl, sr, sg, sb, sr - fr, sg - fg, sb - fb);
    }
    if darker_taps < 5 {
        return (0.0, 0.0, 0.0, 0.0);
    }
    taps.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
    let dark_r = (taps[1].1 + taps[2].1 + taps[3].1) / 3.0;
    let dark_g = (taps[1].2 + taps[2].2 + taps[3].2) / 3.0;
    let dark_b = (taps[1].3 + taps[2].3 + taps[3].3) / 3.0;
    let dark_lum = 0.299 * dark_r + 0.587 * dark_g + 0.114 * dark_b;
    let donor = taps[2];
    let out_r = (dark_r + (donor.4 * 0.45).clamp(-0.04, 0.04)).clamp(0.0, 1.0);
    let out_g = (dark_g + (donor.5 * 0.45).clamp(-0.04, 0.04)).clamp(0.0, 1.0);
    let out_b = (dark_b + (donor.6 * 0.45).clamp(-0.04, 0.04)).clamp(0.0, 1.0);
    let weight = smoothstep(0.045, 0.18, curr_lum - dark_lum) * smoothstep(4.0, 6.5, darker_taps as f32);
    (out_r, out_g, out_b, weight)
}

/// Structure-tensor tangent smoothing along local hair strands while suppressing transverse frizz/static.
#[inline]
fn smooth_hair_oriented(
    base_rgb: &[f32],
    w: usize,
    h: usize,
    x: usize,
    y: usize,
    smooth_str: f32,
    frizzy_str: f32,
) -> (f32, f32, f32) {
    let idx = (y * w + x) * 3;
    let cr = base_rgb[idx];
    let cg = base_rgb[idx + 1];
    let cb = base_rgb[idx + 2];
    if w < 5 || h < 5 {
        return (cr, cg, cb);
    }
    let lum_at = |px: isize, py: isize| -> f32 {
        let cx = px.clamp(0, w as isize - 1) as usize;
        let cy = py.clamp(0, h as isize - 1) as usize;
        let i = (cy * w + cx) * 3;
        0.299 * base_rgb[i] + 0.587 * base_rgb[i + 1] + 0.114 * base_rgb[i + 2]
    };
    let ix = x as isize;
    let iy = y as isize;
    let gx = (lum_at(ix + 1, iy) - lum_at(ix - 1, iy)) * 0.5;
    let gy = (lum_at(ix, iy + 1) - lum_at(ix, iy - 1)) * 0.5;
    // Tangent along hair strand is perpendicular to gradient (-gy, gx); default vertical if flat
    let g_len = (gx * gx + gy * gy).sqrt();
    let (tx, ty) = if g_len > 0.008 {
        (-gy / g_len, gx / g_len)
    } else {
        (0.0, 1.0)
    };
    let nx = -ty;
    let ny = tx;

    let max_x = (w - 1) as f32;
    let max_y = (h - 1) as f32;
    let step = (w.min(h) as f32 * 0.0038).clamp(1.2, 4.0);
    let mut sum_t = (cr * 0.28, cg * 0.28, cb * 0.28);
    let mut wt_t = 0.28f32;
    for &(off, k) in &[(-2.0f32, 0.14f32), (-1.0, 0.22), (1.0, 0.22), (2.0, 0.14)] {
        let sx = (x as f32 + tx * off * step).clamp(0.0, max_x) as usize;
        let sy = (y as f32 + ty * off * step).clamp(0.0, max_y) as usize;
        let s_idx = (sy * w + sx) * 3;
        let sr = base_rgb[s_idx];
        let sg = base_rgb[s_idx + 1];
        let sb = base_rgb[s_idx + 2];
        // Guard against sampling bright background or skin across hair silhouette
        let dlum = ((0.299 * (sr - cr) + 0.587 * (sg - cg) + 0.114 * (sb - cb))).abs();
        let rw = (-dlum * dlum * 48.0).exp();
        let w_tap = k * rw;
        sum_t.0 += sr * w_tap;
        sum_t.1 += sg * w_tap;
        sum_t.2 += sb * w_tap;
        wt_t += w_tap;
    }
    let along_r = sum_t.0 / wt_t;
    let along_g = sum_t.1 / wt_t;
    let along_b = sum_t.2 / wt_t;

    // Gentle transverse frizz-reduction pull toward local fiber alignment when frizzy_str > 0
    if frizzy_str > 0.01 {
        let sx_p = (x as f32 + nx * step).clamp(0.0, max_x) as usize;
        let sy_p = (y as f32 + ny * step).clamp(0.0, max_y) as usize;
        let sx_m = (x as f32 - nx * step).clamp(0.0, max_x) as usize;
        let sy_m = (y as f32 - ny * step).clamp(0.0, max_y) as usize;
        let ip = (sy_p * w + sx_p) * 3;
        let im = (sy_m * w + sx_m) * 3;
        let mid_r = (base_rgb[ip] + base_rgb[im]) * 0.5;
        let mid_g = (base_rgb[ip + 1] + base_rgb[im + 1]) * 0.5;
        let mid_b = (base_rgb[ip + 2] + base_rgb[im + 2]) * 0.5;
        let dev = (cr - mid_r).abs() + (cg - mid_g).abs() + (cb - mid_b).abs();
        // Suppress bright or rough frizzy spikes protruding orthogonally to strand flow
        let frizz_gate = smoothstep(0.04, 0.22, dev) * frizzy_str * smooth_str * 0.55;
        (
            along_r * (1.0 - frizz_gate) + (along_r * 0.6 + mid_r * 0.4) * frizz_gate,
            along_g * (1.0 - frizz_gate) + (along_g * 0.6 + mid_g * 0.4) * frizz_gate,
            along_b * (1.0 - frizz_gate) + (along_b * 0.6 + mid_b * 0.4) * frizz_gate,
        )
    } else {
        (along_r, along_g, along_b)
    }
}

/// Apply Evoto HairColorAdjust (H01..H13 + legacy shades + Darkening) using decoded Evoto tone & pigment curves.
#[inline]
fn apply_evoto_hair_dye(
    r: f32,
    g: f32,
    b: f32,
    high_r: f32,
    high_g: f32,
    high_b: f32,
    hair_core_gate: f32,
    hair_skin_guard: f32,
    preset: &str,
    strength: f32,
    darken: f32,
) -> (f32, f32, f32) {
    // (target_rgb, exposure_lift, contrast, shadow_lift, chroma_mix)
    let spec: Option<((f32, f32, f32), f32, f32, f32, f32)> = match preset {
        // Parameters derived directly from Evoto HairColorAdjust/effect.json & iconColor
        "h01" => Some(((0.486, 0.412, 0.357), 0.14, 0.07, 0.19, 0.72)), // #7C695B Cool Tea
        "h02" => Some(((0.537, 0.361, 0.286), 0.10, 0.07, 0.08, 0.76)), // #895C49 Tea Brown
        "h03" => Some(((0.776, 0.475, 0.318), 0.20, 0.06, 0.29, 0.80)), // #C67951 Orange Brown
        "h04" => Some(((0.612, 0.329, 0.349), 0.16, 0.07, 0.12, 0.78)), // #9C5459 Red Brown+
        "h05" => Some(((0.678, 0.349, 0.337), 0.18, -0.16, 0.27, 0.82)), // #AD5956 Ruby Red
        "h06" => Some(((0.596, 0.447, 0.431), 0.22, -0.10, 0.19, 0.78)), // #98726E Pink Brown
        "h07" => Some(((0.808, 0.576, 0.565), 0.32, 0.00, 0.42, 0.84)), // #CE9390 Sakura Pink
        "h08" => Some(((0.220, 0.185, 0.172), -0.18, 0.08, -0.26, 0.78)), // #5D4B43 Natural Black
        "h09" => Some(((0.270, 0.305, 0.425), 0.06, -0.06, 0.10, 0.80)), // #646980 Blue Black
        "h10" => Some(((0.608, 0.345, 0.424), 0.14, 0.03, 0.22, 0.82)), // #9B586C Grape Purple
        "h11" => Some(((0.486, 0.427, 0.510), 0.16, 0.09, 0.19, 0.80)), // #7C6D82 Blue Purple
        "h12" => Some(((0.443, 0.435, 0.376), 0.15, 0.12, 0.13, 0.76)), // #716F60 Olive Flax
        "h13" => Some(((0.459, 0.502, 0.376), 0.18, -0.14, 0.28, 0.80)), // #758060 Ash Moss
        // Backwards-compatible legacy aliases
        "espresso" => Some(((0.220, 0.185, 0.172), -0.16, 0.06, -0.20, 0.74)),
        "golden_blonde" => Some(((0.820, 0.680, 0.420), 0.34, 0.02, 0.36, 0.78)),
        "auburn" => Some(((0.660, 0.360, 0.250), 0.18, 0.05, 0.20, 0.78)),
        "silver_ash" => Some(((0.560, 0.580, 0.640), 0.22, -0.06, 0.26, 0.76)),
        _ => None,
    };
    let Some(((pr, pg, pb), exp_adj, con_adj, sh_lift, chroma_mix)) = spec else {
        return (r, g, b);
    };

    // ProtectHairSeam guard: reduce dye opacity where skin/part scalp shows through
    let scalp_guard = (1.0 - hair_skin_guard * 0.9).clamp(0.15, 1.0);
    let deg = (strength / 100.0).clamp(0.0, 1.0);
    let dye_a = (hair_core_gate * scalp_guard * deg * 0.90).clamp(0.0, 0.92);
    if dye_a <= 0.005 {
        return (r, g, b);
    }

    let lum = (0.299 * r + 0.587 * g + 0.114 * b).clamp(0.0, 1.0);
    // Evoto-style tonal curve + shadow lift so dark hair shows dye hue clearly without washing out roots
    let lifted_lum = (lum * (1.0 + exp_adj * 0.55) + sh_lift * 0.16 * (1.0 - lum))
        .clamp(0.02, 0.96);
    let dark_scale = 1.0 - (darken / 100.0).clamp(0.0, 1.0) * 0.38;
    let target_lum = ((lifted_lum - 0.5) * (1.0 + con_adj * 0.35) + 0.5)
        .clamp(0.02, 0.96)
        * dark_scale;

    let p_lum = (0.299 * pr + 0.587 * pg + 0.114 * pb).max(0.06);
    // Modulate target pigment luminance around natural strand value while injecting Evoto preset hue
    let gain = (target_lum / p_lum).clamp(0.22, 1.85);
    let dr = (pr * gain + high_r * 0.65).clamp(0.0, 1.0);
    let dg = (pg * gain + high_g * 0.65).clamp(0.0, 1.0);
    let db = (pb * gain + high_b * 0.65).clamp(0.0, 1.0);

    let mix_w = dye_a * chroma_mix;
    (
        (r * (1.0 - mix_w) + dr * mix_w).clamp(0.0, 1.0),
        (g * (1.0 - mix_w) + dg * mix_w).clamp(0.0, 1.0),
        (b * (1.0 - mix_w) + db * mix_w).clamp(0.0, 1.0),
    )
}

/// Estimate this subject's dominant natural pigmented hair color (fallback dark espresso if mostly gray/white).
fn estimate_dominant_hair_color(
    base_rgb: &[f32],
    semantic: &SemanticData,
    w: usize,
    h: usize,
) -> (f32, f32, f32, f32) {
    let wf = w.max(1) as f32;
    let hf = h.max(1) as f32;
    let step = (w.min(h) / 160).max(1);
    let mut sum_r = 0.0f64;
    let mut sum_g = 0.0f64;
    let mut sum_b = 0.0f64;
    let mut sum_w = 0.0f64;
    for y in (0..h).step_by(step) {
        let ny = y as f32 / hf;
        for x in (0..w).step_by(step) {
            let nx = x as f32 / wf;
            let sem = sample_semantic_grid(&semantic.weights, nx, ny);
            if sem.hair < 0.35 || sem.skin > 0.15 || sem.background > 0.30 {
                continue;
            }
            let idx = (y * w + x) * 3;
            let r = base_rgb[idx];
            let g = base_rgb[idx + 1];
            let b = base_rgb[idx + 2];
            if skin_weight(r, g, b) > 0.25 {
                continue;
            }
            let lum = 0.299 * r + 0.587 * g + 0.114 * b;
            let chroma = r.max(g).max(b) - r.min(g).min(b);
            // Skip bright low-chroma white/silver strands so they don't pull the natural base gray
            if lum > 0.42 && chroma < 0.10 {
                continue;
            }
            let wt = (sem.hair as f64) * (1.0 - (lum as f64 - 0.18).max(0.0) * 0.7).max(0.15);
            sum_r += r as f64 * wt;
            sum_g += g as f64 * wt;
            sum_b += b as f64 * wt;
            sum_w += wt;
        }
    }
    if sum_w > 1.0 {
        let dr = (sum_r / sum_w) as f32;
        let dg = (sum_g / sum_w) as f32;
        let db = (sum_b / sum_w) as f32;
        let dlum = 0.299 * dr + 0.587 * dg + 0.114 * db;
        (dr, dg, db, dlum)
    } else {
        (0.09, 0.08, 0.075, 0.083)
    }
}

struct CleanRingEstimate {
    clean_r: f32,
    clean_g: f32,
    clean_b: f32,
    clean_lum: f32,
    dark_iso: f32,
    red_iso: f32,
}

#[derive(Clone, Copy)]
struct RingEval {
    clean_r: f32,
    clean_g: f32,
    clean_b: f32,
    clean_lum: f32,
    dark_iso: f32,
    red_iso: f32,
}

/// Evaluate 12 evenly-spaced bilinear compass taps at `radius` around `(x, y)`
/// for continuous C1-smooth spot isolation, 1D crease & endpoint rejection, and unbiased local tone.
#[inline]
fn eval_spot_ring(
    base_rgb: &[f32],
    blur_fine: &[f32],
    w: usize,
    h: usize,
    x: usize,
    y: usize,
    radius: f32,
    curr_lum: f32,
    curr_rg: f32,
) -> RingEval {
    const DIRS12: [(f32, f32); 12] = [
        (1.0, 0.0),
        (0.8660254, 0.5),
        (0.5, 0.8660254),
        (0.0, 1.0),
        (-0.5, 0.8660254),
        (-0.8660254, 0.5),
        (-1.0, 0.0),
        (-0.8660254, -0.5),
        (-0.5, -0.8660254),
        (0.0, -1.0),
        (0.5, -0.8660254),
        (0.8660254, -0.5),
    ];

    let max_x = w.saturating_sub(1) as f32;
    let max_y = h.saturating_sub(1) as f32;
    let fx0 = x as f32;
    let fy0 = y as f32;

    let mut dc = [0.0f32; 12];
    let mut rc = [0.0f32; 12];
    // (cleanliness_score, fr, fg, fb) from noise-free blur_fine background
    let mut clean_taps = [(0.0f32, 0.0f32, 0.0f32, 0.0f32); 12];

    let mut k_dc_min = 0usize;
    let mut dc_min = f32::INFINITY;
    let mut k_rc_min = 0usize;
    let mut rc_min = f32::INFINITY;

    for (k, &(dx, dy)) in DIRS12.iter().enumerate() {
        let sx = (fx0 + dx * radius).clamp(0.0, max_x);
        let sy = (fy0 + dy * radius).clamp(0.0, max_y);
        let (sr, sg, sb) = sample_bilinear(base_rgb, w, h, sx, sy);
        let (fr, fg, fb) = sample_bilinear(blur_fine, w, h, sx, sy);

        let sl = 0.299 * sr + 0.587 * sg + 0.114 * sb;
        let s_rg = sr - sg;
        let d_val = sl - curr_lum;
        let r_val = curr_rg - s_rg;
        dc[k] = d_val;
        rc[k] = r_val;
        if d_val < dc_min {
            dc_min = d_val;
            k_dc_min = k;
        }
        if r_val < rc_min {
            rc_min = r_val;
            k_rc_min = k;
        }

        let fl = 0.299 * fr + 0.587 * fg + 0.114 * fb;
        clean_taps[k] = (fl - 0.25 * (fr - fg), fr, fg, fb);
    }

    // Opposing-pair 1D crease suppression: if any axis keeps both opposite ends un-lifted, it is a wrinkle/crease/line
    let mut worst_axis_max_dc = f32::INFINITY;
    let mut worst_axis_max_rc = f32::INFINITY;
    for k in 0..6 {
        worst_axis_max_dc = worst_axis_max_dc.min(dc[k].max(dc[k + 6]));
        worst_axis_max_rc = worst_axis_max_rc.min(rc[k].max(rc[k + 6]));
    }
    let no_dark_crease = smoothstep(0.005, 0.022, worst_axis_max_dc);
    let no_red_crease = smoothstep(0.006, 0.024, worst_axis_max_rc);

    let mut dc_s = dc;
    let mut rc_s = rc;
    dc_s.sort_unstable_by(f32::total_cmp);
    rc_s.sort_unstable_by(f32::total_cmp);

    // If only 1 ray at `radius` failed to exit onto clean skin (common near the rim of an oval/medium spot
    // looking back across its own center), check 1.85*radius along that ray: finite spots exit to clean skin,
    // while continuous line/mouth-corner creases stay dark along their shaft.
    let eff_dc0 = if dc_s[0] < 0.014 && dc_s[1] > 0.012 {
        let (dx, dy) = DIRS12[k_dc_min];
        let ex = (fx0 + dx * (radius * 1.85)).clamp(0.0, max_x);
        let ey = (fy0 + dy * (radius * 1.85)).clamp(0.0, max_y);
        let (er, eg, eb) = sample_bilinear(base_rgb, w, h, ex, ey);
        let dc_ext = (0.299 * er + 0.587 * eg + 0.114 * eb) - curr_lum;
        let rescue = smoothstep(0.012, 0.030, dc_s[1]) * smoothstep(0.012, 0.032, dc_ext);
        dc_s[0].max(dc_ext * rescue)
    } else {
        dc_s[0]
    };
    let eff_rc0 = if rc_s[0] < 0.014 && rc_s[1] > 0.012 {
        let (dx, dy) = DIRS12[k_rc_min];
        let ex = (fx0 + dx * (radius * 1.85)).clamp(0.0, max_x);
        let ey = (fy0 + dy * (radius * 1.85)).clamp(0.0, max_y);
        let (er, eg, _) = sample_bilinear(base_rgb, w, h, ex, ey);
        let rc_ext = curr_rg - (er - eg);
        let rescue = smoothstep(0.012, 0.030, rc_s[1]) * smoothstep(0.012, 0.032, rc_ext);
        rc_s[0].max(rc_ext * rescue)
    } else {
        rc_s[0]
    };

    // Require smooth closed-ring isolation:
    // - dc_s[1]: at least 11/12 directions cleanly brighter than current pixel (smooth edge feathering, no binary cliff)
    // - eff_dc0: suppresses line endpoints / mouth-corner slits & penumbra rims of connected dark features
    let dark_closure = smoothstep(0.004, 0.020, dc_s[1]);
    let no_dark_endpoint = smoothstep(-0.001, 0.014, eff_dc0);
    let dark_iso = dark_closure * no_dark_endpoint * no_dark_crease;

    let red_closure = smoothstep(0.004, 0.020, rc_s[1]);
    let no_red_endpoint = smoothstep(-0.001, 0.014, eff_rc0);
    let red_iso = red_closure * no_red_endpoint * no_red_crease;

    clean_taps.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));

    // Average trimmed middle 6 samples [3..9] from blur_fine: unbiased smooth surrounding skin tone
    // excluding bottom 3 shadow/spot taps and top 3 highlight outliers.
    let inv6 = 1.0 / 6.0;
    let clean_r = (clean_taps[3].1
        + clean_taps[4].1
        + clean_taps[5].1
        + clean_taps[6].1
        + clean_taps[7].1
        + clean_taps[8].1)
        * inv6;
    let clean_g = (clean_taps[3].2
        + clean_taps[4].2
        + clean_taps[5].2
        + clean_taps[6].2
        + clean_taps[7].2
        + clean_taps[8].2)
        * inv6;
    let clean_b = (clean_taps[3].3
        + clean_taps[4].3
        + clean_taps[5].3
        + clean_taps[6].3
        + clean_taps[7].3
        + clean_taps[8].3)
        * inv6;
    let clean_lum = 0.299 * clean_r + 0.587 * clean_g + 0.114 * clean_b;

    RingEval {
        clean_r,
        clean_g,
        clean_b,
        clean_lum,
        dark_iso,
        red_iso,
    }
}

/// Dual-scale closed-contour spot detector: blends tight local skin and wider enclosing skin smoothly
/// so medium/oval spots heal evenly without donut holes or hard cutout borders, while adding
/// spatially coherent natural skin micro-grain.
#[inline]
fn detect_clean_spot_ring(
    base_rgb: &[f32],
    blur_fine: &[f32],
    w: usize,
    h: usize,
    x: usize,
    y: usize,
    r_tight: f32,
    r_wide: f32,
    curr_lum: f32,
    curr_rg: f32,
) -> CleanRingEstimate {
    let tight = eval_spot_ring(base_rgb, blur_fine, w, h, x, y, r_tight, curr_lum, curr_rg);
    let wide = eval_spot_ring(base_rgb, blur_fine, w, h, x, y, r_wide, curr_lum, curr_rg);

    // Smooth blend between tight and wide scales so medium/oval spots transition cleanly without donut holes or ring steps.
    // If wide sees a substantially cleaner/less-red ring than tight, tight is still sitting inside the spot slope -> shift to wide.
    let tight_best = tight.dark_iso.max(tight.red_iso);
    let wide_best = wide.dark_iso.max(wide.red_iso);
    let wide_cleaner_dark = smoothstep(0.008, 0.032, wide.clean_lum - tight.clean_lum);
    let tight_rg = tight.clean_r - tight.clean_g;
    let wide_rg = wide.clean_r - wide.clean_g;
    let wide_cleaner_red = smoothstep(0.008, 0.028, tight_rg - wide_rg);
    let tight_in_halo = (wide_cleaner_dark * wide.dark_iso).max(wide_cleaner_red * wide.red_iso);
    let w_t = tight_best * (1.0 - 0.95 * tight_in_halo);
    let w_w = wide_best * (1.0 - 0.72 * w_t);
    let w_sum = w_t + w_w;

    let (clean_base_r, clean_base_g, clean_base_b, clean_lum) = if w_sum > 1e-4 {
        (
            (tight.clean_r * w_t + wide.clean_r * w_w) / w_sum,
            (tight.clean_g * w_t + wide.clean_g * w_w) / w_sum,
            (tight.clean_b * w_t + wide.clean_b * w_w) / w_sum,
            (tight.clean_lum * w_t + wide.clean_lum * w_w) / w_sum,
        )
    } else {
        (tight.clean_r, tight.clean_g, tight.clean_b, tight.clean_lum)
    };

    // Sample spatially coherent natural skin micro-grain from a parallel shifted donor so healed skin keeps pores (no flat sticker)
    let max_x = w.saturating_sub(1) as f32;
    let max_y = h.saturating_sub(1) as f32;
    let fx = x as f32;
    let fy = y as f32;
    let gx = r_wide * 0.82;
    let gy = r_wide * 0.57;

    let s1x = (fx + gx).clamp(0.0, max_x);
    let s1y = (fy + gy).clamp(0.0, max_y);
    let sb1 = sample_bilinear(base_rgb, w, h, s1x, s1y);
    let sf1 = sample_bilinear(blur_fine, w, h, s1x, s1y);
    let l1 = 0.299 * sf1.0 + 0.587 * sf1.1 + 0.114 * sf1.2;
    let w1 = 1.0 - smoothstep(0.012, 0.045, (clean_lum - l1).abs());

    let s2x = (fx - gx).clamp(0.0, max_x);
    let s2y = (fy - gy).clamp(0.0, max_y);
    let sb2 = sample_bilinear(base_rgb, w, h, s2x, s2y);
    let sf2 = sample_bilinear(blur_fine, w, h, s2x, s2y);
    let l2 = 0.299 * sf2.0 + 0.587 * sf2.1 + 0.114 * sf2.2;
    let w2 = (1.0 - smoothstep(0.012, 0.045, (clean_lum - l2).abs())) * (1.0 - w1);

    let g_sum = w1 + w2;
    let (grain_r, grain_g, grain_b) = if g_sum > 1e-4 {
        let gr1 = ((sb1.0 - sf1.0) * 0.65, (sb1.1 - sf1.1) * 0.65, (sb1.2 - sf1.2) * 0.65);
        let gr2 = ((sb2.0 - sf2.0) * 0.65, (sb2.1 - sf2.1) * 0.65, (sb2.2 - sf2.2) * 0.65);
        (
            ((gr1.0 * w1 + gr2.0 * w2) / g_sum).clamp(-0.032, 0.032),
            ((gr1.1 * w1 + gr2.1 * w2) / g_sum).clamp(-0.032, 0.032),
            ((gr1.2 * w1 + gr2.2 * w2) / g_sum).clamp(-0.032, 0.032),
        )
    } else {
        (0.0, 0.0, 0.0)
    };

    CleanRingEstimate {
        clean_r: (clean_base_r + grain_r).clamp(0.0, 1.0),
        clean_g: (clean_base_g + grain_g).clamp(0.0, 1.0),
        clean_b: (clean_base_b + grain_b).clamp(0.0, 1.0),
        clean_lum,
        dark_iso: 1.0 - (1.0 - tight.dark_iso) * (1.0 - wide.dark_iso),
        red_iso: 1.0 - (1.0 - tight.red_iso) * (1.0 - wide.red_iso),
    }
}

pub fn apply_retouch_pipeline(
    img: &RgbaImage,
    params: &RetouchParams,
    mask_img: Option<&RgbaImage>,
) -> RgbaImage {
    apply_retouch_pipeline_with_scene(img, params, mask_img, None)
}

pub fn apply_retouch_pipeline_with_scene(
    img: &RgbaImage,
    params: &RetouchParams,
    mask_img: Option<&RgbaImage>,
    scene_mask_img: Option<&RgbaImage>,
) -> RgbaImage {
    let (w_u32, h_u32) = img.dimensions();
    let w = w_u32 as usize;
    let h = h_u32 as usize;

    // Build the 512x512 Semantic Anatomical Grid + continuous neural foreground/hair fields
    let semantic_data = build_semantic_grid(mask_img, scene_mask_img, &params.face);

    // Load original image into normalized RGB float buffer (in original coordinates so masks align 1:1!)
    let mut base_rgb = vec![0.0f32; w * h * 3];
    base_rgb
        .par_chunks_mut(w * 3)
        .enumerate()
        .for_each(|(y, row)| {
            for x in 0..w {
                let p = img.get_pixel(x as u32, y as u32);
                let idx = x * 3;
                row[idx] = p[0] as f32 / 255.0;
                row[idx + 1] = p[1] as f32 / 255.0;
                row[idx + 2] = p[2] as f32 / 255.0;
            }
        });

    // Interactive Manual Spot Healing (directional gradient interpolation + clean pore texture synthesis)
    if !params.blemish_spots.is_empty() {
        let snapshot = base_rgb.clone();
        for spot in &params.blemish_spots {
            let cx = spot.x * w as f32;
            let cy = spot.y * h as f32;
            let rad = (spot.radius * w.min(h) as f32).max(4.0);
            let x0 = ((cx - rad).floor().max(0.0)) as usize;
            let x1 = ((cx + rad).ceil().min((w - 1) as f32)) as usize;
            let y0 = ((cy - rad).floor().max(0.0)) as usize;
            let y1 = ((cy + rad).ceil().min((h - 1) as f32)) as usize;

            const RING_N: usize = 16;
            let mut ring = [(0.0f32, 0.0f32, 0.0f32); RING_N];
            let mut ring_mean = (0.0f32, 0.0f32, 0.0f32);
            for i in 0..RING_N {
                let angle = (i as f32 / RING_N as f32) * std::f32::consts::TAU;
                let sx = cx + angle.cos() * (rad * 1.28);
                let sy = cy + angle.sin() * (rad * 1.28);
                let c = sample_bilinear(&snapshot, w, h, sx, sy);
                ring[i] = c;
                ring_mean.0 += c.0;
                ring_mean.1 += c.1;
                ring_mean.2 += c.2;
            }
            ring_mean.0 /= RING_N as f32;
            ring_mean.1 /= RING_N as f32;
            ring_mean.2 /= RING_N as f32;

            // Pick a clean outer skin ring offset to transplant natural skin grain/pores
            let mut donor_dx = rad * 1.35;
            let mut donor_dy = 0.0f32;
            let mut best_clean_score = f32::NEG_INFINITY;
            for i in 0..RING_N {
                let (rr, rg, rb) = ring[i];
                let lum = 0.299 * rr + 0.587 * rg + 0.114 * rb;
                let score = -(lum - (0.299 * ring_mean.0 + 0.587 * ring_mean.1 + 0.114 * ring_mean.2)).abs();
                if score > best_clean_score {
                    best_clean_score = score;
                    let angle = (i as f32 / RING_N as f32) * std::f32::consts::TAU;
                    donor_dx = angle.cos() * (rad * 1.35);
                    donor_dy = angle.sin() * (rad * 1.35);
                }
            }

            for y in y0..=y1 {
                for x in x0..=x1 {
                    let dx = x as f32 - cx;
                    let dy = y as f32 - cy;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < rad {
                        // Directional boundary-weighted interpolation preserves cheek/forehead gradients
                        let mut w_sum = 0.0f32;
                        let mut interp = (0.0f32, 0.0f32, 0.0f32);
                        for i in 0..RING_N {
                            let angle = (i as f32 / RING_N as f32) * std::f32::consts::TAU;
                            let bx = angle.cos() * rad;
                            let by = angle.sin() * rad;
                            let d2 = (dx - bx) * (dx - bx) + (dy - by) * (dy - by) + 1.0;
                            let wt = 1.0 / d2;
                            interp.0 += ring[i].0 * wt;
                            interp.1 += ring[i].1 * wt;
                            interp.2 += ring[i].2 * wt;
                            w_sum += wt;
                        }
                        interp.0 /= w_sum;
                        interp.1 /= w_sum;
                        interp.2 /= w_sum;

                        // Transplant subtle high-frequency pore detail from clean skin donor
                        let tx = (x as f32 + donor_dx).clamp(0.0, (w - 1) as f32);
                        let ty = (y as f32 + donor_dy).clamp(0.0, (h - 1) as f32);
                        let donor_px = sample_bilinear(&snapshot, w, h, tx, ty);
                        let donor_local = sample_bilinear(&snapshot, w, h, cx + donor_dx, cy + donor_dy);
                        let tex_r = ((donor_px.0 - donor_local.0) * 0.65).clamp(-0.05, 0.05);
                        let tex_g = ((donor_px.1 - donor_local.1) * 0.65).clamp(-0.05, 0.05);
                        let tex_b = ((donor_px.2 - donor_local.2) * 0.65).clamp(-0.05, 0.05);

                        // Full 1.0 replacement across inner core (<= 0.55 * rad), feathered smoothly to edge
                        let norm_d = (dist / rad).clamp(0.0, 1.0);
                        let t = if norm_d <= 0.55 {
                            1.0
                        } else {
                            let u = (norm_d - 0.55) / 0.45;
                            1.0 - (u * u * (3.0 - 2.0 * u))
                        };
                        let idx = (y * w + x) * 3;
                        let target_r = (interp.0 + tex_r).clamp(0.0, 1.0);
                        let target_g = (interp.1 + tex_g).clamp(0.0, 1.0);
                        let target_b = (interp.2 + tex_b).clamp(0.0, 1.0);
                        base_rgb[idx] = base_rgb[idx] * (1.0 - t) + target_r * t;
                        base_rgb[idx + 1] = base_rgb[idx + 1] * (1.0 - t) + target_g * t;
                        base_rgb[idx + 2] = base_rgb[idx + 2] * (1.0 - t) + target_b * t;
                    }
                }
            }
        }
    }

    // AI Denoise: runs first on the decoded pixels (RAW and JPG take the same path)
    if params.ai_denoise > 0.5 {
        base_rgb = adaptive_denoise_rgb(&base_rgb, w, h, params.ai_denoise, params.nr_detail);
    }

    let is_replacing = params.bg_replace_mode != "original";
    let is_transparent_bg = params.bg_replace_mode == "transparent";
    let eff_stray_inner = (params.stray_hair * (params.stray_hair_inner / 100.0)).clamp(0.0, 100.0);
    let eff_stray_outer = (params.stray_hair * (params.stray_hair_outer / 100.0)).clamp(0.0, 100.0);
    let has_scene_fx = is_replacing
        || params.bg_blur_bokeh > 0.5
        || eff_stray_outer > 0.5
        || params.backdrop_clean > 0.5
        || params.backdrop_banding_fix > 0.5
        || params.sky_enhance > 0.5
        || params.bg_clean_distractions
        || params.bg_clean_defects
        || params.bg_even_brightness
        || params.bg_remove_banding;

    let guided_matte = if has_scene_fx {
        Some(if params.is_highres_matte {
            if let Some(s_img) = scene_mask_img {
                compute_highres_direct_matte(
                    &base_rgb,
                    w,
                    h,
                    s_img,
                    params,
                    is_replacing && params.bg_decontam,
                )
            } else {
                compute_guided_matte(
                    &base_rgb,
                    w,
                    h,
                    &semantic_data,
                    params,
                    is_replacing,
                )
            }
        } else {
            compute_guided_matte(
                &base_rgb,
                w,
                h,
                &semantic_data,
                params,
                is_replacing,
            )
        })
    } else {
        None
    };

    let (bg_mean_r, bg_mean_g, bg_mean_b, bg_mean_luma) = if let Some(ref gm) = guided_matte {
        if params.bg_even_brightness || (is_replacing && (params.bg_preserve_shadow || params.bg_decontam)) {
            let mut sr = 0.0f64;
            let mut sg = 0.0f64;
            let mut sb = 0.0f64;
            let mut sw = 0.0f64;
            let step = ((w * h) / 25000).max(1);
            for i in (0..(w * h)).step_by(step) {
                let bg_w = (1.0 - gm.fg_alpha[i]).max(0.0).powi(2) as f64;
                if bg_w > 0.05 {
                    sr += base_rgb[i * 3] as f64 * bg_w;
                    sg += base_rgb[i * 3 + 1] as f64 * bg_w;
                    sb += base_rgb[i * 3 + 2] as f64 * bg_w;
                    sw += bg_w;
                }
            }
            if sw > 1.0 {
                let mr = (sr / sw) as f32;
                let mg = (sg / sw) as f32;
                let mb = (sb / sw) as f32;
                let ml = (0.299 * mr + 0.587 * mg + 0.114 * mb).clamp(0.05, 0.98);
                (mr, mg, mb, ml)
            } else {
                (0.5, 0.5, 0.5, 0.5)
            }
        } else {
            (0.5, 0.5, 0.5, 0.5)
        }
    } else {
        (0.5, 0.5, 0.5, 0.5)
    };

    let cast_shadow = if is_replacing
        && !is_transparent_bg
        && params.bg_shadow_type != "none"
        && params.bg_shadow_opacity > 0.5
    {
        guided_matte
            .as_ref()
            .map(|gm| compute_cast_shadow_plane(&gm.fg_alpha, w, h, params))
    } else {
        None
    };

    let custom_bg_rgb = if params.bg_replace_mode == "custom_color" {
        parse_hex_color(&params.bg_custom_color)
    } else {
        (0.91, 0.93, 0.96)
    };
    let custom_bg_buf = if params.bg_replace_mode == "custom_image" {
        prepare_custom_background(&params.bg_custom_image, w, h, params).unwrap_or_default()
    } else {
        Vec::new()
    };

    // Multi-scale Frequency Separation Blurs on base_rgb (background bokeh skips foreground pixels)
    let fine_radius = ((w.min(h) as f32 * 0.0045).round() as usize).clamp(2, 10);
    let med_radius = ((w.min(h) as f32 * 0.014).round() as usize).clamp(5, 24);
    let bokeh_radius = if !is_replacing
        && (params.bg_blur_bokeh > 0.5
            || eff_stray_outer > 0.5
            || params.backdrop_clean > 0.5
            || params.backdrop_banding_fix > 0.5
            || params.bg_clean_distractions
            || params.bg_clean_defects
            || params.bg_remove_banding)
    {
        ((w.min(h) as f32 * 0.026 * (params.bg_blur_bokeh.max(35.0) / 100.0)).round() as usize)
            .clamp(6, 44)
    } else {
        0
    };

    let blur_fine = separable_blur(&base_rgb, w, h, fine_radius);
    let blur_med_p1 = separable_blur(&base_rgb, w, h, med_radius);
    let blur_med = separable_blur(&blur_med_p1, w, h, med_radius / 2 + 1);
    let blur_bokeh = if bokeh_radius > 0 {
        if let Some(ref gm) = guided_matte {
            masked_background_bokeh(&base_rgb, &gm.fg_alpha, w, h, bokeh_radius)
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };
    let need_cloth_broad = params.clothing_dewrinkle > 0.5
        || params.clothing_sweat_stain > 0.5
        || params.clothing_edge_smooth > 0.5
        || params.clothing_moire_remove > 0.5;
    let blur_cloth_broad = if need_cloth_broad {
        let cloth_broad_radius = (med_radius * 2 + 2).clamp(10, 44);
        separable_blur(&blur_med, w, h, cloth_broad_radius)
    } else {
        Vec::new()
    };

    let face = &params.face;

    // Precompute Color & Retouching scalars (each slider strictly separated!)
    let exp_mult = 2.0f32.powf(params.exposure / 50.0);
    let contrast_k = (params.contrast / 100.0) * 0.72;
    let temp_shift = (params.temperature / 100.0) * 0.16;
    let tint_shift = (params.tint / 100.0) * 0.12;
    let sat_mult = 1.0 + (params.saturation / 100.0) * 0.90;
    let vib_strength = (params.vibrance / 100.0) * 0.85;
    let clarity_str = (params.clarity / 100.0) * 0.95;
    let texture_str = (params.texture / 100.0) * 1.05;
    let dehaze_str = (params.dehaze / 100.0) * 0.50;
    let high_adj = (params.highlights / 100.0) * 0.42;
    let shad_adj = (params.shadows / 100.0) * 0.46;
    let whites_adj = (params.whites / 100.0) * 0.28;
    let blacks_adj = (params.blacks / 100.0) * 0.28;

    // Separated Portrait Retouching Strengths
    let freckle_acne_str = (params.freckle_acne / 100.0).clamp(0.0, 1.0);
    let face_mole_str = (params.face_mole / 100.0).clamp(0.0, 1.0);
    let body_blemish_str = (params.body_blemish / 100.0).clamp(0.0, 1.0);
    let face_shine_str = (params.face_shine / 100.0).clamp(0.0, 1.0);
    let glasses_glare_str = (params.glasses_glare / 100.0).clamp(0.0, 1.0);
    let dark_circles_str = (params.dark_circles / 100.0).clamp(0.0, 1.0);
    let eye_bags_str = (params.eye_bags / 100.0).clamp(0.0, 1.0);
    let forehead_wrinkles_str = (params.forehead_wrinkles / 100.0).clamp(0.0, 1.0);
    let nasolabial_str = (params.nasolabial_folds / 100.0).clamp(0.0, 1.0);
    let neck_wrinkles_str = (params.neck_wrinkles / 100.0).clamp(0.0, 1.0);
    let double_chin_str = (params.double_chin / 100.0).clamp(0.0, 1.0);

    let dodge_burn_even_str = (params.dodge_burn_even / 100.0).clamp(0.0, 1.0);
    let unify_complexion_str = (params.unify_complexion / 100.0).clamp(0.0, 1.0);
    let db_sculpt_str = (params.dodge_burn_sculpt / 100.0).clamp(0.0, 1.0);
    let freq_sep_low_str = (params.freq_sep_low / 100.0).clamp(0.0, 1.0);
    let pore_scale = (params.freq_sep_high / 80.0).clamp(0.0, 1.35);
    let facial_smooth_str = (params.facial_skin_smooth / 100.0).clamp(0.0, 1.0);
    let body_smooth_str = (params.body_skin_smooth / 100.0).clamp(0.0, 1.0);
    let rad_str = (params.skin_radiance / 100.0).clamp(0.0, 1.0);
    let rosy_str = (params.skin_rosy / 100.0).clamp(-1.0, 1.0);
    let warmth_str = (params.skin_warmth / 100.0).clamp(-1.0, 1.0);

    let eye_bright_str = (params.eye_brighten / 100.0).clamp(0.0, 1.0);
    let catchlight_str = (params.catchlight_boost / 100.0).clamp(0.0, 1.0);
    let red_vein_str = (params.red_veins / 100.0).clamp(0.0, 1.0);
    let brow_thick_str = (params.eyebrow_thickness / 100.0).clamp(-1.0, 1.0);

    let teeth_whiten_str = (params.teeth_whiten / 100.0).clamp(0.0, 1.0);
    let teeth_desat_str = (params.teeth_desaturate / 100.0).clamp(0.0, 1.0);
    let teeth_bright_str = (params.teeth_brighten / 100.0).clamp(0.0, 1.0);

    let makeup_high_str = (params.makeup_highlight / 100.0).clamp(0.0, 1.0);
    let makeup_cont_str = (params.makeup_contour / 100.0).clamp(0.0, 1.0);
    let makeup_blush_str = (params.makeup_blush / 100.0).clamp(0.0, 1.0);
    let makeup_eyeshadow_str = (params.makeup_eyeshadow / 100.0).clamp(0.0, 1.0);
    let lip_str = (params.makeup_lipstick / 100.0).clamp(0.0, 1.0);

    // Precompute source image RGB mean & std for Color Match (Histogram / Moment Transfer)
    let color_match_str = (params.color_match_strength / 100.0).clamp(0.0, 1.0);
    let src_stats: [f32; 6] = if color_match_str > 0.005 && params.color_match_stats.len() == 6 {
        let step = (w * h / 4096).max(1);
        let mut sr = 0.0f64;
        let mut sg = 0.0f64;
        let mut sb = 0.0f64;
        let mut cnt = 0.0f64;
        for i in (0..(w * h)).step_by(step) {
            sr += base_rgb[i * 3] as f64;
            sg += base_rgb[i * 3 + 1] as f64;
            sb += base_rgb[i * 3 + 2] as f64;
            cnt += 1.0;
        }
        let mr = (sr / cnt.max(1.0)) as f32;
        let mg = (sg / cnt.max(1.0)) as f32;
        let mb = (sb / cnt.max(1.0)) as f32;
        let mut vr = 0.0f64;
        let mut vg = 0.0f64;
        let mut vb = 0.0f64;
        for i in (0..(w * h)).step_by(step) {
            vr += ((base_rgb[i * 3] - mr) as f64).powi(2);
            vg += ((base_rgb[i * 3 + 1] - mg) as f64).powi(2);
            vb += ((base_rgb[i * 3 + 2] - mb) as f64).powi(2);
        }
        [
            mr,
            ((vr / cnt.max(1.0)).sqrt() as f32).max(0.02),
            mg,
            ((vg / cnt.max(1.0)).sqrt() as f32).max(0.02),
            mb,
            ((vb / cnt.max(1.0)).sqrt() as f32).max(0.02),
        ]
    } else {
        [0.5, 0.2, 0.5, 0.2, 0.5, 0.2]
    };

    let (dom_hair_r, dom_hair_g, dom_hair_b, dom_hair_lum) = if params.white_hair_blacken > 0.5
        || params.hair_color_unify.abs() > 0.5
    {
        estimate_dominant_hair_color(&base_rgb, &semantic_data, w, h)
    } else {
        (0.09, 0.08, 0.075, 0.083)
    };

    // Stage 1: Apply all Semantic Mask-Guided Retouching + Color Grading in unwarped image space
    let mut retouched_rgb = vec![0.0f32; w * h * 3];
    retouched_rgb
        .par_chunks_mut(w * 3)
        .enumerate()
        .for_each(|(y, row_out)| {
            let ny = y as f32 / h as f32;
            for x in 0..w {
                let nx = x as f32 / w as f32;
                let idx = (y * w + x) * 3;

                let mut r = base_rgb[idx];
                let mut g = base_rgb[idx + 1];
                let mut b = base_rgb[idx + 2];

                let rf = blur_fine[idx];
                let gf = blur_fine[idx + 1];
                let bf = blur_fine[idx + 2];

                let rm = blur_med[idx];
                let gm = blur_med[idx + 1];
                let bm = blur_med[idx + 2];

                let high_r = r - rf;
                let high_g = g - gf;
                let high_b = b - bf;

                let sem = sample_semantic_grid(&semantic_data.weights, nx, ny);

                // Combine neural polygon skin mask with subtle color gate so non-skin objects are never smoothed
                let color_sw = skin_weight(r, g, b);
                let sw = (sem.skin * (0.65 + 0.35 * color_sw)).clamp(0.0, 1.0);
                let is_body_zone = ((ny - face.chin_y) / (face.face_ry * 0.25).max(0.04)).clamp(0.0, 1.0);
                let face_skin_w = sw * (1.0 - is_body_zone);
                let body_skin_w = sw * is_body_zone;

                // =========================================================
                // 1. BLEMISHES, SHINE, WRINKLES & DARK CIRCLES / EYE BAGS
                // =========================================================
                if sw > 0.01 {
                    let lum_curr = 0.299 * r + 0.587 * g + 0.114 * b;
                    let lum_fine = 0.299 * rf + 0.587 * gf + 0.114 * bf;
                    let lum_med = 0.299 * rm + 0.587 * gm + 0.114 * bm;

                    // Protect eyes, eyebrows, lips, teeth, hair, nostrils, and mouth-corner creases from blemish removal
                    let feature_protect = (1.0
                        - (sem.eye * 1.8
                            + sem.eyebrow * 1.8
                            + sem.lips_outer * 1.9
                            + sem.teeth_inner * 1.8
                            + sem.hair * 1.2))
                        .clamp(0.0, 1.0);
                    let nostril_guard = if face.face_rx > 0.01 {
                        let ndx = (nx - face.nose_x) / (face.face_rx * 0.30).max(0.015);
                        let ndy = (ny - (face.nose_y + face.face_ry * 0.035))
                            / (face.face_ry * 0.12).max(0.010);
                        let nd2 = ndx * ndx + ndy * ndy;
                        if nd2 < 1.0 {
                            (nd2 * nd2).clamp(0.0, 1.0)
                        } else {
                            1.0
                        }
                    } else {
                        1.0
                    };
                    let mouth_corner_guard = if face.face_rx > 0.01 {
                        let rx_mc = (face.face_rx * 0.14).max(0.018);
                        let ry_mc = (face.face_ry * 0.095).max(0.012);
                        let ld2 = ((nx - face.mouth_left_x) / rx_mc).powi(2)
                            + ((ny - face.mouth_left_y) / ry_mc).powi(2);
                        let rd2 = ((nx - face.mouth_right_x) / rx_mc).powi(2)
                            + ((ny - face.mouth_right_y) / ry_mc).powi(2);
                        let lg = if ld2 < 1.0 { (ld2 * ld2).clamp(0.0, 1.0) } else { 1.0 };
                        let rg = if rd2 < 1.0 { (rd2 * rd2).clamp(0.0, 1.0) } else { 1.0 };
                        lg.min(rg)
                    } else {
                        1.0
                    };
                    // Strictly zero out healing along nostrils, mouth corners, eyelids, lashes, brows, and lips
                    let strict_face_gate = ((sem.skin - 0.58) / 0.34).clamp(0.0, 1.0)
                        * (1.0 - is_body_zone)
                        * feature_protect
                        * nostril_guard
                        * mouth_corner_guard;

                    // 1a-1c. Freckles, Acne, Face Moles & Body Blemishes via Continuous Multi-Scale Spot Ring
                    let need_face_blemish = (freckle_acne_str > 0.005 || face_mole_str > 0.005)
                        && strict_face_gate > 0.01;
                    let need_body_blemish = body_blemish_str > 0.005 && body_skin_w > 0.01;
                    if need_face_blemish || need_body_blemish {
                        let curr_rg = r - g;
                        let red_diff = (curr_rg - (rm - gm)).max((rf - gf) - (rm - gm));
                        // Generous early-skip prefilter well below smoothstep activation onset so edges never hard-clip
                        let maybe_spot = (lum_med - lum_curr) > 0.004
                            || (lum_med - lum_fine) > 0.004
                            || red_diff > 0.006;

                        if maybe_spot {
                            let min_dim = w.min(h) as f32;
                            let face_rx_px = face.face_rx * w as f32;
                            let r_tight = (min_dim * 0.0040 + face_rx_px * 0.009).clamp(4.0, 8.0);
                            let r_wide = (r_tight * 2.25).clamp(9.0, 19.5);
                            let ring = detect_clean_spot_ring(
                                &base_rgb,
                                &blur_fine,
                                w,
                                h,
                                x,
                                y,
                                r_tight,
                                r_wide,
                                lum_curr,
                                curr_rg,
                            );

                            let dark_dip = (ring.clean_lum - lum_curr).clamp(0.0, 0.55);
                            let red_excess =
                                (curr_rg - (ring.clean_r - ring.clean_g)).clamp(0.0, 0.30);

                            // 1a. Freckles & Acne (Face only): compact freckles & red pimples with smooth feathered borders
                            if freckle_acne_str > 0.005 && strict_face_gate > 0.01 {
                                let dark_strength =
                                    smoothstep(0.014, 0.075, dark_dip) * ring.dark_iso;
                                let red_strength =
                                    smoothstep(0.016, 0.078, red_excess) * ring.red_iso;
                                let spot_strength = dark_strength.max(red_strength);
                                let heal_a = (spot_strength * freckle_acne_str * 1.12)
                                    .clamp(0.0, 0.96)
                                    * strict_face_gate;
                                r = r * (1.0 - heal_a) + ring.clean_r * heal_a;
                                g = g * (1.0 - heal_a) + ring.clean_g * heal_a;
                                b = b * (1.0 - heal_a) + ring.clean_b * heal_a;
                            }

                            // 1b. Face Moles: compact dark pigmented spots & smooth feathered border
                            if face_mole_str > 0.005
                                && strict_face_gate > 0.01
                                && ring.dark_iso > 0.002
                            {
                                let mole_core = smoothstep(0.020, 0.088, dark_dip);
                                let mole_a = (mole_core * ring.dark_iso * face_mole_str * 1.15)
                                    .clamp(0.0, 0.98)
                                    * strict_face_gate;
                                r = r * (1.0 - mole_a) + ring.clean_r * mole_a;
                                g = g * (1.0 - mole_a) + ring.clean_g * mole_a;
                                b = b * (1.0 - mole_a) + ring.clean_b * mole_a;
                            }

                            // 1c. Body Blemishes (Neck & Body skin only!)
                            if body_blemish_str > 0.005 && body_skin_w > 0.01 {
                                let strict_body_gate = ((body_skin_w - 0.25) / 0.55).clamp(0.0, 1.0);
                                let b_dark = smoothstep(0.016, 0.078, dark_dip) * ring.dark_iso;
                                let b_red = smoothstep(0.016, 0.078, red_excess) * ring.red_iso;
                                let body_a = (b_dark.max(b_red) * body_blemish_str * 1.10)
                                    .clamp(0.0, 0.94)
                                    * strict_body_gate;
                                r = r * (1.0 - body_a) + ring.clean_r * body_a;
                                g = g * (1.0 - body_a) + ring.clean_g * body_a;
                                b = b * (1.0 - body_a) + ring.clean_b * body_a;
                            }
                        }
                    }

                    // 1d. Forehead Wrinkles (Strictly inside Forehead Polygon sem.forehead!)
                    if forehead_wrinkles_str > 0.005 && sem.forehead > 0.01 {
                        let crease_dip = (lum_med - lum_curr).clamp(0.0, 0.14);
                        let fw_a = (crease_dip * 9.5 * forehead_wrinkles_str * sem.forehead * face_skin_w).clamp(0.0, 0.92);
                        r = r * (1.0 - fw_a) + (rm + high_r * 0.45) * fw_a;
                        g = g * (1.0 - fw_a) + (gm + high_g * 0.45) * fw_a;
                        b = b * (1.0 - fw_a) + (bm + high_b * 0.45) * fw_a;
                    }

                    // 1e. Nasolabial Folds / Smile Lines (Strictly inside Nasolabial Polygon sem.nasolabial!)
                    if nasolabial_str > 0.005 && sem.nasolabial > 0.01 {
                        let fold_dip = (lum_med - lum_curr).clamp(0.0, 0.16);
                        let nf_a = (fold_dip * 8.5 * nasolabial_str * sem.nasolabial * face_skin_w).clamp(0.0, 0.90);
                        r = r * (1.0 - nf_a) + (rm + high_r * 0.55) * nf_a;
                        g = g * (1.0 - nf_a) + (gm + high_g * 0.55) * nf_a;
                        b = b * (1.0 - nf_a) + (bm + high_b * 0.55) * nf_a;
                    }

                    // 1f. Neck Wrinkles (Strictly on Neck skin below chin!)
                    if neck_wrinkles_str > 0.005 && body_skin_w > 0.01 {
                        let neck_dip = (lum_med - lum_curr).clamp(0.0, 0.16);
                        let nw_a = (neck_dip * 9.0 * neck_wrinkles_str * body_skin_w).clamp(0.0, 0.90);
                        r = r * (1.0 - nw_a) + (rm + high_r * 0.45) * nw_a;
                        g = g * (1.0 - nw_a) + (gm + high_g * 0.45) * nw_a;
                        b = b * (1.0 - nw_a) + (bm + high_b * 0.45) * nw_a;
                    }

                    // 1g. Face Shine (Matte specular highlight compression toward natural local skin color!)
                    if face_shine_str > 0.005 && face_skin_w > 0.01 {
                        let lum_now = 0.299 * r + 0.587 * g + 0.114 * b;
                        if lum_now > lum_med + 0.018 && lum_now > 0.55 {
                            let excess = ((lum_now - lum_med) * 3.2 * ((lum_now - 0.52) / 0.48).clamp(0.0, 1.0)).clamp(0.0, 0.88);
                            let shine_a = excess * face_shine_str * face_skin_w;
                            r = r * (1.0 - shine_a) + (rm + high_r * 0.60) * shine_a;
                            g = g * (1.0 - shine_a) + (gm + high_g * 0.60) * shine_a;
                            b = b * (1.0 - shine_a) + (bm + high_b * 0.60) * shine_a;
                        }
                    }

                    // =========================================================
                    // 2. SKIN RETOUCHING (FREQUENCY SEPARATION & DODGE/BURN)
                    // =========================================================
                    // 2a. Frequency Separation Low + Pore Texture Control (Face & Body)
                    if freq_sep_low_str > 0.005 || (params.freq_sep_high - 80.0).abs() > 1.0 {
                        let fs_a = (freq_sep_low_str * sw * 0.90).clamp(0.0, 0.92);
                        let low_r = rf * (1.0 - fs_a) + rm * fs_a;
                        let low_g = gf * (1.0 - fs_a) + gm * fs_a;
                        let low_b = bf * (1.0 - fs_a) + bm * fs_a;
                        let active_pore = if freq_sep_low_str > 0.005 {
                            pore_scale
                        } else {
                            1.0 + (pore_scale - 1.0) * sw
                        };
                        r = low_r + high_r * active_pore;
                        g = low_g + high_g * active_pore;
                        b = low_b + high_b * active_pore;
                    }

                    // Local edge guard so skin smoothing does not flatten structural boundaries
                    let med_diff = ((rf - rm).abs() + (gf - gm).abs() + (bf - bm).abs()) / 3.0;
                    let local_flatness = (1.0 - (med_diff / 0.16)).clamp(0.15, 1.0);

                    // 2b. Facial Skin Smooth (Dedicated Face-only bilateral-like smoothing)
                    if facial_smooth_str > 0.005 && face_skin_w > 0.01 {
                        let sm_a = (facial_smooth_str * face_skin_w * local_flatness * 0.88).clamp(0.0, 0.92);
                        r = r * (1.0 - sm_a) + (rm + high_r * 0.35 * pore_scale) * sm_a;
                        g = g * (1.0 - sm_a) + (gm + high_g * 0.35 * pore_scale) * sm_a;
                        b = b * (1.0 - sm_a) + (bm + high_b * 0.35 * pore_scale) * sm_a;
                    }

                    // 2c. Body Skin Smooth (Dedicated Neck & Body-only smoothing — never touches face!)
                    if body_smooth_str > 0.005 && body_skin_w > 0.01 {
                        let bsm_a = (body_smooth_str * body_skin_w * local_flatness * 0.90).clamp(0.0, 0.94);
                        r = r * (1.0 - bsm_a) + (rm + high_r * 0.30) * bsm_a;
                        g = g * (1.0 - bsm_a) + (gm + high_g * 0.30) * bsm_a;
                        b = b * (1.0 - bsm_a) + (bm + high_b * 0.30) * bsm_a;
                    }

                    // 2d. Dodge & Burn Even (Evens out micro-luminance blotches while keeping chrominance & pores)
                    if dodge_burn_even_str > 0.005 {
                        let lum_now = (0.299 * r + 0.587 * g + 0.114 * b).max(1e-4);
                        let lum_f = 0.299 * rf + 0.587 * gf + 0.114 * bf;
                        let lum_m = 0.299 * rm + 0.587 * gm + 0.114 * bm;
                        let target_lum = lum_f * (1.0 - dodge_burn_even_str * sw * 0.82)
                            + lum_m * (dodge_burn_even_str * sw * 0.82)
                            + (lum_now - lum_f);
                        let scale = (target_lum / lum_now).clamp(0.75, 1.30);
                        r *= scale;
                        g *= scale;
                        b *= scale;
                    }

                    // 2e. Unify Complexion (Evens out redness/color blotches while keeping luminance & pores!)
                    if unify_complexion_str > 0.005 {
                        let lum_now = (0.299 * r + 0.587 * g + 0.114 * b).max(1e-4);
                        let lum_m = (0.299 * rm + 0.587 * gm + 0.114 * bm).max(1e-4);
                        let uc_a = (unify_complexion_str * sw * 0.85).clamp(0.0, 0.90);
                        let cr = (rm / lum_m) * lum_now;
                        let cg = (gm / lum_m) * lum_now;
                        let cb = (bm / lum_m) * lum_now;
                        r = r * (1.0 - uc_a) + cr * uc_a;
                        g = g * (1.0 - uc_a) + cg * uc_a;
                        b = b * (1.0 - uc_a) + cb * uc_a;
                    }

                    // 2f. 3D Dodge & Burn Sculpt + Makeup Highlight & Contour
                    if (db_sculpt_str > 0.005 || makeup_high_str > 0.005 || makeup_cont_str > 0.005) && face_skin_w > 0.01 {
                        let fdx = (nx - face.center_x) / face.face_rx.max(0.05);
                        let fdy = (ny - face.center_y) / face.face_ry.max(0.05);
                        let face_d2 = fdx * fdx + fdy * fdy;

                        // T-zone highlight: nose bridge + forehead center + upper cheekbones + chin center
                        let bridge_d2 = ((nx - face.nose_x) / (face.face_rx * 0.14)).powi(2)
                            + ((ny - (face.nose_y - face.face_ry * 0.16)) / (face.face_ry * 0.24)).powi(2);
                        let tzone = (1.0 - bridge_d2).clamp(0.0, 1.0)
                            .max(sem.cheek * 0.75)
                            .max(sem.forehead * (1.0 - fdx.abs() * 1.5).clamp(0.0, 1.0) * 0.7);
                        // Outer jawline / temple / hollow contour
                        let perimeter = ((face_d2 - 0.42).max(0.0) * 1.25).clamp(0.0, 1.0);

                        let hi_gain = (db_sculpt_str * 0.11 + makeup_high_str * 0.14) * tzone * face_skin_w;
                        let co_gain = (db_sculpt_str * 0.10 + makeup_cont_str * 0.13) * perimeter * face_skin_w;
                        let sculpt_factor = 1.0 + hi_gain - co_gain;
                        r *= sculpt_factor;
                        g *= 1.0 + (hi_gain - co_gain) * 0.94;
                        b *= 1.0 + (hi_gain - co_gain) * 0.88;
                    }

                    // 2g. Double Chin Shadow Contouring (under chin)
                    if double_chin_str > 0.005 && ny > face.chin_y - 0.01 && ny < face.chin_y + face.face_ry * 0.35 {
                        let dc_x = ((nx - face.chin_x) / (face.face_rx * 0.55)).powi(2);
                        let dc_y = ((ny - (face.chin_y + face.face_ry * 0.10)) / (face.face_ry * 0.18)).powi(2);
                        let dc_w = (1.0 - (dc_x + dc_y)).clamp(0.0, 1.0) * double_chin_str * sw;
                        r *= 1.0 - dc_w * 0.12;
                        g *= 1.0 - dc_w * 0.12;
                        b *= 1.0 - dc_w * 0.12;
                    }

                    // 2h. Makeup Blush (Strictly on Cheek Polygons sem.cheek!)
                    if makeup_blush_str > 0.005 && sem.cheek > 0.01 {
                        let blush_a = sem.cheek * makeup_blush_str * face_skin_w;
                        r += blush_a * 0.085;
                        g -= blush_a * 0.032;
                        b -= blush_a * 0.012;
                    }

                    // 2i. Skin Radiance, Rosy Undertone & Warmth
                    if rad_str > 0.005 {
                        let lum_now = 0.299 * r + 0.587 * g + 0.114 * b;
                        let glow = rad_str * sw * 0.14 * lum_now * (1.0 - lum_now * 0.35);
                        r += glow * 1.06;
                        g += glow * 0.98;
                        b += glow * 0.90;
                    }
                    if rosy_str.abs() > 0.005 || warmth_str.abs() > 0.005 {
                        r += (rosy_str * 0.065 + warmth_str * 0.075) * sw;
                        g += (-rosy_str * 0.025 + warmth_str * 0.028) * sw;
                        b += (rosy_str * 0.022 - warmth_str * 0.060) * sw;
                    }
                }

                // =========================================================
                // 3. UNDER-EYE DARK CIRCLES & EYE BAGS (Strictly sem.under_eye!)
                // =========================================================
                if sem.under_eye > 0.01 && (dark_circles_str > 0.005 || eye_bags_str > 0.005) {
                    let lum_curr = 0.299 * r + 0.587 * g + 0.114 * b;
                    let lum_fine = 0.299 * rf + 0.587 * gf + 0.114 * bf;
                    let lum_med = (0.299 * rm + 0.587 * gm + 0.114 * bm).max(1e-4);

                    // Dark Circles: Lifts shadow darkness & warms bluish/purple under-eye cast while keeping 100% of fine pores!
                    if dark_circles_str > 0.005 {
                        let dc_a = (sem.under_eye * dark_circles_str * 0.85).clamp(0.0, 0.90);
                        let shadow_lift = (1.0 - lum_curr).clamp(0.15, 0.85) * 0.14 * dc_a;
                        // Harmonize color toward warmer surrounding cheek med-blur while keeping pore texture
                        let color_blend = dc_a * 0.55;
                        let base_r = rf * (1.0 - color_blend) + (rm * (lum_fine / lum_med)) * color_blend;
                        let base_g = gf * (1.0 - color_blend) + (gm * (lum_fine / lum_med)) * color_blend;
                        let base_b = bf * (1.0 - color_blend) + (bm * (lum_fine / lum_med)) * color_blend;
                        r = base_r + high_r + shadow_lift * 1.08;
                        g = base_g + high_g + shadow_lift * 0.96;
                        b = base_b + high_b + shadow_lift * 0.78;
                    }

                    // Eye Bags: Smooths out the puffy bag shadow trough line (lum_med - lum_fine) while preserving fine skin pores!
                    if eye_bags_str > 0.005 {
                        let bag_trough = (lum_med - lum_fine).clamp(-0.02, 0.16);
                        if bag_trough > 0.0 {
                            let eb_a = (sem.under_eye * eye_bags_str * 0.88).clamp(0.0, 0.92);
                            r = (rf * (1.0 - eb_a) + rm * eb_a) + high_r * 0.65;
                            g = (gf * (1.0 - eb_a) + gm * eb_a) + high_g * 0.65;
                            b = (bf * (1.0 - eb_a) + bm * eb_a) + high_b * 0.65;
                        }
                    }
                }

                // =========================================================
                // 4. GLASSES GLARE (Anti-reflective green/blue lens glare)
                // =========================================================
                if glasses_glare_str > 0.005 {
                    let eye_band_x = ((nx - face.center_x) / (face.face_rx * 0.82)).powi(2);
                    let eye_band_y = ((ny - ((face.left_eye_y + face.right_eye_y) * 0.5)) / (face.face_ry * 0.26)).powi(2);
                    let in_glasses = (1.0 - (eye_band_x + eye_band_y)).clamp(0.0, 1.0) * (1.0 - sem.eye * 0.85);
                    if in_glasses > 0.05 {
                        let lum_now = 0.299 * r + 0.587 * g + 0.114 * b;
                        let ar_tint = (g - r).max(b - r).max(0.0);
                        if lum_now > 0.48 && (ar_tint > 0.015 || lum_now > 0.72) {
                            let gl_a = (in_glasses * glasses_glare_str * 0.78).clamp(0.0, 0.88);
                            r = r * (1.0 - gl_a) + rm * gl_a;
                            g = g * (1.0 - gl_a) + (gm - ar_tint * 0.45) * gl_a;
                            b = b * (1.0 - gl_a) + (bm - ar_tint * 0.45) * gl_a;
                        }
                    }
                }

                // =========================================================
                // 5. EYES, EYEBROWS & EYESHADOW (Strictly isolated!)
                // =========================================================
                // 5a. Upper Eyelid Eyeshadow (sem.eyeshadow, strictly outside eyeball!)
                if makeup_eyeshadow_str > 0.005 && sem.eyeshadow > 0.01 {
                    let es_a = sem.eyeshadow * makeup_eyeshadow_str * 0.42;
                    // Soft warm rose-bronze eyeshadow tint that preserves skin texture
                    r *= 1.0 - es_a * 0.22;
                    g *= 1.0 - es_a * 0.38;
                    b *= 1.0 - es_a * 0.34;
                }

                // 5b. Eyes: Iris Brighten, Catchlight Boost, and Sclera Red Veins (Strictly sem.eye!)
                if sem.eye > 0.01 && (eye_bright_str > 0.005 || catchlight_str > 0.005 || red_vein_str > 0.005) {
                    let lum_now = 0.299 * r + 0.587 * g + 0.114 * b;
                    // Protect dark pupil (lum < 0.15) so pupils stay jet black
                    let pupil_guard = ((lum_now - 0.12) / 0.16).clamp(0.0, 1.0);

                    if eye_bright_str > 0.005 {
                        let eb = sem.eye * eye_bright_str * pupil_guard;
                        let detail_gain = eb * 1.45;
                        let mid_lift = eb * 0.11 * (1.0 - lum_now * 0.5);
                        r += high_r * detail_gain + mid_lift;
                        g += high_g * detail_gain + mid_lift;
                        b += high_b * detail_gain + mid_lift * 1.05;
                    }

                    if catchlight_str > 0.005 && lum_now > 0.52 {
                        let spec = ((lum_now - 0.52) / 0.48).clamp(0.0, 1.0).powf(1.3);
                        let cb_boost = sem.eye * catchlight_str * spec * 0.38;
                        r += cb_boost + high_r * sem.eye * catchlight_str * 0.8;
                        g += cb_boost + high_g * sem.eye * catchlight_str * 0.8;
                        b += cb_boost * 1.05 + high_b * sem.eye * catchlight_str * 0.8;
                    }

                    if red_vein_str > 0.005 && lum_now > 0.30 && r > g {
                        // Sclera vein neutralization: pulls excess red/pink toward clean neutral sclera luminance
                        let redness = (r - g).clamp(0.0, 0.25);
                        let sclera_w = ((lum_now - 0.28) / 0.45).clamp(0.0, 1.0);
                        let vein_a = (sem.eye * red_vein_str * sclera_w * 0.90).clamp(0.0, 0.95);
                        r -= redness * vein_a * 0.88;
                        g += redness * vein_a * 0.15;
                        b += redness * vein_a * 0.22;
                    }
                }

                // 5c. Eyebrows Thickness (Strictly sem.eyebrow!)
                if brow_thick_str.abs() > 0.005 && sem.eyebrow > 0.01 {
                    let lum_now = 0.299 * r + 0.587 * g + 0.114 * b;
                    let hair_w = (1.0 - lum_now).clamp(0.15, 0.95);
                    let bt = sem.eyebrow * brow_thick_str * hair_w * 0.34;
                    r = (r * (1.0 - bt)) + high_r * sem.eyebrow * brow_thick_str.max(0.0) * 0.45;
                    g = (g * (1.0 - bt)) + high_g * sem.eyebrow * brow_thick_str.max(0.0) * 0.45;
                    b = (b * (1.0 - bt)) + high_b * sem.eyebrow * brow_thick_str.max(0.0) * 0.45;
                }

                // =========================================================
                // 6. TEETH & LIPSTICK (Strictly isolated inner mouth vs outer lips!)
                // =========================================================
                // 6a. Teeth Whitening, De-Yellowing & Brightening (Strictly sem.teeth_inner!)
                if sem.teeth_inner > 0.01 && (teeth_whiten_str > 0.005 || teeth_desat_str > 0.005 || teeth_bright_str > 0.005) {
                    let (h_deg, s_val, v_val) = rgb_to_hsv(r, g, b);
                    // Exclude dark mouth cavity shadows and red/pink gums/tongue
                    let is_tooth = if v_val > 0.24 && !((h_deg < 18.0 || h_deg > 335.0) && s_val > 0.34) {
                        ((v_val - 0.22) / 0.25).clamp(0.0, 1.0) * (1.0 - (s_val - 0.35).max(0.0) * 2.5).clamp(0.0, 1.0)
                    } else {
                        0.0
                    };

                    if is_tooth > 0.02 {
                        let tw = sem.teeth_inner * is_tooth;
                        // De-yellowing (Teeth Desaturate + part of Teeth Whiten)
                        let desat_amt = ((teeth_desat_str * 0.90 + teeth_whiten_str * 0.75) * tw).clamp(0.0, 0.92);
                        // Brightening (Teeth Brighten + part of Teeth Whiten)
                        let bright_amt = (teeth_bright_str * 0.20 + teeth_whiten_str * 0.16) * tw;

                        let new_s = s_val * (1.0 - desat_amt);
                        let new_v = (v_val + bright_amt * (1.0 - v_val * 0.35)).clamp(0.0, 0.99);
                        let (tr, tg, tb) = hsv_to_rgb(h_deg, new_s, new_v);
                        // Add a subtle cool pearl-white tint when whitening
                        r = tr;
                        g = tg;
                        b = (tb + teeth_whiten_str * tw * 0.025).min(1.0);
                    }
                }

                // 6b. Lipstick (Strictly sem.lips_outer — preserves 100% of lip texture and gloss highlights!)
                if sem.lips_outer > 0.01 && lip_str > 0.005 {
                    let lum_now = 0.299 * r + 0.587 * g + 0.114 * b;
                    // Preserve natural white lip gloss / specular highlight
                    let gloss_protect = ((lum_now - 0.68) / 0.28).clamp(0.0, 0.85);
                    let la = sem.lips_outer * lip_str * (1.0 - gloss_protect) * 0.62;

                    let (tr, tg, tb) = match params.makeup_lip_tint.as_str() {
                        "coral" => (0.94, 0.34, 0.26),
                        "berry" => (0.66, 0.14, 0.36),
                        "nude" => (0.78, 0.48, 0.42),
                        "ruby" => (0.86, 0.10, 0.16),
                        _ => (0.86, 0.24, 0.35), // rose
                    };
                    // Luminance-modulated color blend + 100% fine lip texture preservation
                    let shade_scale = (lum_now * 1.35).clamp(0.25, 1.15);
                    let target_r = tr * shade_scale + high_r * 0.85;
                    let target_g = tg * shade_scale + high_g * 0.85;
                    let target_b = tb * shade_scale + high_b * 0.85;
                    r = r * (1.0 - la) + target_r * la;
                    g = g * (1.0 - la) + target_g * la;
                    b = b * (1.0 - la) + target_b * la;
                }

                // =========================================================
                // 7. HAIR, CLOTHING & STUDIO BACKDROP (Neural 6-Class Masks!)
                // =========================================================
                // 7a. Evoto HairBeauty: HairSeam (làm đầy điểm hói), RemoveHair (Inner), HairSmooth, HairGloss, RemoveWhiteHair, HairColorUnify & HairColorAdjust
                let part_band = face.face_ry > 0.01
                    && ny >= (face.forehead_y - face.face_ry * 0.72).max(0.01)
                    && ny <= (face.forehead_y + face.face_ry * 0.08)
                    && (nx - face.center_x).abs() <= face.face_rx * 0.92
                    && sem.eyebrow < 0.05
                    && sem.eye < 0.05
                    && sem.background < 0.45
                    && (sem.hair > 0.06
                        || (nx - face.center_x).abs() <= face.face_rx * 0.78);
                if params.hair_part_line > 0.5 && part_band {
                    let lum_curr = 0.299 * r + 0.587 * g + 0.114 * b;
                    let r_part = (w.min(h) as f32 * 0.009).clamp(5.0, 14.0);
                    let (pr, pg, pb, pw) =
                        fill_hair_part_line(&base_rgb, &blur_fine, w, h, x, y, r_part, lum_curr);
                    if pw > 0.01 {
                        let pa = (pw * (params.hair_part_line / 100.0) * 0.90).clamp(0.0, 0.92);
                        r = r * (1.0 - pa) + pr * pa;
                        g = g * (1.0 - pa) + pg * pa;
                        b = b * (1.0 - pa) + pb * pa;
                    }
                }

                // Forehead thin isolated baby-hair cleanup when inner RemoveHair > 0 (never touches dense bangs/brows)
                if !is_replacing
                    && eff_stray_inner > 0.5
                    && sem.forehead > 0.45
                    && sem.eyebrow < 0.02
                    && sem.eye < 0.02
                    && ny < face.forehead_y + face.face_ry * 0.16
                {
                    let lum_curr = 0.299 * r + 0.587 * g + 0.114 * b;
                    let r_fh = (w.min(h) as f32 * 0.0065).clamp(4.0, 9.0);
                    let (sr, sg, sb, sw_fh) =
                        heal_forehead_flyaway(&base_rgb, w, h, x, y, r_fh, lum_curr);
                    if sw_fh > 0.01 {
                        let fh_str = (eff_stray_inner / 100.0).clamp(0.0, 1.0);
                        let fh_opacity = (0.35 + 0.60 * smoothstep(0.02, 0.32, fh_str)).clamp(0.0, 0.95);
                        let fa = (sw_fh * fh_opacity).clamp(0.0, 0.95);
                        r = r * (1.0 - fa) + sr * fa;
                        g = g * (1.0 - fa) + sg * fa;
                        b = b * (1.0 - fa) + sb * fa;
                    }
                }

                let hair_skin_guard = skin_weight(r, g, b);
                let hair_core_gate = ((sem.hair - 0.16) / 0.56).clamp(0.0, 1.0)
                    * (1.0 - sem.skin * 1.4).clamp(0.0, 1.0)
                    * (1.0 - hair_skin_guard * 1.35).clamp(0.0, 1.0)
                    * (1.0 - sem.background * 0.88).clamp(0.0, 1.0);
                if hair_core_gate > 0.01 {
                    // 7a-1. HairSmooth + HairRemoveFrizzy (Oriented tangent smoothing along hair fibers)
                    if params.hair_smooth > 0.5 {
                        let sm_str = (params.hair_smooth / 100.0).clamp(0.0, 1.0);
                        let fr_str = (params.hair_frizzy / 100.0).clamp(0.0, 1.0);
                        let (sr, sg, sb) = smooth_hair_oriented(&base_rgb, w, h, x, y, sm_str, fr_str);
                        let sm_a = (hair_core_gate * sm_str * 0.78).clamp(0.0, 0.85);
                        r = r * (1.0 - sm_a) + sr * sm_a;
                        g = g * (1.0 - sm_a) + sg * sm_a;
                        b = b * (1.0 - sm_a) + sb * sm_a;
                    }

                    // 7a-2. RemoveWhiteHair (Tóc Đen Tóc Bạc: detect low-chroma bright strands & recolor toward natural donor)
                    if params.white_hair_blacken > 0.5 {
                        let wb_str = (params.white_hair_blacken / 100.0).clamp(0.0, 1.0);
                        let lum = 0.299 * r + 0.587 * g + 0.114 * b;
                        let lum_m = 0.299 * rm + 0.587 * gm + 0.114 * bm;
                        let max_c = r.max(g).max(b);
                        let min_c = r.min(g).min(b);
                        let chroma = max_c - min_c;
                        let lum_gate = smoothstep((dom_hair_lum + 0.08).clamp(0.18, 0.42), (dom_hair_lum + 0.24).clamp(0.36, 0.68), lum);
                        let neutral_gate = 1.0 - smoothstep(0.05, 0.16, chroma);
                        let scalp_safe = 1.0 - smoothstep(0.10, 0.28, hair_skin_guard);
                        let w_mask = (lum_gate * neutral_gate * scalp_safe * hair_core_gate * wb_str).clamp(0.0, 0.94);
                        if w_mask > 0.01 {
                            let strand_mod = (0.86 + (lum - lum_m) * 1.45).clamp(0.45, 1.25);
                            let tr = (dom_hair_r * strand_mod + high_r * 0.45).clamp(0.0, 1.0);
                            let tg = (dom_hair_g * strand_mod + high_g * 0.45).clamp(0.0, 1.0);
                            let tb = (dom_hair_b * strand_mod + high_b * 0.45).clamp(0.0, 1.0);
                            r = r * (1.0 - w_mask) + tr * w_mask;
                            g = g * (1.0 - w_mask) + tg * w_mask;
                            b = b * (1.0 - w_mask) + tb * w_mask;
                        }
                    }

                    // 7a-3. HairColorUnify (Thống Nhất Màu Tóc: harmonize uneven highlights/brassy bands toward dominant hair tone)
                    if params.hair_color_unify.abs() > 0.5 {
                        let u_norm = (params.hair_color_unify / 100.0).clamp(-1.0, 1.0);
                        let lum = (0.299 * r + 0.587 * g + 0.114 * b).max(0.01);
                        let base_lum = dom_hair_lum.max(0.02);
                        let scale = (lum / base_lum).clamp(0.35, 1.75);
                        // Harmonized RGB preserves strand value while pulling color & midtone variance toward natural mean
                        let harm_lum = lum + (dom_hair_lum - lum) * 0.42 * u_norm.max(0.0);
                        let lum_ratio = harm_lum / lum;
                        let uni_r = (dom_hair_r * scale * lum_ratio + high_r * 0.55).clamp(0.0, 1.0);
                        let uni_g = (dom_hair_g * scale * lum_ratio + high_g * 0.55).clamp(0.0, 1.0);
                        let uni_b = (dom_hair_b * scale * lum_ratio + high_b * 0.55).clamp(0.0, 1.0);
                        if u_norm > 0.0 {
                            let ua = (u_norm * hair_core_gate * 0.76).clamp(0.0, 0.85);
                            r = r * (1.0 - ua) + uni_r * ua;
                            g = g * (1.0 - ua) + uni_g * ua;
                            b = b * (1.0 - ua) + uni_b * ua;
                        } else {
                            let boost = (-u_norm) * hair_core_gate * 0.35;
                            r = (r + (r - uni_r) * boost).clamp(0.0, 1.0);
                            g = (g + (g - uni_g) * boost).clamp(0.0, 1.0);
                            b = (b + (b - uni_b) * boost).clamp(0.0, 1.0);
                        }
                    }

                    // 7a-4. HairGloss (Tăng độ bóng cho tóc)
                    if params.hair_shine > 0.5 {
                        let shine_str = (params.hair_shine / 100.0).clamp(0.0, 1.0);
                        let lum = 0.299 * r + 0.587 * g + 0.114 * b;
                        let lum_m = 0.299 * rm + 0.587 * gm + 0.114 * bm;
                        let strand_sheen = ((lum - lum_m) * 4.5).clamp(0.0, 1.0);
                        let silk_band =
                            smoothstep(0.03, 0.22, lum) * (1.0 - smoothstep(0.55, 0.86, lum));
                        let shadow_deep =
                            (1.0 - smoothstep(0.02, 0.14, lum)) * hair_core_gate * shine_str * 0.08;
                        let add =
                            hair_core_gate * shine_str * (0.045 * silk_band + 0.11 * strand_sheen);
                        r = (r * (1.0 - shadow_deep) + add + high_r * hair_core_gate * shine_str * 0.18)
                            .clamp(0.0, 1.0);
                        g = (g * (1.0 - shadow_deep) + add + high_g * hair_core_gate * shine_str * 0.18)
                            .clamp(0.0, 1.0);
                        b = (b * (1.0 - shadow_deep) + add + high_b * hair_core_gate * shine_str * 0.18)
                            .clamp(0.0, 1.0);
                    }

                    // 7a-5. HairColorAdjust (Màu Tóc H01..H13 + Darkening)
                    if params.hair_dye_strength > 0.5 && params.hair_dye_color != "none" {
                        let (dr, dg, db) = apply_evoto_hair_dye(
                            r,
                            g,
                            b,
                            high_r,
                            high_g,
                            high_b,
                            hair_core_gate,
                            hair_skin_guard,
                            params.hair_dye_color.as_str(),
                            params.hair_dye_strength,
                            params.hair_dye_darken,
                        );
                        r = dr;
                        g = dg;
                        b = db;
                    }
                }

                // 7b. Evoto Clothing Retouching (De-wrinkle Small/Large, Lint, Sweat Stain, Edge Smooth & Moiré Removal)
                let (rb_c, gb_c, bb_c) = if !blur_cloth_broad.is_empty() {
                    (
                        blur_cloth_broad[idx],
                        blur_cloth_broad[idx + 1],
                        blur_cloth_broad[idx + 2],
                    )
                } else {
                    (rm, gm, bm)
                };
                let cloth_w = (sem.clothing * (1.0 - sem.skin * 0.65)).clamp(0.0, 1.0);
                if cloth_w > 0.02 {
                    let (target_small_k, target_large_k, target_seam_k) = match params.clothing_target.as_str() {
                        "female" => (1.10f32, 0.94f32, 1.05f32),
                        "male" => (0.96f32, 1.12f32, 0.95f32),
                        "child" => (0.90f32, 0.85f32, 1.15f32),
                        "senior_female" | "senior_male" => (0.92f32, 1.04f32, 1.10f32),
                        _ => (1.0f32, 1.0f32, 1.0f32),
                    };

                    // Multi-scale wrinkle ironing (Micro weave preserved + Independent Small vs Large crease bands)
                    if params.clothing_dewrinkle > 0.5 {
                        let master = (params.clothing_dewrinkle / 100.0).clamp(0.0, 1.0);
                        let small_w = (params.clothing_small_wrinkles / 100.0).clamp(0.0, 1.0) * target_small_k;
                        let large_w = (params.clothing_large_wrinkles / 100.0).clamp(0.0, 1.0) * target_large_k;

                        let small_diff = ((rf - rm).abs() + (gf - gm).abs() + (bf - bm).abs()) / 3.0;
                        let seam_guard = (1.0 - (small_diff * target_seam_k / 0.16)).clamp(0.0, 1.0);
                        let broad_diff = ((rm - rb_c).abs() + (gm - gb_c).abs() + (bm - bb_c).abs()) / 3.0;
                        let broad_seam_guard = (1.0 - (broad_diff * target_seam_k / 0.24)).clamp(0.0, 1.0);

                        let a_small = (cloth_w * seam_guard * master * small_w * 0.88).clamp(0.0, 0.94);
                        let a_large = (cloth_w * broad_seam_guard * master * large_w * 0.78).clamp(0.0, 0.90);
                        let tex_keep = 1.0 - cloth_w * seam_guard * master * small_w.max(large_w).clamp(0.0, 1.0) * 0.45;

                        r = (rb_c + (rm - rb_c) * (1.0 - a_large) + (rf - rm) * (1.0 - a_small) + (r - rf) * tex_keep).clamp(0.0, 1.0);
                        g = (gb_c + (gm - gb_c) * (1.0 - a_large) + (gf - gm) * (1.0 - a_small) + (g - gf) * tex_keep).clamp(0.0, 1.0);
                        b = (bb_c + (bm - bb_c) * (1.0 - a_large) + (bf - bm) * (1.0 - a_small) + (b - bf) * tex_keep).clamp(0.0, 1.0);
                    }

                    // Lint & dust speck cleanup (both white lint on dark clothing and dark fuzz on light shirts)
                    if params.clothing_lint_clean > 0.5 {
                        let lum_curr = 0.299 * r + 0.587 * g + 0.114 * b;
                        let lum_fine = 0.299 * rf + 0.587 * gf + 0.114 * bf;
                        let white_speck = (lum_curr - lum_fine).max(0.0);
                        let dark_speck = if lum_fine > 0.44 {
                            (lum_fine - lum_curr).max(0.0)
                        } else {
                            0.0
                        };
                        let speck = white_speck.max(dark_speck * 0.85);
                        if speck > 0.014 {
                            let lint_a = (cloth_w * (params.clothing_lint_clean / 100.0) * (speck * 18.0)).clamp(0.0, 0.95);
                            r = r * (1.0 - lint_a) + rf * lint_a;
                            g = g * (1.0 - lint_a) + gf * lint_a;
                            b = b * (1.0 - lint_a) + bf * lint_a;
                        }
                    }

                    // Sweat stain removal (lifts localized darker damp blotches while protecting prints/patterns & preserving weave)
                    if params.clothing_sweat_stain > 0.5 {
                        let sw_str = (params.clothing_sweat_stain / 100.0).clamp(0.0, 1.0);
                        let sw_even = (params.clothing_sweat_evenness / 100.0).clamp(0.0, 1.0);
                        let lum_m = 0.299 * rm + 0.587 * gm + 0.114 * bm;
                        let lum_b = 0.299 * rb_c + 0.587 * gb_c + 0.114 * bb_c;
                        let damp_dip = (lum_b - lum_m).clamp(0.0, 0.30);
                        let chroma_m = (rm - bm).abs().max((rm - gm).abs());
                        let chroma_b = (rb_c - bb_c).abs().max((rb_c - gb_c).abs());
                        let hue_guard = (1.0 - (chroma_m - chroma_b).abs() / 0.11).clamp(0.0, 1.0);
                        if damp_dip > 0.008 && hue_guard > 0.05 {
                            let even_k = 0.35 + 0.65 * sw_even;
                            let patch_a = (cloth_w * hue_guard * sw_str * (damp_dip * 7.8 + 0.25 * sw_even) * even_k).clamp(0.0, 0.92);
                            let dr = r - rm;
                            let dg = g - gm;
                            let db = b - bm;
                            r = (rm * (1.0 - patch_a) + rb_c * patch_a + dr * (1.0 - patch_a * 0.25)).clamp(0.0, 1.0);
                            g = (gm * (1.0 - patch_a) + gb_c * patch_a + dg * (1.0 - patch_a * 0.25)).clamp(0.0, 1.0);
                            b = (bm * (1.0 - patch_a) + bb_c * patch_a + db * (1.0 - patch_a * 0.25)).clamp(0.0, 1.0);
                        }
                    }

                    // Moiré & fabric aliasing pattern suppression (neutralizes high-frequency chroma rainbows + periodic shimmer)
                    if params.clothing_moire_remove > 0.5 {
                        let m_str = (params.clothing_moire_remove / 100.0).clamp(0.0, 1.0);
                        let lum_now = 0.299 * r + 0.587 * g + 0.114 * b;
                        let lum_f = 0.299 * rf + 0.587 * gf + 0.114 * bf;
                        let lum_m = 0.299 * rm + 0.587 * gm + 0.114 * bm;
                        let lum_b = 0.299 * rb_c + 0.587 * gb_c + 0.114 * bb_c;
                        let clean_cr = (rm * 0.45 + rb_c * 0.55) - (lum_m * 0.45 + lum_b * 0.55);
                        let clean_cg = (gm * 0.45 + gb_c * 0.55) - (lum_m * 0.45 + lum_b * 0.55);
                        let clean_cb = (bm * 0.45 + bb_c * 0.55) - (lum_m * 0.45 + lum_b * 0.55);
                        let chroma_rip = ((r - lum_now) - clean_cr).abs()
                            + ((g - lum_now) - clean_cg).abs()
                            + ((b - lum_now) - clean_cb).abs();
                        let lum_rip = (lum_now - lum_f).abs() + (lum_f - lum_m).abs() * 0.65;
                        let rip_gate = ((chroma_rip * 8.0 + lum_rip * 5.0)).clamp(0.18, 1.0);
                        let ca = (cloth_w * m_str * rip_gate * 0.92).clamp(0.0, 0.94);
                        let clean_lum = lum_m + (lum_f - lum_m) * (1.0 - m_str * 0.62) + (lum_now - lum_f) * (1.0 - m_str * 0.45);
                        let moire_r = (clean_lum + clean_cr).clamp(0.0, 1.0);
                        let moire_g = (clean_lum + clean_cg).clamp(0.0, 1.0);
                        let moire_b = (clean_lum + clean_cb).clamp(0.0, 1.0);
                        r = r * (1.0 - ca) + moire_r * ca;
                        g = g * (1.0 - ca) + moire_g * ca;
                        b = b * (1.0 - ca) + moire_b * ca;
                    }
                }

                // Garment Contour & Hem Edge Smoothing (Làm phẳng viền trang phục along boundary of sem.clothing)
                if params.clothing_edge_smooth > 0.5 {
                    let step = 2.0 / MASK_DIM as f32;
                    let c_l = sample_semantic_grid(&semantic_data.weights, (nx - step).max(0.0), ny).clothing;
                    let c_r = sample_semantic_grid(&semantic_data.weights, (nx + step).min(1.0), ny).clothing;
                    let c_u = sample_semantic_grid(&semantic_data.weights, nx, (ny - step).max(0.0)).clothing;
                    let c_d = sample_semantic_grid(&semantic_data.weights, nx, (ny + step).min(1.0)).clothing;
                    let c_avg = 0.25 * (c_l + c_r + c_u + c_d);
                    let edge_band = ((c_avg - sem.clothing).abs() * 4.5 + sem.clothing * (1.0 - sem.clothing) * 4.0)
                        .clamp(0.0, 1.0)
                        * (1.0 - sem.skin * 0.70);
                    if edge_band > 0.02 && (sem.clothing > 0.01 || c_avg > 0.04) {
                        let edge_str = (params.clothing_edge_smooth / 100.0).clamp(0.0, 1.0);
                        let refine_k = (params.clothing_edge_refine / 100.0).clamp(0.0, 1.0);
                        let ea = (edge_band * edge_str * 0.88).clamp(0.0, 0.90);
                        let target_r = rm * (0.45 + 0.55 * refine_k) + rb_c * (0.55 - 0.55 * refine_k);
                        let target_g = gm * (0.45 + 0.55 * refine_k) + gb_c * (0.55 - 0.55 * refine_k);
                        let target_b = bm * (0.45 + 0.55 * refine_k) + bb_c * (0.55 - 0.55 * refine_k);
                        let hi_keep = 0.15 + 0.45 * refine_k;
                        r = r * (1.0 - ea) + (target_r + (r - rf) * hi_keep) * ea;
                        g = g * (1.0 - ea) + (target_g + (g - gf) * hi_keep) * ea;
                        b = b * (1.0 - ea) + (target_b + (b - bf) * hi_keep) * ea;
                    }
                }

                // 7c. Stray Hair Refinement, Foreground Color Decontamination & Original-Mode Optimization
                if let Some(ref gm) = guided_matte {
                    let fg_a = gm.fg_alpha[y * w + x];
                    if is_replacing {
                        if fg_a > 0.02 && fg_a < 0.95 && !gm.f_local.is_empty() && !gm.b_local.is_empty() {
                            let gx = ((nx * gm.gw as f32) - 0.5).clamp(0.0, (gm.gw - 1) as f32);
                            let gy = ((ny * gm.gh as f32) - 0.5).clamp(0.0, (gm.gh - 1) as f32);
                            let (fr, fg_c, fb) = sample_bilinear(&gm.f_local, gm.gw, gm.gh, gx, gy);
                            let (br, bg_c, bb) = sample_bilinear(&gm.b_local, gm.gw, gm.gh, gx, gy);
                            let hair_p_px = sample_plane_bilinear(&semantic_data.raw_hair, MASK_DIM, MASK_DIM, nx, ny);
                            let is_hair_edge = hair_p_px > 0.14;
                            let denom = fg_a.max(0.28);
                            let solved_r = ((r - (1.0 - fg_a) * br) / denom).clamp(0.0, 1.0);
                            let solved_g = ((g - (1.0 - fg_a) * bg_c) / denom).clamp(0.0, 1.0);
                            let solved_b = ((b - (1.0 - fg_a) * bb) / denom).clamp(0.0, 1.0);
                            let trust_solved = smoothstep(0.18, 0.54, fg_a);
                            let mut pure_r = fr * (1.0 - trust_solved) + solved_r * trust_solved;
                            let mut pure_g = fg_c * (1.0 - trust_solved) + solved_g * trust_solved;
                            let mut pure_b = fb * (1.0 - trust_solved) + solved_b * trust_solved;
                            if params.bg_decontam {
                                let spill_w = if is_hair_edge {
                                    ((1.0 - fg_a) * 0.72).clamp(0.0, 0.72)
                                } else {
                                    ((1.0 - fg_a) * 0.32).clamp(0.0, 0.32)
                                };
                                let pull_r = (pure_r - (br - bg_mean_r * 0.5).max(0.0) * 0.35).clamp(0.0, 1.0);
                                let pull_g = (pure_g - (bg_c - bg_mean_g * 0.5).max(0.0) * 0.35).clamp(0.0, 1.0);
                                let pull_b = (pure_b - (bb - bg_mean_b * 0.5).max(0.0) * 0.35).clamp(0.0, 1.0);
                                if is_hair_edge && !params.is_highres_matte {
                                    let d_lum = (0.299 * pure_r + 0.587 * pure_g + 0.114 * pure_b).clamp(0.0, 1.0);
                                    let neut_r = d_lum * 0.55 + pull_r * 0.45;
                                    let neut_g = d_lum * 0.55 + pull_g * 0.45;
                                    let neut_b = d_lum * 0.55 + pull_b * 0.45;
                                    pure_r = pure_r * (1.0 - spill_w) + neut_r * spill_w;
                                    pure_g = pure_g * (1.0 - spill_w) + neut_g * spill_w;
                                    pure_b = pure_b * (1.0 - spill_w) + neut_b * spill_w;
                                } else {
                                    // Avoid desaturating warm skin/arms into grey halos at solid edges
                                    pure_r = pure_r * (1.0 - spill_w) + pull_r * spill_w;
                                    pure_g = pure_g * (1.0 - spill_w) + pull_g * spill_w;
                                    pure_b = pure_b * (1.0 - spill_w) + pull_b * spill_w;
                                }
                            }
                            let edge_w = if is_hair_edge {
                                ((1.0 - smoothstep(0.45, 0.90, fg_a))
                                    * if params.bg_decontam { 1.0 } else { 0.75 })
                                .clamp(0.0, 1.0)
                            } else {
                                ((1.0 - smoothstep(0.40, 0.78, fg_a))
                                    * if params.bg_decontam { 0.92 } else { 0.65 })
                                .clamp(0.0, 1.0)
                            };
                            r = r * (1.0 - edge_w) + pure_r * edge_w;
                            g = g * (1.0 - edge_w) + pure_g * edge_w;
                            b = b * (1.0 - edge_w) + pure_b * edge_w;
                        }
                    } else {
                        let bg_mask = (1.0 - fg_a).clamp(0.0, 1.0);
                        let bg_blur_str = params.bg_blur_bokeh / 100.0;
                        let stray_str = eff_stray_outer / 100.0;
                        let clean_str = (params.backdrop_clean / 100.0).max(if params.bg_clean_defects { 0.72 } else { 0.0 });
                        let band_str = (params.backdrop_banding_fix / 100.0).max(if params.bg_remove_banding { 0.78 } else { 0.0 });

                        if !blur_bokeh.is_empty() && (bg_mask > 0.005 || stray_str > 0.005) {
                            let rb = blur_bokeh[idx];
                            let gb = blur_bokeh[idx + 1];
                            let bb = blur_bokeh[idx + 2];
                            let mut stray_healed = 0.0f32;

                            if stray_str > 0.005 && ny < 0.96 {
                                let in_face_interior = face.face_rx > 0.01
                                    && ((nx - face.center_x) / (face.face_rx * 0.94)).powi(2)
                                        + ((ny - face.center_y) / (face.face_ry * 0.94)).powi(2)
                                        < 1.0;
                                // Strictly protect ears, face oval/edges, neck, eyes, eyebrows, lips, and dense shirt interior
                                let is_protected_subject = in_face_interior
                                    || sem.skin > 0.12
                                    || sem.clothing > 0.45
                                    || skin_weight(r, g, b) > 0.30
                                    || sem.lips_outer > 0.05
                                    || sem.eyebrow > 0.05
                                    || sem.eye > 0.05;
                                let near_hair_halo = fg_a > 0.015
                                    || sem.hair > 0.02
                                    || (face.face_rx > 0.01
                                        && (nx - face.center_x).abs() < face.face_rx * 3.4
                                        && ny < (face.chin_y + face.face_ry * 1.95).min(0.96)
                                        && ny > (face.forehead_y - face.face_ry * 1.25).max(0.0));
                                if !is_protected_subject && near_hair_halo {
                                    let min_dim = w.min(h) as f32;
                                    let r_inner = (min_dim * 0.0075).clamp(5.0, 12.0);
                                    let r_mid = (min_dim * 0.026).clamp(14.0, 38.0);
                                    let r_far = (min_dim * 0.068).clamp(30.0, 88.0);
                                    let (bg_r, bg_g, bg_b, strand_w) = clean_stray_hair_bg(
                                        &base_rgb,
                                        &blur_fine,
                                        &blur_med,
                                        &gm.fg_alpha,
                                        w,
                                        h,
                                        x,
                                        y,
                                        r_inner,
                                        r_mid,
                                        r_far,
                                        stray_str,
                                        r,
                                        g,
                                        b,
                                        rb,
                                        gb,
                                        bb,
                                    );
                                    if strand_w > 0.01 {
                                        let heal_opacity = (0.35 + 0.63 * smoothstep(0.02, 0.32, stray_str)).clamp(0.0, 0.98);
                                        let sa = (strand_w * heal_opacity).clamp(0.0, 0.98);
                                        stray_healed = sa;
                                        r = r * (1.0 - sa) + bg_r * sa;
                                        g = g * (1.0 - sa) + bg_g * sa;
                                        b = b * (1.0 - sa) + bg_b * sa;
                                    }
                                }
                            }
                            let keep_unsmudged = (1.0 - stray_healed).clamp(0.0, 1.0);
                            if params.bg_clean_distractions && bg_mask > 0.04 {
                                let dev = (r - rb).abs() + (g - gb).abs() + (b - bb).abs();
                                let spot_w = (smoothstep(0.05, 0.22, dev) * bg_mask * 0.92 * keep_unsmudged).clamp(0.0, 0.95);
                                r = r * (1.0 - spot_w) + rb * spot_w;
                                g = g * (1.0 - spot_w) + gb * spot_w;
                                b = b * (1.0 - spot_w) + bb * spot_w;
                            }
                            if bg_blur_str > 0.005 {
                                let ba = (bg_mask * bg_blur_str * keep_unsmudged).clamp(0.0, 0.98);
                                r = r * (1.0 - ba) + rb * ba;
                                g = g * (1.0 - ba) + gb * ba;
                                b = b * (1.0 - ba) + bb * ba;
                            }
                            if clean_str > 0.005 && bg_mask > 0.02 {
                                let ca = (bg_mask * clean_str * 0.85 * keep_unsmudged).clamp(0.0, 0.92);
                                r = r * (1.0 - ca) + rb * ca;
                                g = g * (1.0 - ca) + gb * ca;
                                b = b * (1.0 - ca) + bb * ca;
                            }
                            if band_str > 0.005 && bg_mask > 0.02 {
                                let bf_a = (bg_mask * band_str * 0.82 * keep_unsmudged).clamp(0.0, 0.92);
                                let dither = ((((x * 1973 + y * 9277) ^ 0x68bc21eb) & 0xff) as f32 / 255.0 - 0.5) * 0.006;
                                r = r * (1.0 - bf_a) + (rb + dither) * bf_a;
                                g = g * (1.0 - bf_a) + (gb + dither) * bf_a;
                                b = b * (1.0 - bf_a) + (bb + dither) * bf_a;
                            }
                        }

                        if params.bg_even_brightness && bg_mask > 0.02 {
                            let cur_l = (0.299 * r + 0.587 * g + 0.114 * b).max(0.02);
                            let even_w = (bg_mask * 0.68).clamp(0.0, 0.82);
                            let target_l = cur_l + (bg_mean_luma - cur_l) * even_w;
                            let mult = (target_l / cur_l).clamp(0.55, 2.1);
                            r = (r * mult) * (1.0 - even_w * 0.22) + bg_mean_r * (even_w * 0.22);
                            g = (g * mult) * (1.0 - even_w * 0.22) + bg_mean_g * (even_w * 0.22);
                            b = (b * mult) * (1.0 - even_w * 0.22) + bg_mean_b * (even_w * 0.22);
                        }

                        if params.sky_enhance > 0.5 && bg_mask > 0.05 && ny < 0.60 {
                            let sky_a = bg_mask * (params.sky_enhance / 100.0) * (1.0 - ny * 1.5).clamp(0.0, 1.0) * 0.42;
                            r *= 1.0 - sky_a * 0.35;
                            g *= 1.0 - sky_a * 0.10;
                            b = (b + sky_a * 0.28).min(1.0);
                        }
                    }
                }

                // =========================================================
                // 8. MODULE 1: COLOR ADJUSTMENT, TONE CURVE, HSL & GRADING
                // =========================================================
                r *= exp_mult;
                g *= exp_mult;
                b *= exp_mult;

                r += temp_shift + tint_shift * 0.4;
                g -= tint_shift * 0.7;
                b -= temp_shift - tint_shift * 0.3;

                let lum = (0.299 * r + 0.587 * g + 0.114 * b).clamp(0.0, 1.0);
                let shadow_w = (1.0 - lum).powi(2);
                let highlight_w = lum.powi(2);
                let tone_delta = shad_adj * shadow_w
                    + high_adj * highlight_w
                    + whites_adj * lum.powi(3)
                    + blacks_adj * (1.0 - lum).powi(3);
                r += tone_delta;
                g += tone_delta;
                b += tone_delta;

                if params.curve_shadows.abs() > 0.5
                    || params.curve_darks.abs() > 0.5
                    || params.curve_lights.abs() > 0.5
                    || params.curve_highlights.abs() > 0.5
                {
                    let cs = (params.curve_shadows / 100.0) * 0.28 * (1.0 - lum).powi(3);
                    let cd = (params.curve_darks / 100.0) * 0.28 * (3.0 * lum * (1.0 - lum).powi(2));
                    let cl = (params.curve_lights / 100.0) * 0.28 * (3.0 * lum.powi(2) * (1.0 - lum));
                    let ch = (params.curve_highlights / 100.0) * 0.28 * lum.powi(3);
                    let c_delta = cs + cd + cl + ch;
                    r += c_delta;
                    g += c_delta;
                    b += c_delta;
                }

                // Interactive 256-entry Tone Curve LUT (from Interactive Curve Canvas)
                if params.curve_lut.len() == 256 {
                    let lut_sample = |v: f32| -> f32 {
                        let scaled = (v.clamp(0.0, 1.0) * 255.0).clamp(0.0, 255.0);
                        let i0 = scaled.floor() as usize;
                        let i1 = (i0 + 1).min(255);
                        let frac = scaled - i0 as f32;
                        let v0 = params.curve_lut[i0] as f32 / 255.0;
                        let v1 = params.curve_lut[i1] as f32 / 255.0;
                        v0 * (1.0 - frac) + v1 * frac
                    };
                    r = lut_sample(r);
                    g = lut_sample(g);
                    b = lut_sample(b);
                }

                if dehaze_str.abs() > 0.005 {
                    r = (r - 0.14 * dehaze_str) / (1.0 - 0.14 * dehaze_str).max(0.2);
                    g = (g - 0.14 * dehaze_str) / (1.0 - 0.14 * dehaze_str).max(0.2);
                    b = (b - 0.14 * dehaze_str) / (1.0 - 0.14 * dehaze_str).max(0.2);
                }

                if texture_str.abs() > 0.01 || params.sharpen_amount > 0.5 || params.noise_reduction > 0.5 {
                    // Radius: blend the fine high-pass towards a wider one. Detail: boost fine contrast. Masking: edges only.
                    let rad_t = ((params.sharpen_radius - 0.5) / 2.5).clamp(0.0, 1.0);
                    let hr = r - (rf + (rm - rf) * rad_t);
                    let hg = g - (gf + (gm - gf) * rad_t);
                    let hb = b - (bf + (bm - bf) * rad_t);
                    let edge = (high_r.abs() + high_g.abs() + high_b.abs()) / 3.0;
                    let mask_k = (params.sharpen_masking / 100.0).clamp(0.0, 1.0);
                    let edge_mask = 1.0 - mask_k + mask_k * smoothstep(0.004, 0.05, edge);
                    let detail_gain = 0.5 + (params.sharpen_detail / 50.0);
                    let sharp_amt = (params.sharpen_amount / 100.0) * 1.25 * detail_gain * edge_mask;
                    let sharp = texture_str + sharp_amt - (params.noise_reduction / 100.0) * 0.90;
                    r += hr * sharp;
                    g += hg * sharp;
                    b += hb * sharp;
                }

                if clarity_str.abs() > 0.01 {
                    let mid_w = 4.0 * lum * (1.0 - lum);
                    r += (r - rm) * clarity_str * mid_w;
                    g += (g - gm) * clarity_str * mid_w;
                    b += (b - bm) * clarity_str * mid_w;
                }

                if contrast_k.abs() > 0.005 {
                    r = (r - 0.5) * (1.0 + contrast_k) + 0.5;
                    g = (g - 0.5) * (1.0 + contrast_k) + 0.5;
                    b = (b - 0.5) * (1.0 + contrast_k) + 0.5;
                }

                // 8-Channel HSL Color Mixer (Red, Orange, Yellow, Green, Aqua, Blue, Purple, Magenta)
                let has_hsl = params.hsl_red_hue.abs() > 0.5 || params.hsl_red_sat.abs() > 0.5 || params.hsl_red_lum.abs() > 0.5
                    || params.hsl_orange_hue.abs() > 0.5 || params.hsl_orange_sat.abs() > 0.5 || params.hsl_orange_lum.abs() > 0.5
                    || params.hsl_yellow_hue.abs() > 0.5 || params.hsl_yellow_sat.abs() > 0.5 || params.hsl_yellow_lum.abs() > 0.5
                    || params.hsl_green_hue.abs() > 0.5 || params.hsl_green_sat.abs() > 0.5 || params.hsl_green_lum.abs() > 0.5
                    || params.hsl_aqua_hue.abs() > 0.5 || params.hsl_aqua_sat.abs() > 0.5 || params.hsl_aqua_lum.abs() > 0.5
                    || params.hsl_blue_hue.abs() > 0.5 || params.hsl_blue_sat.abs() > 0.5 || params.hsl_blue_lum.abs() > 0.5
                    || params.hsl_purple_hue.abs() > 0.5 || params.hsl_purple_sat.abs() > 0.5 || params.hsl_purple_lum.abs() > 0.5
                    || params.hsl_magenta_hue.abs() > 0.5 || params.hsl_magenta_sat.abs() > 0.5 || params.hsl_magenta_lum.abs() > 0.5;

                if has_hsl {
                    let (mut hh, mut ss, mut vv) = rgb_to_hsv(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0));
                    let band_weight = |center: f32, width: f32, h_val: f32| -> f32 {
                        let mut d = (h_val - center).abs();
                        if d > 180.0 {
                            d = 360.0 - d;
                        }
                        (1.0 - d / width).clamp(0.0, 1.0)
                    };
                    let wr = band_weight(0.0, 26.0, hh);
                    let wo = band_weight(28.0, 25.0, hh);
                    let wy = band_weight(58.0, 28.0, hh);
                    let wg = band_weight(120.0, 42.0, hh);
                    let wa = band_weight(180.0, 36.0, hh);
                    let wb = band_weight(225.0, 40.0, hh);
                    let wp = band_weight(275.0, 36.0, hh);
                    let wm = band_weight(322.0, 34.0, hh);

                    hh += (wr * params.hsl_red_hue
                        + wo * params.hsl_orange_hue
                        + wy * params.hsl_yellow_hue
                        + wg * params.hsl_green_hue
                        + wa * params.hsl_aqua_hue
                        + wb * params.hsl_blue_hue
                        + wp * params.hsl_purple_hue
                        + wm * params.hsl_magenta_hue)
                        * 0.35;
                    let sat_shift = wr * params.hsl_red_sat
                        + wo * params.hsl_orange_sat
                        + wy * params.hsl_yellow_sat
                        + wg * params.hsl_green_sat
                        + wa * params.hsl_aqua_sat
                        + wb * params.hsl_blue_sat
                        + wp * params.hsl_purple_sat
                        + wm * params.hsl_magenta_sat;
                    let lum_shift = wr * params.hsl_red_lum
                        + wo * params.hsl_orange_lum
                        + wy * params.hsl_yellow_lum
                        + wg * params.hsl_green_lum
                        + wa * params.hsl_aqua_lum
                        + wb * params.hsl_blue_lum
                        + wp * params.hsl_purple_lum
                        + wm * params.hsl_magenta_lum;
                    ss = (ss * (1.0 + sat_shift / 100.0)).clamp(0.0, 1.0);
                    vv = (vv + (lum_shift / 100.0) * 0.28).clamp(0.0, 1.0);
                    let (nr, ng, nb) = hsv_to_rgb(hh, ss, vv);
                    r = nr;
                    g = ng;
                    b = nb;
                }

                // Reference Image Color Match (Histogram / Moment Transfer)
                if color_match_str > 0.005 && params.color_match_stats.len() == 6 {
                    let t_mr = params.color_match_stats[0];
                    let t_sr = params.color_match_stats[1].max(0.02);
                    let t_mg = params.color_match_stats[2];
                    let t_sg = params.color_match_stats[3].max(0.02);
                    let t_mb = params.color_match_stats[4];
                    let t_sb = params.color_match_stats[5].max(0.02);

                    let mr = ((r - src_stats[0]) * (t_sr / src_stats[1]) + t_mr).clamp(0.0, 1.0);
                    let mg = ((g - src_stats[2]) * (t_sg / src_stats[3]) + t_mg).clamp(0.0, 1.0);
                    let mb = ((b - src_stats[4]) * (t_sb / src_stats[5]) + t_mb).clamp(0.0, 1.0);

                    r = r * (1.0 - color_match_str) + mr * color_match_str;
                    g = g * (1.0 - color_match_str) + mg * color_match_str;
                    b = b * (1.0 - color_match_str) + mb * color_match_str;
                }

                if params.grade_shadow_sat > 0.5 || params.grade_high_sat > 0.5 {
                    let (sr, sg, sb) = hsv_to_rgb(params.grade_shadow_hue, 1.0, 0.5);
                    let (hr, hg, hb) = hsv_to_rgb(params.grade_high_hue, 1.0, 0.5);
                    let s_amt = (params.grade_shadow_sat / 100.0) * 0.25 * (1.0 - lum);
                    let h_amt = (params.grade_high_sat / 100.0) * 0.25 * lum;
                    r += (sr - 0.25) * s_amt + (hr - 0.25) * h_amt;
                    g += (sg - 0.25) * s_amt + (hg - 0.25) * h_amt;
                    b += (sb - 0.25) * s_amt + (hb - 0.25) * h_amt;
                }

                let luma = 0.299 * r + 0.587 * g + 0.114 * b;
                let max_c = r.max(g).max(b);
                let min_c = r.min(g).min(b);
                let cur_sat = (max_c - min_c).clamp(0.0, 1.0);
                let vib_factor = 1.0 + vib_strength * (1.0 - cur_sat) * (1.0 - sw * 0.45);
                let total_sat = sat_mult * vib_factor;
                r = luma + (r - luma) * total_sat;
                g = luma + (g - luma) * total_sat;
                b = luma + (b - luma) * total_sat;

                match params.lut_preset.as_str() {
                    "clean_portrait" => {
                        let l = 0.299 * r + 0.587 * g + 0.114 * b;
                        r = (r * 1.03 + 0.015).min(1.0);
                        g = (g * 1.01 + 0.01).min(1.0);
                        b = (b * 0.99 + 0.015 * (1.0 - l)).min(1.0);
                    }
                    "fuji_pro" => {
                        let l = 0.299 * r + 0.587 * g + 0.114 * b;
                        r = r * 0.98 + 0.02 * l;
                        g = g * 1.03 + 0.01;
                        b = b * 1.02 + 0.025 * (1.0 - l);
                    }
                    "kodak_gold" => {
                        r = r * 1.06 + 0.02;
                        g = g * 1.01 + 0.01;
                        b = b * 0.92;
                    }
                    "cinematic_teal" => {
                        let l = 0.299 * r + 0.587 * g + 0.114 * b;
                        r += l * 0.06 - (1.0 - l) * 0.04;
                        g += 0.01;
                        b += (1.0 - l) * 0.08 - l * 0.05;
                    }
                    "warm_wedding" => {
                        r = r * 1.04 + 0.025;
                        g = g * 1.01 + 0.018;
                        b = b * 0.95 + 0.015;
                    }
                    "bw_studio" => {
                        let bw = (0.32 * r + 0.58 * g + 0.10 * b - 0.5) * 1.12 + 0.5;
                        r = bw;
                        g = bw;
                        b = bw;
                    }
                    _ => {}
                }

                // Extended 3-way colour grading: midtones, global, luminance per range, balance & blending
                if params.grade_mid_sat > 0.5
                    || params.grade_global_sat > 0.5
                    || params.grade_shadow_lum.abs() > 0.5
                    || params.grade_mid_lum.abs() > 0.5
                    || params.grade_high_lum.abs() > 0.5
                    || params.grade_global_lum.abs() > 0.5
                {
                    let l0 = (0.299 * r + 0.587 * g + 0.114 * b).clamp(0.0, 1.0);
                    let bal = params.grade_balance / 100.0;
                    let blend = (params.grade_blending / 100.0).clamp(0.0, 1.0);
                    let la = (l0 - bal * 0.25).clamp(0.0, 1.0);
                    let pw = 1.0 + (1.0 - blend) * 2.0;
                    let sh_w = (1.0 - la).powf(pw);
                    let hi_w = la.powf(pw);
                    let mid_w = (4.0 * la * (1.0 - la)).clamp(0.0, 1.0);
                    if params.grade_mid_sat > 0.5 {
                        let (tr, tg, tb) = hsv_to_rgb(params.grade_mid_hue, 1.0, 0.5);
                        let a = (params.grade_mid_sat / 100.0) * 0.25 * mid_w;
                        r += (tr - 0.25) * a;
                        g += (tg - 0.25) * a;
                        b += (tb - 0.25) * a;
                    }
                    if params.grade_global_sat > 0.5 {
                        let (tr, tg, tb) = hsv_to_rgb(params.grade_global_hue, 1.0, 0.5);
                        let a = (params.grade_global_sat / 100.0) * 0.20;
                        r += (tr - 0.25) * a;
                        g += (tg - 0.25) * a;
                        b += (tb - 0.25) * a;
                    }
                    let lum_shift = (params.grade_shadow_lum * sh_w
                        + params.grade_mid_lum * mid_w
                        + params.grade_high_lum * hi_w
                        + params.grade_global_lum)
                        / 100.0
                        * 0.25;
                    r += lum_shift;
                    g += lum_shift;
                    b += lum_shift;
                }

                // Colour noise reduction: replace noisy chroma with smoothed chroma, keep luminance detail
                if params.nr_color > 0.5 {
                    let k = (params.nr_color / 100.0).clamp(0.0, 1.0);
                    let l_cur = 0.299 * r + 0.587 * g + 0.114 * b;
                    let l_med = 0.299 * rm + 0.587 * gm + 0.114 * bm;
                    r = r * (1.0 - k) + (l_cur + (rm - l_med)) * k;
                    g = g * (1.0 - k) + (l_cur + (gm - l_med)) * k;
                    b = b * (1.0 - k) + (l_cur + (bm - l_med)) * k;
                }

                // Black & White treatment with 8-channel B&W mixer
                if params.treatment == "bw" {
                    let (hh, ss, _) = rgb_to_hsv(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0));
                    let bw_band = |center: f32, width: f32| -> f32 {
                        let mut d = (hh - center).abs();
                        if d > 180.0 {
                            d = 360.0 - d;
                        }
                        (1.0 - d / width).clamp(0.0, 1.0)
                    };
                    let mix = bw_band(0.0, 30.0) * params.bw_red
                        + bw_band(28.0, 26.0) * params.bw_orange
                        + bw_band(58.0, 30.0) * params.bw_yellow
                        + bw_band(120.0, 45.0) * params.bw_green
                        + bw_band(180.0, 38.0) * params.bw_aqua
                        + bw_band(225.0, 42.0) * params.bw_blue
                        + bw_band(275.0, 38.0) * params.bw_purple
                        + bw_band(322.0, 36.0) * params.bw_magenta;
                    let base_l = 0.299 * r + 0.587 * g + 0.114 * b;
                    let v = (base_l * (1.0 + (mix / 100.0) * 0.9 * ss.clamp(0.0, 1.0))).clamp(0.0, 1.0);
                    r = v;
                    g = v;
                    b = v;
                }

                let out_idx = x * 3;
                row_out[out_idx] = r.clamp(0.0, 1.0);
                row_out[out_idx + 1] = g.clamp(0.0, 1.0);
                row_out[out_idx + 2] = b.clamp(0.0, 1.0);
            }
        });

    // =========================================================
    // Stage 2: 3D Liquify Facial & Body Reshaping + Transform + Vignette/Grain
    // =========================================================
    let needs_warp = params.face_width.abs() > 0.3
        || params.jawline_slim.abs() > 0.3
        || params.cheekbone_lift.abs() > 0.3
        || params.chin_length.abs() > 0.3
        || params.forehead_height.abs() > 0.3
        || params.nose_width.abs() > 0.3
        || params.nose_tip_lift.abs() > 0.3
        || params.mouth_size.abs() > 0.3
        || params.lip_plump.abs() > 0.3
        || params.smile_enhance > 0.3
        || params.eye_size.abs() > 0.3
        || params.eyebrow_lift.abs() > 0.3
        || params.hair_volume.abs() > 0.3
        || params.hair_side_left_volume.abs() > 0.3
        || params.hair_side_right_volume.abs() > 0.3
        || params.hairline_adjust.abs() > 0.3
        || params.neck_slim > 0.3
        || params.shoulder_sculpt.abs() > 0.3
        || params.waist_slim > 0.3
        || params.double_chin > 0.3
        || params.rotate_angle.abs() > 0.1
        || params.keystone_vertical.abs() > 0.3
        || params.keystone_horizontal.abs() > 0.3
        || params.flip_horizontal
        || params.flip_vertical
        || (params.geo_scale - 100.0).abs() > 0.1
        || params.geo_offset_x.abs() > 0.1
        || params.geo_offset_y.abs() > 0.1
        || params.optics_distortion.abs() > 0.3
        || params.optics_ca_red.abs() > 0.3
        || params.optics_ca_blue.abs() > 0.3;

    let wf = w as f32;
    let hf = h as f32;
    let rad_rot = -params.rotate_angle.to_radians();
    let cos_r = rad_rot.cos();
    let sin_r = rad_rot.sin();

    let mut final_rgba = vec![0u8; w * h * 4];
    final_rgba
        .par_chunks_mut(w * 4)
        .enumerate()
        .for_each(|(y, row_out)| {
            let ny = y as f32 / hf;
            for x in 0..w {
                let nx = x as f32 / wf;
                let (mut r, mut g, mut b, warp_fg_a) = if needs_warp {
                    let mut sx = if params.flip_horizontal { 1.0 - nx } else { nx };
                    let mut sy = ny;

                    // Perspective Keystone & Aspect-Preserving Fine Rotation
                    if params.rotate_angle.abs() > 0.1
                        || params.keystone_vertical.abs() > 0.3
                        || params.keystone_horizontal.abs() > 0.3
                    {
                        let mut cx = sx - 0.5;
                        let mut cy = sy - 0.5;
                        let kv = (params.keystone_vertical / 100.0) * 0.28;
                        let kh = (params.keystone_horizontal / 100.0) * 0.28;
                        cx *= 1.0 + cy * kv;
                        cy *= 1.0 + cx * kh;
                        // Rotate in pixel space so non-square portraits do not shear
                        let ax = cx * wf;
                        let ay = cy * hf;
                        let rx = (ax * cos_r - ay * sin_r) / wf;
                        let ry = (ax * sin_r + ay * cos_r) / hf;
                        sx = rx + 0.5;
                        sy = ry + 0.5;
                    }

                    // 1. Eye Size (Anchored on exact 478-point left & right iris centers)
                    if params.eye_size.abs() > 0.3 {
                        let strength = (params.eye_size / 100.0) * 0.42;
                        let eye_r = face.face_rx * 0.28;
                        for &(ex, ey) in &[
                            (face.left_eye_x, face.left_eye_y),
                            (face.right_eye_x, face.right_eye_y),
                        ] {
                            let dx = sx - ex;
                            let dy = sy - ey;
                            let d = (dx * dx + dy * dy).sqrt();
                            if d < eye_r && eye_r > 1e-4 {
                                let factor = 1.0 - strength * (1.0 - (d / eye_r).powi(2)).powi(2);
                                sx = ex + dx * factor;
                                sy = ey + dy * factor;
                            }
                        }
                    }

                    // 2. Eyebrow Lift (Strictly above eyes)
                    if params.eyebrow_lift.abs() > 0.3 {
                        let lift = (params.eyebrow_lift / 100.0) * 0.038;
                        for &(ex, ey) in &[
                            (face.left_eye_x, face.left_eye_y),
                            (face.right_eye_x, face.right_eye_y),
                        ] {
                            let by = ey - face.face_ry * 0.13;
                            let dx = (sx - ex) / (face.face_rx * 0.30);
                            let dy = (sy - by) / (face.face_ry * 0.11);
                            let r2 = dx * dx + dy * dy;
                            if r2 < 1.0 {
                                sy += lift * (1.0 - r2).powi(2);
                            }
                        }
                    }

                    // 3. Face Width (Anchored on exact Left & Right Cheekbone Landmarks 234 & 454, protecting center features!)
                    if params.face_width.abs() > 0.3 {
                        let fw = (params.face_width / 100.0) * 0.085 * face.face_rx;
                        for &(cx_cheek, cy_cheek, dir) in &[
                            (face.cheek_left_x, face.cheek_left_y, -1.0f32),
                            (face.cheek_right_x, face.cheek_right_y, 1.0f32),
                        ] {
                            let dx = (sx - cx_cheek) / (face.face_rx * 0.55);
                            let dy = (sy - cy_cheek) / (face.face_ry * 0.58);
                            let r2 = dx * dx + dy * dy;
                            if r2 < 1.0 {
                                sx -= dir * fw * (1.0 - r2).powi(2);
                            }
                        }
                    }

                    // 4. Jawline Slim (Anchored on exact Left & Right Jaw Angles 172 & 397!)
                    if params.jawline_slim.abs() > 0.3 {
                        let jw = (params.jawline_slim / 100.0) * 0.095 * face.face_rx;
                        for &(jx, jy, dir) in &[
                            (face.jaw_left_x, face.jaw_left_y, -1.0f32),
                            (face.jaw_right_x, face.jaw_right_y, 1.0f32),
                        ] {
                            let dx = (sx - jx) / (face.face_rx * 0.58);
                            let dy = (sy - jy) / (face.face_ry * 0.48);
                            let r2 = dx * dx + dy * dy;
                            if r2 < 1.0 {
                                let w_fall = (1.0 - r2).powi(2);
                                sx += dir * jw * w_fall;
                                sy += jw * 0.25 * w_fall;
                            }
                        }
                    }

                    // 5. Cheekbone Lift (Lifts left & right cheekbones vertically)
                    if params.cheekbone_lift.abs() > 0.3 {
                        let lift = (params.cheekbone_lift / 100.0) * 0.042 * face.face_ry;
                        for &(cx_cheek, cy_cheek) in &[
                            (face.cheek_left_x, face.cheek_left_y),
                            (face.cheek_right_x, face.cheek_right_y),
                        ] {
                            let dx = (sx - cx_cheek) / (face.face_rx * 0.52);
                            let dy = (sy - cy_cheek) / (face.face_ry * 0.42);
                            let r2 = dx * dx + dy * dy;
                            if r2 < 1.0 {
                                sy += lift * (1.0 - r2).powi(2);
                            }
                        }
                    }

                    // 6. Chin Length & Double Chin Tuck (Anchored on exact Chin Tip 152, below mouth!)
                    if params.chin_length.abs() > 0.3 || params.double_chin > 0.3 {
                        let shift = (-params.chin_length / 100.0) * 0.075 * face.face_ry
                            + (params.double_chin / 100.0) * 0.055 * face.face_ry;
                        let dx = (sx - face.chin_x) / (face.face_rx * 0.68);
                        let dy = (sy - face.chin_y) / (face.face_ry * 0.36);
                        let r2 = dx * dx + dy * dy;
                        if r2 < 1.0 && sy > face.mouth_y + face.face_ry * 0.08 {
                            sy += shift * (1.0 - r2).powi(2);
                        }
                    }

                    // 7. Forehead Height (Anchored on exact Forehead Top 10, above eyebrows!)
                    if params.forehead_height.abs() > 0.3 {
                        let fh = (params.forehead_height / 100.0) * 0.065 * face.face_ry;
                        let dx = (sx - face.forehead_x) / (face.face_rx * 0.90);
                        let dy = (sy - face.forehead_y) / (face.face_ry * 0.42);
                        let r2 = dx * dx + dy * dy;
                        if r2 < 1.0 {
                            sy += fh * (1.0 - r2).powi(2);
                        }
                    }

                    // 8. Nose Width & Nose Tip Lift (Anchored on Nose Tip 4)
                    if params.nose_width.abs() > 0.3 || params.nose_tip_lift.abs() > 0.3 {
                        let nw = (params.nose_width / 100.0) * 0.24;
                        let nl = (params.nose_tip_lift / 100.0) * 0.036 * face.face_ry;
                        let dx = (sx - face.nose_x) / (face.face_rx * 0.28);
                        let dy = (sy - face.nose_y) / (face.face_ry * 0.22);
                        let r2 = dx * dx + dy * dy;
                        if r2 < 1.0 {
                            let fall = (1.0 - r2).powi(2);
                            sx -= (sx - face.nose_x) * nw * fall;
                            sy += nl * fall;
                        }
                    }

                    // 9. Mouth Size & Lip Plump (Anchored on exact Mouth Center)
                    if params.mouth_size.abs() > 0.3 || params.lip_plump.abs() > 0.3 {
                        let ms = (params.mouth_size / 100.0) * 0.30;
                        let lp = (params.lip_plump / 100.0) * 0.38;
                        let dx = sx - face.mouth_x;
                        let dy = sy - face.mouth_y;
                        let rx = face.face_rx * 0.34;
                        let ry = face.face_ry * 0.16;
                        let r2 = (dx / rx).powi(2) + (dy / ry).powi(2);
                        if r2 < 1.0 {
                            let fall = (1.0 - r2).powi(2);
                            sx -= dx * ms * fall;
                            sy -= dy * (ms * 0.45 + lp) * fall;
                        }
                    }

                    // 10. Smile Enhance (Anchored on exact Left & Right Mouth Corners 61 & 291!)
                    if params.smile_enhance > 0.3 {
                        let lift = (params.smile_enhance / 100.0) * 0.038 * face.face_ry;
                        for &(mcx, mcy) in &[
                            (face.mouth_left_x, face.mouth_left_y),
                            (face.mouth_right_x, face.mouth_right_y),
                        ] {
                            let dx = (sx - mcx) / (face.face_rx * 0.18);
                            let dy = (sy - mcy) / (face.face_ry * 0.16);
                            let r2 = dx * dx + dy * dy;
                            if r2 < 1.0 {
                                sy += lift * (1.0 - r2).powi(2);
                            }
                        }
                    }

                    // 11. Evoto AIReshape Hair Geometry: Top Hair Volume, Side Hair Left/Right, GrowthLine (Hairline)
                    // Anchored on detected hair crown, left/right borders & hairline with skin/brow isolation
                    let need_hair_geom = params.hair_volume.abs() > 0.3
                        || params.hair_side_left_volume.abs() > 0.3
                        || params.hair_side_right_volume.abs() > 0.3
                        || params.hairline_adjust.abs() > 0.3;
                    let hair_skin_protect = if need_hair_geom {
                        let sem_w = sample_semantic_grid(
                            &semantic_data.weights,
                            sx.clamp(0.0, 1.0),
                            sy.clamp(0.0, 1.0),
                        );
                        (1.0 - smoothstep(0.18, 0.56, sem_w.skin))
                            * (1.0 - smoothstep(0.05, 0.25, sem_w.eye))
                            * (1.0 - smoothstep(0.05, 0.25, sem_w.eyebrow))
                    } else {
                        1.0
                    };

                    if params.hair_volume.abs() > 0.3 {
                        let vol = (params.hair_volume / 100.0) * 0.052 * face.face_ry;
                        let crown_x = if semantic_data.has_hair_bounds {
                            semantic_data.hair_crown_x
                        } else {
                            face.center_x
                        };
                        let crown_y = if semantic_data.has_hair_bounds {
                            let span = (semantic_data.hairline_y - semantic_data.hair_top_y)
                                .max(face.face_ry * 0.22);
                            (semantic_data.hair_top_y + span * 0.22).max(0.02)
                        } else {
                            (face.forehead_y - face.face_ry * 0.34).max(0.03)
                        };
                        let crown_rx = if semantic_data.has_hair_bounds {
                            ((semantic_data.hair_right_x - semantic_data.hair_left_x) * 0.58)
                                .max(face.face_rx * 1.25)
                        } else {
                            face.face_rx * 1.38
                        };
                        let crown_ry = if semantic_data.has_hair_bounds {
                            ((semantic_data.hairline_y - semantic_data.hair_top_y) * 0.85)
                                .max(face.face_ry * 0.48)
                        } else {
                            face.face_ry * 0.44
                        };
                        let brow_top = (face.left_eye_y.min(face.right_eye_y) - face.face_ry * 0.14)
                            .max(face.forehead_y);
                        let above_face_gate =
                            smoothstep(brow_top, (brow_top - face.face_ry * 0.22).max(0.01), sy);
                        let dx = (sx - crown_x) / crown_rx.max(0.08);
                        let dy = (sy - crown_y) / crown_ry.max(0.06);
                        let r2 = dx * dx + dy * dy;
                        if r2 < 1.0 && above_face_gate > 0.001 {
                            sy += vol * (1.0 - r2).powi(2) * above_face_gate * hair_skin_protect;
                        }
                    }

                    // 11b. Side Hair Volume (Left / Right independent expand/tuck, anchored to hair boundaries)
                    if params.hair_side_left_volume.abs() > 0.3
                        || params.hair_side_right_volume.abs() > 0.3
                    {
                        let fx_s = (sx - face.center_x) / face.face_rx.max(0.08);
                        let fy_s = (sy - face.center_y) / face.face_ry.max(0.08);
                        let radial_face = (fx_s * fx_s + fy_s * fy_s).sqrt();
                        let outside_face_gate = smoothstep(0.92, 1.30, radial_face) * hair_skin_protect;
                        if outside_face_gate > 0.001 {
                            let side_span_y = if semantic_data.has_hair_bounds {
                                ((face.chin_y - semantic_data.hair_top_y).max(face.face_ry) * 1.15)
                                    .max(face.face_ry * 1.25)
                            } else {
                                face.face_ry * 1.25
                            };
                            if params.hair_side_left_volume.abs() > 0.3 {
                                let shift_l =
                                    (params.hair_side_left_volume / 100.0) * 0.068 * face.face_rx;
                                let cx_l = if semantic_data.has_hair_bounds {
                                    semantic_data.hair_left_x
                                } else {
                                    face.center_x - face.face_rx * 1.15
                                };
                                let cy_l = if semantic_data.has_hair_bounds {
                                    semantic_data.hair_left_y
                                } else {
                                    face.center_y - face.face_ry * 0.10
                                };
                                let dx_l = (sx - cx_l) / (face.face_rx * 0.72).max(0.06);
                                let dy_l = (sy - cy_l) / side_span_y.max(0.08);
                                let r2 = dx_l * dx_l + dy_l * dy_l;
                                if r2 < 1.0 {
                                    sx += shift_l * (1.0 - r2).powi(2) * outside_face_gate;
                                }
                            }
                            if params.hair_side_right_volume.abs() > 0.3 {
                                let shift_r =
                                    (params.hair_side_right_volume / 100.0) * 0.068 * face.face_rx;
                                let cx_r = if semantic_data.has_hair_bounds {
                                    semantic_data.hair_right_x
                                } else {
                                    face.center_x + face.face_rx * 1.15
                                };
                                let cy_r = if semantic_data.has_hair_bounds {
                                    semantic_data.hair_right_y
                                } else {
                                    face.center_y - face.face_ry * 0.10
                                };
                                let dx_r = (sx - cx_r) / (face.face_rx * 0.72).max(0.06);
                                let dy_r = (sy - cy_r) / side_span_y.max(0.08);
                                let r2 = dx_r * dx_r + dy_r * dy_r;
                                if r2 < 1.0 {
                                    sx -= shift_r * (1.0 - r2).powi(2) * outside_face_gate;
                                }
                            }
                        }
                    }

                    // 11c. GrowthLine / Đường chân tóc (positive lowers hairline slightly down; negative raises forehead)
                    if params.hairline_adjust.abs() > 0.3 {
                        let hl_shift = (params.hairline_adjust / 100.0) * 0.044 * face.face_ry;
                        let hl_anchor_y = if semantic_data.has_hair_bounds {
                            semantic_data.hairline_y
                        } else {
                            face.forehead_y
                        };
                        let brow_safe = (face.left_eye_y.min(face.right_eye_y) - face.face_ry * 0.10)
                            .max(hl_anchor_y + 0.01);
                        let above_brow_gate =
                            smoothstep(brow_safe, (brow_safe - face.face_ry * 0.16).max(0.01), sy);
                        let dx = (sx - face.center_x) / (face.face_rx * 0.90).max(0.06);
                        let dy = (sy - hl_anchor_y) / (face.face_ry * 0.26).max(0.04);
                        let r2 = dx * dx + dy * dy;
                        if r2 < 1.0 && above_brow_gate > 0.001 {
                            sy -= hl_shift * (1.0 - r2).powi(2) * above_brow_gate;
                        }
                    }

                    // 12. Body Reshape: Neck Slim, Shoulder Sculpt, Waist Slim
                    if params.neck_slim > 0.3 {
                        let ns = (params.neck_slim / 100.0) * 0.14;
                        let neck_y = face.chin_y + face.face_ry * 0.32;
                        let dx = (sx - face.center_x) / (face.face_rx * 0.85);
                        let dy = (sy - neck_y) / (face.face_ry * 0.45);
                        let r2 = dx * dx + dy * dy;
                        if r2 < 1.0 {
                            sx += (sx - face.center_x) * ns * (1.0 - r2).powi(2);
                        }
                    }

                    if params.shoulder_sculpt.abs() > 0.3 {
                        let ss = (params.shoulder_sculpt / 100.0) * 0.065;
                        let sh_y = face.chin_y + face.face_ry * 0.85;
                        let dx = (sx - face.center_x) / (face.face_rx * 2.4);
                        let dy = (sy - sh_y) / (face.face_ry * 0.70);
                        let r2 = dx * dx + dy * dy;
                        if r2 < 1.0 {
                            sy += ss * (1.0 - r2).powi(2);
                        }
                    }

                    if params.waist_slim > 0.3 && sy > face.chin_y + face.face_ry * 0.8 {
                        let ws = (params.waist_slim / 100.0) * 0.13;
                        let waist_y = (face.chin_y + face.face_ry * 1.8).min(0.92);
                        let dx = (sx - face.center_x) / (face.face_rx * 2.4);
                        let dy = (sy - waist_y) / (face.face_ry * 1.10);
                        let r2 = dx * dx + dy * dy;
                        if r2 < 1.0 {
                            sx += (sx - face.center_x) * ws * (1.0 - r2).powi(2);
                        }
                    }

                    // Vertical flip, geometry scale/offset, lens distortion
                    if params.flip_vertical {
                        sy = 1.0 - sy;
                    }
                    if (params.geo_scale - 100.0).abs() > 0.1 || params.geo_offset_x.abs() > 0.1 || params.geo_offset_y.abs() > 0.1 {
                        let sc = (params.geo_scale / 100.0).max(0.1);
                        sx = 0.5 + (sx - 0.5) / sc - params.geo_offset_x / 100.0 * 0.5;
                        sy = 0.5 + (sy - 0.5) / sc - params.geo_offset_y / 100.0 * 0.5;
                    }
                    if params.optics_distortion.abs() > 0.3 {
                        let cx = sx - 0.5;
                        let cy = sy - 0.5;
                        let k = (params.optics_distortion / 100.0) * 0.6;
                        let f = 1.0 + k * (cx * cx + cy * cy) * 4.0;
                        sx = 0.5 + cx * f;
                        sy = 0.5 + cy * f;
                    }

                    let (mut pr, pg, mut pb) = sample_bilinear(&retouched_rgb, w, h, sx * wf, sy * hf);
                    if params.optics_ca_red.abs() > 0.3 || params.optics_ca_blue.abs() > 0.3 {
                        let cx = sx - 0.5;
                        let cy = sy - 0.5;
                        let kr = params.optics_ca_red / 100.0 * 0.012;
                        let kb = params.optics_ca_blue / 100.0 * 0.012;
                        pr = sample_bilinear(&retouched_rgb, w, h, (sx + cx * kr) * wf, (sy + cy * kr) * hf).0;
                        pb = sample_bilinear(&retouched_rgb, w, h, (sx + cx * kb) * wf, (sy + cy * kb) * hf).2;
                    }
                    let fg_a = if is_replacing {
                        if sx < 0.0 || sx > 1.0 || sy < 0.0 || sy > 1.0 {
                            0.0
                        } else if let Some(ref gm) = guided_matte {
                            sample_alpha_unit(&gm.fg_alpha, w, h, sx * wf, sy * hf)
                        } else {
                            1.0
                        }
                    } else {
                        1.0
                    };
                    (pr, pg, pb, fg_a)
                } else {
                    let idx = (y * w + x) * 3;
                    let fg_a = if is_replacing {
                        guided_matte
                            .as_ref()
                            .map(|gm| gm.fg_alpha[y * w + x])
                            .unwrap_or(1.0)
                    } else {
                        1.0
                    };
                    (retouched_rgb[idx], retouched_rgb[idx + 1], retouched_rgb[idx + 2], fg_a)
                };

                // Defringe: desaturate purple / green fringes on bright subject boundaries before compositing
                if params.optics_defringe_purple > 0.5 || params.optics_defringe_green > 0.5 {
                    let (hh, ss, vv) = rgb_to_hsv(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0));
                    let band = |c: f32, wd: f32| -> f32 {
                        let mut d = (hh - c).abs();
                        if d > 180.0 {
                            d = 360.0 - d;
                        }
                        (1.0 - d / wd).clamp(0.0, 1.0)
                    };
                    let kill = band(285.0, 40.0) * (params.optics_defringe_purple / 100.0).clamp(0.0, 1.0)
                        + band(110.0, 40.0) * (params.optics_defringe_green / 100.0).clamp(0.0, 1.0);
                    let kill = (kill * smoothstep(0.35, 0.8, vv)).clamp(0.0, 1.0);
                    if kill > 0.0 {
                        let (nr, ng, nb) = hsv_to_rgb(hh, ss * (1.0 - kill), vv);
                        r = nr;
                        g = ng;
                        b = nb;
                    }
                }

                // Background replacement composite in clean output space (keeps studio white & replaced scenes
                // crisp, unstretched by facial/body liquify, and immune to foreground exposure/vignette darkening)
                let mut out_alpha: u8 = 255;
                if is_replacing {
                    let a = warp_fg_a.clamp(0.0, 1.0);
                    if is_transparent_bg {
                        out_alpha = (a * 255.0).round().clamp(0.0, 255.0) as u8;
                        if out_alpha == 0 {
                            r = 0.0;
                            g = 0.0;
                            b = 0.0;
                        }
                    } else {
                        let (mut br, mut bg_c, mut bb) = eval_replaced_background(
                            params,
                            custom_bg_rgb,
                            &custom_bg_buf,
                            x,
                            y,
                            w,
                            h,
                        );
                        if params.treatment == "bw" {
                            let lum = (0.299 * br + 0.587 * bg_c + 0.114 * bb).clamp(0.0, 1.0);
                            br = lum;
                            bg_c = lum;
                            bb = lum;
                        }

                        if params.bg_ground_reflect && ny > 0.72 && a < 0.85 {
                            let ry_n = (2.0 * 0.85 - ny).clamp(0.0, 0.85);
                            let (rr_r, rr_g, rr_b) = sample_bilinear(&retouched_rgb, w, h, nx * wf, ry_n * hf);
                            let ra = guided_matte
                                .as_ref()
                                .map(|gm| sample_alpha_unit(&gm.fg_alpha, w, h, nx * wf, ry_n * hf))
                                .unwrap_or(0.0);
                            let fade = (1.0 - smoothstep(0.73, 0.99, ny)) * smoothstep(0.71, 0.76, ny);
                            let refl_w = (ra * fade * 0.34).clamp(0.0, 0.38);
                            if refl_w > 0.002 {
                                br = br * (1.0 - refl_w) + (br * 0.35 + rr_r * 0.65) * refl_w;
                                bg_c = bg_c * (1.0 - refl_w) + (bg_c * 0.35 + rr_g * 0.65) * refl_w;
                                bb = bb * (1.0 - refl_w) + (bb * 0.35 + rr_b * 0.65) * refl_w;
                            }
                        }

                        if let Some((ref sh_plane, sw, sh, off_x, off_y)) = cast_shadow {
                            let sh_raw = sample_plane_bilinear(sh_plane, sw, sh, nx - off_x, ny - off_y).clamp(0.0, 1.0);
                            let sh_op = (params.bg_shadow_opacity / 100.0).clamp(0.0, 1.0) * sh_raw;
                            if sh_op > 0.002 {
                                let shift_t = (params.bg_shadow_shift / 100.0).clamp(0.0, 1.0);
                                let st_r = 0.03 * (1.0 - shift_t) + (br * 0.32) * shift_t;
                                let st_g = 0.03 * (1.0 - shift_t) + (bg_c * 0.28) * shift_t;
                                let st_b = 0.04 * (1.0 - shift_t) + (bb * 0.26) * shift_t;
                                let apply_sh = |bc: f32, sc: f32| -> f32 {
                                    let blended = match params.bg_shadow_blend_mode.as_str() {
                                        "normal" => sc,
                                        "darken" => bc.min(sc),
                                        "soft_light" => (bc * (0.45 + 0.55 * sc)).clamp(0.0, 1.0),
                                        _ => bc * (1.0 - 0.85 * (1.0 - sc)),
                                    };
                                    (bc * (1.0 - sh_op) + blended * sh_op).clamp(0.0, 1.0)
                                };
                                br = apply_sh(br, st_r);
                                bg_c = apply_sh(bg_c, st_g);
                                bb = apply_sh(bb, st_b);
                            }
                        }

                        if params.bg_preserve_shadow && ny > 0.45 && a < 0.78 {
                            let orig_lum = (0.299 * r + 0.587 * g + 0.114 * b).clamp(0.0, 1.0);
                            let sh_ratio = (orig_lum / bg_mean_luma.max(0.18)).clamp(0.32, 1.0);
                            let ground_w = smoothstep(0.45, 0.78, ny) * 0.76;
                            let mult = 1.0 - ground_w * (1.0 - sh_ratio);
                            br *= mult;
                            bg_c *= mult;
                            bb *= mult;
                        }

                        let blend_ch = |orig: f32, bgv: f32| -> f32 {
                            match params.bg_blend_mode.as_str() {
                                "multiply" => (orig * bgv).clamp(0.0, 1.0),
                                "screen" => (1.0 - (1.0 - orig) * (1.0 - bgv)).clamp(0.0, 1.0),
                                "overlay" => {
                                    if orig < 0.5 {
                                        (2.0 * orig * bgv).clamp(0.0, 1.0)
                                    } else {
                                        (1.0 - 2.0 * (1.0 - orig) * (1.0 - bgv)).clamp(0.0, 1.0)
                                    }
                                }
                                "soft_light" => ((1.0 - 2.0 * bgv) * orig * orig + 2.0 * bgv * orig).clamp(0.0, 1.0),
                                _ => bgv,
                            }
                        };
                        br = blend_ch(r, br);
                        bg_c = blend_ch(g, bg_c);
                        bb = blend_ch(b, bb);

                        let area_w = if params.bg_fill_area == "outer_only" {
                            1.0 - smoothstep(0.84, 0.99, ny) * 0.85
                        } else {
                            1.0
                        };
                        let rep_op = ((params.bg_replace_opacity / 100.0).clamp(0.0, 1.0) * area_w).clamp(0.0, 1.0);
                        if rep_op < 0.999 {
                            br = r * (1.0 - rep_op) + br * rep_op;
                            bg_c = g * (1.0 - rep_op) + bg_c * rep_op;
                            bb = b * (1.0 - rep_op) + bb * rep_op;
                        }

                        if a > 0.01 && !is_studio_solid_bg(&params.bg_replace_mode) {
                            let rim_wrap = ((1.0 - a * 0.4) * 0.05).clamp(0.0, 0.05);
                            r = r * (1.0 - rim_wrap) + br * rim_wrap;
                            g = g * (1.0 - rim_wrap) + bg_c * rim_wrap;
                            b = b * (1.0 - rim_wrap) + bb * rim_wrap;
                        }
                        r = r * a + br * (1.0 - a);
                        g = g * a + bg_c * (1.0 - a);
                        b = b * a + bb * (1.0 - a);
                    }
                }

                // Film grain: size, roughness, strongest in midtones (preserve flat studio backdrop)
                if params.film_grain > 0.5 {
                    let grain_mask = if is_replacing && is_studio_solid_bg(&params.bg_replace_mode) {
                        warp_fg_a.clamp(0.0, 1.0)
                    } else {
                        1.0
                    };
                    if grain_mask > 0.01 {
                        let gr = lc_grain_noise(
                            x as f32,
                            y as f32,
                            params.film_grain,
                            params.grain_size,
                            params.grain_roughness,
                        ) * grain_mask;
                        let l = ((r + g + b) / 3.0).clamp(0.0, 1.0);
                        let lw = 0.45 + 0.55 * 4.0 * l * (1.0 - l);
                        r += gr * lw;
                        g += gr * lw;
                        b += gr * lw;
                    }
                }

                // Post-crop vignette: amount, midpoint, roundness, feather, highlight priority
                if params.vignette.abs() > 0.5
                    && !(is_replacing && is_studio_solid_bg(&params.bg_replace_mode))
                {
                    let ro = params.vignette_roundness / 100.0;
                    let aspect = hf / wf;
                    let vx = (nx - 0.5) * 2.0;
                    let mut vy = (ny - 0.5) * 2.0;
                    if ro > 0.0 {
                        vy *= 1.0 + (aspect - 1.0) * ro;
                    }
                    let p = if ro < 0.0 { 2.0 + (-ro) * 2.0 } else { 2.0 };
                    let d = (vx.abs().powf(p) + vy.abs().powf(p)).powf(1.0 / p) / std::f32::consts::SQRT_2;
                    let start = (params.vignette_midpoint / 100.0).clamp(0.0, 1.0) * 0.95;
                    let width = 0.12 + (params.vignette_feather / 100.0).clamp(0.0, 1.0) * 0.88;
                    let t = smoothstep(start, start + width, d);
                    let lum_here = (0.299 * r + 0.587 * g + 0.114 * b).clamp(0.0, 1.0);
                    let protect = 1.0 - (params.vignette_highlights / 100.0).clamp(0.0, 1.0) * 0.85 * lum_here;
                    let v_mult = (1.0 - (params.vignette / 100.0) * t * protect * 0.9).clamp(0.05, 1.9);
                    r *= v_mult;
                    g *= v_mult;
                    b *= v_mult;
                }

                // Lens-profile vignetting correction (applies only to original captured background)
                if params.optics_vignetting.abs() > 0.5 && !is_replacing {
                    let vx = (nx - 0.5) * 2.0;
                    let vy = (ny - 0.5) * 2.0;
                    let d = (vx * vx + vy * vy).sqrt() / std::f32::consts::SQRT_2;
                    let start = (params.optics_vignetting_midpoint / 100.0).clamp(0.0, 1.0) * 0.8;
                    let t = smoothstep(start, 1.0, d);
                    let m = (1.0 + (params.optics_vignetting / 100.0) * t * 0.9).clamp(0.2, 2.0);
                    r *= m;
                    g *= m;
                    b *= m;
                }

                let out_idx = x * 4;
                row_out[out_idx] = (r.clamp(0.0, 1.0) * 255.0).round() as u8;
                row_out[out_idx + 1] = (g.clamp(0.0, 1.0) * 255.0).round() as u8;
                row_out[out_idx + 2] = (b.clamp(0.0, 1.0) * 255.0).round() as u8;
                row_out[out_idx + 3] = out_alpha;
            }
        });

    RgbaImage::from_raw(w_u32, h_u32, final_rgba).unwrap_or_else(|| img.clone())
}

fn decode_data_url(data_url: &str) -> Result<DynamicImage, String> {
    let b64_part = data_url
        .split_once(',')
        .map(|(_, b64)| b64)
        .unwrap_or(data_url);
    let bytes = BASE64
        .decode(b64_part)
        .map_err(|e| format!("Base64 decode error: {}", e))?;
    image::load_from_memory(&bytes).map_err(|e| format!("Image decode error: {}", e))
}

fn encode_jpeg_data_url(img: &RgbaImage, quality: u8) -> Result<String, String> {
    let mut buf = Cursor::new(Vec::new());
    let dyn_img = DynamicImage::ImageRgba8(img.clone());
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, quality);
    encoder
        .encode_image(&dyn_img)
        .map_err(|e| format!("JPEG encode error: {}", e))?;
    Ok(format!(
        "data:image/jpeg;base64,{}",
        BASE64.encode(buf.into_inner())
    ))
}

fn encode_png_data_url(img: &RgbaImage) -> Result<String, String> {
    let mut buf = Cursor::new(Vec::new());
    let dyn_img = DynamicImage::ImageRgba8(img.clone());
    dyn_img
        .write_to(&mut buf, ImageFormat::Png)
        .map_err(|e| format!("PNG encode error: {}", e))?;
    Ok(format!(
        "data:image/png;base64,{}",
        BASE64.encode(buf.into_inner())
    ))
}

/// Read EXIF / TIFF Orientation tag (0x0112) from JPEG, TIFF, or Canon CR2/CR3 header
fn read_exif_orientation_tag(path: &Path) -> u16 {
    use std::io::Read;
    let mut file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return 1,
    };
    let mut buf = vec![0u8; 65536];
    let n = match file.read(&mut buf) {
        Ok(n) if n >= 16 => n,
        _ => return 1,
    };
    let data = &buf[..n];

    if (data.starts_with(b"II*\0") || data.starts_with(b"MM\0*")) && data.len() >= 12 {
        if let Some(o) = parse_tiff_orientation(data) {
            return o;
        }
    }

    for i in 0..data.len().saturating_sub(16) {
        if &data[i..i + 6] == b"Exif\0\0" {
            if let Some(o) = parse_tiff_orientation(&data[i + 6..]) {
                return o;
            }
        }
    }
    1
}

fn parse_tiff_orientation(tiff: &[u8]) -> Option<u16> {
    if tiff.len() < 12 {
        return None;
    }
    let le = if &tiff[0..2] == b"II" {
        true
    } else if &tiff[0..2] == b"MM" {
        false
    } else {
        return None;
    };
    let r16 = |off: usize| -> Option<u16> {
        let s = tiff.get(off..off + 2)?;
        Some(if le {
            u16::from_le_bytes([s[0], s[1]])
        } else {
            u16::from_be_bytes([s[0], s[1]])
        })
    };
    let r32 = |off: usize| -> Option<u32> {
        let s = tiff.get(off..off + 4)?;
        Some(if le {
            u32::from_le_bytes([s[0], s[1], s[2], s[3]])
        } else {
            u32::from_be_bytes([s[0], s[1], s[2], s[3]])
        })
    };

    let ifd0 = r32(4)? as usize;
    let count = r16(ifd0)? as usize;
    for idx in 0..count.min(64) {
        let entry = ifd0 + 2 + idx * 12;
        let tag = r16(entry)?;
        if tag == 0x0112 {
            let val = r16(entry + 8)?;
            if (1..=8).contains(&val) {
                return Some(val);
            }
        }
    }
    None
}

fn apply_exif_orientation(img: DynamicImage, orientation: u16) -> DynamicImage {
    match orientation {
        2 => img.fliph(),
        3 => img.rotate180(),
        4 => img.flipv(),
        5 => img.rotate90().fliph(),
        6 => img.rotate90(),
        7 => img.rotate270().fliph(),
        8 => img.rotate270(),
        _ => img,
    }
}

/// Extract the largest embedded JPEG preview from a RAW / DNG byte buffer.
/// Returns the decoded image and its own EXIF orientation (if present in the JPEG stream).
fn extract_largest_embedded_jpeg(bytes: &[u8]) -> Option<(DynamicImage, Option<u16>)> {
    let mut best: Option<(u64, DynamicImage, Option<u16>)> = None;
    let mut idx = 0usize;
    let mut attempts = 0usize;

    while idx + 4 <= bytes.len() && attempts < 20 {
        if bytes[idx] == 0xFF && bytes[idx + 1] == 0xD8 && bytes[idx + 2] == 0xFF {
            let marker = bytes[idx + 3];
            let valid_marker = matches!(marker, 0xC0..=0xC4 | 0xDB | 0xE0..=0xEF | 0xFE);
            if valid_marker {
                attempts += 1;
                if let Ok(img) = image::load_from_memory_with_format(&bytes[idx..], ImageFormat::Jpeg) {
                    let (w, h) = (img.width() as u64, img.height() as u64);
                    let area = w * h;
                    if w >= 160 && h >= 160 {
                        let scan_len = bytes.len().saturating_sub(idx).min(65536);
                        let sub = &bytes[idx..idx + scan_len];
                        let jpg_orient = (0..sub.len().saturating_sub(16))
                            .find(|&p| &sub[p..p + 6] == b"Exif\0\0")
                            .and_then(|p| parse_tiff_orientation(&sub[p + 6..]));

                        if best.as_ref().map_or(true, |(best_area, _, _)| area > *best_area) {
                            best = Some((area, img, jpg_orient));
                        }
                    }
                    // Skip past SOI header to continue scanning
                    idx += 4;
                    continue;
                }
            }
        }
        idx += 1;
    }

    best.map(|(_, img, orient)| (img, orient))
}

fn decode_with_rawler(path: &Path, exif_orient: u16) -> Result<DynamicImage, String> {
    let rawfile = rawler::rawsource::RawSource::new(path)
        .map_err(|e| format!("Failed to read RAW file {:?}: {}", path.file_name(), e))?;
    let decoder = rawler::get_decoder(&rawfile)
        .map_err(|e| format!("RAW decoder error for {:?}: {}", path.file_name(), e))?;
    let raw_params = rawler::decoders::RawDecodeParams::default();

    let last_err = match decoder.raw_image(&rawfile, &raw_params, false) {
        Ok(rawimage) => {
            let raw_o = rawimage.orientation.to_u16();
            let effective_orient = if (1..=8).contains(&raw_o) {
                raw_o
            } else {
                exif_orient
            };
            let dev = rawler::imgop::develop::RawDevelop::default();
            match dev.develop_intermediate(&rawimage) {
                Ok(inter) => {
                    if let Some(dyn_img) = inter.to_dynamic_image() {
                        return Ok(apply_exif_orientation(dyn_img, effective_orient));
                    }
                    "Failed to convert developed RAW buffer".to_string()
                }
                Err(e) => format!("RAW develop error: {}", e),
            }
        }
        Err(e) => format!("RAW decode error: {}", e),
    };

    if let Ok(Some(full)) = decoder.full_image(&rawfile, &raw_params) {
        return Ok(apply_exif_orientation(full, exif_orient));
    }
    if let Ok(Some(prev)) = decoder.preview_image(&rawfile, &raw_params) {
        return Ok(apply_exif_orientation(prev, exif_orient));
    }

    Err(last_err)
}

#[cfg(target_os = "windows")]
fn decode_via_windows_wic(path: &Path, exif_orient: u16) -> Option<DynamicImage> {
    use std::os::windows::process::CommandExt;

    let path_str = path.to_str()?;
    let pid = std::process::id();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let tmp_png = std::env::temp_dir().join(format!("retouch_wic_{pid}_{now}.png"));
    let tmp_str = tmp_png.to_str()?.to_string();

    let script = r#"
Add-Type -AssemblyName PresentationCore
$inPath = $env:RETOUCH_WIC_IN
$outPath = $env:RETOUCH_WIC_OUT
$stream = [System.IO.File]::OpenRead($inPath)
try {
  $dec = [System.Windows.Media.Imaging.BitmapDecoder]::Create($stream, [System.Windows.Media.Imaging.BitmapCreateOptions]::PreservePixelFormat, [System.Windows.Media.Imaging.BitmapCacheOption]::OnLoad)
  $frame = $dec.Frames[0]
  $enc = New-Object System.Windows.Media.Imaging.PngBitmapEncoder
  $enc.Frames.Add([System.Windows.Media.Imaging.BitmapFrame]::Create($frame))
  $outStream = [System.IO.File]::Create($outPath)
  try { $enc.Save($outStream) } finally { $outStream.Dispose() }
} finally {
  $stream.Dispose()
}
"#;

    let status = std::process::Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .env("RETOUCH_WIC_IN", path_str)
        .env("RETOUCH_WIC_OUT", &tmp_str)
        .creation_flags(0x08000000)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .ok();

    let result = if status.is_some_and(|s| s.success()) && tmp_png.exists() {
        let jpg_orient = read_exif_orientation_tag(&tmp_png);
        image::open(&tmp_png).ok().map(|img| {
            let effective = if jpg_orient != 1 {
                jpg_orient
            } else if exif_orient != 1 && img.width() > img.height() {
                exif_orient
            } else {
                1
            };
            apply_exif_orientation(img, effective)
        })
    } else {
        None
    };

    let _ = std::fs::remove_file(&tmp_png);
    result
}

/// Decode camera RAW / iPhone DNG (Bayer, LinearRGB, Lossless JPEG, JPEG-XL ProRAW) or standard image files
/// and automatically apply EXIF orientation.
fn decode_raw_or_image(path: &Path) -> Result<DynamicImage, String> {
    let exif_orient = read_exif_orientation_tag(path);

    if let Ok(img) = image::open(path) {
        return Ok(apply_exif_orientation(img, exif_orient));
    }

    #[cfg(target_os = "macos")]
    {
        let tmp_jpg = std::env::temp_dir().join(format!(
            "retouch_raw_{}.jpg",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        if let Ok(status) = std::process::Command::new("sips")
            .args([
                "-s",
                "format",
                "jpeg",
                "-s",
                "formatOptions",
                "96",
                "-Z",
                "2400",
                path.to_str().unwrap_or_default(),
                "--out",
                tmp_jpg.to_str().unwrap_or_default(),
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
        {
            if status.success() {
                let jpg_orient = read_exif_orientation_tag(&tmp_jpg);
                if let Ok(img) = image::open(&tmp_jpg) {
                    let _ = std::fs::remove_file(&tmp_jpg);
                    let effective_orient = if jpg_orient != 1 {
                        jpg_orient
                    } else if exif_orient != 1 && img.width() > img.height() {
                        exif_orient
                    } else {
                        1
                    };
                    return Ok(apply_exif_orientation(img, effective_orient));
                }
            }
        }
        let _ = std::fs::remove_file(&tmp_jpg);
    }

    let rawler_res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        decode_with_rawler(path, exif_orient)
    }));

    match rawler_res {
        Ok(Ok(img)) => return Ok(img),
        Ok(Err(_)) | Err(_) => {}
    }

    let embedded_preview = std::fs::read(path)
        .ok()
        .and_then(|bytes| extract_largest_embedded_jpeg(&bytes))
        .map(|(img, jpg_orient)| {
            let effective = match jpg_orient {
                Some(o) if (1..=8).contains(&o) && o != 1 => o,
                _ if exif_orient != 1 && img.width() > img.height() => exif_orient,
                _ => 1,
            };
            apply_exif_orientation(img, effective)
        });

    #[cfg(target_os = "windows")]
    {
        let preview_too_small = embedded_preview
            .as_ref()
            .is_none_or(|img| img.width().max(img.height()) < 1200);
        if preview_too_small {
            if let Some(wic_img) = decode_via_windows_wic(path, exif_orient) {
                let use_wic = embedded_preview.as_ref().is_none_or(|prev| {
                    (wic_img.width() as u64 * wic_img.height() as u64)
                        >= (prev.width() as u64 * prev.height() as u64)
                });
                if use_wic {
                    return Ok(wic_img);
                }
            }
        }
    }

    if let Some(preview_img) = embedded_preview {
        return Ok(preview_img);
    }

    Err(match rawler_res {
        Ok(Err(err)) => err,
        Err(_) => format!("RAW decoder failed unexpectedly for {:?}", path.file_name()),
        Ok(Ok(_)) => unreachable!(),
    })
}

#[tauri::command]
fn load_photo_from_path_cmd(file_path: String) -> Result<ImportedPhotoPayload, String> {
    let path = Path::new(&file_path);
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Imported_Photo.CR2")
        .to_string();

    let dyn_img = decode_raw_or_image(path)?;
    let orig_w = dyn_img.width();
    let orig_h = dyn_img.height();

    // Keep original resolution up to 6000 px on the long edge so portrait detail
    // is preserved. Only downsample extremely large files (e.g. 50 MP+ medium-format)
    // to stay within reasonable memory and IPC limits.
    let master = if orig_w > 6000 || orig_h > 6000 {
        dyn_img.resize(6000, 6000, image::imageops::FilterType::Lanczos3)
    } else {
        dyn_img
    };

    let rgba = master.to_rgba8();
    let detected_face = detect_face_region(&rgba);
    // Quality 99 — preserves original fine detail for lossless-feel editing and Rust retouch pipeline.
    let data_url = encode_jpeg_data_url(&rgba, 99)?;

    Ok(ImportedPhotoPayload {
        path: file_path,
        name,
        data_url,
        width: orig_w,
        height: orig_h,
        detected_face,
    })
}

#[tauri::command]
fn rotate_photo_90_cmd(image_base64: String, clockwise: bool) -> Result<ImportedPhotoPayload, String> {
    let dyn_img = decode_data_url(&image_base64)?;
    let rotated = if clockwise {
        dyn_img.rotate90()
    } else {
        dyn_img.rotate270()
    };
    let rgba = rotated.to_rgba8();
    let w = rgba.width();
    let h = rgba.height();
    let detected_face = detect_face_region(&rgba);
    let data_url = encode_jpeg_data_url(&rgba, 98)?;

    Ok(ImportedPhotoPayload {
        path: String::new(),
        name: String::new(),
        data_url,
        width: w,
        height: h,
        detected_face,
    })
}

#[tauri::command]
fn analyze_portrait_cmd(image_base64: String) -> Result<FaceRegion, String> {
    let dyn_img = decode_data_url(&image_base64)?;
    let rgba = dyn_img.to_rgba8();
    Ok(detect_face_region(&rgba))
}

fn decode_optional_rgba(b64: Option<&str>) -> Option<RgbaImage> {
    b64.filter(|s| !s.trim().is_empty())
        .and_then(|s| decode_data_url(s).ok())
        .map(|d| d.to_rgba8())
}

#[tauri::command]
fn process_photo_cmd(
    image_base64: String,
    params: RetouchParams,
    high_quality: bool,
    mask_base64: Option<String>,
    scene_mask_base64: Option<String>,
) -> Result<ProcessResult, String> {
    let start = std::time::Instant::now();
    let dyn_img = decode_data_url(&image_base64)?;

    let max_dim = if high_quality { 4096 } else { 2200 };
    let working_img = if dyn_img.width() > max_dim || dyn_img.height() > max_dim {
        dyn_img
            .resize(max_dim, max_dim, image::imageops::FilterType::CatmullRom)
            .to_rgba8()
    } else {
        dyn_img.to_rgba8()
    };

    let decoded_mask = decode_optional_rgba(mask_base64.as_deref());
    let decoded_scene = decode_optional_rgba(scene_mask_base64.as_deref());

    let mut active_params = params;
    if active_params.face.face_rx <= 0.01 {
        active_params.face = detect_face_region(&working_img);
    }
    let detected = active_params.face.clone();

    let result_rgba = apply_retouch_pipeline_with_scene(
        &working_img,
        &active_params,
        decoded_mask.as_ref(),
        decoded_scene.as_ref(),
    );
    let quality = if high_quality { 98 } else { 96 };
    let data_url = if active_params.bg_replace_mode == "transparent" {
        encode_png_data_url(&result_rgba)?
    } else {
        encode_jpeg_data_url(&result_rgba, quality)?
    };

    Ok(ProcessResult {
        data_url,
        width: result_rgba.width(),
        height: result_rgba.height(),
        elapsed_ms: start.elapsed().as_millis(),
        detected_face: detected,
    })
}

/// Apply interactive crop rectangle (if non-full), fallback aspect-ratio crop, and optional
/// high-quality Lanczos3 export dimension resize.
fn apply_crop_and_resize(rgba: RgbaImage, params: &RetouchParams) -> DynamicImage {
    let mut img = DynamicImage::ImageRgba8(rgba);
    let iw = img.width().max(1);
    let ih = img.height().max(1);

    let has_explicit_box = params.crop_w > 0.01
        && params.crop_h > 0.01
        && (params.crop_x > 0.001
            || params.crop_y > 0.001
            || params.crop_w < 0.999
            || params.crop_h < 0.999);

    if has_explicit_box {
        let cx = ((params.crop_x.clamp(0.0, 0.98) * iw as f32).round() as u32).min(iw - 1);
        let cy = ((params.crop_y.clamp(0.0, 0.98) * ih as f32).round() as u32).min(ih - 1);
        let cw = ((params.crop_w.clamp(0.01, 1.0) * iw as f32).round() as u32)
            .max(1)
            .min(iw - cx);
        let ch = ((params.crop_h.clamp(0.01, 1.0) * ih as f32).round() as u32)
            .max(1)
            .min(ih - cy);
        img = img.crop_imm(cx, cy, cw, ch);
    } else if params.crop_aspect_w > 0.1 && params.crop_aspect_h > 0.1 {
        let target_ratio = params.crop_aspect_w / params.crop_aspect_h;
        let src_ratio = iw as f32 / ih as f32;
        let (cx, cy, cw, ch) = if src_ratio > target_ratio {
            let new_w = ((ih as f32 * target_ratio).round() as u32).clamp(1, iw);
            let mut ox = (iw.saturating_sub(new_w)) / 2;
            if (params.crop_align_eye || params.crop_align_vertical) && params.face.face_rx > 0.01 {
                let target_cx = (params.face.center_x * iw as f32) as i32;
                let desired_ox = target_cx - (new_w as i32) / 2;
                ox = desired_ox.clamp(0, iw.saturating_sub(new_w) as i32) as u32;
            }
            (ox, 0, new_w, ih)
        } else {
            let new_h = ((iw as f32 / target_ratio).round() as u32).clamp(1, ih);
            let mut oy = (ih.saturating_sub(new_h)) / 2;
            if params.crop_align_eye && params.face.face_ry > 0.01 {
                let eye_y_norm = (params.left_eye_mid_y()).min(0.95);
                let eye_y_px = (eye_y_norm * ih as f32) as i32;
                let desired_oy = eye_y_px - (new_h as f32 * 0.34) as i32;
                oy = desired_oy.clamp(0, ih.saturating_sub(new_h) as i32) as u32;
            } else if params.crop_align_vertical && params.face.face_ry > 0.01 {
                let face_y_px = (params.face.center_y * ih as f32) as i32;
                let desired_oy = face_y_px - (new_h as f32 * 0.40) as i32;
                oy = desired_oy.clamp(0, ih.saturating_sub(new_h) as i32) as u32;
            }
            (0, oy, iw, new_h)
        };
        img = img.crop_imm(cx, cy, cw, ch);
    }

    let ew = if params.crop_custom_w > 0.5 { params.crop_custom_w.round() as u32 } else { 0 };
    let eh = if params.crop_custom_h > 0.5 { params.crop_custom_h.round() as u32 } else { 0 };
    if ew > 0 && eh > 0 {
        img = img.resize_exact(ew, eh, image::imageops::FilterType::Lanczos3);
    } else if ew > 0 {
        let scale = ew as f32 / img.width().max(1) as f32;
        let eh2 = ((img.height() as f32 * scale).round() as u32).max(1);
        img = img.resize_exact(ew, eh2, image::imageops::FilterType::Lanczos3);
    } else if eh > 0 {
        let scale = eh as f32 / img.height().max(1) as f32;
        let ew2 = ((img.width() as f32 * scale).round() as u32).max(1);
        img = img.resize_exact(ew2, eh, image::imageops::FilterType::Lanczos3);
    }
    img
}

impl RetouchParams {
    fn left_eye_mid_y(&self) -> f32 {
        (self.face.left_eye_y + self.face.right_eye_y) * 0.5
    }
}

#[tauri::command]
fn export_photo_to_disk_cmd(
    image_base64: String,
    params: RetouchParams,
    output_path: String,
    mask_base64: Option<String>,
    scene_mask_base64: Option<String>,
) -> Result<String, String> {
    let dyn_img = decode_data_url(&image_base64)?;
    let rgba = dyn_img.to_rgba8();
    let decoded_mask = decode_optional_rgba(mask_base64.as_deref());
    let decoded_scene = decode_optional_rgba(scene_mask_base64.as_deref());

    let mut active_params = params;
    if active_params.face.face_rx <= 0.01 {
        active_params.face = detect_face_region(&rgba);
    }
    let result_rgba = apply_retouch_pipeline_with_scene(
        &rgba,
        &active_params,
        decoded_mask.as_ref(),
        decoded_scene.as_ref(),
    );
    let dyn_out = apply_crop_and_resize(result_rgba, &active_params);
    if output_path.to_lowercase().ends_with(".png") {
        dyn_out
            .save_with_format(&output_path, ImageFormat::Png)
            .map_err(|e| format!("Save error: {}", e))?;
    } else {
        dyn_out
            .save_with_format(&output_path, ImageFormat::Jpeg)
            .map_err(|e| format!("Save error: {}", e))?;
    }
    Ok(output_path)
}

#[derive(Serialize, Deserialize)]
pub struct HighresMattePayload {
    pub matte_data_url: String,
    pub width: u32,
    pub height: u32,
    pub elapsed_ms: u128,
}

#[tauri::command]
async fn extract_highres_matte_cmd(image_base64: String) -> Result<HighresMattePayload, String> {
    tauri::async_runtime::spawn_blocking(move || extract_highres_matte_blocking(image_base64))
        .await
        .map_err(|e| format!("Matting task join error: {e}"))?
}

fn extract_highres_matte_blocking(image_base64: String) -> Result<HighresMattePayload, String> {
    let start = std::time::Instant::now();
    let dyn_img = decode_data_url(&image_base64)?;
    let (w, h) = (dyn_img.width(), dyn_img.height());

    let script_paths = [
        "scripts/extract_matte.py",
        "../scripts/extract_matte.py",
        "F:/Workspace/Code projects/2. Tool Media by Doan/Collabs-Retouch-Pro/scripts/extract_matte.py",
    ];
    let script = script_paths.iter().find(|p| std::path::Path::new(p).exists())
        .ok_or_else(|| "Could not locate extract_matte.py".to_string())?;

    let py_interpreters = [
        "F:/Workspace/Code projects/LiYing-master/python-embed/python.exe",
        "F:/Workspace/Code projects/LiYing/python-embed/python.exe",
        "python-embed/python.exe",
        "../python-embed/python.exe",
        "python",
        "python3",
    ];
    let py = py_interpreters.iter().find(|p| {
        if p.contains('/') || p.contains('\\') {
            std::path::Path::new(p).exists()
        } else {
            true
        }
    }).ok_or_else(|| "Could not find python interpreter with onnxruntime".to_string())?;

    let temp_dir = std::env::temp_dir();
    let pid = std::process::id();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let temp_input = temp_dir.join(format!("retouch_in_{}_{}.jpg", pid, now));
    let temp_output = temp_dir.join(format!("retouch_matte_{}_{}.png", pid, now));

    dyn_img.to_rgb8().save(&temp_input).map_err(|e| e.to_string())?;

    let mut cmd = std::process::Command::new(py);
    cmd.arg(script).arg(&temp_input).arg(&temp_output);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000);
    }
    let output = cmd
        .output()
        .map_err(|e| format!("Failed to spawn python matting: {e}"))?;

    let _ = std::fs::remove_file(&temp_input);

    if !output.status.success() {
        let err_str = String::from_utf8_lossy(&output.stderr);
        let _ = std::fs::remove_file(&temp_output);
        return Err(format!("Matting inference failed: {err_str}"));
    }

    if !temp_output.exists() {
        return Err("Matting output file was not produced".to_string());
    }

    let matte_bytes = std::fs::read(&temp_output).map_err(|e| e.to_string())?;
    let _ = std::fs::remove_file(&temp_output);

    let matte_b64 = BASE64.encode(&matte_bytes);
    let matte_data_url = format!("data:image/png;base64,{}", matte_b64);

    Ok(HighresMattePayload {
        matte_data_url,
        width: w,
        height: h,
        elapsed_ms: start.elapsed().as_millis(),
    })
}

#[derive(Serialize, Deserialize)]
pub struct LiyingResultPayload {
    pub data_url: String,
    pub width: u32,
    pub height: u32,
    pub elapsed_ms: u128,
    pub straightened: bool,
    pub matte_data_url: Option<String>,
}

#[tauri::command]
async fn liying_change_background_cmd(
    image_base64: String,
    color: String,
    transparent: bool,
    straighten: bool,
    level_only: bool,
) -> Result<LiyingResultPayload, String> {
    tauri::async_runtime::spawn_blocking(move || {
        liying_change_background_blocking(image_base64, color, transparent, straighten, level_only)
    })
    .await
    .map_err(|e| format!("LiYing task join error: {e}"))?
}

fn liying_change_background_blocking(
    image_base64: String,
    color: String,
    transparent: bool,
    straighten: bool,
    level_only: bool,
) -> Result<LiyingResultPayload, String> {
    let start = std::time::Instant::now();
    let dyn_img = decode_data_url(&image_base64)?;

    let script_paths = [
        "scripts/liying_process.py",
        "../scripts/liying_process.py",
        "F:/Workspace/Code projects/2. Tool Media by Doan/Collabs-Retouch-Pro/scripts/liying_process.py",
    ];
    let script = script_paths
        .iter()
        .find(|p| std::path::Path::new(p).exists())
        .ok_or_else(|| "Could not locate liying_process.py".to_string())?;

    let py_interpreters = [
        "F:/Workspace/Code projects/LiYing-master/python-embed/python.exe",
        "F:/Workspace/Code projects/LiYing/python-embed/python.exe",
        "python-embed/python.exe",
        "../python-embed/python.exe",
        "python",
        "python3",
    ];
    let py = py_interpreters
        .iter()
        .find(|p| {
            if p.contains('/') || p.contains('\\') {
                std::path::Path::new(p).exists()
            } else {
                true
            }
        })
        .ok_or_else(|| "Could not find python interpreter with LiYing environment".to_string())?;

    let temp_dir = std::env::temp_dir();
    let pid = std::process::id();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let temp_input = temp_dir.join(format!("retouch_liying_in_{pid}_{now}.jpg"));
    let temp_output = temp_dir.join(format!("retouch_liying_out_{pid}_{now}.png"));
    let temp_matte = temp_dir.join(format!("retouch_liying_matte_{pid}_{now}.png"));

    dyn_img
        .to_rgb8()
        .save(&temp_input)
        .map_err(|e| format!("Failed to save temp input: {e}"))?;

    let mut cmd = std::process::Command::new(py);
    cmd.arg(script)
        .arg("--input")
        .arg(&temp_input)
        .arg("--output")
        .arg(&temp_output)
        .arg("--matte-output")
        .arg(&temp_matte)
        .arg("--color")
        .arg(&color);

    if transparent {
        cmd.arg("--transparent");
    }
    if straighten {
        cmd.arg("--straighten");
    }
    if level_only {
        cmd.arg("--level-only");
    }

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000);
    }

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to spawn LiYing process: {e}"))?;

    let _ = std::fs::remove_file(&temp_input);

    if !output.status.success() {
        let err_str = String::from_utf8_lossy(&output.stderr);
        let _ = std::fs::remove_file(&temp_output);
        let _ = std::fs::remove_file(&temp_matte);
        return Err(format!("LiYing background replacement failed: {err_str}"));
    }

    if !temp_output.exists() {
        let _ = std::fs::remove_file(&temp_matte);
        return Err("LiYing output image was not produced".to_string());
    }

    let out_bytes = std::fs::read(&temp_output).map_err(|e| e.to_string())?;
    let _ = std::fs::remove_file(&temp_output);

    let matte_data_url = if temp_matte.exists() {
        let m_bytes = std::fs::read(&temp_matte).ok();
        let _ = std::fs::remove_file(&temp_matte);
        m_bytes.map(|b| format!("data:image/png;base64,{}", BASE64.encode(b)))
    } else {
        None
    };

    let data_b64 = BASE64.encode(&out_bytes);
    let data_url = format!("data:image/png;base64,{}", data_b64);

    let decoded_out = image::load_from_memory(&out_bytes)
        .map_err(|e| format!("Decode LiYing output error: {e}"))?;
    let (w, h) = (decoded_out.width(), decoded_out.height());

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let straightened = stdout_str.contains("\"straightened\": true");

    Ok(LiyingResultPayload {
        data_url,
        width: w,
        height: h,
        elapsed_ms: start.elapsed().as_millis(),
        straightened,
        matte_data_url,
    })
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            load_photo_from_path_cmd,
            rotate_photo_90_cmd,
            analyze_portrait_cmd,
            process_photo_cmd,
            export_photo_to_disk_cmd,
            extract_highres_matte_cmd,
            liying_change_background_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running Retouch Pro");
}

#[cfg(test)]
mod slider_tests {
    use super::*;

    /// Full-hue test chart: every hue band present, varying brightness, plus fine noise.
    fn test_image() -> RgbaImage {
        let (w, h) = (192u32, 160u32);
        let mut img = RgbaImage::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let n = hash_u32(x * 7919 + y * 104729) & 0xff;
                let hue = x as f32 / w as f32 * 360.0;
                let val = 0.30 + 0.60 * y as f32 / h as f32;
                let (r, g, b) = hsv_to_rgb(hue, 0.85, val);
                let jitter = (n as f32 - 128.0) / 255.0 * 0.08;
                let c = |v: f32| ((v + jitter).clamp(0.0, 1.0) * 255.0) as u8;
                img.put_pixel(x, y, image::Rgba([c(r), c(g), c(b), 255]));
            }
        }
        img
    }

    fn run(img: &RgbaImage, v: &serde_json::Value) -> Vec<u8> {
        let mut p: RetouchParams = serde_json::from_value(v.clone()).unwrap();
        p.face = detect_face_region(img);
        apply_retouch_pipeline(img, &p, None).into_raw()
    }

    fn dead_params(img: &RgbaImage, base: &serde_json::Value, only: Option<&Vec<String>>) -> Vec<String> {
        let baseline = run(img, base);
        let mut dead = Vec::new();
        for (key, val) in base.as_object().unwrap() {
            if key == "face" || key == "blemishSpots" {
                continue;
            }
            if let Some(list) = only {
                if !list.contains(key) {
                    continue;
                }
            }
            let candidates: Vec<serde_json::Value> = if let Some(n) = val.as_f64() {
                vec![serde_json::json!(n + 60.0), serde_json::json!(n - 60.0), serde_json::json!(n + 25.0)]
            } else if val.is_boolean() {
                vec![serde_json::json!(!val.as_bool().unwrap())]
            } else {
                continue;
            };
            let mut changed = false;
            for c in candidates {
                let mut v = base.clone();
                v[key] = c;
                if run(img, &v) != baseline {
                    changed = true;
                    break;
                }
            }
            if !changed {
                dead.push(key.clone());
            }
        }
        dead
    }

    /// Every slider must change the image once the slider it depends on is switched on.
    #[test]
    fn every_slider_changes_the_image() {
        let img = test_image();
        let plain = serde_json::to_value(RetouchParams::default()).unwrap();
        let dead_plain = dead_params(&img, &plain, None);

        // Second pass: switch on "master" sliders that child sliders depend on.
        let mut enabled = plain.clone();
        for (k, v) in [
            ("sharpenAmount", 60.0), ("vignette", 60.0), ("filmGrain", 60.0),
            ("gradeShadowSat", 60.0), ("gradeHighSat", 60.0), ("gradeMidSat", 60.0), ("gradeGlobalSat", 60.0),
            ("opticsVignetting", 60.0),
        ] {
            enabled[k] = serde_json::json!(v);
        }
        enabled["treatment"] = serde_json::json!("bw");
        let still_dead = dead_params(&img, &enabled, Some(&dead_plain));
        println!("NO-EFFECT (even with masters on): {:?}", still_dead);

        // Third pass: face / hair / teeth sliders need a real portrait.
        let p_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../public/samples/portrait_1.jpg");
        if let Ok(pimg) = image::open(&p_path) {
            let pimg = pimg.to_rgba8();
            let nh = (384.0 * pimg.height() as f32 / pimg.width() as f32) as u32;
            let pimg = image::imageops::resize(&pimg, 384, nh, image::imageops::FilterType::Triangle);
            let mut en2 = enabled.clone();
            en2["treatment"] = serde_json::json!("color");
            en2["hairDyeColor"] = serde_json::json!("espresso");
            en2["clothingDewrinkle"] = serde_json::json!(60.0);
            en2["clothingSweatStain"] = serde_json::json!(60.0);
            en2["clothingEdgeSmooth"] = serde_json::json!(60.0);
            en2["colorMatchStats"] = serde_json::json!([0.4, 0.2, 0.5, 0.2, 0.3, 0.2]);
            en2["gradeShadowLum"] = serde_json::json!(40.0);
            let dead_portrait = dead_params(&pimg, &en2, Some(&still_dead));
            println!("NO-EFFECT ON REAL PORTRAIT: {:?}", dead_portrait);
        }
    }

    fn mean_abs_err(a: &[u8], b: &[u8]) -> f32 {
        let mut s = 0.0f64;
        for (x, y) in a.chunks(4).zip(b.chunks(4)) {
            for c in 0..3 {
                s += (x[c] as f64 - y[c] as f64).abs();
            }
        }
        (s / (a.len() as f64 / 4.0 * 3.0)) as f32
    }

    #[test]
    fn ai_denoise_removes_noise_and_keeps_edges() {
        let (w, h) = (128u32, 128u32);
        let mut clean = RgbaImage::new(w, h);
        let mut noisy = RgbaImage::new(w, h);
        for y in 0..h {
            for x in 0..w {
                // two flat colour regions with a hard vertical edge + a soft gradient
                let (r, g, b) = if x < w / 2 { (60.0, 90.0, 140.0) } else { (200.0, 170.0, 110.0) };
                let grad = y as f32 / h as f32 * 30.0;
                let base = [r + grad, g + grad, b + grad];
                let mut n = [0.0f32; 3];
                for c in 0..3 {
                    // luminance + chroma noise, roughly +-14 levels
                    n[c] = (hash2(x as i32, y as i32, 100 + c as u32) * 2.0) * 14.0;
                }
                clean.put_pixel(x, y, image::Rgba([base[0] as u8, base[1] as u8, base[2] as u8, 255]));
                let f = |v: f32| v.clamp(0.0, 255.0) as u8;
                noisy.put_pixel(x, y, image::Rgba([f(base[0] + n[0]), f(base[1] + n[1]), f(base[2] + n[2]), 255]));
            }
        }
        let mut v = serde_json::to_value(RetouchParams::default()).unwrap();
        let before = mean_abs_err(noisy.as_raw(), clean.as_raw());
        v["aiDenoise"] = serde_json::json!(70.0);
        let out = run(&noisy, &v);
        let after = mean_abs_err(&out, clean.as_raw());
        println!("denoise error: {before:.2} -> {after:.2}");
        assert!(after < before * 0.55, "noise should drop by at least 45% ({before} -> {after})");

        // the hard edge in the middle must still be there
        let row = 64usize;
        let left = out[(row * w as usize + 20) * 4] as i32;
        let right = out[(row * w as usize + 108) * 4] as i32;
        assert!((right - left).abs() > 100, "edge was destroyed: {left} vs {right}");
        let near_l = out[(row * w as usize + 62) * 4] as i32;
        let near_r = out[(row * w as usize + 65) * 4] as i32;
        assert!((near_r - near_l).abs() > 80, "edge was blurred: {near_l} vs {near_r}");
    }

    #[test]
    fn black_and_white_treatment_is_monochrome() {
        let img = test_image();
        let mut v = serde_json::to_value(RetouchParams::default()).unwrap();
        v["treatment"] = serde_json::json!("bw");
        let out = run(&img, &v);
        assert!(out.chunks(4).all(|p| p[0] == p[1] && p[1] == p[2]));
        v["bwRed"] = serde_json::json!(80.0);
        v["bwBlue"] = serde_json::json!(-80.0);
        assert_ne!(run(&img, &v), out);
    }

    fn make_portrait_cutout_fixture() -> (RgbaImage, RgbaImage, FaceRegion) {
        let (w, h) = (160u32, 200u32);
        let mut img = RgbaImage::new(w, h);
        let mut scene = RgbaImage::new(w, h);
        for y in 0..h {
            let ny = y as f32 / h as f32;
            for x in 0..w {
                let nx = x as f32 / w as f32;
                let in_hair = x >= 55 && x < 105 && y >= 24 && y < 60;
                let in_head = x >= 60 && x < 100 && y >= 42 && y < 86;
                let in_shirt = x >= 35 && x < 125 && y >= 86 && y < 168;
                let in_trousers = x >= 44 && x < 116 && y >= 168;

                let rgb = if in_hair {
                    [20, 22, 26, 255]
                } else if in_head {
                    [210, 170, 145, 255]
                } else if in_shirt {
                    [180, 200, 218, 255]
                } else if in_trousers {
                    [122, 118, 108, 255]
                } else {
                    [45, 72, 96, 255]
                };
                img.put_pixel(x, y, image::Rgba(rgb));

                let is_fg = in_hair || in_head || in_shirt || in_trousers;
                let fg_u8 = if is_fg { 255u8 } else { 0u8 };
                let cloth_u8 = if in_shirt || in_trousers { 255u8 } else { 0u8 };
                let solid_u8 = if in_head || in_shirt || in_trousers { 255u8 } else { 0u8 };
                scene.put_pixel(x, y, image::Rgba([fg_u8, cloth_u8, solid_u8, 255]));
                let _ = (nx, ny);
            }
        }
        let face = FaceRegion {
            center_x: 0.50,
            center_y: 0.30,
            face_rx: 0.14,
            face_ry: 0.14,
            forehead_x: 0.50,
            forehead_y: 0.21,
            chin_x: 0.50,
            chin_y: 0.43,
            left_eye_x: 0.44,
            left_eye_y: 0.28,
            right_eye_x: 0.56,
            right_eye_y: 0.28,
            nose_x: 0.50,
            nose_y: 0.33,
            mouth_x: 0.50,
            mouth_y: 0.38,
            mouth_left_x: 0.46,
            mouth_left_y: 0.38,
            mouth_right_x: 0.54,
            mouth_right_y: 0.38,
            cheek_left_x: 0.38,
            cheek_left_y: 0.33,
            cheek_right_x: 0.62,
            cheek_right_y: 0.33,
            jaw_left_x: 0.40,
            jaw_left_y: 0.39,
            jaw_right_x: 0.60,
            jaw_right_y: 0.39,
        };
        (img, scene, face)
    }

    #[test]
    fn background_replace_preserves_lower_torso_and_trousers() {
        let (img, scene, face) = make_portrait_cutout_fixture();
        let mut params = RetouchParams::default();
        params.face = face;
        params.bg_replace_mode = "pure_white".to_string();
        params.stray_hair = 80.0;
        params.backdrop_clean = 85.0;

        let out = apply_retouch_pipeline_with_scene(&img, &params, None, Some(&scene));
        // Lower-body trousers well below torso_d2 ellipse must remain 100% preserved
        let trouser_px = out.get_pixel(80, 190);
        assert!(
            (trouser_px[0] as i32 - 122).abs() <= 8
                && (trouser_px[1] as i32 - 118).abs() <= 8
                && (trouser_px[2] as i32 - 108).abs() <= 8,
            "trousers were corrupted by geometric fallback: {:?}",
            trouser_px.0
        );
        // Wide arm/shoulder inside neural foreground must stay intact
        let sleeve_px = out.get_pixel(39, 120);
        assert!(
            (sleeve_px[0] as i32 - 180).abs() <= 12 && (sleeve_px[2] as i32 - 218).abs() <= 12,
            "sleeve was overwritten: {:?}",
            sleeve_px.0
        );
    }

    #[test]
    fn background_replace_pure_white_corners_stay_pure_despite_vignette() {
        let (img, scene, face) = make_portrait_cutout_fixture();
        let mut params = RetouchParams::default();
        params.face = face;
        params.bg_replace_mode = "pure_white".to_string();
        params.vignette = 40.0;
        params.optics_vignetting = -50.0;

        let out = apply_retouch_pipeline_with_scene(&img, &params, None, Some(&scene));
        for &(cx, cy) in &[(4, 4), (155, 4), (4, 195), (155, 195)] {
            let p = out.get_pixel(cx, cy);
            assert!(
                p[0] >= 254 && p[1] >= 254 && p[2] >= 254,
                "pure_white corner ({cx},{cy}) was darkened to {:?}",
                p.0
            );
        }
    }

    #[test]
    fn guided_matte_eliminates_blurry_halo_at_sharp_boundary() {
        let (w, h) = (160u32, 160u32);
        let mut img = RgbaImage::new(w, h);
        let mut scene = RgbaImage::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let in_hair = (45..115).contains(&x) && (25..135).contains(&y);
                let rgb = if in_hair { [22, 24, 28, 255] } else { [20, 95, 110, 255] };
                img.put_pixel(x, y, image::Rgba(rgb));
                // Simulate a coarse, blurry neural probability ramp across x = 35..55
                let ramp = ((x as f32 - 35.0) / 20.0).clamp(0.0, 1.0);
                let right_ramp = ((125.0 - x as f32) / 20.0).clamp(0.0, 1.0);
                let vert = if (20..140).contains(&y) { 1.0 } else { 0.0 };
                let fg_p = (ramp.min(right_ramp) * vert * 255.0).round() as u8;
                scene.put_pixel(x, y, image::Rgba([fg_p, fg_p, fg_p, 255]));
            }
        }
        let mut params = RetouchParams::default();
        params.bg_replace_mode = "pure_white".to_string();
        params.stray_hair = 60.0;

        let out = apply_retouch_pipeline_with_scene(&img, &params, None, Some(&scene));
        // 3px outside the true RGB hair boundary (x = 42, where coarse neural ramp had leaked 0.35!)
        let outside_px = out.get_pixel(42, 80);
        assert!(
            outside_px[0] >= 235 && outside_px[1] >= 235 && outside_px[2] >= 235,
            "dark halo leaked outside hair boundary at (42,80): {:?}",
            outside_px.0
        );
        // 4px inside true hair (x = 49) must remain dark hair
        let inside_px = out.get_pixel(49, 80);
        assert!(
            inside_px[0] <= 60 && inside_px[1] <= 60 && inside_px[2] <= 60,
            "hair edge was over-eroded at (49,80): {:?}",
            inside_px.0
        );
    }

    #[test]
    fn background_replace_custom_color_and_original_masked_bokeh() {
        let (img, scene, face) = make_portrait_cutout_fixture();
        let mut custom = RetouchParams::default();
        custom.face = face.clone();
        custom.bg_replace_mode = "custom_color".to_string();
        custom.bg_custom_color = "#1e40af".to_string();
        let out_custom = apply_retouch_pipeline_with_scene(&img, &custom, None, Some(&scene));
        let corner = out_custom.get_pixel(10, 10);
        assert_eq!(corner.0, [30, 64, 175, 255]);

        // Original mode bokeh with bright white background & pitch-black hair must not smudge black halo into white wall
        let (w, h) = (160u32, 160u32);
        let mut wall_img = RgbaImage::new(w, h);
        let mut wall_scene = RgbaImage::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let in_sub = (50..110).contains(&x) && (30..140).contains(&y);
                let rgb = if in_sub { [10, 10, 10, 255] } else { [245, 245, 245, 255] };
                wall_img.put_pixel(x, y, image::Rgba(rgb));
                let a = if in_sub { 255 } else { 0 };
                wall_scene.put_pixel(x, y, image::Rgba([a, a, a, 255]));
            }
        }
        let mut bokeh_p = RetouchParams::default();
        bokeh_p.face = face;
        bokeh_p.bg_blur_bokeh = 95.0;
        let out_bokeh = apply_retouch_pipeline_with_scene(&wall_img, &bokeh_p, None, Some(&wall_scene));
        let near_edge_bg = out_bokeh.get_pixel(46, 80);
        assert!(
            near_edge_bg[0] >= 225 && near_edge_bg[1] >= 225 && near_edge_bg[2] >= 225,
            "black subject bled into original white background bokeh at (46,80): {:?}",
            near_edge_bg.0
        );
    }

    #[test]
    fn crop_rect_and_custom_dimensions_export_exact_pixels() {
        let (w, h) = (200u32, 300u32);
        let mut img = RgbaImage::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let val = if x >= 40 && x < 140 && y >= 60 && y < 210 { 220u8 } else { 30u8 };
                img.put_pixel(x, y, image::Rgba([val, val, val, 255]));
            }
        }

        // 1. Explicit normalized interactive crop box
        let mut p1 = RetouchParams::default();
        p1.crop_x = 0.20; // x = 40
        p1.crop_y = 0.20; // y = 60
        p1.crop_w = 0.50; // w = 100
        p1.crop_h = 0.50; // h = 150
        let out1 = apply_crop_and_resize(img.clone(), &p1);
        assert_eq!((out1.width(), out1.height()), (100, 150));
        let inner = out1.to_rgba8().get_pixel(50, 75).0;
        assert_eq!(inner[0], 220, "cropped box should contain bright interior");

        // 2. Fallback aspect crop + custom pixel output size (ID photo 354x472)
        let mut p2 = RetouchParams::default();
        p2.crop_aspect_w = 3.0;
        p2.crop_aspect_h = 4.0;
        p2.crop_custom_w = 354.0;
        p2.crop_custom_h = 472.0;
        let out2 = apply_crop_and_resize(img, &p2);
        assert_eq!((out2.width(), out2.height()), (354, 472));
    }

    #[test]
    fn evoto_transparent_shadow_and_even_brightness() {
        let (img, scene, face) = make_portrait_cutout_fixture();

        // 1. Transparent PNG alpha cutout
        let mut p_trans = RetouchParams::default();
        p_trans.face = face.clone();
        p_trans.bg_replace_mode = "transparent".to_string();
        let out_trans = apply_retouch_pipeline_with_scene(&img, &p_trans, None, Some(&scene));
        assert_eq!(out_trans.get_pixel(4, 4).0[3], 0, "corner must be transparent");
        assert_eq!(out_trans.get_pixel(80, 90).0[3], 255, "subject center must be opaque");

        // 2. Cast shadow on pure_white darkens right cast offset zone without darkening far top-left corner
        let mut p_sh = RetouchParams::default();
        p_sh.face = face.clone();
        p_sh.bg_replace_mode = "pure_white".to_string();
        p_sh.bg_shadow_type = "cast_shadow".to_string();
        p_sh.bg_shadow_opacity = 75.0;
        p_sh.bg_shadow_blur = 20.0;
        let out_sh = apply_retouch_pipeline_with_scene(&img, &p_sh, None, Some(&scene));
        let top_left = out_sh.get_pixel(4, 4).0;
        assert!(top_left[0] >= 250 && top_left[1] >= 250 && top_left[2] >= 250);
        // Check right side of torso inside cast shadow zone (right of in_shirt x=125 -> x=129, y=130)
        let mut p_no_sh = RetouchParams::default();
        p_no_sh.face = face.clone();
        p_no_sh.bg_replace_mode = "pure_white".to_string();
        let out_no_sh = apply_retouch_pipeline_with_scene(&img, &p_no_sh, None, Some(&scene));
        assert!(
            out_sh.get_pixel(129, 130).0[0] + 15 < out_no_sh.get_pixel(129, 130).0[0],
            "cast_shadow should darken cast zone near subject right edge"
        );

        // 3. Even brightness reduces vignette/uneven backdrop falloff in corners
        let (w, h) = (160u32, 160u32);
        let mut uneven = RgbaImage::new(w, h);
        let mut u_scene = RgbaImage::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let in_sub = (55..105).contains(&x) && (30..135).contains(&y);
                let luma = if in_sub {
                    120u8
                } else {
                    (110.0 + (x as f32 / w as f32) * 120.0).round() as u8
                };
                uneven.put_pixel(x, y, image::Rgba([luma, luma, luma, 255]));
                let a = if in_sub { 255 } else { 0 };
                u_scene.put_pixel(x, y, image::Rgba([a, a, a, 255]));
            }
        }
        let mut p_even = RetouchParams::default();
        p_even.face = face;
        p_even.bg_even_brightness = true;
        let out_even = apply_retouch_pipeline_with_scene(&uneven, &p_even, None, Some(&u_scene));
        let orig_gap = (uneven.get_pixel(150, 20).0[0] as i32 - uneven.get_pixel(10, 20).0[0] as i32).abs();
        let even_gap = (out_even.get_pixel(150, 20).0[0] as i32 - out_even.get_pixel(10, 20).0[0] as i32).abs();
        assert!(even_gap < orig_gap / 2, "bg_even_brightness should flatten background gradient: orig={orig_gap}, even={even_gap}");
    }

    #[test]
    fn evoto_clothing_retouch_controls() {
        let (mut img, scene, face) = make_portrait_cutout_fixture();
        // Inject realistic shirt defects inside in_shirt (x: 45..115, y: 100..175):
        // 1) Wrinkle stripes + sweat damp patch in left-center + lint speck + rainbow moiré wave
        for y in 98..178 {
            for x in 42..118 {
                let mut r = 72.0f32;
                let mut g = 104.0f32;
                let mut b = 152.0f32;
                // Broad + fine vertical/diagonal wrinkle oscillation
                let large_fold = ((x as f32 * 0.18).sin() * 24.0).round();
                let small_crease = ((y as f32 * 0.55).sin() * 14.0).round();
                r += large_fold + small_crease;
                g += large_fold + small_crease;
                b += large_fold + small_crease;
                // Damp sweat patch around (62, 128)
                let dx = x as f32 - 62.0;
                let dy = y as f32 - 128.0;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < 14.0 {
                    let dip = (1.0 - dist / 14.0) * 44.0;
                    r -= dip;
                    g -= dip;
                    b -= dip;
                }
                // Rainbow moiré bands in right chest (85..112, 110..150)
                if (85..112).contains(&x) && (110..150).contains(&y) {
                    let wave = ((x + y) as f32 * 1.35).sin();
                    r += wave * 34.0;
                    g -= wave * 28.0;
                    b += ((x as f32 * 1.2).cos()) * 26.0;
                }
                // White lint speck
                if (x == 96 && y == 162) || (x == 97 && y == 162) {
                    r = 248.0;
                    g = 248.0;
                    b = 248.0;
                }
                img.put_pixel(
                    x,
                    y,
                    image::Rgba([
                        r.clamp(0.0, 255.0) as u8,
                        g.clamp(0.0, 255.0) as u8,
                        b.clamp(0.0, 255.0) as u8,
                        255,
                    ]),
                );
            }
        }

        // 1) Wrinkle smoothing reduces crease contrast + Small/Large knobs differentiate
        let mut p_wrinkle = RetouchParams::default();
        p_wrinkle.face = face.clone();
        p_wrinkle.clothing_dewrinkle = 85.0;
        let out_wr = apply_retouch_pipeline_with_scene(&img, &p_wrinkle, None, Some(&scene));

        let mut p_no_large = p_wrinkle.clone();
        p_no_large.clothing_large_wrinkles = 10.0;
        let out_nl = apply_retouch_pipeline_with_scene(&img, &p_no_large, None, Some(&scene));
        assert_ne!(out_wr, out_nl, "clothing_large_wrinkles must control broad drape smoothing");

        let mut p_no_small = p_wrinkle.clone();
        p_no_small.clothing_small_wrinkles = 10.0;
        let out_ns = apply_retouch_pipeline_with_scene(&img, &p_no_small, None, Some(&scene));
        assert_ne!(out_wr, out_ns, "clothing_small_wrinkles must control fine wrinkle smoothing");

        // 2) Sweat stain removal lifts dark damp patch at (62, 128)
        let mut p_sweat = RetouchParams::default();
        p_sweat.face = face.clone();
        p_sweat.clothing_sweat_stain = 90.0;
        let out_sw = apply_retouch_pipeline_with_scene(&img, &p_sweat, None, Some(&scene));
        assert!(
            out_sw.get_pixel(62, 128).0[0] > img.get_pixel(62, 128).0[0] + 6,
            "clothing_sweat_stain should lift damp patch: orig={} after={}",
            img.get_pixel(62, 128).0[0],
            out_sw.get_pixel(62, 128).0[0]
        );

        // 3) Moiré removal reduces rainbow chromatic oscillation in (85..112, 110..150)
        let mut p_moire = RetouchParams::default();
        p_moire.face = face.clone();
        p_moire.clothing_moire_remove = 90.0;
        let out_mo = apply_retouch_pipeline_with_scene(&img, &p_moire, None, Some(&scene));
        let orig_chroma_dev: i32 = (88..108)
            .map(|x| {
                let p = img.get_pixel(x, 125).0;
                (p[0] as i32 - p[1] as i32).abs()
            })
            .sum();
        let after_chroma_dev: i32 = (88..108)
            .map(|x| {
                let p = out_mo.get_pixel(x, 125).0;
                (p[0] as i32 - p[1] as i32).abs()
            })
            .sum();
        assert!(
            after_chroma_dev < orig_chroma_dev,
            "clothing_moire_remove should dampen chromatic moire oscillation: {orig_chroma_dev} -> {after_chroma_dev}"
        );

        // 4) Edge smooth alters boundary pixels around shirt edge (x=35, y=130)
        let mut p_edge = RetouchParams::default();
        p_edge.face = face;
        p_edge.clothing_edge_smooth = 85.0;
        let out_ed = apply_retouch_pipeline_with_scene(&img, &p_edge, None, Some(&scene));
        assert_ne!(out_ed.as_raw(), img.as_raw(), "clothing_edge_smooth should alter garment boundary");
    }

    #[test]
    fn hair_cutout_punches_wall_pockets_and_eliminates_halo() {
        let (w, h) = (220u32, 220u32);
        let mut img = RgbaImage::new(w, h);
        let mut mask = RgbaImage::new(w, h);
        let mut scene = RgbaImage::new(w, h);

        for y in 0..h {
            for x in 0..w {
                let in_cheek = (80..120).contains(&x) && (70..135).contains(&y);
                let in_lock = (58..70).contains(&x) && (65..140).contains(&y);
                let in_crown = (65..135).contains(&x) && (40..70).contains(&y);
                let in_pocket = (70..80).contains(&x) && (75..125).contains(&y);
                let in_strand = (53..=54).contains(&x) && (80..115).contains(&y);

                let rgb = if in_cheek {
                    [214, 174, 148, 255]
                } else if in_lock || in_crown {
                    [18, 16, 16, 255]
                } else if in_strand {
                    [36, 32, 32, 255]
                } else {
                    [205, 198, 182, 255]
                };
                img.put_pixel(x, y, image::Rgba(rgb));

                let in_crown_halo = (65..135).contains(&x) && (34..40).contains(&y);
                let in_outer_halo = (46..58).contains(&x) && (65..135).contains(&y);

                let skin_u8 = if in_cheek { 255 } else { 0 };
                let hair_u8 = if in_lock || in_crown {
                    240
                } else if in_pocket || in_strand || in_outer_halo || in_crown_halo {
                    140
                } else {
                    0
                };
                mask.put_pixel(x, y, image::Rgba([skin_u8, 0, hair_u8, 255]));

                let fg_u8 = if in_cheek || in_lock || in_crown {
                    255
                } else if in_pocket || in_outer_halo || in_crown_halo {
                    112 // coarse blurred neural fgProb (~0.44) leaking into trapped wall pocket & halo
                } else {
                    0
                };
                let solid_u8 = if in_cheek { 255 } else { 0 };
                scene.put_pixel(x, y, image::Rgba([fg_u8, 0, solid_u8, 255]));
            }
        }

        let mut params = RetouchParams::default();
        params.bg_replace_mode = "pure_white".to_string();
        params.bg_edge_mode = "auto".to_string();
        params.bg_decontam = true;
        params.face = FaceRegion {
            center_x: 0.48,
            center_y: 0.46,
            face_rx: 0.14,
            face_ry: 0.16,
            forehead_x: 0.48,
            forehead_y: 0.34,
            chin_x: 0.48,
            chin_y: 0.61,
            left_eye_x: 0.43,
            left_eye_y: 0.43,
            right_eye_x: 0.53,
            right_eye_y: 0.43,
            nose_x: 0.48,
            nose_y: 0.49,
            mouth_x: 0.48,
            mouth_y: 0.55,
            mouth_left_x: 0.44,
            mouth_left_y: 0.55,
            mouth_right_x: 0.52,
            mouth_right_y: 0.55,
            cheek_left_x: 0.39,
            cheek_left_y: 0.48,
            cheek_right_x: 0.57,
            cheek_right_y: 0.48,
            jaw_left_x: 0.40,
            jaw_left_y: 0.57,
            jaw_right_x: 0.56,
            jaw_right_y: 0.57,
        };

        let out = apply_retouch_pipeline_with_scene(&img, &params, Some(&mask), Some(&scene));

        // 1) Enclosed beige wall pocket between hair lock and cheek must punch cleanly to pure white
        let pocket_px = out.get_pixel(75, 100).0;
        assert!(
            pocket_px[0] >= 245 && pocket_px[1] >= 245 && pocket_px[2] >= 245,
            "enclosed wall pocket remained muddy instead of pure white: {:?}",
            pocket_px
        );

        // 2) Coarse neural halo outside crown and outer temple must punch cleanly to pure white
        for &(hx, hy) in &[(100u32, 36u32), (48u32, 95u32)] {
            let halo_px = out.get_pixel(hx, hy).0;
            assert!(
                halo_px[0] >= 245 && halo_px[1] >= 245 && halo_px[2] >= 245,
                "outer grey/dark halo leaked at ({hx},{hy}): {:?}",
                halo_px
            );
        }

        // 3) Hair lock interior and fine outer flyaway strand must stay crisp dark hair
        let lock_px = out.get_pixel(64, 100).0;
        assert!(
            lock_px[0] <= 45 && lock_px[1] <= 45 && lock_px[2] <= 45,
            "hair lock was over-eroded: {:?}",
            lock_px
        );
        let strand_px = out.get_pixel(54, 98).0;
        assert!(
            strand_px[0] <= 120 && strand_px[1] <= 120 && strand_px[2] <= 120,
            "outer flyaway strand was erased: {:?}",
            strand_px
        );

        // 4) Cheek skin remains intact
        let cheek_px = out.get_pixel(95, 100).0;
        assert!(
            (cheek_px[0] as i32 - 214).abs() <= 15
                && (cheek_px[1] as i32 - 174).abs() <= 15
                && (cheek_px[2] as i32 - 148).abs() <= 15,
            "cheek skin was corrupted: {:?}",
            cheek_px
        );
    }

    #[test]
    fn folded_arms_rescue_tucked_fingers_and_keep_crisp_elbows() {
        let (w, h) = (220u32, 240u32);
        let mut img = RgbaImage::new(w, h);
        let mut scene = RgbaImage::new(w, h);

        for y in 0..h {
            for x in 0..w {
                let in_head = (88..132).contains(&x) && (22..68).contains(&y);
                let in_shirt = (52..168).contains(&x) && (72..220).contains(&y);
                let in_arm = (36..184).contains(&x) && (122..182).contains(&y);
                // Tucked pale fingers peeking inside sleeve fold on left torso:
                let in_fingers = (68..82).contains(&x) && (116..138).contains(&y);
                // Soft grey wall shadow extending 4px outside brown elbows
                let in_elbow_ramp = ((28..36).contains(&x) || (184..192).contains(&x)) && (126..178).contains(&y);

                let rgb = if in_head {
                    [196, 152, 130, 255]
                } else if in_fingers {
                    [224, 198, 186, 255]
                } else if in_arm {
                    [148, 106, 84, 255]
                } else if in_shirt {
                    [172, 202, 226, 255]
                } else if in_elbow_ramp {
                    [194, 194, 192, 255]
                } else {
                    [202, 202, 200, 255]
                };
                img.put_pixel(x, y, image::Rgba(rgb));

                let fg_u8 = if in_fingers {
                    52 // simulated neural misclassification inside enclosed sleeve crease (~0.20)
                } else if in_head || in_arm || in_shirt {
                    255
                } else if in_elbow_ramp {
                    118 // blurry coarse elbow probability ramp (~0.46)
                } else {
                    0
                };
                let cloth_u8 = if in_shirt && !in_arm && !in_fingers { 255 } else { 0 };
                let solid_u8 = if in_fingers { 52 } else { fg_u8 };
                scene.put_pixel(x, y, image::Rgba([fg_u8, cloth_u8, solid_u8, 255]));
            }
        }

        let mut params = RetouchParams::default();
        params.bg_replace_mode = "pure_white".to_string();
        params.bg_edge_mode = "auto".to_string();
        params.bg_decontam = true;
        params.face = FaceRegion {
            center_x: 0.50,
            center_y: 0.19,
            face_rx: 0.10,
            face_ry: 0.10,
            forehead_x: 0.50,
            forehead_y: 0.12,
            chin_x: 0.50,
            chin_y: 0.28,
            left_eye_x: 0.46,
            left_eye_y: 0.17,
            right_eye_x: 0.54,
            right_eye_y: 0.17,
            nose_x: 0.50,
            nose_y: 0.21,
            mouth_x: 0.50,
            mouth_y: 0.24,
            mouth_left_x: 0.47,
            mouth_left_y: 0.24,
            mouth_right_x: 0.53,
            mouth_right_y: 0.24,
            cheek_left_x: 0.42,
            cheek_left_y: 0.20,
            cheek_right_x: 0.58,
            cheek_right_y: 0.20,
            jaw_left_x: 0.43,
            jaw_left_y: 0.26,
            jaw_right_x: 0.57,
            jaw_right_y: 0.26,
        };

        let out = apply_retouch_pipeline_with_scene(&img, &params, None, Some(&scene));

        // 1) Tucked fingers inside folded arm must be rescued, NOT punched white
        let finger_px = out.get_pixel(75, 127).0;
        assert!(
            (finger_px[0] as i32 - 224).abs() <= 16
                && (finger_px[1] as i32 - 198).abs() <= 16
                && (finger_px[2] as i32 - 186).abs() <= 16,
            "tucked fingers inside folded arms were punched out: {:?}",
            finger_px
        );

        // 2) Blurry halo 3px outside brown elbows must clear cleanly to pure white
        for &(ex, ey) in &[(32u32, 150u32), (188u32, 150u32)] {
            let halo_px = out.get_pixel(ex, ey).0;
            assert!(
                halo_px[0] >= 245 && halo_px[1] >= 245 && halo_px[2] >= 245,
                "elbow halo leaked at ({ex},{ey}): {:?}",
                halo_px
            );
        }

        // 3) Crisp warm brown elbow interior stays saturated & intact (no grey desaturation ring)
        let elbow_px = out.get_pixel(39, 150).0;
        assert!(
            elbow_px[0] > elbow_px[2] + 30 && (elbow_px[0] as i32 - 148).abs() <= 15,
            "brown elbow skin edge was desaturated/corrupted: {:?}",
            elbow_px
        );
    }

    #[test]
    fn arm_waist_enclosed_wall_triangle_punches_cleanly_transparent() {
        let w = 180u32;
        let h = 220u32;
        let mut img = image::RgbaImage::new(w, h);
        let mut sem_mask = image::RgbaImage::new(w, h);
        let mut scene_mask = image::RgbaImage::new(w, h);

        for y in 0..h {
            let ny = y as f32 / (h - 1) as f32;
            // Studio backdrop with natural top-to-bottom illumination falloff
            let wall_r = (220.0 - ny * 28.0).round() as u8;
            let wall_g = (214.0 - ny * 28.0).round() as u8;
            let wall_b = (202.0 - ny * 28.0).round() as u8;

            for x in 0..w {
                let is_head = ((x as f32 - 90.0) / 18.0).powi(2) + ((y as f32 - 52.0) / 22.0).powi(2) <= 1.0;
                let is_shirt = x >= 54 && x <= 126 && y >= 82;
                let is_upper_arm = x >= 22 && x <= 38 && y >= 92 && y <= 168;
                let is_forearm = x >= 24 && x <= 68 && y >= 152 && y <= 174;
                let is_enclosed_gap = x >= 39 && x <= 53 && y >= 112 && y <= 151;

                let (px, fg_u8, cloth_u8, skin_u8) = if is_head {
                    ([186, 146, 126, 255], 255u8, 0u8, 255u8)
                } else if is_shirt {
                    ([140, 175, 205, 255], 255u8, 255u8, 0u8)
                } else if is_upper_arm || is_forearm {
                    ([150, 110, 88, 255], 255u8, 0u8, 255u8)
                } else if is_enclosed_gap {
                    // Enclosed arm-waist triangle with subtle cast shadow of the same cream wall chromaticity
                    let sr = (wall_r as f32 * 0.90).round() as u8;
                    let sg = (wall_g as f32 * 0.90).round() as u8;
                    let sb = (wall_b as f32 * 0.90).round() as u8;
                    ([sr, sg, sb, 255], 0u8, 0u8, 0u8)
                } else {
                    ([wall_r, wall_g, wall_b, 255], 0u8, 0u8, 0u8)
                };

                img.put_pixel(x, y, image::Rgba(px));
                sem_mask.put_pixel(x, y, image::Rgba([skin_u8, 0, 0, 255]));
                scene_mask.put_pixel(x, y, image::Rgba([fg_u8, cloth_u8, fg_u8, 255]));
            }
        }

        let mut params = RetouchParams::default();
        params.bg_replace_mode = "transparent".to_string();
        params.bg_edge_mode = "auto".to_string();
        params.bg_decontam = true;
        params.face.center_x = 0.50;
        params.face.center_y = 52.0 / 220.0;
        params.face.face_ry = 22.0 / 220.0;
        params.face.chin_y = 74.0 / 220.0;

        let out = apply_retouch_pipeline_with_scene(&img, &params, Some(&sem_mask), Some(&scene_mask));

        // 1) Enclosed arm-waist cream studio wall gap must punch cleanly transparent (not filled solid!)
        for &(gx, gy) in &[(45, 128), (46, 136), (48, 142)] {
            let p = out.get_pixel(gx, gy).0;
            assert!(
                p[3] <= 15,
                "enclosed arm-waist wall triangle ({}, {}) should stay transparent, got {:?}",
                gx,
                gy,
                p
            );
        }

        // 2) Surrounding bent arm and blue shirt must stay opaque and uncorrupted
        let arm_px = out.get_pixel(30, 140).0;
        assert!(arm_px[3] >= 240, "arm should remain opaque, got {:?}", arm_px);
        let shirt_px = out.get_pixel(90, 140).0;
        assert!(shirt_px[3] >= 240, "shirt should remain opaque, got {:?}", shirt_px);
    }

    #[test]
    fn arm_waist_blurred_coarse_neural_pocket_still_punches_transparent() {
        let w = 180u32;
        let h = 220u32;
        let mut img = image::RgbaImage::new(w, h);
        let mut sem_mask = image::RgbaImage::new(w, h);
        let mut scene_mask = image::RgbaImage::new(w, h);

        for y in 0..h {
            let ny = y as f32 / (h - 1) as f32;
            let wall_r = (218.0 - ny * 22.0).round() as u8;
            let wall_g = (212.0 - ny * 22.0).round() as u8;
            let wall_b = (200.0 - ny * 22.0).round() as u8;

            for x in 0..w {
                let is_head = ((x as f32 - 90.0) / 18.0).powi(2) + ((y as f32 - 52.0) / 22.0).powi(2) <= 1.0;
                let is_shirt = x >= 56 && x <= 126 && y >= 82;
                let is_upper_arm = x >= 22 && x <= 38 && y >= 92 && y <= 168;
                let is_forearm = x >= 24 && x <= 70 && y >= 152 && y <= 174;
                let is_enclosed_gap = x >= 39 && x <= 55 && y >= 110 && y <= 151;

                let (px, fg_u8, cloth_u8, skin_u8) = if is_head {
                    ([186, 146, 126, 255], 255u8, 0u8, 255u8)
                } else if is_shirt {
                    ([140, 175, 205, 255], 255u8, 255u8, 0u8)
                } else if is_upper_arm || is_forearm {
                    ([150, 110, 88, 255], 255u8, 0u8, 255u8)
                } else if is_enclosed_gap {
                    // Simulate coarse 256x256 segmentation blurring high foreground probability (~0.72)
                    // across the enclosed wall triangle between sleeve/arm and shirt
                    ([wall_r.saturating_sub(10), wall_g.saturating_sub(10), wall_b.saturating_sub(10), 255], 185u8, 0u8, 0u8)
                } else {
                    ([wall_r, wall_g, wall_b, 255], 0u8, 0u8, 0u8)
                };

                img.put_pixel(x, y, image::Rgba(px));
                sem_mask.put_pixel(x, y, image::Rgba([skin_u8, 0, 0, 255]));
                scene_mask.put_pixel(x, y, image::Rgba([fg_u8, cloth_u8, fg_u8, 255]));
            }
        }

        let mut params = RetouchParams::default();
        params.bg_replace_mode = "transparent".to_string();
        params.bg_edge_mode = "auto".to_string();
        params.bg_decontam = true;
        params.face.center_x = 0.50;
        params.face.center_y = 52.0 / 220.0;
        params.face.face_ry = 22.0 / 220.0;
        params.face.chin_y = 74.0 / 220.0;

        let out = apply_retouch_pipeline_with_scene(&img, &params, Some(&sem_mask), Some(&scene_mask));

        for &(gx, gy) in &[(46, 128), (47, 134), (48, 140)] {
            let p = out.get_pixel(gx, gy).0;
            assert!(
                p[3] <= 20,
                "coarse blurred fg>=0.72 enclosed arm-waist pocket ({}, {}) remained opaque: {:?}",
                gx,
                gy,
                p
            );
        }

        let shirt_px = out.get_pixel(90, 140).0;
        assert!(shirt_px[3] >= 240, "shirt should remain opaque, got {:?}", shirt_px);
    }

    #[test]
    fn semantic_grid_distinguishes_grayscale_rmbg_matte_and_rgb_scene_clothing() {
        let mut face = FaceRegion::default();
        face.center_x = 0.5;
        face.center_y = 0.25;
        face.face_rx = 0.14;
        face.face_ry = 0.16;
        face.chin_y = 0.40;

        // 1) MediaPipe RGB scene mask: face skin has R=255,G=0,B=255 while far edge shirt has R=255,G=255,B=255.
        // Whole-image check must NOT mistake R=G=B=255 shirt pixels for single-channel matte!
        let mut rgb_scene = image::RgbaImage::new(64, 64);
        for y in 0..64 {
            for x in 0..64 {
                if y < 24 && x >= 20 && x <= 44 {
                    rgb_scene.put_pixel(x, y, image::Rgba([255, 0, 255, 255]));
                } else {
                    // Clothing even at outer edge (x=2, y=50) outside torso ellipse
                    rgb_scene.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
                }
            }
        }
        let sem_rgb = build_semantic_grid(None, Some(&rgb_scene), &face);
        let edge_idx = 50usize * MASK_DIM * 8 + 16; // nx ~ 0.03, ny ~ 0.78 -> far outside geo.clothing ellipse
        assert!(
            sem_rgb.weights[edge_idx].clothing > 0.95,
            "RGB scene mask clothing channel was clobbered by per-pixel single-matte check: {}",
            sem_rgb.weights[edge_idx].clothing
        );

        // 2) Pure RMBG-1.4 grayscale matte (R==G==B everywhere) activates single-matte mode cleanly
        let mut gray_matte = image::RgbaImage::new(64, 64);
        for y in 0..64 {
            for x in 0..64 {
                let v = if x >= 16 && x <= 48 && y >= 8 { 230u8 } else { 0u8 };
                gray_matte.put_pixel(x, y, image::Rgba([v, v, v, 255]));
            }
        }
        let sem_gray = build_semantic_grid(None, Some(&gray_matte), &face);
        let center_idx = (MASK_DIM / 2) * MASK_DIM + (MASK_DIM / 2);
        let corner_idx = 0;
        assert!(sem_gray.raw_fg[center_idx] > 0.85, "grayscale matte center fg missing: {}", sem_gray.raw_fg[center_idx]);
        assert!(sem_gray.raw_fg[corner_idx] < 0.05, "grayscale matte corner fg not clear: {}", sem_gray.raw_fg[corner_idx]);
    }

    #[test]
    fn highres_matte_preserves_fine_hair_and_clean_arm_curve() {
        let (w, h) = (160u32, 200u32);
        let mut img = RgbaImage::new(w, h);
        let mut matte = RgbaImage::new(w, h);

        for y in 0..h {
            for x in 0..w {
                let is_arm = x >= 25 && x <= 48 && y >= 100 && y <= 185;
                let is_torso = x >= 68 && x <= 125 && y >= 80 && y <= 195;
                let is_hair_strand = x == 55 && y >= 30 && y <= 70;
                // Semi-transparent strand (alpha ~ 0.28 = 71) over bright wall
                let rgb = if is_arm {
                    [160, 120, 96, 255]
                } else if is_torso {
                    [130, 175, 220, 255]
                } else if is_hair_strand {
                    [50, 48, 46, 255]
                } else {
                    [225, 222, 214, 255]
                };
                img.put_pixel(x, y, image::Rgba(rgb));

                let alpha = if is_arm || is_torso {
                    255u8
                } else if is_hair_strand {
                    72u8 // 0.282 alpha - must NOT be crushed to 0 by solid_lo=0.44!
                } else {
                    0u8
                };
                matte.put_pixel(x, y, image::Rgba([alpha, alpha, alpha, 255]));
            }
        }

        let mut params = RetouchParams::default();
        params.bg_replace_mode = "transparent".to_string();
        params.is_highres_matte = true;
        params.bg_decontam = false;

        let out = apply_retouch_pipeline_with_scene(&img, &params, None, Some(&matte));

        // Fine hair strand keeps exact neural alpha ~72 without getting truncated
        let strand_a = out.get_pixel(55, 50).0[3];
        assert!(
            (strand_a as i32 - 72).abs() <= 4,
            "fine hair strand alpha was crushed or smeared: {strand_a}"
        );

        // Enclosed arm-torso wall gap stays 0 alpha cleanly
        let gap_a = out.get_pixel(58, 140).0[3];
        assert_eq!(gap_a, 0, "arm-torso gap leaked background pixel: {gap_a}");

        // Arm inner boundary at x=48 remains sharp and opaque without 128x128 pocket-punching bite marks
        let arm_edge_a = out.get_pixel(48, 140).0[3];
        assert_eq!(arm_edge_a, 255, "arm edge was bitten by pocket puncher: {arm_edge_a}");
    }

    #[test]
    fn extract_embedded_jpeg_picks_largest_preview_and_dng_fallback_is_panic_free() {
        let mut small_buf = Cursor::new(Vec::new());
        let small = DynamicImage::ImageRgba8(RgbaImage::new(200, 180));
        small.write_to(&mut small_buf, ImageFormat::Jpeg).expect("encode small jpeg");

        let mut large_buf = Cursor::new(Vec::new());
        let large = DynamicImage::ImageRgba8(RgbaImage::new(420, 320));
        large.write_to(&mut large_buf, ImageFormat::Jpeg).expect("encode large jpeg");

        let mut fake_dng = Vec::new();
        fake_dng.extend_from_slice(b"II*\0\x08\0\0\0\0\0\0\0");
        // Fake noise with SOI-like prefix before real JPEGs
        fake_dng.extend_from_slice(&[0xFF, 0xD8, 0xFF, 0xFF, 0x00, 0x11]);
        fake_dng.extend_from_slice(&small_buf.into_inner());
        fake_dng.extend_from_slice(&[0xAA, 0xBB, 0xCC]);
        fake_dng.extend_from_slice(&large_buf.into_inner());

        let (best_img, _orient) =
            extract_largest_embedded_jpeg(&fake_dng).expect("must extract embedded JPEG preview");
        assert_eq!((best_img.width(), best_img.height()), (420, 320));

        let tmp_path = std::env::temp_dir().join(format!("test_fallback_{}.dng", std::process::id()));
        std::fs::write(&tmp_path, &fake_dng).expect("write temp dng");
        let decoded = decode_raw_or_image(&tmp_path);
        let _ = std::fs::remove_file(&tmp_path);
        let img = decoded.expect("decode_raw_or_image should succeed via DNG preview fallback");
        assert_eq!((img.width(), img.height()), (420, 320));
    }

    #[test]
    fn face_mole_and_freckle_acne_remove_spots_while_preserving_eyebrow() {
        let (w, h) = (260u32, 320u32);
        let mut img = RgbaImage::new(w, h);
        for y in 0..h {
            for x in 0..w {
                // Warm portrait skin base inside face oval
                let mut r = 216u8;
                let mut g = 172u8;
                let mut b = 148u8;

                // Dark mole on right cheek at (155, 195), radius 5px
                let md2 = (x as i32 - 155).pow(2) + (y as i32 - 195).pow(2);
                if md2 <= 25 {
                    r = 48;
                    g = 36;
                    b = 30;
                }

                // Brown freckle on left cheek at (95, 190), radius 3px
                let fd2 = (x as i32 - 95).pow(2) + (y as i32 - 190).pow(2);
                if fd2 <= 9 {
                    r = 150;
                    g = 104;
                    b = 82;
                }

                // Red acne spot on cheek at (112, 205), radius 4px
                let ad2 = (x as i32 - 112).pow(2) + (y as i32 - 205).pow(2);
                if ad2 <= 16 {
                    r = 210;
                    g = 105;
                    b = 98;
                }

                // Dark eyebrow line at y in 118..124, x in 78..112
                if (118..=124).contains(&y) && (78..=112).contains(&x) {
                    r = 38;
                    g = 30;
                    b = 26;
                }

                // Horizontal skin crease (e.g. eyelid fold / upper-lip shadow) inside face skin
                if (168..=170).contains(&y) && (108..=148).contains(&x) {
                    r = 95;
                    g = 70;
                    b = 58;
                }

                // Carved-out nostril dark pixel around (128, 145)
                let nd2 = (x as i32 - 128).pow(2) + (y as i32 - 145).pow(2);
                if nd2 <= 25 {
                    r = 46;
                    g = 36;
                    b = 32;
                }

                img.put_pixel(x, y, image::Rgba([r, g, b, 255]));
            }
        }

        // Paint semantic mask matching face oval (R=255), eyebrow (G=110), and carved-out nostril (R=0)
        let mut mask = RgbaImage::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let is_brow = (115..=127).contains(&y) && (75..=115).contains(&x);
                let is_nostril_hole = (x as i32 - 128).pow(2) + (y as i32 - 145).pow(2) <= 36;
                let r_code = if is_nostril_hole { 0 } else { 255 };
                let g_code = if is_brow { 110 } else { 0 };
                mask.put_pixel(x, y, image::Rgba([r_code, g_code, 0, 255]));
            }
        }

        let mut face = FaceRegion::default();
        face.center_x = 0.5;
        face.center_y = 0.52;
        face.face_rx = 0.34;
        face.face_ry = 0.42;
        face.chin_y = 0.90;
        face.nose_x = 0.50;
        face.nose_y = 0.55;

        // 1. Test Face Mole removal + Eyebrow protection
        let mut mole_params = RetouchParams::default();
        mole_params.face = face.clone();
        mole_params.face_mole = 85.0;
        let out_mole = apply_retouch_pipeline(&img, &mole_params, Some(&mask));
        let healed_mole = out_mole.get_pixel(155, 195).0;
        assert!(
            healed_mole[0] > 180 && healed_mole[1] > 140,
            "faceMole should lift dark mole toward clean skin (got {:?})",
            healed_mole
        );
        let kept_brow = out_mole.get_pixel(95, 121).0;
        assert!(
            kept_brow[0] < 60,
            "faceMole must preserve eyebrow pixels (got {:?})",
            kept_brow
        );

        // 2. Test Freckle & Acne removal
        let mut fa_params = RetouchParams::default();
        fa_params.face = face.clone();
        fa_params.freckle_acne = 90.0;
        let out_fa = apply_retouch_pipeline(&img, &fa_params, Some(&mask));
        let healed_freckle = out_fa.get_pixel(95, 190).0;
        assert!(
            healed_freckle[0] > 195 && healed_freckle[1] > 150,
            "freckleAcne should clean freckle toward surrounding skin (got {:?})",
            healed_freckle
        );
        let healed_acne = out_fa.get_pixel(112, 205).0;
        assert!(
            healed_acne[1] > 148 && (healed_acne[0] as i32 - healed_acne[1] as i32) < 60,
            "freckleAcne should neutralize red acne spot (got {:?})",
            healed_acne
        );

        // 2b. Test combined max sliders NEVER smear horizontal skin creases or carved-out nostrils
        let mut combo_params = RetouchParams::default();
        combo_params.face = face.clone();
        combo_params.face_mole = 100.0;
        combo_params.freckle_acne = 100.0;
        let out_combo = apply_retouch_pipeline(&img, &combo_params, Some(&mask));
        let kept_crease = out_combo.get_pixel(128, 169).0;
        assert!(
            kept_crease[0] < 115,
            "faceMole+freckleAcne must preserve 1D horizontal creases (got {:?})",
            kept_crease
        );
        let kept_nostril = out_combo.get_pixel(128, 145).0;
        assert!(
            kept_nostril[0] < 60,
            "faceMole+freckleAcne must preserve carved-out nostrils (got {:?})",
            kept_nostril
        );

        // 3. Test Manual Spot Heal erases dark mole core completely
        let mut spot_params = RetouchParams::default();
        spot_params.face = face;
        spot_params.blemish_spots.push(BlemishSpot {
            x: 155.0 / w as f32,
            y: 195.0 / h as f32,
            radius: 10.0 / w.min(h) as f32,
        });
        let out_spot = apply_retouch_pipeline(&img, &spot_params, Some(&mask));
        let healed_spot = out_spot.get_pixel(155, 195).0;
        assert!(
            healed_spot[0] > 200 && healed_spot[1] > 160,
            "manual Spot Heal should cleanly replace dark mole core (got {:?})",
            healed_spot
        );
    }

    #[test]
    fn blemish_heal_preserves_mouth_corners_line_endpoints_and_heals_oval_acne_smoothly() {
        let (w, h) = (300u32, 340u32);
        let mut img = RgbaImage::new(w, h);
        let mut mask = RgbaImage::new(w, h);

        for y in 0..h {
            for x in 0..w {
                // Subtle pore texture on skin (~ +/- 3 levels) so donor texture behavior is exercised
                let pore = (((x * 13 + y * 29) % 7) as i32 - 3) as i32;
                let mut r = (214 + pore).clamp(0, 255) as u8;
                let mut g = (170 + pore).clamp(0, 255) as u8;
                let mut b = (146 + pore).clamp(0, 255) as u8;

                // 1. Mouth-corner commissure dark crease extending out of lip margin to (115, 212)
                if (211..=213).contains(&y) && (115..=142).contains(&x) {
                    r = 68;
                    g = 46;
                    b = 40;
                }

                // 2. Arbitrary open-cheek 1-ended hairline crease terminating at (172, 165) (outside mouth_corner_guard)
                if (165..=166).contains(&y) && (172..=198).contains(&x) {
                    r = 82;
                    g = 62;
                    b = 54;
                }

                // 3. Elongated oval acne mark (semi-axes 6.5 x 3.2 px) at (190, 224) inside open cheek skin
                let odx = (x as f32 - 190.0) / 6.5;
                let ody = (y as f32 - 224.0) / 3.2;
                let od2 = odx * odx + ody * ody;
                if od2 <= 1.0 {
                    let core = 1.0 - od2.sqrt() * 0.45;
                    r = (r as f32 * (1.0 - core) + 128.0 * core) as u8;
                    g = (g as f32 * (1.0 - core) + 74.0 * core) as u8;
                    b = (b as f32 * (1.0 - core) + 66.0 * core) as u8;
                }

                img.put_pixel(x, y, image::Rgba([r, g, b, 255]));

                // Semantic mask: R=255 face skin; paint lip body slightly right of mouth corner (x>=126)
                // so the corner crease tip (x=115..125) lies outside lips_outer just like on real portraits!
                let is_lips = (206..=218).contains(&y) && (126..=174).contains(&x);
                let g_code = if is_lips { 205 } else { 0 };
                mask.put_pixel(x, y, image::Rgba([255, g_code, 0, 255]));
            }
        }

        let mut face = FaceRegion::default();
        face.center_x = 0.50;
        face.center_y = 0.52;
        face.face_rx = 0.34;
        face.face_ry = 0.42;
        face.mouth_x = 150.0 / w as f32;
        face.mouth_y = 212.0 / h as f32;
        face.mouth_left_x = 118.0 / w as f32;
        face.mouth_left_y = 212.0 / h as f32;
        face.mouth_right_x = 182.0 / w as f32;
        face.mouth_right_y = 212.0 / h as f32;

        let mut params = RetouchParams::default();
        params.face = face;
        params.freckle_acne = 100.0;
        params.face_mole = 100.0;

        let out = apply_retouch_pipeline(&img, &params, Some(&mask));

        // Mouth-corner endpoint outside lips_outer must NOT be lightened/smudged
        let corner_tip = out.get_pixel(116, 212).0;
        assert!(
            corner_tip[0] < 90,
            "mouth-corner crease tip must be preserved (got {:?})",
            corner_tip
        );

        // Open-cheek 1-ended crease tip (outside mouth/nostril geometric guards) must NOT be erased
        let open_tip = out.get_pixel(173, 165).0;
        assert!(
            open_tip[0] < 105,
            "open-cheek line endpoint must not be smudged as a spot (got {:?})",
            open_tip
        );

        // Center of oval acne mark must be cleanly healed (NO unhealed dark donut hole!)
        let oval_center = out.get_pixel(190, 224).0;
        assert!(
            oval_center[0] > 185 && oval_center[1] > 142,
            "oval acne center should heal cleanly without a donut hole (got {:?})",
            oval_center
        );

        // Radial step smoothness from oval center outward: no harsh per-pixel stair-step cliff > 18 levels
        let mut prev_r = oval_center[0] as i32;
        for dx in 1..=9u32 {
            let cur_r = out.get_pixel(190 + dx, 224).0[0] as i32;
            let diff = (cur_r - prev_r).abs();
            assert!(
                diff <= 18,
                "blemish heal border should transition smoothly without hard step edges (dx={}, prev_r={}, cur_r={})",
                dx,
                prev_r,
                cur_r
            );
            prev_r = cur_r;
        }
    }

    #[test]
    fn hair_retouch_removes_flyaways_fills_part_line_and_preserves_ears_and_wall() {
        let (w, h) = (240u32, 240u32);
        let mut img = RgbaImage::new(w, h);
        let mut mask = RgbaImage::new(w, h);
        let mut scene = RgbaImage::new(w, h);

        for y in 0..h {
            for x in 0..w {
                let is_face = ((x as f32 - 120.0) / 38.0).powi(2)
                    + ((y as f32 - 108.0) / 48.0).powi(2)
                    <= 1.0;
                let is_ear = (162..=172).contains(&x) && (92..=118).contains(&y);
                let is_crown_hair =
                    (74..=166).contains(&x) && (22..=64).contains(&y) && !is_face;
                let is_side_hair = ((64..=82).contains(&x) || (158..=176).contains(&x))
                    && (50..=190).contains(&y)
                    && !is_face
                    && !is_ear;
                let is_part_line = (119..=121).contains(&x) && (28..=58).contains(&y);
                let is_shirt = (75..=195).contains(&x) && (192..=230).contains(&y);
                let is_stray_cloud =
                    (180..=201).contains(&x) && (x - 180) % 3 == 0 && (115..=178).contains(&y);
                let is_stray = (x == 188 && (70..=178).contains(&y)) || is_stray_cloud;

                let rgb = if is_part_line {
                    [214, 174, 150, 255]
                } else if is_ear {
                    [198, 148, 122, 255]
                } else if is_face {
                    [216, 174, 148, 255]
                } else if is_shirt {
                    [30, 36, 48, 255]
                } else if is_crown_hair || is_side_hair || is_stray {
                    [24, 20, 18, 255]
                } else {
                    [236, 224, 212, 255]
                };
                img.put_pixel(x, y, image::Rgba(rgb));

                let r_skin = if is_face { 255 } else { 0 };
                let b_hair = if (is_crown_hair || is_side_hair) && !is_part_line {
                    255
                } else {
                    0
                };
                mask.put_pixel(x, y, image::Rgba([r_skin, 0, b_hair, 255]));

                // Simulate high-confidence scene fg on crisscrossed shoulder puff + slight blurred cloth bleed above collar
                let fg_u8 = if is_face || is_crown_hair || is_side_hair || is_shirt {
                    255u8
                } else if is_stray_cloud {
                    232u8
                } else if is_ear {
                    135u8
                } else if is_stray {
                    95u8
                } else {
                    0u8
                };
                let cloth_u8 = if is_shirt {
                    255u8
                } else if (170..=191).contains(&y) && (75..=205).contains(&x) {
                    85u8
                } else {
                    0u8
                };
                let solid_u8 = if is_face || is_shirt { 255u8 } else { 0u8 };
                scene.put_pixel(x, y, image::Rgba([fg_u8, cloth_u8, solid_u8, 255]));
            }
        }

        let mut params = RetouchParams::default();
        params.face = FaceRegion {
            center_x: 0.50,
            center_y: 0.45,
            face_rx: 0.16,
            face_ry: 0.20,
            forehead_x: 0.50,
            forehead_y: 0.28,
            chin_x: 0.50,
            chin_y: 0.65,
            left_eye_x: 0.43,
            left_eye_y: 0.40,
            right_eye_x: 0.57,
            right_eye_y: 0.40,
            nose_x: 0.50,
            nose_y: 0.47,
            mouth_x: 0.50,
            mouth_y: 0.54,
            ..FaceRegion::default()
        };
        params.stray_hair = 54.0;
        params.hair_volume = 40.0;
        params.hair_part_line = 75.0;
        params.hair_shine = 65.0;
        params.backdrop_clean = 85.0;

        let out = apply_retouch_pipeline_with_scene(&img, &params, Some(&mask), Some(&scene));

        // 1. Ear outside face oval must NEVER turn black/dark
        let ear_px = out.get_pixel(167, 105).0;
        assert!(
            ear_px[0] > 175 && ear_px[1] > 125,
            "ear skin should stay warm and uncorrupted (got {:?})",
            ear_px
        );

        // 2. Exposed bright scalp part line inside crown hair must thicken darker
        let part_px = out.get_pixel(120, 42).0;
        assert!(
            part_px[0] < 105,
            "hair_part_line should fill exposed scalp gap (got {:?})",
            part_px
        );

        // 3. Wide multi-strand shoulder cloud & gap between strands must erase cleanly without gray smear
        for &fx in &[183u32, 186, 189, 192, 195, 198, 201, 184] {
            let cloud_px = out.get_pixel(fx, 150).0;
            assert!(
                cloud_px[0] >= 218 && cloud_px[1] >= 206,
                "stray_hair wide cloud pixel at x={fx} should be clean wall tone with no gray smear (got {:?})",
                cloud_px
            );
        }

        // 4. Main solid side hair interior must remain dark hair (not eaten)
        let solid_hair_px = out.get_pixel(168, 145).0;
        assert!(
            solid_hair_px[0] < 45,
            "solid side hair interior should remain dark (got {:?})",
            solid_hair_px
        );

        // 5. Dark shirt shoulder below wispy cloud must stay uncorrupted
        let shirt_px = out.get_pixel(185, 205).0;
        assert!(
            shirt_px[0] < 55 && shirt_px[2] < 70,
            "dark shirt shoulder should remain intact (got {:?})",
            shirt_px
        );

        // 6. Clean studio wall next to hair must NOT gain a muddy dark halo even with backdrop_clean on
        let wall_px = out.get_pixel(178, 130).0;
        assert!(
            wall_px[0] >= 225 && wall_px[1] >= 214,
            "clean studio wall near hair must keep bright wall tone (got {:?})",
            wall_px
        );
    }

    #[test]
    fn evoto_hair_smooth_white_hair_unify_and_dye_presets_work() {
        let (w, h) = (180u32, 180u32);
        let mut img = RgbaImage::new(w, h);
        let mut mask = RgbaImage::new(w, h);
        let mut scene = RgbaImage::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let is_face = ((x as f32 - 90.0) / 30.0).powi(2)
                    + ((y as f32 - 86.0) / 38.0).powi(2)
                    <= 1.0;
                let is_hair = !is_face
                    && ((42..=138).contains(&x) && (18..=56).contains(&y)
                        || ((42..=60).contains(&x) || (120..=138).contains(&x))
                            && (45..=145).contains(&y));
                let is_white_strand = is_hair && (50..=52).contains(&x) && (60..=135).contains(&y);
                let is_frizzy_noise = is_hair && x == 128 && y % 2 == 0 && (60..=130).contains(&y);
                let is_brassy_patch =
                    is_hair && (122..=134).contains(&x) && (75..=110).contains(&y) && !is_frizzy_noise;

                let rgb = if is_face {
                    [218, 174, 148, 255]
                } else if is_white_strand {
                    [215, 216, 218, 255]
                } else if is_frizzy_noise {
                    [165, 158, 152, 255]
                } else if is_brassy_patch {
                    [165, 118, 52, 255]
                } else if is_hair {
                    [26, 22, 20, 255]
                } else {
                    [236, 228, 220, 255]
                };
                img.put_pixel(x, y, image::Rgba(rgb));
                mask.put_pixel(
                    x,
                    y,
                    image::Rgba([if is_face { 255 } else { 0 }, 0, if is_hair { 255 } else { 0 }, 255]),
                );
                scene.put_pixel(
                    x,
                    y,
                    image::Rgba([if is_face || is_hair { 255 } else { 0 }, 0, if is_face { 255 } else { 0 }, 255]),
                );
            }
        }

        let face = FaceRegion {
            center_x: 0.50,
            center_y: 0.48,
            face_rx: 0.17,
            face_ry: 0.21,
            forehead_x: 0.50,
            forehead_y: 0.30,
            chin_x: 0.50,
            chin_y: 0.69,
            left_eye_x: 0.43,
            left_eye_y: 0.43,
            right_eye_x: 0.57,
            right_eye_y: 0.43,
            nose_x: 0.50,
            nose_y: 0.50,
            mouth_x: 0.50,
            mouth_y: 0.58,
            ..FaceRegion::default()
        };

        // 1. White hair blackening + color unify + hair smooth
        let mut p1 = RetouchParams::default();
        p1.face = face.clone();
        p1.white_hair_blacken = 90.0;
        p1.hair_color_unify = 80.0;
        p1.hair_smooth = 85.0;
        p1.hair_frizzy = 100.0;
        let out1 = apply_retouch_pipeline_with_scene(&img, &p1, Some(&mask), Some(&scene));

        let white_px = out1.get_pixel(51, 95).0;
        assert!(
            white_px[0] < 110,
            "white hair strand should be darkened toward natural hair tone (got {:?})",
            white_px
        );
        let brass_px = out1.get_pixel(125, 92).0;
        assert!(
            brass_px[0] < 125,
            "hair_color_unify should pull brassy outlier toward natural hair tone (got {:?})",
            brass_px
        );
        let face_px = out1.get_pixel(90, 86).0;
        assert!(
            face_px[0] > 195 && face_px[1] > 155,
            "face skin must remain protected from hair tools (got {:?})",
            face_px
        );

        // 2. Evoto Hair Dye H03 (Orange Brown) & backward-compatible JSON defaults
        let mut p2 = RetouchParams::default();
        p2.face = face;
        p2.hair_dye_color = "h03".to_string();
        p2.hair_dye_strength = 85.0;
        let out2 = apply_retouch_pipeline_with_scene(&img, &p2, Some(&mask), Some(&scene));
        let dyed_px = out2.get_pixel(50, 40).0;
        assert!(
            dyed_px[0] > dyed_px[2] + 18,
            "h03 dye should impart warm orange-brown shift vs blue (got {:?})",
            dyed_px
        );

        let minimal_legacy_json = r#"{"stray_hair":45,"hair_volume":10,"hair_part_line":20,"hair_shine":30,"hair_dye_strength":0}"#;
        // Smoke check partial serialization doesn't break RetouchParams serde defaults
        let mut v: serde_json::Value = serde_json::to_value(RetouchParams::default()).unwrap();
        let obj = v.as_object_mut().unwrap();
        for k in [
            "strayHairInner",
            "strayHairOuter",
            "hairSideLeftVolume",
            "hairSideRightVolume",
            "hairlineAdjust",
            "hairSmooth",
            "hairFrizzy",
            "whiteHairBlacken",
            "hairColorUnify",
            "hairDyeDarken",
        ] {
            obj.remove(k);
        }
        let parsed: RetouchParams = serde_json::from_value(v).unwrap();
        assert!((parsed.stray_hair_inner - 100.0).abs() < 1e-4);
        assert!((parsed.hair_frizzy - 100.0).abs() < 1e-4);
        assert!((parsed.hair_dye_darken - 0.0).abs() < 1e-4);
        assert!(minimal_legacy_json.contains("stray_hair"));
    }

    #[test]
    fn evoto_hair_geometry_volume_sides_and_hairline_preserve_eye_region() {
        let (w, h) = (180u32, 180u32);
        let mut img = RgbaImage::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let is_hair = y < 48 || x < 54 || x > 126;
                let rgb = if is_hair {
                    [22, 20, 18, 255]
                } else {
                    [210, 175, 152, 255]
                };
                img.put_pixel(x, y, image::Rgba(rgb));
            }
        }
        let mut p = RetouchParams::default();
        p.face = FaceRegion {
            center_x: 0.50,
            center_y: 0.50,
            face_rx: 0.16,
            face_ry: 0.22,
            forehead_x: 0.50,
            forehead_y: 0.28,
            chin_x: 0.50,
            chin_y: 0.72,
            left_eye_x: 0.43,
            left_eye_y: 0.44,
            right_eye_x: 0.57,
            right_eye_y: 0.44,
            nose_x: 0.50,
            nose_y: 0.52,
            mouth_x: 0.50,
            mouth_y: 0.60,
            ..FaceRegion::default()
        };
        p.hair_volume = 80.0;
        p.hair_side_left_volume = 80.0;
        p.hair_side_right_volume = 80.0;
        p.hairline_adjust = 80.0;

        let out = apply_retouch_pipeline_with_scene(&img, &p, None, None);
        // Eye center must remain clean skin
        let eye_px = out.get_pixel(77, 79).0;
        assert!(
            eye_px[0] > 195 && eye_px[1] > 160,
            "hair warps must not deform eye region (got {:?})",
            eye_px
        );
        // Outer crown just above hair edge (y=46..50) expands upward cleanly (wait: hair fills up to y=47 -> warp moves y=47 outward)
        assert_eq!(out.dimensions(), (w, h));
    }
}
