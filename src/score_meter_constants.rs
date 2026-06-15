// Generated from gravity_arcade.swf score-meter placements.
// Placement/color-transform source: reference/swf_deep.json.

use super::{ScoreMeterFinalMarkerPlacement, SwfPoint};

#[cfg(test)]
pub const PLAYFIELD_SPRITE_ID: u16 = 140;
#[cfg(test)]
pub const RED_SCORE_METER_SPRITE_ID: u16 = 136;
#[cfg(test)]
pub const BLUE_SCORE_METER_SPRITE_ID: u16 = 139;
#[cfg(test)]
pub const RED_FINAL_SCORE_METER_SPRITE_ID: u16 = 135;
#[cfg(test)]
pub const BLUE_FINAL_SCORE_METER_SPRITE_ID: u16 = 138;
#[cfg(test)]
pub const RED_FINAL_OVERLAY_SPRITE_ID: u16 = 134;
#[cfg(test)]
pub const BLUE_FINAL_OVERLAY_SPRITE_ID: u16 = 137;
#[cfg(test)]
pub const FINAL_OVERLAY_OUTLINE_SHAPE_ID: u16 = 133;

pub const RAMP_STEPS: usize = 9;
pub const FINAL_MARKERS: usize = 13;
pub const FINAL_OVERLAY_FRAMES: usize = 13;
pub const BASE_Y: f32 = 214.05;
pub const BLUE_X: f32 = 19.0;
pub const RED_X: f32 = 530.95;
pub const GROUP_START_FRAMES: [u8; 7] = [2, 11, 21, 31, 41, 51, 61];

pub const LOCAL_YS: [[f32; 2]; 7] = [
    [0.4, 0.4],
    [-27.0, 27.8],
    [-54.4, 55.2],
    [-81.8, 82.6],
    [-109.2, 110.0],
    [-136.6, 137.4],
    [-164.0, 164.8],
];

pub const RED_PHASE_SCALES: [f32; RAMP_STEPS] = [
    0.240_005_5,
    0.335_006_7,
    0.430_007_9,
    0.525_009_2,
    0.620_010_4,
    0.714_996_3,
    0.809_997_6,
    0.904_998_8,
    1.0,
];

pub const BLUE_PHASE_SCALES: [f32; RAMP_STEPS] = [
    0.241_073_6,
    0.335_937_5,
    0.430_801_4,
    0.525_665_3,
    0.620_544_4,
    0.715_408_3,
    0.810_272_2,
    0.905_136_1,
    1.0,
];

pub const RED_PHASE_MULTS: [u8; RAMP_STEPS] = [0, 14, 28, 43, 57, 71, 86, 100, 115];
pub const BLUE_PHASE_MULTS: [u8; RAMP_STEPS] = [0, 15, 30, 45, 60, 75, 90, 105, 120];

pub const RED_PHASE_ADDS: [[u8; 3]; RAMP_STEPS] = [
    [45, 32, 0],
    [39, 28, 0],
    [34, 24, 0],
    [28, 20, 0],
    [22, 16, 0],
    [17, 12, 0],
    [11, 8, 0],
    [5, 4, 0],
    [0, 0, 0],
];

pub const BLUE_PHASE_ADDS: [[u8; 3]; RAMP_STEPS] = [
    [0, 46, 47],
    [0, 40, 41],
    [0, 34, 36],
    [0, 29, 30],
    [0, 23, 23],
    [0, 17, 17],
    [0, 11, 12],
    [0, 5, 6],
    [0, 0, 0],
];

pub const RED_FINAL_MARKERS: [ScoreMeterFinalMarkerPlacement; FINAL_MARKERS] = [
    ScoreMeterFinalMarkerPlacement::new(0.0, 0.4, 1.0, 0, [0, 0, 0]),
    ScoreMeterFinalMarkerPlacement::new(0.0, -27.0, 0.240_005_5, 0, [102, 73, 0]),
    ScoreMeterFinalMarkerPlacement::new(0.0, 27.8, 0.240_005_5, 0, [102, 73, 0]),
    ScoreMeterFinalMarkerPlacement::new(0.0, -54.4, 0.335_006_7, 32, [89, 64, 0]),
    ScoreMeterFinalMarkerPlacement::new(0.0, 55.2, 0.335_006_7, 32, [89, 64, 0]),
    ScoreMeterFinalMarkerPlacement::new(0.0, -81.8, 0.430_007_9, 64, [77, 55, 0]),
    ScoreMeterFinalMarkerPlacement::new(0.0, 82.6, 0.430_007_9, 64, [77, 55, 0]),
    ScoreMeterFinalMarkerPlacement::new(0.0, -109.2, 0.525_009_2, 96, [64, 46, 0]),
    ScoreMeterFinalMarkerPlacement::new(0.0, 110.0, 0.525_009_2, 96, [64, 46, 0]),
    ScoreMeterFinalMarkerPlacement::new(0.0, -136.6, 0.620_010_4, 128, [51, 37, 0]),
    ScoreMeterFinalMarkerPlacement::new(0.0, 137.4, 0.620_010_4, 128, [51, 37, 0]),
    ScoreMeterFinalMarkerPlacement::new(0.0, -164.0, 0.714_996_3, 160, [38, 27, 0]),
    ScoreMeterFinalMarkerPlacement::new(0.0, 164.8, 0.714_996_3, 160, [38, 27, 0]),
];

pub const BLUE_FINAL_MARKERS: [ScoreMeterFinalMarkerPlacement; FINAL_MARKERS] = [
    ScoreMeterFinalMarkerPlacement::new(0.0, 0.4, 1.0, 0, [0, 0, 0]),
    ScoreMeterFinalMarkerPlacement::new(0.0, -27.0, 0.241_073_6, 0, [0, 99, 102]),
    ScoreMeterFinalMarkerPlacement::new(0.0, 27.8, 0.241_073_6, 0, [0, 99, 102]),
    ScoreMeterFinalMarkerPlacement::new(0.0, -54.4, 0.335_937_5, 32, [0, 87, 89]),
    ScoreMeterFinalMarkerPlacement::new(0.0, 55.2, 0.335_937_5, 32, [0, 87, 89]),
    ScoreMeterFinalMarkerPlacement::new(0.0, -81.8, 0.430_801_4, 64, [0, 74, 77]),
    ScoreMeterFinalMarkerPlacement::new(0.0, 82.6, 0.430_801_4, 64, [0, 74, 77]),
    ScoreMeterFinalMarkerPlacement::new(0.0, -109.2, 0.525_665_3, 96, [0, 62, 64]),
    ScoreMeterFinalMarkerPlacement::new(0.0, 110.0, 0.525_665_3, 96, [0, 62, 64]),
    ScoreMeterFinalMarkerPlacement::new(0.0, -136.6, 0.620_544_4, 128, [0, 50, 51]),
    ScoreMeterFinalMarkerPlacement::new(0.0, 137.4, 0.620_544_4, 128, [0, 50, 51]),
    ScoreMeterFinalMarkerPlacement::new(0.0, 164.8, 0.715_408_3, 160, [0, 37, 38]),
    ScoreMeterFinalMarkerPlacement::new(0.0, -164.0, 0.715_408_3, 160, [0, 37, 38]),
];

pub const RED_FINAL_OVERLAY_LOCAL_POSITIONS: [SwfPoint; FINAL_OVERLAY_FRAMES] = [
    SwfPoint::new(0.05, -164.0),
    SwfPoint::new(0.05, -136.6),
    SwfPoint::new(-0.15, -109.2),
    SwfPoint::new(0.05, -81.8),
    SwfPoint::new(0.05, -54.4),
    SwfPoint::new(0.2, -27.0),
    SwfPoint::new(0.0, 0.4),
    SwfPoint::new(0.05, 27.8),
    SwfPoint::new(-0.15, 55.2),
    SwfPoint::new(0.05, 82.6),
    SwfPoint::new(0.05, 110.0),
    SwfPoint::new(0.05, 137.4),
    SwfPoint::new(0.05, 164.8),
];

pub const BLUE_FINAL_OVERLAY_LOCAL_POSITIONS: [SwfPoint; FINAL_OVERLAY_FRAMES] = [
    SwfPoint::new(0.0, -164.0),
    SwfPoint::new(0.0, -136.6),
    SwfPoint::new(-0.2, -109.2),
    SwfPoint::new(0.0, -81.8),
    SwfPoint::new(0.0, -54.4),
    SwfPoint::new(0.15, -27.0),
    SwfPoint::new(-0.05, 0.4),
    SwfPoint::new(0.0, 27.8),
    SwfPoint::new(-0.2, 55.2),
    SwfPoint::new(0.0, 82.6),
    SwfPoint::new(0.0, 110.0),
    SwfPoint::new(0.0, 137.4),
    SwfPoint::new(0.0, 164.8),
];

pub const RED_FINAL_OVERLAY_RGB: [[u8; 3]; FINAL_OVERLAY_FRAMES] = [
    [255, 210, 136],
    [246, 196, 113],
    [238, 182, 91],
    [230, 168, 68],
    [221, 154, 45],
    [213, 140, 23],
    [204, 126, 0],
    [213, 140, 23],
    [221, 154, 45],
    [230, 168, 68],
    [238, 182, 91],
    [246, 196, 113],
    [255, 210, 136],
];

pub const BLUE_FINAL_OVERLAY_RGB: [[u8; 3]; FINAL_OVERLAY_FRAMES] = [
    [34, 224, 255],
    [28, 207, 238],
    [23, 191, 221],
    [17, 174, 204],
    [11, 157, 187],
    [6, 141, 170],
    [0, 124, 153],
    [6, 141, 170],
    [11, 157, 187],
    [17, 174, 204],
    [23, 191, 221],
    [28, 207, 238],
    [34, 224, 255],
];
