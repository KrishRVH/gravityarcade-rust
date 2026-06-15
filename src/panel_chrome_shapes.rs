// Generated from gravity_arcade.swf DefineShape 37, 80, and 91 via reference/extract_shapes.py --contours.
// Coordinates are stored as SWF stage pixels.

use super::{
    PanelChromeContour, PanelChromeFillShapeDefinition, PanelChromeLineShapeDefinition,
    PanelChromeMaskShapeDefinition, SwfBounds, SwfPathSegment, SwfPoint,
};

pub const FLATTEN_STEPS: u8 = 4;
#[cfg(test)]
pub const PANEL_FILL_DEFINE_SHAPE_ID: u16 = 37;
#[cfg(test)]
pub const RETAINED_MASK_DEFINE_SHAPE_ID: u16 = 80;
#[cfg(test)]
pub const PANEL_OUTLINE_DEFINE_SHAPE_ID: u16 = 91;
#[cfg(test)]
pub const PLAYFIELD_MASK_DEFINE_SHAPE_ID: u16 = 141;
#[cfg(test)]
pub const PANEL_FILL_BOUNDS_TWIPS: [i16; 4] = [141, 10860, 720, 7859];
#[cfg(test)]
pub const RETAINED_MASK_BOUNDS_TWIPS: [i16; 4] = [0, 10999, 0, 7999];
#[cfg(test)]
pub const PANEL_OUTLINE_BOUNDS_TWIPS: [i16; 4] = [121, 10880, 700, 7879];
#[cfg(test)]
pub const PLAYFIELD_MASK_BOUNDS_TWIPS: [i16; 4] = [0, 10999, 0, 7999];
pub const PANEL_FILL_RGB: [u8; 3] = [0, 0, 102];
pub const PANEL_SHADOW_RGB: [u8; 3] = [0, 0, 82];
pub const MASK_FILL_RGB: [u8; 3] = [99, 0, 0];
pub const OUTLINE_RGB: [u8; 3] = [255, 255, 255];
pub const OUTLINE_LINE_WIDTH: f32 = 2.0;

pub const PANEL_CENTER_BOUNDS: SwfBounds = SwfBounds {
    x_min: 35.05,
    x_max: 515.0,
    y_min: 36.0,
    y_max: 392.95,
};

const PANEL_CENTER_SEGMENTS: [SwfPathSegment; 4] = [
    SwfPathSegment::Line(SwfPoint::new(515.0, 392.95)),
    SwfPathSegment::Line(SwfPoint::new(35.05, 392.95)),
    SwfPathSegment::Line(SwfPoint::new(35.05, 36.0)),
    SwfPathSegment::Line(SwfPoint::new(515.0, 36.0)),
];

const PANEL_LEFT_SHADOW_SEGMENTS: [SwfPathSegment; 8] = [
    SwfPathSegment::Line(SwfPoint::new(12.05, 392.95)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(10.0, 392.95),
        to: SwfPoint::new(8.5, 391.45),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(7.05, 390.0),
        to: SwfPoint::new(7.05, 387.95),
    },
    SwfPathSegment::Line(SwfPoint::new(7.05, 41.0)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(7.05, 38.95),
        to: SwfPoint::new(8.5, 37.45),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(10.0, 36.0),
        to: SwfPoint::new(12.05, 36.0),
    },
    SwfPathSegment::Line(SwfPoint::new(35.05, 36.0)),
    SwfPathSegment::Line(SwfPoint::new(35.05, 392.95)),
];

const PANEL_RIGHT_SHADOW_SEGMENTS: [SwfPathSegment; 15] = [
    SwfPathSegment::Line(SwfPoint::new(515.0, 392.95)),
    SwfPathSegment::Line(SwfPoint::new(515.0, 36.0)),
    SwfPathSegment::Line(SwfPoint::new(538.0, 36.0)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(540.0, 36.0),
        to: SwfPoint::new(541.5, 37.45),
    },
    SwfPathSegment::Line(SwfPoint::new(541.55, 37.5)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(543.0, 38.95),
        to: SwfPoint::new(543.0, 41.0),
    },
    SwfPathSegment::Line(SwfPoint::new(543.0, 387.95)),
    SwfPathSegment::Line(SwfPoint::new(543.0, 388.1)),
    SwfPathSegment::Line(SwfPoint::new(543.0, 388.15)),
    SwfPathSegment::Line(SwfPoint::new(542.75, 389.5)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(542.4, 390.6),
        to: SwfPoint::new(541.5, 391.45),
    },
    SwfPathSegment::Line(SwfPoint::new(539.55, 392.7)),
    SwfPathSegment::Line(SwfPoint::new(538.2, 392.95)),
    SwfPathSegment::Line(SwfPoint::new(538.1, 392.95)),
    SwfPathSegment::Line(SwfPoint::new(538.0, 392.95)),
];

const PANEL_OUTLINE_PRIMARY_SEGMENTS: [SwfPathSegment; 19] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(540.0, 36.0),
        to: SwfPoint::new(538.0, 36.0),
    },
    SwfPathSegment::Line(SwfPoint::new(12.05, 36.0)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(10.0, 36.0),
        to: SwfPoint::new(8.5, 37.45),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(7.05, 38.95),
        to: SwfPoint::new(7.05, 41.0),
    },
    SwfPathSegment::Line(SwfPoint::new(7.05, 387.95)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(7.05, 390.0),
        to: SwfPoint::new(8.5, 391.45),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(10.0, 392.95),
        to: SwfPoint::new(12.05, 392.95),
    },
    SwfPathSegment::Line(SwfPoint::new(538.0, 392.95)),
    SwfPathSegment::Line(SwfPoint::new(538.1, 392.95)),
    SwfPathSegment::Line(SwfPoint::new(538.2, 392.95)),
    SwfPathSegment::Line(SwfPoint::new(539.55, 392.7)),
    SwfPathSegment::Line(SwfPoint::new(541.5, 391.45)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(542.4, 390.6),
        to: SwfPoint::new(542.75, 389.5),
    },
    SwfPathSegment::Line(SwfPoint::new(543.0, 388.15)),
    SwfPathSegment::Line(SwfPoint::new(543.0, 388.1)),
    SwfPathSegment::Line(SwfPoint::new(543.0, 387.95)),
    SwfPathSegment::Line(SwfPoint::new(543.0, 41.0)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(543.0, 38.95),
        to: SwfPoint::new(541.55, 37.5),
    },
    SwfPathSegment::Line(SwfPoint::new(541.5, 37.45)),
];

const PANEL_OUTLINE_LOWER_RIGHT_SEGMENTS: [SwfPathSegment; 2] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(540.65, 392.35),
        to: SwfPoint::new(541.55, 391.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(542.4, 390.6),
        to: SwfPoint::new(542.75, 389.5),
    },
];

const RETAINED_MASK_OUTLINE_PRIMARY_SEGMENTS: [SwfPathSegment; 19] = [
    SwfPathSegment::Line(SwfPoint::new(543.0, 388.15)),
    SwfPathSegment::Line(SwfPoint::new(543.0, 388.1)),
    SwfPathSegment::Line(SwfPoint::new(543.0, 387.95)),
    SwfPathSegment::Line(SwfPoint::new(543.0, 41.0)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(543.0, 38.95),
        to: SwfPoint::new(541.55, 37.5),
    },
    SwfPathSegment::Line(SwfPoint::new(541.5, 37.45)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(540.0, 36.0),
        to: SwfPoint::new(538.0, 36.0),
    },
    SwfPathSegment::Line(SwfPoint::new(12.05, 36.0)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(10.0, 36.0),
        to: SwfPoint::new(8.5, 37.45),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(7.05, 38.95),
        to: SwfPoint::new(7.05, 41.0),
    },
    SwfPathSegment::Line(SwfPoint::new(7.05, 387.95)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(7.05, 390.0),
        to: SwfPoint::new(8.5, 391.45),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(10.0, 392.95),
        to: SwfPoint::new(12.05, 392.95),
    },
    SwfPathSegment::Line(SwfPoint::new(538.0, 392.95)),
    SwfPathSegment::Line(SwfPoint::new(538.1, 392.95)),
    SwfPathSegment::Line(SwfPoint::new(538.2, 392.95)),
    SwfPathSegment::Line(SwfPoint::new(539.55, 392.7)),
    SwfPathSegment::Line(SwfPoint::new(541.5, 391.45)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(542.4, 390.6),
        to: SwfPoint::new(542.75, 389.5),
    },
];

const RETAINED_MASK_OUTLINE_LOWER_RIGHT_SEGMENTS: [SwfPathSegment; 2] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(542.4, 390.6),
        to: SwfPoint::new(541.55, 391.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(540.65, 392.35),
        to: SwfPoint::new(539.55, 392.7),
    },
];

const PLAYFIELD_MASK_TOP_LEFT_SEGMENTS: [SwfPathSegment; 4] = [
    SwfPathSegment::Line(SwfPoint::new(7.05, 36.0)),
    SwfPathSegment::Line(SwfPoint::new(12.05, 36.0)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(10.0, 36.0),
        to: SwfPoint::new(8.5, 37.45),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(7.05, 38.95),
        to: SwfPoint::new(7.05, 41.0),
    },
];

const PLAYFIELD_MASK_TOP_RIGHT_SEGMENTS: [SwfPathSegment; 5] = [
    SwfPathSegment::Line(SwfPoint::new(543.0, 36.0)),
    SwfPathSegment::Line(SwfPoint::new(543.0, 41.0)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(543.0, 38.95),
        to: SwfPoint::new(541.55, 37.5),
    },
    SwfPathSegment::Line(SwfPoint::new(541.5, 37.45)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(540.0, 36.0),
        to: SwfPoint::new(538.0, 36.0),
    },
];

const PLAYFIELD_MASK_BOTTOM_LEFT_SEGMENTS: [SwfPathSegment; 4] = [
    SwfPathSegment::Line(SwfPoint::new(7.05, 392.95)),
    SwfPathSegment::Line(SwfPoint::new(12.05, 392.95)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(10.0, 392.95),
        to: SwfPoint::new(8.5, 391.45),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(7.05, 390.0),
        to: SwfPoint::new(7.05, 387.95),
    },
];

const PLAYFIELD_MASK_BOTTOM_RIGHT_SEGMENTS: [SwfPathSegment; 10] = [
    SwfPathSegment::Line(SwfPoint::new(543.0, 392.95)),
    SwfPathSegment::Line(SwfPoint::new(543.0, 387.95)),
    SwfPathSegment::Line(SwfPoint::new(543.0, 388.1)),
    SwfPathSegment::Line(SwfPoint::new(543.0, 388.15)),
    SwfPathSegment::Line(SwfPoint::new(542.75, 389.5)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(542.4, 390.6),
        to: SwfPoint::new(541.55, 391.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(540.65, 392.35),
        to: SwfPoint::new(539.55, 392.7),
    },
    SwfPathSegment::Line(SwfPoint::new(538.2, 392.95)),
    SwfPathSegment::Line(SwfPoint::new(538.1, 392.95)),
    SwfPathSegment::Line(SwfPoint::new(538.0, 392.95)),
];

pub const PANEL_FILL_SHAPE: PanelChromeFillShapeDefinition = PanelChromeFillShapeDefinition {
    bounds: SwfBounds {
        x_min: 7.05,
        x_max: 543.0,
        y_min: 36.0,
        y_max: 392.95,
    },
    center: PanelChromeContour {
        start: SwfPoint::new(515.0, 36.0),
        segments: &PANEL_CENTER_SEGMENTS,
    },
    left_shadow: PanelChromeContour {
        start: SwfPoint::new(35.05, 392.95),
        segments: &PANEL_LEFT_SHADOW_SEGMENTS,
    },
    right_shadow: PanelChromeContour {
        start: SwfPoint::new(538.0, 392.95),
        segments: &PANEL_RIGHT_SHADOW_SEGMENTS,
    },
};

pub const PANEL_OUTLINE_SHAPE: PanelChromeLineShapeDefinition = PanelChromeLineShapeDefinition {
    bounds: SwfBounds {
        x_min: 6.05,
        x_max: 544.0,
        y_min: 35.0,
        y_max: 393.95,
    },
    primary: PanelChromeContour {
        start: SwfPoint::new(541.5, 37.45),
        segments: &PANEL_OUTLINE_PRIMARY_SEGMENTS,
    },
    lower_right: PanelChromeContour {
        start: SwfPoint::new(539.55, 392.7),
        segments: &PANEL_OUTLINE_LOWER_RIGHT_SEGMENTS,
    },
};

pub const RETAINED_MASK_OUTLINE_SHAPE: PanelChromeLineShapeDefinition =
    PanelChromeLineShapeDefinition {
        bounds: SwfBounds {
            x_min: 0.0,
            x_max: 549.95,
            y_min: 0.0,
            y_max: 399.95,
        },
        primary: PanelChromeContour {
            start: SwfPoint::new(542.75, 389.5),
            segments: &RETAINED_MASK_OUTLINE_PRIMARY_SEGMENTS,
        },
        lower_right: PanelChromeContour {
            start: SwfPoint::new(542.75, 389.5),
            segments: &RETAINED_MASK_OUTLINE_LOWER_RIGHT_SEGMENTS,
        },
    };

pub const PLAYFIELD_MASK_SHAPE: PanelChromeMaskShapeDefinition = PanelChromeMaskShapeDefinition {
    bounds: SwfBounds {
        x_min: 0.0,
        x_max: 549.95,
        y_min: 0.0,
        y_max: 399.95,
    },
    top_left: PanelChromeContour {
        start: SwfPoint::new(7.05, 41.0),
        segments: &PLAYFIELD_MASK_TOP_LEFT_SEGMENTS,
    },
    top_right: PanelChromeContour {
        start: SwfPoint::new(538.0, 36.0),
        segments: &PLAYFIELD_MASK_TOP_RIGHT_SEGMENTS,
    },
    bottom_left: PanelChromeContour {
        start: SwfPoint::new(7.05, 387.95),
        segments: &PLAYFIELD_MASK_BOTTOM_LEFT_SEGMENTS,
    },
    bottom_right: PanelChromeContour {
        start: SwfPoint::new(538.0, 392.95),
        segments: &PLAYFIELD_MASK_BOTTOM_RIGHT_SEGMENTS,
    },
};
