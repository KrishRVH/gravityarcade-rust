// Generated from gravity_arcade.swf DefineFont 26 and DefineFontInfo 26.
// Coordinates are stored as SWF text-local centipixels; divide by 100 for pixels.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CounterDigitSegment {
    Line([i16; 2]),
    Quad { control: [i16; 2], to: [i16; 2] },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CounterDigitContour {
    pub(super) start: [i16; 2],
    pub(super) segments: &'static [CounterDigitSegment],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CounterDigitDefinition {
    pub(super) text: &'static str,
    pub(super) font_id: u16,
    pub(super) color_rgb: [u8; 3],
    pub(super) advance_centipx: i16,
    pub(super) bounds_centipx: [i16; 4],
    pub(super) contours: &'static [CounterDigitContour],
}

const DIGIT_0_CONTOUR_0: [CounterDigitSegment; 7] = [
    CounterDigitSegment::Quad {
        control: [155, 407],
        to: [155, 736],
    },
    CounterDigitSegment::Quad {
        control: [155, 1092],
        to: [307, 1092],
    },
    CounterDigitSegment::Quad {
        control: [395, 1092],
        to: [436, 1014],
    },
    CounterDigitSegment::Quad {
        control: [477, 934],
        to: [477, 736],
    },
    CounterDigitSegment::Quad {
        control: [477, 600],
        to: [461, 535],
    },
    CounterDigitSegment::Quad {
        control: [445, 472],
        to: [410, 439],
    },
    CounterDigitSegment::Quad {
        control: [375, 407],
        to: [321, 407],
    },
];

const DIGIT_0_CONTOUR_1: [CounterDigitSegment; 8] = [
    CounterDigitSegment::Quad {
        control: [458, 307],
        to: [526, 404],
    },
    CounterDigitSegment::Quad {
        control: [594, 501],
        to: [594, 744],
    },
    CounterDigitSegment::Quad {
        control: [594, 953],
        to: [523, 1072],
    },
    CounterDigitSegment::Quad {
        control: [452, 1192],
        to: [315, 1192],
    },
    CounterDigitSegment::Quad {
        control: [179, 1192],
        to: [109, 1096],
    },
    CounterDigitSegment::Quad {
        control: [38, 998],
        to: [38, 725],
    },
    CounterDigitSegment::Quad {
        control: [38, 540],
        to: [112, 423],
    },
    CounterDigitSegment::Quad {
        control: [188, 307],
        to: [323, 307],
    },
];

const DIGIT_0_CONTOURS: [CounterDigitContour; 2] = [
    CounterDigitContour {
        start: [321, 407],
        segments: &DIGIT_0_CONTOUR_0,
    },
    CounterDigitContour {
        start: [323, 307],
        segments: &DIGIT_0_CONTOUR_1,
    },
];

pub const DIGIT_0: CounterDigitDefinition = CounterDigitDefinition {
    text: "0",
    font_id: 26,
    color_rgb: [140, 140, 176],
    advance_centipx: 630,
    bounds_centipx: [38, 594, 307, 1192],
    contours: &DIGIT_0_CONTOURS,
};

const DIGIT_1_CONTOUR_0: [CounterDigitSegment; 8] = [
    CounterDigitSegment::Line([409, 1180]),
    CounterDigitSegment::Line([292, 1180]),
    CounterDigitSegment::Line([292, 523]),
    CounterDigitSegment::Line([117, 632]),
    CounterDigitSegment::Line([117, 521]),
    CounterDigitSegment::Quad {
        control: [184, 489],
        to: [259, 430],
    },
    CounterDigitSegment::Quad {
        control: [333, 371],
        to: [374, 319],
    },
    CounterDigitSegment::Line([409, 319]),
];

const DIGIT_1_CONTOURS: [CounterDigitContour; 1] = [CounterDigitContour {
    start: [409, 319],
    segments: &DIGIT_1_CONTOUR_0,
}];

pub const DIGIT_1: CounterDigitDefinition = CounterDigitDefinition {
    text: "1",
    font_id: 26,
    color_rgb: [140, 140, 176],
    advance_centipx: 630,
    bounds_centipx: [117, 409, 319, 1180],
    contours: &DIGIT_1_CONTOURS,
};

const DIGIT_2_CONTOUR_0: [CounterDigitSegment; 16] = [
    CounterDigitSegment::Quad {
        control: [212, 407],
        to: [171, 431],
    },
    CounterDigitSegment::Quad {
        control: [129, 456],
        to: [110, 494],
    },
    CounterDigitSegment::Line([35, 432]),
    CounterDigitSegment::Quad {
        control: [55, 375],
        to: [114, 341],
    },
    CounterDigitSegment::Quad {
        control: [171, 307],
        to: [254, 307],
    },
    CounterDigitSegment::Quad {
        control: [377, 307],
        to: [448, 364],
    },
    CounterDigitSegment::Quad {
        control: [517, 421],
        to: [517, 525],
    },
    CounterDigitSegment::Quad {
        control: [517, 622],
        to: [424, 769],
    },
    CounterDigitSegment::Line([232, 1075]),
    CounterDigitSegment::Line([568, 1075]),
    CounterDigitSegment::Line([568, 1180]),
    CounterDigitSegment::Line([41, 1180]),
    CounterDigitSegment::Line([41, 1157]),
    CounterDigitSegment::Line([309, 745]),
    CounterDigitSegment::Quad {
        control: [394, 615],
        to: [394, 525],
    },
    CounterDigitSegment::Quad {
        control: [394, 407],
        to: [260, 407],
    },
];

const DIGIT_2_CONTOURS: [CounterDigitContour; 1] = [CounterDigitContour {
    start: [260, 407],
    segments: &DIGIT_2_CONTOUR_0,
}];

pub const DIGIT_2: CounterDigitDefinition = CounterDigitDefinition {
    text: "2",
    font_id: 26,
    color_rgb: [140, 140, 176],
    advance_centipx: 630,
    bounds_centipx: [35, 568, 307, 1180],
    contours: &DIGIT_2_CONTOURS,
};

const DIGIT_3_CONTOUR_0: [CounterDigitSegment; 23] = [
    CounterDigitSegment::Quad {
        control: [523, 426],
        to: [523, 520],
    },
    CounterDigitSegment::Quad {
        control: [523, 592],
        to: [484, 647],
    },
    CounterDigitSegment::Quad {
        control: [444, 702],
        to: [390, 721],
    },
    CounterDigitSegment::Quad {
        control: [464, 745],
        to: [509, 803],
    },
    CounterDigitSegment::Quad {
        control: [552, 860],
        to: [552, 942],
    },
    CounterDigitSegment::Quad {
        control: [552, 1062],
        to: [476, 1127],
    },
    CounterDigitSegment::Quad {
        control: [401, 1192],
        to: [264, 1192],
    },
    CounterDigitSegment::Quad {
        control: [206, 1192],
        to: [151, 1171],
    },
    CounterDigitSegment::Quad {
        control: [96, 1150],
        to: [64, 1119],
    },
    CounterDigitSegment::Line([121, 1029]),
    CounterDigitSegment::Quad {
        control: [178, 1092],
        to: [266, 1092],
    },
    CounterDigitSegment::Quad {
        control: [429, 1092],
        to: [429, 933],
    },
    CounterDigitSegment::Quad {
        control: [429, 860],
        to: [381, 816],
    },
    CounterDigitSegment::Quad {
        control: [334, 770],
        to: [255, 770],
    },
    CounterDigitSegment::Line([246, 770]),
    CounterDigitSegment::Line([246, 675]),
    CounterDigitSegment::Line([251, 675]),
    CounterDigitSegment::Quad {
        control: [400, 675],
        to: [400, 544],
    },
    CounterDigitSegment::Quad {
        control: [400, 407],
        to: [260, 407],
    },
    CounterDigitSegment::Quad {
        control: [184, 407],
        to: [138, 457],
    },
    CounterDigitSegment::Line([87, 377]),
    CounterDigitSegment::Quad {
        control: [141, 307],
        to: [268, 307],
    },
    CounterDigitSegment::Quad {
        control: [381, 307],
        to: [452, 367],
    },
];

const DIGIT_3_CONTOURS: [CounterDigitContour; 1] = [CounterDigitContour {
    start: [452, 367],
    segments: &DIGIT_3_CONTOUR_0,
}];

pub const DIGIT_3: CounterDigitDefinition = CounterDigitDefinition {
    text: "3",
    font_id: 26,
    color_rgb: [140, 140, 176],
    advance_centipx: 630,
    bounds_centipx: [64, 552, 307, 1192],
    contours: &DIGIT_3_CONTOURS,
};

const DIGIT_4_CONTOUR_0: [CounterDigitSegment; 11] = [
    CounterDigitSegment::Line([517, 857]),
    CounterDigitSegment::Line([604, 857]),
    CounterDigitSegment::Line([604, 947]),
    CounterDigitSegment::Line([517, 947]),
    CounterDigitSegment::Line([517, 1180]),
    CounterDigitSegment::Line([405, 1180]),
    CounterDigitSegment::Line([405, 947]),
    CounterDigitSegment::Line([15, 947]),
    CounterDigitSegment::Line([15, 880]),
    CounterDigitSegment::Line([476, 319]),
    CounterDigitSegment::Line([517, 319]),
];

const DIGIT_4_CONTOUR_1: [CounterDigitSegment; 3] = [
    CounterDigitSegment::Line([405, 552]),
    CounterDigitSegment::Line([154, 857]),
    CounterDigitSegment::Line([405, 857]),
];

const DIGIT_4_CONTOURS: [CounterDigitContour; 2] = [
    CounterDigitContour {
        start: [517, 319],
        segments: &DIGIT_4_CONTOUR_0,
    },
    CounterDigitContour {
        start: [405, 857],
        segments: &DIGIT_4_CONTOUR_1,
    },
];

pub const DIGIT_4: CounterDigitDefinition = CounterDigitDefinition {
    text: "4",
    font_id: 26,
    color_rgb: [140, 140, 176],
    advance_centipx: 630,
    bounds_centipx: [15, 604, 319, 1180],
    contours: &DIGIT_4_CONTOURS,
};

const DIGIT_5_CONTOUR_0: [CounterDigitSegment; 16] = [
    CounterDigitSegment::Line([207, 418]),
    CounterDigitSegment::Line([207, 627]),
    CounterDigitSegment::Quad {
        control: [251, 594],
        to: [318, 594],
    },
    CounterDigitSegment::Quad {
        control: [437, 594],
        to: [500, 666],
    },
    CounterDigitSegment::Quad {
        control: [564, 737],
        to: [564, 869],
    },
    CounterDigitSegment::Quad {
        control: [564, 1192],
        to: [275, 1192],
    },
    CounterDigitSegment::Quad {
        control: [156, 1192],
        to: [76, 1125],
    },
    CounterDigitSegment::Line([122, 1028]),
    CounterDigitSegment::Quad {
        control: [202, 1092],
        to: [275, 1092],
    },
    CounterDigitSegment::Quad {
        control: [441, 1092],
        to: [441, 886],
    },
    CounterDigitSegment::Quad {
        control: [441, 694],
        to: [278, 694],
    },
    CounterDigitSegment::Quad {
        control: [199, 694],
        to: [136, 762],
    },
    CounterDigitSegment::Line([96, 734]),
    CounterDigitSegment::Line([96, 319]),
    CounterDigitSegment::Line([524, 319]),
    CounterDigitSegment::Line([524, 418]),
];

const DIGIT_5_CONTOURS: [CounterDigitContour; 1] = [CounterDigitContour {
    start: [524, 418],
    segments: &DIGIT_5_CONTOUR_0,
}];

pub const DIGIT_5: CounterDigitDefinition = CounterDigitDefinition {
    text: "5",
    font_id: 26,
    color_rgb: [140, 140, 176],
    advance_centipx: 630,
    bounds_centipx: [76, 564, 319, 1192],
    contours: &DIGIT_5_CONTOURS,
};

const DIGIT_6_CONTOUR_0: [CounterDigitSegment; 11] = [
    CounterDigitSegment::Quad {
        control: [400, 390],
        to: [302, 509],
    },
    CounterDigitSegment::Quad {
        control: [204, 626],
        to: [193, 690],
    },
    CounterDigitSegment::Quad {
        control: [243, 643],
        to: [329, 643],
    },
    CounterDigitSegment::Quad {
        control: [443, 643],
        to: [511, 716],
    },
    CounterDigitSegment::Quad {
        control: [580, 789],
        to: [580, 915],
    },
    CounterDigitSegment::Quad {
        control: [580, 1042],
        to: [510, 1118],
    },
    CounterDigitSegment::Quad {
        control: [438, 1195],
        to: [332, 1195],
    },
    CounterDigitSegment::Quad {
        control: [49, 1195],
        to: [49, 820],
    },
    CounterDigitSegment::Quad {
        control: [49, 664],
        to: [161, 501],
    },
    CounterDigitSegment::Quad {
        control: [271, 339],
        to: [394, 307],
    },
    CounterDigitSegment::Line([451, 369]),
];

const DIGIT_6_CONTOUR_1: [CounterDigitSegment; 6] = [
    CounterDigitSegment::Quad {
        control: [172, 1096],
        to: [323, 1096],
    },
    CounterDigitSegment::Quad {
        control: [387, 1096],
        to: [423, 1048],
    },
    CounterDigitSegment::Quad {
        control: [459, 1000],
        to: [459, 921],
    },
    CounterDigitSegment::Quad {
        control: [459, 840],
        to: [421, 791],
    },
    CounterDigitSegment::Quad {
        control: [382, 743],
        to: [321, 743],
    },
    CounterDigitSegment::Quad {
        control: [172, 743],
        to: [172, 912],
    },
];

const DIGIT_6_CONTOURS: [CounterDigitContour; 2] = [
    CounterDigitContour {
        start: [451, 369],
        segments: &DIGIT_6_CONTOUR_0,
    },
    CounterDigitContour {
        start: [172, 912],
        segments: &DIGIT_6_CONTOUR_1,
    },
];

pub const DIGIT_6: CounterDigitDefinition = CounterDigitDefinition {
    text: "6",
    font_id: 26,
    color_rgb: [140, 140, 176],
    advance_centipx: 630,
    bounds_centipx: [49, 580, 307, 1195],
    contours: &DIGIT_6_CONTOURS,
};

const DIGIT_7_CONTOUR_0: [CounterDigitSegment; 12] = [
    CounterDigitSegment::Line([531, 513]),
    CounterDigitSegment::Line([451, 678]),
    CounterDigitSegment::Line([369, 862]),
    CounterDigitSegment::Line([300, 1037]),
    CounterDigitSegment::Quad {
        control: [270, 1117],
        to: [251, 1180],
    },
    CounterDigitSegment::Line([121, 1180]),
    CounterDigitSegment::Quad {
        control: [164, 1043],
        to: [267, 817],
    },
    CounterDigitSegment::Quad {
        control: [369, 589],
        to: [455, 430],
    },
    CounterDigitSegment::Line([49, 430]),
    CounterDigitSegment::Line([49, 319]),
    CounterDigitSegment::Line([606, 319]),
    CounterDigitSegment::Line([606, 364]),
];

const DIGIT_7_CONTOURS: [CounterDigitContour; 1] = [CounterDigitContour {
    start: [606, 364],
    segments: &DIGIT_7_CONTOUR_0,
}];

pub const DIGIT_7: CounterDigitDefinition = CounterDigitDefinition {
    text: "7",
    font_id: 26,
    color_rgb: [140, 140, 176],
    advance_centipx: 630,
    bounds_centipx: [49, 606, 319, 1180],
    contours: &DIGIT_7_CONTOURS,
};

const DIGIT_8_CONTOUR_0: [CounterDigitSegment; 14] = [
    CounterDigitSegment::Quad {
        control: [548, 571],
        to: [511, 629],
    },
    CounterDigitSegment::Quad {
        control: [475, 687],
        to: [425, 715],
    },
    CounterDigitSegment::Quad {
        control: [580, 801],
        to: [580, 942],
    },
    CounterDigitSegment::Quad {
        control: [580, 1062],
        to: [509, 1127],
    },
    CounterDigitSegment::Quad {
        control: [437, 1192],
        to: [313, 1192],
    },
    CounterDigitSegment::Quad {
        control: [53, 1192],
        to: [53, 942],
    },
    CounterDigitSegment::Quad {
        control: [53, 871],
        to: [97, 800],
    },
    CounterDigitSegment::Quad {
        control: [142, 731],
        to: [207, 703],
    },
    CounterDigitSegment::Quad {
        control: [152, 674],
        to: [118, 621],
    },
    CounterDigitSegment::Quad {
        control: [83, 569],
        to: [83, 513],
    },
    CounterDigitSegment::Quad {
        control: [83, 418],
        to: [149, 362],
    },
    CounterDigitSegment::Quad {
        control: [213, 307],
        to: [315, 307],
    },
    CounterDigitSegment::Quad {
        control: [427, 307],
        to: [488, 362],
    },
    CounterDigitSegment::Quad {
        control: [548, 418],
        to: [548, 514],
    },
];

const DIGIT_8_CONTOUR_1: [CounterDigitSegment; 5] = [
    CounterDigitSegment::Quad {
        control: [200, 591],
        to: [350, 667],
    },
    CounterDigitSegment::Quad {
        control: [430, 592],
        to: [430, 511],
    },
    CounterDigitSegment::Quad {
        control: [430, 463],
        to: [398, 435],
    },
    CounterDigitSegment::Quad {
        control: [367, 407],
        to: [315, 407],
    },
    CounterDigitSegment::Quad {
        control: [200, 407],
        to: [200, 512],
    },
];

const DIGIT_8_CONTOUR_2: [CounterDigitSegment; 7] = [
    CounterDigitSegment::Quad {
        control: [170, 824],
        to: [170, 942],
    },
    CounterDigitSegment::Quad {
        control: [170, 1007],
        to: [210, 1050],
    },
    CounterDigitSegment::Quad {
        control: [250, 1092],
        to: [313, 1092],
    },
    CounterDigitSegment::Quad {
        control: [379, 1092],
        to: [421, 1050],
    },
    CounterDigitSegment::Quad {
        control: [463, 1008],
        to: [463, 942],
    },
    CounterDigitSegment::Quad {
        control: [463, 895],
        to: [434, 853],
    },
    CounterDigitSegment::Quad {
        control: [404, 811],
        to: [299, 753],
    },
];

const DIGIT_8_CONTOURS: [CounterDigitContour; 3] = [
    CounterDigitContour {
        start: [548, 514],
        segments: &DIGIT_8_CONTOUR_0,
    },
    CounterDigitContour {
        start: [200, 512],
        segments: &DIGIT_8_CONTOUR_1,
    },
    CounterDigitContour {
        start: [299, 753],
        segments: &DIGIT_8_CONTOUR_2,
    },
];

pub const DIGIT_8: CounterDigitDefinition = CounterDigitDefinition {
    text: "8",
    font_id: 26,
    color_rgb: [140, 140, 176],
    advance_centipx: 630,
    bounds_centipx: [53, 580, 307, 1192],
    contours: &DIGIT_8_CONTOURS,
};
