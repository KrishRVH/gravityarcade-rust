// Generated from gravity_arcade.swf DefineShape 41 via reference/extract_shapes.py.
// Coordinates are stored as SWF shape-local pixels.

use super::{GravityPreviewArrowPolyline, GravityPreviewArrowShapeDefinition, SwfBounds, SwfPoint};

#[cfg(test)]
pub const DEFINE_SHAPE_ID: u16 = 41;
#[cfg(test)]
pub const BOUNDS_TWIPS: [i16; 4] = [-355, 139, -153, 153];
pub const LINE_RGB: [u8; 3] = [255, 251, 204];
pub const LINE_WIDTH: f32 = 2.0;

const HEAD_POINTS: [SwfPoint; 2] = [SwfPoint::new(5.95, 0.15), SwfPoint::new(0.2, 6.65)];

const SHAFT_POINTS: [SwfPoint; 1] = [SwfPoint::new(5.95, 0.15)];

pub const SHAPE: GravityPreviewArrowShapeDefinition = GravityPreviewArrowShapeDefinition {
    bounds: SwfBounds {
        x_min: -17.75,
        x_max: 6.95,
        y_min: -7.65,
        y_max: 7.65,
    },
    head: GravityPreviewArrowPolyline {
        start: SwfPoint::new(-0.05, -6.65),
        points: &HEAD_POINTS,
    },
    shaft: GravityPreviewArrowPolyline {
        start: SwfPoint::new(-16.75, 0.15),
        points: &SHAFT_POINTS,
    },
};
