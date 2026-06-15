// Generated from gravity_arcade.swf DefineShape 92, 93, and 94 via reference/extract_shapes.py --contours.
// Coordinates are stored as SWF shape-local pixels.

use super::{SideMarkerContour, SideMarkerShapeDefinition, SwfBounds, SwfPathSegment, SwfPoint};

pub const FLATTEN_STEPS: u8 = 4;
#[cfg(test)]
pub const RED_FILL_DEFINE_SHAPE_ID: u16 = 92;
#[cfg(test)]
pub const OUTLINE_DEFINE_SHAPE_ID: u16 = 93;
#[cfg(test)]
pub const BLUE_FILL_DEFINE_SHAPE_ID: u16 = 94;
#[cfg(test)]
pub const RED_FILL_BOUNDS_TWIPS: [i16; 4] = [-112, 113, -112, 113];
#[cfg(test)]
pub const OUTLINE_BOUNDS_TWIPS: [i16; 4] = [-180, 180, -180, 180];
#[cfg(test)]
pub const BLUE_FILL_BOUNDS_TWIPS: [i16; 4] = [-113, 112, -112, 113];
pub const RED_FILL_RGB: [u8; 3] = [255, 171, 34];
pub const OUTLINE_RGB: [u8; 3] = [0, 0, 0];
pub const BLUE_FILL_RGB: [u8; 3] = [127, 212, 212];
pub const OUTLINE_LINE_WIDTH: f32 = 1.0;

const RED_FILL_SEGMENTS: [SwfPathSegment; 8] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(5.65, -2.3),
        to: SwfPoint::new(5.65, 0.05),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(5.65, 2.35),
        to: SwfPoint::new(4.0, 4.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(2.3, 5.65),
        to: SwfPoint::new(0.0, 5.65),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-2.3, 5.65),
        to: SwfPoint::new(-3.95, 4.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.65, 2.35),
        to: SwfPoint::new(-5.6, 0.05),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.65, -2.3),
        to: SwfPoint::new(-3.95, -3.95),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-2.3, -5.6),
        to: SwfPoint::new(0.0, -5.6),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(2.3, -5.6),
        to: SwfPoint::new(4.0, -3.95),
    },
];

const OUTLINE_SEGMENTS: [SwfPathSegment; 8] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(8.5, -3.5),
        to: SwfPoint::new(8.5, 0.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(8.5, 3.5),
        to: SwfPoint::new(6.0, 6.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(3.5, 8.5),
        to: SwfPoint::new(0.0, 8.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-3.5, 8.5),
        to: SwfPoint::new(-6.0, 6.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-8.5, 3.5),
        to: SwfPoint::new(-8.5, 0.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-8.5, -3.5),
        to: SwfPoint::new(-6.0, -6.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-3.5, -8.5),
        to: SwfPoint::new(0.0, -8.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(3.5, -8.5),
        to: SwfPoint::new(6.0, -6.0),
    },
];

const BLUE_FILL_SEGMENTS: [SwfPathSegment; 8] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(2.25, -5.6),
        to: SwfPoint::new(3.95, -3.95),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(5.6, -2.3),
        to: SwfPoint::new(5.6, 0.05),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(5.6, 2.35),
        to: SwfPoint::new(3.95, 4.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(2.25, 5.65),
        to: SwfPoint::new(-0.05, 5.65),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-2.35, 5.65),
        to: SwfPoint::new(-4.0, 4.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.7, 2.35),
        to: SwfPoint::new(-5.65, 0.05),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.7, -2.3),
        to: SwfPoint::new(-4.0, -3.95),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-2.35, -5.6),
        to: SwfPoint::new(-0.05, -5.6),
    },
];

pub const RED_FILL_SHAPE: SideMarkerShapeDefinition = SideMarkerShapeDefinition {
    bounds: SwfBounds {
        x_min: -5.6,
        x_max: 5.65,
        y_min: -5.6,
        y_max: 5.65,
    },
    contour: SideMarkerContour {
        start: SwfPoint::new(4.0, -3.95),
        segments: &RED_FILL_SEGMENTS,
    },
};

pub const OUTLINE_SHAPE: SideMarkerShapeDefinition = SideMarkerShapeDefinition {
    bounds: SwfBounds {
        x_min: -9.0,
        x_max: 9.0,
        y_min: -9.0,
        y_max: 9.0,
    },
    contour: SideMarkerContour {
        start: SwfPoint::new(6.0, -6.0),
        segments: &OUTLINE_SEGMENTS,
    },
};

pub const BLUE_FILL_SHAPE: SideMarkerShapeDefinition = SideMarkerShapeDefinition {
    bounds: SwfBounds {
        x_min: -5.65,
        x_max: 5.6,
        y_min: -5.6,
        y_max: 5.65,
    },
    contour: SideMarkerContour {
        start: SwfPoint::new(-0.05, -5.6),
        segments: &BLUE_FILL_SEGMENTS,
    },
};
