// Generated from gravity_arcade.swf DefineShape 147 and 151 via reference/extract_shapes.py --contours.
// Coordinates are stored as SWF shape-local pixels.

use super::{MatchPipContour, MatchPipSlot, SwfPathSegment, SwfPoint};

pub const MATCH_PIP_FLATTEN_STEPS: u8 = 4;
#[cfg(test)]
pub const BLUE_DEFINE_SHAPE_ID: u16 = 147;
#[cfg(test)]
pub const RED_DEFINE_SHAPE_ID: u16 = 151;
#[cfg(test)]
pub const BLUE_BOUNDS_TWIPS: [i16; 4] = [-223, 223, -223, 223];
#[cfg(test)]
pub const RED_BOUNDS_TWIPS: [i16; 4] = [-223, 223, -223, 223];
pub const BLUE_FILL_RGB: [[u8; 3]; 6] = [
    [140, 140, 176],
    [0, 184, 209],
    [115, 232, 255],
    [0, 197, 237],
    [204, 249, 255],
    [74, 212, 255],
];
pub const RED_FILL_RGB: [[u8; 3]; 6] = [
    [140, 140, 176],
    [255, 222, 115],
    [209, 149, 0],
    [237, 170, 0],
    [255, 204, 74],
    [255, 241, 204],
];

const BLUE_8C8CB0_99_200_SEGMENTS: [SwfPathSegment; 2] = [
    SwfPathSegment::Line(SwfPoint::new(5.0, 10.0)),
    SwfPathSegment::Line(SwfPoint::new(4.95, 10.0)),
];

const BLUE_8C8CB0_99_200_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Outer,
    start: SwfPoint::new(4.95, 10.0),
    segments: &BLUE_8C8CB0_99_200_SEGMENTS,
};

const BLUE_00B8D1_NEG_162_NEG_154_SEGMENTS: [SwfPathSegment; 8] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(-9.8, -4.9),
        to: SwfPoint::new(-9.6, -0.9),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-9.45, 2.7),
        to: SwfPoint::new(-7.45, 5.6),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.3, 8.8),
        to: SwfPoint::new(-2.05, 9.65),
    },
    SwfPathSegment::Line(SwfPoint::new(4.95, 10.0)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(2.7, 11.15),
        to: SwfPoint::new(0.0, 11.15),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-4.65, 11.15),
        to: SwfPoint::new(-7.9, 7.85),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-11.2, 4.6),
        to: SwfPoint::new(-11.15, 0.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-11.2, -4.5),
        to: SwfPoint::new(-8.1, -7.7),
    },
];

const BLUE_00B8D1_NEG_162_NEG_154_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Core,
    start: SwfPoint::new(-8.1, -7.7),
    segments: &BLUE_00B8D1_NEG_162_NEG_154_SEGMENTS,
};

const BLUE_73E8FF_20_NEG_197_SEGMENTS: [SwfPathSegment; 8] = [
    SwfPathSegment::Line(SwfPoint::new(-2.15, -9.35)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.65, -7.65),
        to: SwfPoint::new(-5.5, -4.15),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.35, -0.6),
        to: SwfPoint::new(-2.9, 1.95),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-0.5, 4.45),
        to: SwfPoint::new(2.9, 4.85),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(6.85, 5.2),
        to: SwfPoint::new(8.75, 2.35),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(10.3, 0.0),
        to: SwfPoint::new(9.2, -3.9),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(8.55, -6.1),
        to: SwfPoint::new(5.95, -8.1),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(3.3, -10.1),
        to: SwfPoint::new(1.0, -9.85),
    },
];

const BLUE_73E8FF_20_NEG_197_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Highlight,
    start: SwfPoint::new(1.0, -9.85),
    segments: &BLUE_73E8FF_20_NEG_197_SEGMENTS,
};

const BLUE_73E8FF_NEG_192_NEG_18_SEGMENTS: [SwfPathSegment; 12] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(-9.8, -4.9),
        to: SwfPoint::new(-8.1, -7.7),
    },
    SwfPathSegment::Line(SwfPoint::new(-7.9, -7.9)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-4.65, -11.15),
        to: SwfPoint::new(0.0, -11.15),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(4.6, -11.15),
        to: SwfPoint::new(7.85, -7.9),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(11.15, -4.6),
        to: SwfPoint::new(11.15, 0.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(11.15, 4.6),
        to: SwfPoint::new(7.85, 7.85),
    },
    SwfPathSegment::Line(SwfPoint::new(5.8, 9.55)),
    SwfPathSegment::Line(SwfPoint::new(5.0, 10.0)),
    SwfPathSegment::Line(SwfPoint::new(4.95, 10.0)),
    SwfPathSegment::Line(SwfPoint::new(-2.05, 9.65)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.3, 8.8),
        to: SwfPoint::new(-7.45, 5.6),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-9.45, 2.7),
        to: SwfPoint::new(-9.6, -0.9),
    },
];

const BLUE_73E8FF_NEG_192_NEG_18_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Highlight,
    start: SwfPoint::new(-9.6, -0.9),
    segments: &BLUE_73E8FF_NEG_192_NEG_18_SEGMENTS,
};

const BLUE_73E8FF_NEG_43_127_SEGMENTS: [SwfPathSegment; 2] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(0.6, 9.1),
        to: SwfPoint::new(4.3, 7.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(0.95, 6.15),
        to: SwfPoint::new(-2.15, 6.35),
    },
];

const BLUE_73E8FF_NEG_43_127_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Highlight,
    start: SwfPoint::new(-2.15, 6.35),
    segments: &BLUE_73E8FF_NEG_43_127_SEGMENTS,
};

const BLUE_00C5ED_125_NEG_68_SEGMENTS: [SwfPathSegment; 3] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(7.3, -6.2),
        to: SwfPoint::new(3.55, -7.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-0.25, -5.1),
        to: SwfPoint::new(3.8, -2.7),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(5.6, -1.6),
        to: SwfPoint::new(6.25, -3.4),
    },
];

const BLUE_00C5ED_125_NEG_68_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Mid,
    start: SwfPoint::new(6.25, -3.4),
    segments: &BLUE_00C5ED_125_NEG_68_SEGMENTS,
};

const BLUE_00C5ED_NEG_43_NEG_187_SEGMENTS: [SwfPathSegment; 8] = [
    SwfPathSegment::Line(SwfPoint::new(1.0, -9.85)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(3.3, -10.1),
        to: SwfPoint::new(5.95, -8.1),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(8.55, -6.1),
        to: SwfPoint::new(9.2, -3.9),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(10.3, 0.0),
        to: SwfPoint::new(8.75, 2.35),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(6.85, 5.2),
        to: SwfPoint::new(2.9, 4.85),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-0.5, 4.45),
        to: SwfPoint::new(-2.9, 1.95),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.35, -0.6),
        to: SwfPoint::new(-5.5, -4.15),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.65, -7.65),
        to: SwfPoint::new(-2.15, -9.35),
    },
];

const BLUE_00C5ED_NEG_43_NEG_187_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Mid,
    start: SwfPoint::new(-2.15, -9.35),
    segments: &BLUE_00C5ED_NEG_43_NEG_187_SEGMENTS,
};

const BLUE_00C5ED_150_18_SEGMENTS: [SwfPathSegment; 2] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(3.45, 1.65),
        to: SwfPoint::new(6.75, 2.4),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(7.65, 2.65),
        to: SwfPoint::new(7.5, 0.9),
    },
];

const BLUE_00C5ED_150_18_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Mid,
    start: SwfPoint::new(7.5, 0.9),
    segments: &BLUE_00C5ED_150_18_SEGMENTS,
};

const BLUE_00C5ED_28_NEG_169_SEGMENTS: [SwfPathSegment; 2] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(-0.25, -8.65),
        to: SwfPoint::new(-1.4, -7.7),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(1.6, -5.75),
        to: SwfPoint::new(1.4, -8.45),
    },
];

const BLUE_00C5ED_28_NEG_169_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Mid,
    start: SwfPoint::new(1.4, -8.45),
    segments: &BLUE_00C5ED_28_NEG_169_SEGMENTS,
};

const BLUE_CCF9FF_135_48_SEGMENTS: [SwfPathSegment; 2] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(3.45, 1.65),
        to: SwfPoint::new(7.5, 0.9),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(7.65, 2.65),
        to: SwfPoint::new(6.75, 2.4),
    },
];

const BLUE_CCF9FF_135_48_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Shine,
    start: SwfPoint::new(6.75, 2.4),
    segments: &BLUE_CCF9FF_135_48_SEGMENTS,
};

const BLUE_CCF9FF_86_140_SEGMENTS: [SwfPathSegment; 2] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(0.6, 9.1),
        to: SwfPoint::new(-2.15, 6.35),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(0.95, 6.15),
        to: SwfPoint::new(4.3, 7.0),
    },
];

const BLUE_CCF9FF_86_140_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Shine,
    start: SwfPoint::new(4.3, 7.0),
    segments: &BLUE_CCF9FF_86_140_SEGMENTS,
};

const BLUE_CCF9FF_NEG_28_NEG_154_SEGMENTS: [SwfPathSegment; 2] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(-0.25, -8.65),
        to: SwfPoint::new(1.4, -8.45),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(1.6, -5.75),
        to: SwfPoint::new(-1.4, -7.7),
    },
];

const BLUE_CCF9FF_NEG_28_NEG_154_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Shine,
    start: SwfPoint::new(-1.4, -7.7),
    segments: &BLUE_CCF9FF_NEG_28_NEG_154_SEGMENTS,
};

const BLUE_4AD4FF_71_NEG_140_SEGMENTS: [SwfPathSegment; 3] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(7.3, -6.2),
        to: SwfPoint::new(6.25, -3.4),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(5.6, -1.6),
        to: SwfPoint::new(3.8, -2.7),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-0.25, -5.1),
        to: SwfPoint::new(3.55, -7.0),
    },
];

const BLUE_4AD4FF_71_NEG_140_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Accent,
    start: SwfPoint::new(3.55, -7.0),
    segments: &BLUE_4AD4FF_71_NEG_140_SEGMENTS,
};

const RED_8C8CB0_99_200_SEGMENTS: [SwfPathSegment; 2] = [
    SwfPathSegment::Line(SwfPoint::new(5.0, 10.0)),
    SwfPathSegment::Line(SwfPoint::new(4.95, 10.0)),
];

const RED_8C8CB0_99_200_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Outer,
    start: SwfPoint::new(4.95, 10.0),
    segments: &RED_8C8CB0_99_200_SEGMENTS,
};

const RED_FFDE73_20_NEG_197_SEGMENTS: [SwfPathSegment; 8] = [
    SwfPathSegment::Line(SwfPoint::new(-2.15, -9.35)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.65, -7.65),
        to: SwfPoint::new(-5.5, -4.15),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.35, -0.6),
        to: SwfPoint::new(-2.9, 1.95),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-0.5, 4.45),
        to: SwfPoint::new(2.9, 4.85),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(6.85, 5.2),
        to: SwfPoint::new(8.75, 2.35),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(10.3, 0.0),
        to: SwfPoint::new(9.2, -3.9),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(8.55, -6.1),
        to: SwfPoint::new(5.95, -8.1),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(3.3, -10.1),
        to: SwfPoint::new(1.0, -9.85),
    },
];

const RED_FFDE73_20_NEG_197_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Highlight,
    start: SwfPoint::new(1.0, -9.85),
    segments: &RED_FFDE73_20_NEG_197_SEGMENTS,
};

const RED_FFDE73_NEG_192_NEG_18_SEGMENTS: [SwfPathSegment; 12] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(-9.8, -4.9),
        to: SwfPoint::new(-8.1, -7.7),
    },
    SwfPathSegment::Line(SwfPoint::new(-7.9, -7.9)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-4.65, -11.15),
        to: SwfPoint::new(0.0, -11.15),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(4.6, -11.15),
        to: SwfPoint::new(7.85, -7.9),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(11.15, -4.6),
        to: SwfPoint::new(11.15, 0.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(11.15, 4.6),
        to: SwfPoint::new(7.85, 7.85),
    },
    SwfPathSegment::Line(SwfPoint::new(5.8, 9.55)),
    SwfPathSegment::Line(SwfPoint::new(5.0, 10.0)),
    SwfPathSegment::Line(SwfPoint::new(4.95, 10.0)),
    SwfPathSegment::Line(SwfPoint::new(-2.05, 9.65)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.3, 8.8),
        to: SwfPoint::new(-7.45, 5.6),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-9.45, 2.7),
        to: SwfPoint::new(-9.6, -0.9),
    },
];

const RED_FFDE73_NEG_192_NEG_18_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Highlight,
    start: SwfPoint::new(-9.6, -0.9),
    segments: &RED_FFDE73_NEG_192_NEG_18_SEGMENTS,
};

const RED_FFDE73_NEG_43_127_SEGMENTS: [SwfPathSegment; 2] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(0.6, 9.1),
        to: SwfPoint::new(4.3, 7.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(0.95, 6.15),
        to: SwfPoint::new(-2.15, 6.35),
    },
];

const RED_FFDE73_NEG_43_127_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Highlight,
    start: SwfPoint::new(-2.15, 6.35),
    segments: &RED_FFDE73_NEG_43_127_SEGMENTS,
};

const RED_D19500_NEG_162_NEG_154_SEGMENTS: [SwfPathSegment; 8] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(-9.8, -4.9),
        to: SwfPoint::new(-9.6, -0.9),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-9.45, 2.7),
        to: SwfPoint::new(-7.45, 5.6),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.3, 8.8),
        to: SwfPoint::new(-2.05, 9.65),
    },
    SwfPathSegment::Line(SwfPoint::new(4.95, 10.0)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(2.7, 11.15),
        to: SwfPoint::new(0.0, 11.15),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-4.65, 11.15),
        to: SwfPoint::new(-7.9, 7.85),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-11.2, 4.6),
        to: SwfPoint::new(-11.15, 0.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-11.2, -4.5),
        to: SwfPoint::new(-8.1, -7.7),
    },
];

const RED_D19500_NEG_162_NEG_154_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Core,
    start: SwfPoint::new(-8.1, -7.7),
    segments: &RED_D19500_NEG_162_NEG_154_SEGMENTS,
};

const RED_EDAA00_125_NEG_68_SEGMENTS: [SwfPathSegment; 3] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(7.3, -6.2),
        to: SwfPoint::new(3.55, -7.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-0.25, -5.1),
        to: SwfPoint::new(3.8, -2.7),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(5.6, -1.6),
        to: SwfPoint::new(6.25, -3.4),
    },
];

const RED_EDAA00_125_NEG_68_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Mid,
    start: SwfPoint::new(6.25, -3.4),
    segments: &RED_EDAA00_125_NEG_68_SEGMENTS,
};

const RED_EDAA00_NEG_43_NEG_187_SEGMENTS: [SwfPathSegment; 8] = [
    SwfPathSegment::Line(SwfPoint::new(1.0, -9.85)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(3.3, -10.1),
        to: SwfPoint::new(5.95, -8.1),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(8.55, -6.1),
        to: SwfPoint::new(9.2, -3.9),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(10.3, 0.0),
        to: SwfPoint::new(8.75, 2.35),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(6.85, 5.2),
        to: SwfPoint::new(2.9, 4.85),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-0.5, 4.45),
        to: SwfPoint::new(-2.9, 1.95),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.35, -0.6),
        to: SwfPoint::new(-5.5, -4.15),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.65, -7.65),
        to: SwfPoint::new(-2.15, -9.35),
    },
];

const RED_EDAA00_NEG_43_NEG_187_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Mid,
    start: SwfPoint::new(-2.15, -9.35),
    segments: &RED_EDAA00_NEG_43_NEG_187_SEGMENTS,
};

const RED_EDAA00_150_18_SEGMENTS: [SwfPathSegment; 2] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(3.45, 1.65),
        to: SwfPoint::new(6.75, 2.4),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(7.65, 2.65),
        to: SwfPoint::new(7.5, 0.9),
    },
];

const RED_EDAA00_150_18_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Mid,
    start: SwfPoint::new(7.5, 0.9),
    segments: &RED_EDAA00_150_18_SEGMENTS,
};

const RED_EDAA00_28_NEG_169_SEGMENTS: [SwfPathSegment; 2] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(-0.25, -8.65),
        to: SwfPoint::new(-1.4, -7.7),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(1.6, -5.75),
        to: SwfPoint::new(1.4, -8.45),
    },
];

const RED_EDAA00_28_NEG_169_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Mid,
    start: SwfPoint::new(1.4, -8.45),
    segments: &RED_EDAA00_28_NEG_169_SEGMENTS,
};

const RED_FFCC4A_71_NEG_140_SEGMENTS: [SwfPathSegment; 3] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(7.3, -6.2),
        to: SwfPoint::new(6.25, -3.4),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(5.6, -1.6),
        to: SwfPoint::new(3.8, -2.7),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-0.25, -5.1),
        to: SwfPoint::new(3.55, -7.0),
    },
];

const RED_FFCC4A_71_NEG_140_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Accent,
    start: SwfPoint::new(3.55, -7.0),
    segments: &RED_FFCC4A_71_NEG_140_SEGMENTS,
};

const RED_FFF1CC_135_48_SEGMENTS: [SwfPathSegment; 2] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(3.45, 1.65),
        to: SwfPoint::new(7.5, 0.9),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(7.65, 2.65),
        to: SwfPoint::new(6.75, 2.4),
    },
];

const RED_FFF1CC_135_48_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Shine,
    start: SwfPoint::new(6.75, 2.4),
    segments: &RED_FFF1CC_135_48_SEGMENTS,
};

const RED_FFF1CC_86_140_SEGMENTS: [SwfPathSegment; 2] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(0.6, 9.1),
        to: SwfPoint::new(-2.15, 6.35),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(0.95, 6.15),
        to: SwfPoint::new(4.3, 7.0),
    },
];

const RED_FFF1CC_86_140_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Shine,
    start: SwfPoint::new(4.3, 7.0),
    segments: &RED_FFF1CC_86_140_SEGMENTS,
};

const RED_FFF1CC_NEG_28_NEG_154_SEGMENTS: [SwfPathSegment; 2] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(-0.25, -8.65),
        to: SwfPoint::new(1.4, -8.45),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(1.6, -5.75),
        to: SwfPoint::new(-1.4, -7.7),
    },
];

const RED_FFF1CC_NEG_28_NEG_154_CONTOUR: MatchPipContour = MatchPipContour {
    slot: MatchPipSlot::Shine,
    start: SwfPoint::new(-1.4, -7.7),
    segments: &RED_FFF1CC_NEG_28_NEG_154_SEGMENTS,
};

pub const BLUE_MATCH_PIP_CONTOURS: [MatchPipContour; 12] = [
    BLUE_8C8CB0_99_200_CONTOUR,
    BLUE_00B8D1_NEG_162_NEG_154_CONTOUR,
    BLUE_73E8FF_20_NEG_197_CONTOUR,
    BLUE_73E8FF_NEG_192_NEG_18_CONTOUR,
    BLUE_73E8FF_NEG_43_127_CONTOUR,
    BLUE_00C5ED_125_NEG_68_CONTOUR,
    BLUE_00C5ED_NEG_43_NEG_187_CONTOUR,
    BLUE_00C5ED_150_18_CONTOUR,
    BLUE_00C5ED_28_NEG_169_CONTOUR,
    BLUE_CCF9FF_135_48_CONTOUR,
    BLUE_CCF9FF_86_140_CONTOUR,
    BLUE_CCF9FF_NEG_28_NEG_154_CONTOUR,
];
pub const BLUE_MATCH_PIP_ACCENT: MatchPipContour = BLUE_4AD4FF_71_NEG_140_CONTOUR;

pub const RED_MATCH_PIP_CONTOURS: [MatchPipContour; 12] = [
    RED_8C8CB0_99_200_CONTOUR,
    RED_FFDE73_20_NEG_197_CONTOUR,
    RED_FFDE73_NEG_192_NEG_18_CONTOUR,
    RED_FFDE73_NEG_43_127_CONTOUR,
    RED_D19500_NEG_162_NEG_154_CONTOUR,
    RED_EDAA00_125_NEG_68_CONTOUR,
    RED_EDAA00_NEG_43_NEG_187_CONTOUR,
    RED_EDAA00_150_18_CONTOUR,
    RED_EDAA00_28_NEG_169_CONTOUR,
    RED_FFCC4A_71_NEG_140_CONTOUR,
    RED_FFF1CC_135_48_CONTOUR,
    RED_FFF1CC_86_140_CONTOUR,
];
pub const RED_MATCH_PIP_TOP_SHINE: MatchPipContour = RED_FFF1CC_NEG_28_NEG_154_CONTOUR;
