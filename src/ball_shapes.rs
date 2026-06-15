// Generated from gravity_arcade.swf DefineShape 2, 4, and 9 via reference/extract_shapes.py --contours.
// Coordinates are stored as SWF shape-local pixels.

use super::{RadialGradientShape, SwfPathSegment, SwfPoint};

#[cfg(test)]
pub const BALL_GLOW_DEFINE_SHAPE_ID: u16 = 2;
#[cfg(test)]
pub const BALL_GLOW_BOUNDS_TWIPS: [i16; 4] = [-280, 280, -280, 280];
pub const BALL_GLOW_GRADIENT_RGBA: [[u8; 5]; 2] =
    [[0, 153, 255, 255, 255], [230, 153, 255, 255, 0]];

const BALL_GLOW_SEGMENTS: [SwfPathSegment; 8] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(14.0, -5.8),
        to: SwfPoint::new(14.0, 0.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(14.0, 5.8),
        to: SwfPoint::new(9.85, 9.85),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(5.8, 14.0),
        to: SwfPoint::new(0.0, 14.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.8, 14.0),
        to: SwfPoint::new(-9.9, 9.85),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-14.0, 5.8),
        to: SwfPoint::new(-14.0, 0.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-14.0, -5.8),
        to: SwfPoint::new(-9.9, -9.9),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.8, -14.0),
        to: SwfPoint::new(0.0, -14.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(5.8, -14.0),
        to: SwfPoint::new(9.85, -9.9),
    },
];

pub const BALL_GLOW_SHAPE: RadialGradientShape = RadialGradientShape {
    base_radius: 14.0,
    start: SwfPoint::new(9.85, -9.9),
    segments: &BALL_GLOW_SEGMENTS,
};

#[cfg(test)]
pub const BALL_DIE_RING_DEFINE_SHAPE_ID: u16 = 4;
#[cfg(test)]
pub const BALL_DIE_RING_BOUNDS_TWIPS: [i16; 4] = [-230, 230, -230, 230];
pub const BALL_DIE_RING_GRADIENT_RGBA: [[u8; 5]; 3] = [
    [154, 222, 245, 255, 0],
    [212, 221, 254, 255, 255],
    [255, 222, 253, 255, 0],
];

const BALL_DIE_RING_SEGMENTS: [SwfPathSegment; 8] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(11.5, -4.75),
        to: SwfPoint::new(11.5, 0.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(11.5, 4.75),
        to: SwfPoint::new(8.1, 8.1),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(4.75, 11.5),
        to: SwfPoint::new(0.0, 11.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-4.75, 11.5),
        to: SwfPoint::new(-8.15, 8.1),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-11.5, 4.75),
        to: SwfPoint::new(-11.5, 0.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-11.5, -4.75),
        to: SwfPoint::new(-8.15, -8.15),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-4.75, -11.5),
        to: SwfPoint::new(0.0, -11.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(4.75, -11.5),
        to: SwfPoint::new(8.1, -8.15),
    },
];

pub const BALL_DIE_RING_SHAPE: RadialGradientShape = RadialGradientShape {
    base_radius: 11.5,
    start: SwfPoint::new(8.1, -8.15),
    segments: &BALL_DIE_RING_SEGMENTS,
};

#[cfg(test)]
pub const BALL_FIRE_DEFINE_SHAPE_ID: u16 = 9;
#[cfg(test)]
pub const BALL_FIRE_BOUNDS_TWIPS: [i16; 4] = [-110, 110, -110, 110];
pub const BALL_FIRE_GRADIENT_RGBA: [[u8; 5]; 2] = [[0, 255, 68, 68, 255], [250, 238, 0, 0, 0]];

const BALL_FIRE_SEGMENTS: [SwfPathSegment; 8] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(5.5, -2.3),
        to: SwfPoint::new(5.5, 0.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(5.5, 2.3),
        to: SwfPoint::new(3.9, 3.9),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(2.3, 5.5),
        to: SwfPoint::new(0.0, 5.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-2.3, 5.5),
        to: SwfPoint::new(-3.9, 3.9),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.5, 2.3),
        to: SwfPoint::new(-5.5, 0.0),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-5.5, -2.3),
        to: SwfPoint::new(-3.9, -3.9),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-2.3, -5.5),
        to: SwfPoint::new(0.0, -5.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(2.3, -5.5),
        to: SwfPoint::new(3.9, -3.9),
    },
];

pub const BALL_FIRE_SHAPE: RadialGradientShape = RadialGradientShape {
    base_radius: 5.5,
    start: SwfPoint::new(3.9, -3.9),
    segments: &BALL_FIRE_SEGMENTS,
};
