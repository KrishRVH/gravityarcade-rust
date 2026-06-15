// Generated from gravity_arcade.swf DefineShape 84 and 87 via reference/extract_shapes.py --contours.
// Coordinates are stored as SWF shape-local pixels.

use super::{GoalLineShapeDefinition, SwfBounds, SwfPathSegment, SwfPoint};

#[cfg(test)]
pub const RED_DEFINE_SHAPE_ID: u16 = 84;
#[cfg(test)]
pub const BLUE_DEFINE_SHAPE_ID: u16 = 87;
#[cfg(test)]
pub const BOUNDS_TWIPS: [i16; 4] = [-20, 20, -3589, 3590];
pub const RED_FILL_RGB: [u8; 3] = [255, 166, 34];
pub const BLUE_FILL_RGB: [u8; 3] = [0, 179, 221];

const SEGMENTS: [SwfPathSegment; 10] = [
    SwfPathSegment::Line(SwfPoint::new(0.0, -179.45)),
    SwfPathSegment::Line(SwfPoint::new(0.7, -179.15)),
    SwfPathSegment::Line(SwfPoint::new(1.0, -178.45)),
    SwfPathSegment::Line(SwfPoint::new(1.0, 178.5)),
    SwfPathSegment::Line(SwfPoint::new(0.7, 179.2)),
    SwfPathSegment::Line(SwfPoint::new(0.0, 179.5)),
    SwfPathSegment::Line(SwfPoint::new(-0.7, 179.2)),
    SwfPathSegment::Line(SwfPoint::new(-1.0, 178.5)),
    SwfPathSegment::Line(SwfPoint::new(-1.0, -178.45)),
    SwfPathSegment::Line(SwfPoint::new(-0.7, -179.15)),
];

pub const SHAPE: GoalLineShapeDefinition = GoalLineShapeDefinition {
    bounds: SwfBounds {
        x_min: -1.0,
        x_max: 1.0,
        y_min: -179.45,
        y_max: 179.5,
    },
    start: SwfPoint::new(-0.7, -179.15),
    segments: &SEGMENTS,
};
