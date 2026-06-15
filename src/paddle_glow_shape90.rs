// Generated from gravity_arcade.swf DefineShape 90 via reference/extract_shapes.py --contours.
// Coordinates are stored as SWF shape-local pixels.

use super::{
    AlphaStop, PaddleGlowContour, PaddleGlowShapeDefinition, SwfBounds, SwfPathSegment, SwfPoint,
};

pub const FLATTEN_STEPS: u8 = 4;
#[cfg(test)]
pub const DEFINE_SHAPE_ID: u16 = 90;
#[cfg(test)]
pub const BOUNDS_TWIPS: [i16; 4] = [-128, 134, -683, 674];
#[cfg(test)]
pub const BODY_GRADIENT_RGBA: [[u8; 5]; 4] = [
    [0, 102, 255, 255, 0],
    [87, 173, 255, 255, 255],
    [166, 161, 255, 255, 238],
    [255, 153, 255, 255, 0],
];
#[cfg(test)]
pub const TOP_CAP_GRADIENT_RGBA: [[u8; 5]; 2] = [[74, 153, 255, 255, 255], [255, 153, 255, 255, 0]];
#[cfg(test)]
pub const BOTTOM_CAP_GRADIENT_RGBA: [[u8; 5]; 2] =
    [[74, 153, 255, 255, 255], [255, 153, 255, 255, 0]];

pub const LINEAR_ALPHA_STOPS: [AlphaStop; 4] = [
    AlphaStop {
        ratio: 0.0,
        alpha: 0.0,
    },
    AlphaStop {
        ratio: 87.0 / 255.0,
        alpha: 1.0,
    },
    AlphaStop {
        ratio: 166.0 / 255.0,
        alpha: 238.0 / 255.0,
    },
    AlphaStop {
        ratio: 1.0,
        alpha: 0.0,
    },
];
pub const RADIAL_PEAK_STOP_RATIO: f32 = 74.0 / 255.0;

const BODY_SEGMENTS: [SwfPathSegment; 6] = [
    SwfPathSegment::Line(SwfPoint::new(6.7, -22.25)),
    SwfPathSegment::Line(SwfPoint::new(6.7, 21.65)),
    SwfPathSegment::Line(SwfPoint::new(6.65, 21.65)),
    SwfPathSegment::Line(SwfPoint::new(-6.4, 21.65)),
    SwfPathSegment::Line(SwfPoint::new(-6.4, -22.25)),
    SwfPathSegment::Line(SwfPoint::new(6.65, -22.25)),
];

const TOP_CAP_SEGMENTS: [SwfPathSegment; 5] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(-6.35, -27.2),
        to: SwfPoint::new(-4.5, -30.7),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-2.6, -34.15),
        to: SwfPoint::new(0.15, -34.15),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(2.85, -34.15),
        to: SwfPoint::new(4.75, -30.7),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(6.6, -27.2),
        to: SwfPoint::new(6.65, -22.25),
    },
    SwfPathSegment::Line(SwfPoint::new(-6.4, -22.25)),
];

const BOTTOM_CAP_SEGMENTS: [SwfPathSegment; 7] = [
    SwfPathSegment::Line(SwfPoint::new(6.65, 21.7)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(6.65, 26.7),
        to: SwfPoint::new(4.75, 30.2),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(2.85, 33.7),
        to: SwfPoint::new(0.15, 33.7),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-2.6, 33.7),
        to: SwfPoint::new(-4.5, 30.2),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-6.4, 26.7),
        to: SwfPoint::new(-6.4, 21.7),
    },
    SwfPathSegment::Line(SwfPoint::new(-6.4, 21.65)),
    SwfPathSegment::Line(SwfPoint::new(6.65, 21.65)),
];

pub const SHAPE: PaddleGlowShapeDefinition = PaddleGlowShapeDefinition {
    bounds: SwfBounds {
        x_min: -6.4,
        x_max: 6.7,
        y_min: -34.15,
        y_max: 33.7,
    },
    body_bounds: SwfBounds {
        x_min: -6.4,
        x_max: 6.7,
        y_min: -22.25,
        y_max: 21.65,
    },
    body: PaddleGlowContour {
        start: SwfPoint::new(6.65, -22.25),
        segments: &BODY_SEGMENTS,
    },
    top_cap: PaddleGlowContour {
        start: SwfPoint::new(-6.4, -22.25),
        segments: &TOP_CAP_SEGMENTS,
    },
    bottom_cap: PaddleGlowContour {
        start: SwfPoint::new(6.65, 21.65),
        segments: &BOTTOM_CAP_SEGMENTS,
    },
};
