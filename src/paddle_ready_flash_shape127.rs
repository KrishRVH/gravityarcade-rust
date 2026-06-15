// Generated from gravity_arcade.swf DefineShape 127 via reference/extract_shapes.py --contours.
// Coordinates are stored as SWF shape-local pixels.

use super::{PaddleReadyFlashShapeDefinition, SwfBounds, SwfPathSegment, SwfPoint};

pub const FLATTEN_STEPS: u8 = 4;
#[cfg(test)]
pub const DEFINE_SHAPE_ID: u16 = 127;
#[cfg(test)]
pub const BOUNDS_TWIPS: [i16; 4] = [-77, 87, -82, 82];
pub const FILL_RGBA: [u8; 4] = [102, 102, 102, 171];

const SEGMENTS: [SwfPathSegment; 8] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(4.35, -1.7),
        to: SwfPoint::new(4.35, 0.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(4.35, 1.7),
        to: SwfPoint::new(3.15, 2.9),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(1.95, 4.1),
        to: SwfPoint::new(0.25, 4.1),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-1.45, 4.1),
        to: SwfPoint::new(-2.65, 2.9),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-3.85, 1.7),
        to: SwfPoint::new(-3.85, 0.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-3.85, -1.7),
        to: SwfPoint::new(-2.65, -2.9),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-1.45, -4.1),
        to: SwfPoint::new(0.25, -4.1),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(1.95, -4.1),
        to: SwfPoint::new(3.15, -2.9),
    },
];

pub const SHAPE: PaddleReadyFlashShapeDefinition = PaddleReadyFlashShapeDefinition {
    bounds: SwfBounds {
        x_min: -3.85,
        x_max: 4.35,
        y_min: -4.1,
        y_max: 4.1,
    },
    start: SwfPoint::new(3.15, -2.9),
    segments: &SEGMENTS,
};
