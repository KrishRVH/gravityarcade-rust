//! SWF-derived 30 Hz simulation for Gravity Arcade.
//!
//! Coordinates are in the original Flash stage space where the playfield
//! sprite `w` is centered at `(273.45, 214.45)` and its local coordinates
//! range roughly from `-235..235` horizontally and `-170..170` vertically.
#![allow(
    clippy::unreadable_literal,
    reason = "Long decimals are recovered SWF transform values kept verbatim for auditability"
)]

pub const STAGE_W: f64 = 550.0;
pub const STAGE_H: f64 = 400.0;
pub const TICK_HZ: f64 = 30.0;
pub const WORLD_X: f64 = 273.45;
pub const WORLD_Y: f64 = 214.45;
pub const X_MIN_BALL: f64 = -235.0;
pub const X_MAX_BALL: f64 = 235.0;
pub const Y_MIN: f64 = -150.0;
pub const Y_MAX: f64 = 150.0;
pub const Y_MIN_BALL: f64 = -170.0;
pub const Y_MAX_BALL: f64 = 170.0;
pub const LEFT_PADDLE_X: f64 = -238.45;
pub const RIGHT_PADDLE_X: f64 = 241.25;
pub const BLUE_PADDLE_INITIAL_Y: f64 = -0.25;
pub const RED_PADDLE_INITIAL_Y: f64 = -0.3;
pub const PADDLE_SIZE: f64 = 30.0;
pub const PADDLE_ACCEL: f64 = 1.0;
pub const PADDLE_FRICTION: f64 = 0.8;
pub const FIRE_THRESHOLD: u32 = 20;
pub const ENERGY_MAX_FRAME: u32 = 200;
pub const BALL_BASE_SPEED: f64 = 10.0;
pub const BALL_MAX_SPEED: f64 = 70.0;
pub const BALL_GLOW_RADIUS: f64 = 14.0;
pub const BALL_CORE_RADIUS: f64 = 11.5;
pub const BALL_FIRE_RADIUS: f64 = 5.5;
pub const BALL_DIE_TICKS: u32 = 11;
pub const SLOPE: f64 = 50_000.0;
pub const MERGE_DISTANCE: f64 = 15.0;
pub const ENERGY_FACTOR: f64 = 500.0;
pub const SCORE_METER_MAX: i32 = 700;
pub const SCORE_METER_MAX_FRAME: u8 = 71;
/// The score meter clamps at frame 71 (`score = 700`), but the round win
/// branch only fires when `int(score / 10) + 1 > 71`, so score must reach 710.
pub const MATCH_SCORE: i32 = 710;
pub const FIRE_BONUS: i32 = 40;
/// The SWF sets `firebonusMod` but reads `firebonusMulti`; AS1/SWF5 coerces
/// that undefined read to `0`, so the runtime bonus is zero.
pub const FIRE_BONUS_MULTI: f64 = 0.0;
pub const VELOCITY_MULTI: f64 = 0.3;
pub const ROUND_INTRO_TICKS: u32 = 60;
/// Sprite 162's `newgame` visual runs from frame 195 through the frame-267
/// zero-alpha display update; frame 268 removes the display and stops.
pub const ROUND_INTRO_VISUAL_TICKS: u32 = 73;
/// Sprite 162 frame 212 starts the round-start sound while the round display
/// first appears.
pub const ROUND_INTRO_SOUND_TICKS: u32 = 56;
/// The non-final blue/red win announce runs from sprite 162 frame 2/98 until
/// frame 97/194 jumps to the `newgame` label, delaying score reset by 96 ticks.
pub const MATCH_WIN_ANNOUNCE_TICKS: u32 = 96;
/// Final blue wins path in sprite 162 plays the `bluefinal` label at frame 269
/// until frame 430 sends the root timeline back to the menu.
pub const BLUE_FINAL_WIN_ANNOUNCE_TICKS: u32 = 162;
/// Final red wins path in sprite 162 plays the `redfinal` label at frame 432
/// until frame 588 sends the root timeline back to the menu.
pub const RED_FINAL_WIN_ANNOUNCE_TICKS: u32 = 157;
/// `setEnergy` switches the ball clip to its burning frame when
/// `ballE / 8 + 30 > 100`.
pub const BURN_ENERGY_THRESHOLD: f64 = 560.0;
/// Player sprite 132 jumps to the `paralised` label on frame 4 and returns to
/// its movement loop at frame 63, giving 60 disabled frames at 30 Hz.
pub const PADDLE_STUN_TICKS: u32 = 60;

const BALL_DIE_VISUALS: [DyingBallVisual; BALL_DIE_TICKS as usize] = [
    DyingBallVisual {
        scale: 0.586944580078125,
        alpha: 0.0,
    },
    DyingBallVisual {
        scale: 0.6173553466796875,
        alpha: 95.0 / 256.0,
    },
    DyingBallVisual {
        scale: 0.6685638427734375,
        // Sprite 15 frame 3 moves the existing display-list entry without a
        // new color transform, so the frame-2 alpha multiplier carries over.
        alpha: 95.0 / 256.0,
    },
    DyingBallVisual {
        scale: 0.7942352294921875,
        alpha: 224.0 / 256.0,
    },
    DyingBallVisual {
        scale: 0.9199066162109375,
        alpha: 192.0 / 256.0,
    },
    DyingBallVisual {
        scale: 1.045562744140625,
        alpha: 160.0 / 256.0,
    },
    DyingBallVisual {
        scale: 1.171234130859375,
        alpha: 128.0 / 256.0,
    },
    DyingBallVisual {
        scale: 1.296905517578125,
        alpha: 96.0 / 256.0,
    },
    DyingBallVisual {
        scale: 1.422576904296875,
        alpha: 64.0 / 256.0,
    },
    DyingBallVisual {
        scale: 1.5482330322265625,
        alpha: 32.0 / 256.0,
    },
    DyingBallVisual {
        scale: 1.6739044189453125,
        alpha: 0.0,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Blue,
    Red,
}

impl Side {
    #[must_use]
    pub const fn opponent(self) -> Self {
        match self {
            Self::Blue => Self::Red,
            Self::Red => Self::Blue,
        }
    }
}

#[must_use]
pub const fn final_win_announce_ticks(side: Side) -> u32 {
    match side {
        Side::Blue => BLUE_FINAL_WIN_ANNOUNCE_TICKS,
        Side::Red => RED_FINAL_WIN_ANNOUNCE_TICKS,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarisation {
    Neutral,
    SameRepels,
    OppositeRepels,
    AllRepel,
}

impl Polarisation {
    #[must_use]
    pub const fn swf_value(self) -> i32 {
        match self {
            Self::Neutral => 0,
            Self::SameRepels => 1,
            Self::OppositeRepels => -1,
            Self::AllRepel => 2,
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Neutral => "gravity",
            Self::SameRepels => "same repel",
            Self::OppositeRepels => "opposite repel",
            Self::AllRepel => "all repel",
        }
    }

    #[must_use]
    pub const fn next(self) -> Self {
        match self {
            Self::Neutral => Self::OppositeRepels,
            Self::OppositeRepels => Self::SameRepels,
            Self::SameRepels => Self::AllRepel,
            Self::AllRepel => Self::Neutral,
        }
    }

    #[must_use]
    pub const fn reverses_force(self, same_color: bool) -> bool {
        match self {
            Self::Neutral => false,
            Self::SameRepels => same_color,
            Self::OppositeRepels => !same_color,
            Self::AllRepel => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeedMode {
    Normal,
    Fast,
}

impl SpeedMode {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Fast => "fast",
        }
    }

    #[must_use]
    pub const fn next(self) -> Self {
        match self {
            Self::Normal => Self::Fast,
            Self::Fast => Self::Normal,
        }
    }

    #[must_use]
    pub const fn reflect_mod(self) -> f64 {
        match self {
            Self::Normal => 1.0,
            Self::Fast => 1.15,
        }
    }

    #[must_use]
    pub const fn score_energy_multi(self) -> f64 {
        match self {
            Self::Normal => 10.0,
            Self::Fast => 50.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GravityStrength {
    G1,
    G2,
    G3,
    G4,
    G5,
}

impl GravityStrength {
    #[must_use]
    pub const fn factor(self) -> f64 {
        match self {
            Self::G1 => 10.0,
            Self::G2 => 17.0,
            Self::G3 => 50.0,
            Self::G4 => 120.0,
            Self::G5 => 210.0,
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::G1 => "1",
            Self::G2 => "2",
            Self::G3 => "3",
            Self::G4 => "4",
            Self::G5 => "5",
        }
    }

    #[must_use]
    pub const fn next(self) -> Self {
        match self {
            Self::G1 => Self::G2,
            Self::G2 => Self::G3,
            Self::G3 => Self::G4,
            Self::G4 => Self::G5,
            Self::G5 => Self::G1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub matches: u8,
    pub polarisation: Polarisation,
    pub gravity: GravityStrength,
    pub speed: SpeedMode,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            matches: 3,
            polarisation: Polarisation::Neutral,
            gravity: GravityStrength::G2,
            speed: SpeedMode::Normal,
        }
    }
}

impl Settings {
    #[must_use]
    pub const fn wins_needed(self) -> u8 {
        self.matches / 2 + 1
    }

    pub fn cycle_matches(&mut self) {
        self.matches = match self.matches {
            1 => 3,
            3 => 5,
            5 => 7,
            _ => 1,
        };
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "Per-tick input mirrors the SWF's direct key-state checks"
)]
pub struct Controls {
    pub blue_up: bool,
    pub blue_down: bool,
    pub blue_fire: bool,
    pub red_up: bool,
    pub red_down: bool,
    pub red_fire: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub side: Side,
    pub y: f64,
    pub v: f64,
    pub energy_frame: u32,
    pub gun_ready: bool,
    pub stun_ticks: u32,
}

impl Player {
    #[must_use]
    pub const fn new(side: Side) -> Self {
        Self {
            side,
            y: initial_paddle_y(side),
            v: 0.0,
            energy_frame: ENERGY_MAX_FRAME,
            gun_ready: false,
            stun_ticks: 0,
        }
    }

    pub fn tick(&mut self, up: bool, down: bool) -> bool {
        let was_stunned = self.tick_velocity(up, down);
        self.apply_position();
        was_stunned
    }

    fn tick_velocity(&mut self, up: bool, down: bool) -> bool {
        let was_stunned = self.stun_ticks > 0;
        if was_stunned {
            self.stun_ticks -= 1;
            self.v = 0.0;
        } else if down {
            self.v += PADDLE_ACCEL;
            if self.v < 2.0 {
                self.v += PADDLE_ACCEL;
            }
        } else if up {
            self.v -= PADDLE_ACCEL;
            if -2.0 < self.v {
                self.v -= PADDLE_ACCEL;
            }
        } else {
            self.v *= PADDLE_FRICTION;
        }

        if self.energy_frame < ENERGY_MAX_FRAME {
            self.energy_frame += 1;
        }
        was_stunned
    }

    fn apply_position(&mut self) {
        let next_y = self.y + self.v;
        if next_y < Y_MIN {
            self.y = Y_MIN;
            self.v = 0.0;
        } else if next_y > Y_MAX {
            self.y = Y_MAX;
            self.v = 0.0;
        } else {
            self.y = next_y;
        }
    }
}

#[must_use]
pub const fn initial_paddle_y(side: Side) -> f64 {
    match side {
        Side::Blue => BLUE_PADDLE_INITIAL_Y,
        Side::Red => RED_PADDLE_INITIAL_Y,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PaddleChargeVisual {
    pub sx: f64,
    pub sy: f64,
    pub color: (u8, u8, u8),
    pub ready_flash: Option<PaddleReadyFlash>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PaddleReadyFlash {
    pub scale: f64,
    pub alpha: f64,
}

#[must_use]
pub fn paddle_charge_visual(side: Side, energy_frame: u32) -> PaddleChargeVisual {
    let idx = energy_frame.clamp(1, ENERGY_MAX_FRAME) as usize - 1;
    let (sx, sy) = PADDLE_CHARGE_SCALE[idx.min(PADDLE_CHARGE_SCALE.len() - 1)];
    let color = match side {
        Side::Blue => BLUE_PADDLE_CHARGE_COLOR[idx],
        Side::Red => RED_PADDLE_CHARGE_COLOR[idx],
    };
    PaddleChargeVisual {
        sx,
        sy,
        color,
        ready_flash: paddle_ready_flash(idx as u32 + 1),
    }
}

#[must_use]
pub fn paddle_stun_color(stun_ticks: u32) -> Option<(u8, u8, u8)> {
    if stun_ticks == 0 {
        return None;
    }

    match (PADDLE_STUN_TICKS.saturating_sub(stun_ticks)) % 3 {
        0 => Some((255, 0, 0)),
        1 => Some((238, 255, 68)),
        _ => Some((255, 255, 255)),
    }
}

#[must_use]
pub const fn score_meter_frame(score: i32) -> u8 {
    let frame = score / 10 + 1;
    if frame < 1 {
        1
    } else if frame > SCORE_METER_MAX_FRAME as i32 {
        SCORE_METER_MAX_FRAME
    } else {
        frame as u8
    }
}

fn paddle_ready_flash(frame: u32) -> Option<PaddleReadyFlash> {
    match frame {
        197 => Some(PaddleReadyFlash {
            scale: 0.4878082275390625,
            alpha: 0.0,
        }),
        198 => Some(PaddleReadyFlash {
            scale: 0.658538818359375,
            alpha: 85.0 / 256.0,
        }),
        199 => Some(PaddleReadyFlash {
            scale: 0.8292694091796875,
            alpha: 171.0 / 256.0,
        }),
        200 => Some(PaddleReadyFlash {
            scale: 1.0,
            alpha: 1.0,
        }),
        _ => None,
    }
}

const PADDLE_CHARGE_SCALE: [(f64, f64); 63] = [
    (0.531463623046875, 0.7529144287109375),
    (0.5450286865234375, 0.76007080078125),
    (0.5583953857421875, 0.767120361328125),
    (0.571563720703125, 0.7740631103515625),
    (0.5845489501953125, 0.780914306640625),
    (0.5973358154296875, 0.78765869140625),
    (0.6099395751953125, 0.7942962646484375),
    (0.6223297119140625, 0.8008270263671875),
    (0.634521484375, 0.8072662353515625),
    (0.64654541015625, 0.8135986328125),
    (0.6583709716796875, 0.8198394775390625),
    (0.6699981689453125, 0.8259735107421875),
    (0.6814117431640625, 0.8319854736328125),
    (0.692657470703125, 0.837921142578125),
    (0.703704833984375, 0.84375),
    (0.7145538330078125, 0.8494720458984375),
    (0.725189208984375, 0.8550872802734375),
    (0.73565673828125, 0.860595703125),
    (0.7459259033203125, 0.8660125732421875),
    (0.7559967041015625, 0.8713226318359375),
    (0.7658843994140625, 0.8765411376953125),
    (0.77557373046875, 0.8816375732421875),
    (0.7850494384765625, 0.8866424560546875),
    (0.7943572998046875, 0.8915557861328125),
    (0.803466796875, 0.8963623046875),
    (0.8123626708984375, 0.9010467529296875),
    (0.8210906982421875, 0.9056549072265625),
    (0.8296051025390625, 0.9101409912109375),
    (0.8379364013671875, 0.9145355224609375),
    (0.8460693359375, 0.9188232421875),
    (0.85400390625, 0.9230194091796875),
    (0.8617706298828125, 0.9271087646484375),
    (0.8693084716796875, 0.9310760498046875),
    (0.876678466796875, 0.934967041015625),
    (0.88385009765625, 0.938751220703125),
    (0.89080810546875, 0.942413330078125),
    (0.8975982666015625, 0.9459991455078125),
    (0.9041748046875, 0.949462890625),
    (0.9105682373046875, 0.9528350830078125),
    (0.9167633056640625, 0.9561004638671875),
    (0.922760009765625, 0.959259033203125),
    (0.9285736083984375, 0.9623260498046875),
    (0.9341888427734375, 0.965301513671875),
    (0.9396209716796875, 0.9681549072265625),
    (0.9448394775390625, 0.9709014892578125),
    (0.9498748779296875, 0.97357177734375),
    (0.9547119140625, 0.9761199951171875),
    (0.9593658447265625, 0.97857666015625),
    (0.96380615234375, 0.9809112548828125),
    (0.96807861328125, 0.9831695556640625),
    (0.972137451171875, 0.9853057861328125),
    (0.97601318359375, 0.9873504638671875),
    (0.9796905517578125, 0.989288330078125),
    (0.9831695556640625, 0.991119384765625),
    (0.9864654541015625, 0.99285888671875),
    (0.9895477294921875, 0.9944915771484375),
    (0.992462158203125, 0.9960174560546875),
    (0.9951629638671875, 0.9974517822265625),
    (0.9976806640625, 0.998779296875),
    (1.0, 1.0),
    (1.0, 1.0),
    (1.0, 1.0),
    (1.0, 1.0),
];

const BLUE_PADDLE_CHARGE_COLOR: [(u8, u8, u8); 200] = [
    (221, 236, 255),
    (222, 236, 255),
    (222, 236, 255),
    (223, 236, 255),
    (224, 236, 255),
    (224, 236, 255),
    (225, 236, 255),
    (225, 236, 255),
    (226, 236, 255),
    (227, 236, 255),
    (227, 237, 255),
    (228, 237, 255),
    (228, 237, 255),
    (229, 237, 255),
    (229, 237, 255),
    (230, 237, 255),
    (231, 237, 255),
    (231, 237, 255),
    (232, 237, 255),
    (232, 237, 255),
    (233, 237, 255),
    (233, 237, 255),
    (233, 237, 255),
    (234, 237, 255),
    (234, 237, 255),
    (235, 237, 255),
    (235, 237, 255),
    (236, 237, 255),
    (236, 237, 255),
    (236, 237, 255),
    (237, 237, 255),
    (237, 237, 255),
    (238, 237, 255),
    (238, 237, 255),
    (238, 238, 255),
    (239, 238, 255),
    (239, 238, 255),
    (239, 238, 255),
    (240, 238, 255),
    (240, 238, 255),
    (240, 238, 255),
    (240, 238, 255),
    (241, 238, 255),
    (241, 238, 255),
    (241, 238, 255),
    (242, 238, 255),
    (242, 238, 255),
    (242, 238, 255),
    (242, 238, 255),
    (242, 238, 255),
    (243, 238, 255),
    (243, 238, 255),
    (243, 238, 255),
    (243, 238, 255),
    (243, 238, 255),
    (243, 238, 255),
    (244, 238, 255),
    (244, 238, 255),
    (244, 238, 255),
    (244, 238, 255),
    (244, 238, 255),
    (243, 238, 255),
    (243, 237, 255),
    (243, 237, 255),
    (242, 237, 255),
    (242, 237, 255),
    (242, 236, 255),
    (241, 236, 255),
    (241, 236, 255),
    (240, 235, 255),
    (239, 235, 255),
    (239, 235, 255),
    (238, 234, 255),
    (238, 234, 255),
    (237, 233, 255),
    (236, 233, 255),
    (236, 233, 255),
    (235, 232, 255),
    (234, 232, 255),
    (233, 231, 255),
    (232, 230, 255),
    (232, 230, 255),
    (231, 229, 255),
    (230, 229, 255),
    (229, 228, 255),
    (228, 227, 255),
    (227, 227, 255),
    (226, 226, 255),
    (225, 225, 255),
    (224, 225, 255),
    (223, 224, 255),
    (221, 223, 255),
    (220, 223, 255),
    (219, 222, 255),
    (218, 221, 255),
    (217, 220, 255),
    (215, 219, 255),
    (214, 218, 255),
    (213, 218, 255),
    (211, 217, 255),
    (210, 216, 255),
    (208, 215, 255),
    (207, 214, 255),
    (206, 213, 255),
    (204, 212, 255),
    (202, 213, 255),
    (200, 213, 255),
    (198, 214, 255),
    (196, 215, 255),
    (194, 215, 255),
    (192, 216, 255),
    (190, 216, 255),
    (188, 217, 255),
    (186, 218, 254),
    (185, 218, 254),
    (183, 219, 254),
    (181, 219, 254),
    (179, 220, 254),
    (177, 220, 254),
    (175, 221, 254),
    (173, 222, 254),
    (172, 222, 254),
    (170, 223, 254),
    (168, 223, 254),
    (166, 224, 254),
    (165, 224, 254),
    (163, 225, 254),
    (161, 226, 254),
    (159, 226, 254),
    (158, 227, 254),
    (156, 227, 254),
    (154, 228, 254),
    (153, 228, 253),
    (151, 229, 253),
    (149, 229, 253),
    (148, 230, 253),
    (146, 230, 253),
    (144, 231, 253),
    (143, 231, 253),
    (141, 232, 253),
    (140, 232, 253),
    (138, 233, 253),
    (137, 233, 253),
    (135, 234, 253),
    (133, 234, 253),
    (132, 235, 253),
    (130, 235, 253),
    (129, 236, 253),
    (127, 236, 253),
    (126, 237, 253),
    (125, 237, 253),
    (123, 238, 253),
    (122, 238, 253),
    (120, 238, 253),
    (119, 239, 252),
    (117, 239, 252),
    (116, 240, 252),
    (115, 240, 252),
    (113, 241, 252),
    (112, 241, 252),
    (111, 242, 252),
    (109, 242, 252),
    (108, 242, 252),
    (107, 243, 252),
    (105, 243, 252),
    (104, 244, 252),
    (103, 244, 252),
    (102, 244, 252),
    (100, 245, 252),
    (99, 245, 252),
    (98, 246, 252),
    (97, 246, 252),
    (96, 246, 252),
    (94, 247, 252),
    (93, 247, 252),
    (92, 247, 252),
    (91, 248, 252),
    (90, 248, 252),
    (89, 248, 252),
    (88, 249, 252),
    (86, 249, 252),
    (85, 249, 252),
    (84, 250, 251),
    (83, 250, 251),
    (82, 250, 251),
    (81, 251, 251),
    (80, 251, 251),
    (79, 251, 251),
    (78, 252, 251),
    (77, 252, 251),
    (76, 252, 251),
    (75, 253, 251),
    (74, 253, 251),
    (73, 253, 251),
    (72, 254, 251),
    (72, 254, 251),
    (71, 254, 251),
    (70, 254, 251),
    (69, 255, 251),
    (68, 255, 251),
];

const RED_PADDLE_CHARGE_COLOR: [(u8, u8, u8); 200] = [
    (255, 239, 187),
    (255, 239, 187),
    (255, 239, 188),
    (255, 239, 188),
    (255, 239, 189),
    (255, 239, 189),
    (255, 240, 190),
    (255, 240, 190),
    (255, 240, 191),
    (255, 240, 191),
    (255, 240, 192),
    (255, 240, 192),
    (255, 240, 192),
    (255, 240, 193),
    (255, 240, 193),
    (255, 240, 194),
    (255, 240, 194),
    (255, 240, 194),
    (255, 240, 195),
    (255, 240, 195),
    (255, 241, 196),
    (255, 241, 196),
    (255, 241, 196),
    (255, 241, 197),
    (255, 241, 197),
    (255, 241, 197),
    (255, 241, 198),
    (255, 241, 198),
    (255, 241, 198),
    (255, 241, 198),
    (255, 241, 199),
    (255, 241, 199),
    (255, 241, 199),
    (255, 241, 200),
    (255, 241, 200),
    (255, 241, 200),
    (255, 241, 200),
    (255, 241, 201),
    (255, 241, 201),
    (255, 241, 201),
    (255, 242, 201),
    (255, 242, 201),
    (255, 242, 202),
    (255, 242, 202),
    (255, 242, 202),
    (255, 242, 202),
    (255, 242, 202),
    (255, 242, 203),
    (255, 242, 203),
    (255, 242, 203),
    (255, 242, 203),
    (255, 242, 203),
    (255, 242, 203),
    (255, 242, 203),
    (255, 242, 204),
    (255, 242, 204),
    (255, 242, 204),
    (255, 242, 204),
    (255, 242, 204),
    (255, 242, 204),
    (255, 242, 203),
    (255, 242, 201),
    (255, 241, 200),
    (255, 241, 198),
    (255, 241, 196),
    (255, 241, 194),
    (255, 240, 192),
    (255, 240, 190),
    (255, 240, 188),
    (255, 239, 185),
    (255, 239, 183),
    (255, 239, 180),
    (255, 238, 177),
    (255, 238, 174),
    (255, 237, 171),
    (255, 237, 168),
    (255, 237, 165),
    (255, 236, 161),
    (255, 236, 157),
    (255, 235, 154),
    (255, 234, 150),
    (255, 234, 146),
    (255, 233, 142),
    (255, 233, 137),
    (255, 232, 133),
    (255, 231, 128),
    (255, 231, 124),
    (255, 230, 119),
    (255, 229, 114),
    (255, 229, 109),
    (255, 228, 104),
    (255, 227, 98),
    (255, 227, 93),
    (255, 226, 87),
    (255, 225, 82),
    (255, 224, 76),
    (255, 223, 70),
    (255, 222, 64),
    (255, 222, 57),
    (255, 221, 51),
    (255, 220, 44),
    (255, 219, 38),
    (255, 218, 31),
    (255, 217, 24),
    (255, 216, 17),
    (255, 215, 17),
    (255, 213, 17),
    (255, 212, 18),
    (255, 211, 18),
    (255, 209, 18),
    (255, 208, 18),
    (255, 207, 19),
    (255, 205, 19),
    (255, 204, 19),
    (255, 203, 19),
    (255, 202, 20),
    (255, 200, 20),
    (255, 199, 20),
    (255, 198, 20),
    (255, 197, 21),
    (255, 195, 21),
    (255, 194, 21),
    (255, 193, 21),
    (255, 192, 21),
    (255, 191, 22),
    (255, 189, 22),
    (255, 188, 22),
    (255, 187, 22),
    (255, 186, 23),
    (255, 185, 23),
    (255, 184, 23),
    (255, 182, 23),
    (255, 181, 23),
    (255, 180, 24),
    (255, 179, 24),
    (255, 178, 24),
    (255, 177, 24),
    (255, 176, 24),
    (255, 175, 25),
    (255, 174, 25),
    (255, 172, 25),
    (255, 171, 25),
    (255, 170, 25),
    (255, 169, 26),
    (255, 168, 26),
    (255, 167, 26),
    (255, 166, 26),
    (255, 165, 26),
    (255, 164, 27),
    (255, 163, 27),
    (255, 162, 27),
    (255, 161, 27),
    (255, 160, 27),
    (255, 159, 27),
    (255, 158, 28),
    (255, 157, 28),
    (255, 157, 28),
    (255, 156, 28),
    (255, 155, 28),
    (255, 154, 28),
    (255, 153, 29),
    (255, 152, 29),
    (255, 151, 29),
    (255, 150, 29),
    (255, 149, 29),
    (255, 148, 29),
    (255, 148, 30),
    (255, 147, 30),
    (255, 146, 30),
    (255, 145, 30),
    (255, 144, 30),
    (255, 143, 30),
    (255, 143, 31),
    (255, 142, 31),
    (255, 141, 31),
    (255, 140, 31),
    (255, 140, 31),
    (255, 139, 31),
    (255, 138, 31),
    (255, 137, 32),
    (255, 137, 32),
    (255, 136, 32),
    (255, 135, 32),
    (255, 134, 32),
    (255, 134, 32),
    (255, 133, 32),
    (255, 132, 32),
    (255, 132, 33),
    (255, 131, 33),
    (255, 130, 33),
    (255, 130, 33),
    (255, 129, 33),
    (255, 128, 33),
    (255, 128, 33),
    (255, 127, 33),
    (255, 126, 34),
    (255, 126, 34),
    (255, 125, 34),
    (255, 125, 34),
    (255, 124, 34),
];

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub prev_y: f64,
    pub vx: f64,
    pub vy: f64,
    pub energy: f64,
    pub color: Side,
    pub age: u32,
}

impl Ball {
    #[must_use]
    pub fn visual_scale(self) -> f64 {
        (self.energy / 8.0 + 30.0).min(100.0) / 100.0
    }

    #[must_use]
    pub fn visual_radius(self) -> f64 {
        BALL_CORE_RADIUS * self.visual_scale()
    }

    #[must_use]
    pub fn is_burning(self) -> bool {
        self.energy > BURN_ENERGY_THRESHOLD
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DyingBall {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub energy: f64,
    pub color: Side,
    pub burning: bool,
    pub age: u32,
}

impl DyingBall {
    #[must_use]
    pub fn from_scored_ball(ball: Ball) -> Self {
        Self {
            id: ball.id,
            x: ball.x,
            y: ball.y,
            vx: ball.vx,
            vy: ball.vy,
            energy: ball.energy,
            color: ball.color,
            burning: ball.is_burning(),
            age: 0,
        }
    }

    #[expect(
        clippy::suboptimal_flops,
        reason = "Preserve the SWF's explicit two-times-bound reflection arithmetic"
    )]
    pub fn tick(&mut self) {
        let slope_x = self.x;
        let next_x = self.x + self.vx;
        let mut next_y = self.y + self.vy;
        self.vx *= 0.5;
        self.vy *= 0.5;
        if self.burning {
            self.vx += slope_x / SLOPE;
        }

        if next_y < Y_MIN_BALL {
            next_y = 2.0 * Y_MIN_BALL - next_y;
            self.vy = -self.vy;
        }
        if next_y > Y_MAX_BALL {
            next_y = 2.0 * Y_MAX_BALL - next_y;
            self.vy = -self.vy;
        }

        self.x = next_x;
        self.y = next_y;
        self.age = self.age.saturating_add(1);
    }

    #[must_use]
    pub const fn is_finished(self) -> bool {
        self.age >= BALL_DIE_TICKS
    }

    #[must_use]
    pub fn visual(self) -> DyingBallVisual {
        ball_die_visual(self.age)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DyingBallVisual {
    pub scale: f64,
    pub alpha: f64,
}

#[must_use]
pub fn ball_die_visual(age: u32) -> DyingBallVisual {
    BALL_DIE_VISUALS[age.min(BALL_DIE_TICKS - 1) as usize]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundEvent {
    Shot { side: Side },
    PaddleHit { side: Side },
    WallBounce,
    Merge,
    Score { side: Side, burning: bool },
    MatchWin { side: Side },
    RoundLostSound { side: Side },
    FinalMatchWinSound { side: Side },
    RoundStart,
    RoundIntroSound,
}

#[derive(Debug, Clone)]
pub struct World {
    pub settings: Settings,
    pub blue: Player,
    pub red: Player,
    pub balls: Vec<Ball>,
    pub dying_balls: Vec<DyingBall>,
    pub next_ball_id: u32,
    pub blue_score: i32,
    pub red_score: i32,
    pub blue_matches: u8,
    pub red_matches: u8,
    pub round_intro_ticks: u32,
    pub round_intro_visual_ticks: u32,
    pub match_win_announce_ticks: u32,
    pub match_win_announce_side: Option<Side>,
    pub final_win_announce_ticks: u32,
    pub final_win_announce_side: Option<Side>,
    pub tick: u64,
    pub winner: Option<Side>,
    pub events: Vec<RoundEvent>,
}

#[derive(Debug, Clone, Copy)]
struct PairScan {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    energy: f64,
}

impl World {
    #[must_use]
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            blue: Player::new(Side::Blue),
            red: Player::new(Side::Red),
            balls: Vec::new(),
            dying_balls: Vec::new(),
            next_ball_id: 10,
            blue_score: 0,
            red_score: 0,
            blue_matches: 0,
            red_matches: 0,
            round_intro_ticks: ROUND_INTRO_TICKS,
            round_intro_visual_ticks: ROUND_INTRO_VISUAL_TICKS,
            match_win_announce_ticks: 0,
            match_win_announce_side: None,
            final_win_announce_ticks: 0,
            final_win_announce_side: None,
            tick: 0,
            winner: None,
            events: Vec::new(),
        }
    }

    pub fn reset_match(&mut self) {
        let settings = self.settings;
        *self = Self::new(settings);
    }

    pub fn tick(&mut self, input: Controls) {
        self.events.clear();
        self.tick_dying_balls();
        if self.winner.is_some() {
            let was_announcing_final_win = self.tick_final_win_announce();
            if was_announcing_final_win {
                self.blue.tick(input.blue_up, input.blue_down);
                self.red.tick(input.red_up, input.red_down);
                self.tick = self.tick.saturating_add(1);
            }
            return;
        }

        let was_announcing_match_win = self.tick_match_win_announce();
        let blue_was_stunned = self.blue.tick_velocity(input.blue_up, input.blue_down);
        let red_was_stunned = self.red.tick_velocity(input.red_up, input.red_down);
        if was_announcing_match_win {
            self.blue.apply_position();
            self.red.apply_position();
            self.tick = self.tick.saturating_add(1);
            return;
        }

        self.tick_round_intro_visual();

        let moving_balls = self.balls.len();
        if input.blue_fire && !blue_was_stunned {
            self.try_fire(Side::Blue);
        }
        if input.red_fire && !red_was_stunned {
            self.try_fire(Side::Red);
        }
        self.tick_round_intro();

        self.blue.apply_position();
        self.red.apply_position();
        self.move_balls(moving_balls);
        self.apply_pair_gravity_and_merges();
        self.tick = self.tick.saturating_add(1);
    }

    fn player_mut(&mut self, side: Side) -> &mut Player {
        match side {
            Side::Blue => &mut self.blue,
            Side::Red => &mut self.red,
        }
    }

    fn player(&self, side: Side) -> &Player {
        match side {
            Side::Blue => &self.blue,
            Side::Red => &self.red,
        }
    }

    fn tick_round_intro(&mut self) {
        if self.round_intro_ticks == 0 {
            return;
        }
        self.round_intro_ticks -= 1;
        if self.round_intro_ticks == 0 {
            self.blue.gun_ready = true;
            self.red.gun_ready = true;
        }
    }

    fn tick_round_intro_visual(&mut self) {
        let old_ticks = self.round_intro_visual_ticks;
        self.round_intro_visual_ticks = self.round_intro_visual_ticks.saturating_sub(1);
        if old_ticks > ROUND_INTRO_SOUND_TICKS
            && self.round_intro_visual_ticks == ROUND_INTRO_SOUND_TICKS
        {
            self.events.push(RoundEvent::RoundIntroSound);
        }
    }

    fn tick_match_win_announce(&mut self) -> bool {
        if self.match_win_announce_ticks == 0 {
            return false;
        }

        self.match_win_announce_ticks -= 1;
        if let Some(side) = self.match_win_announce_side {
            let elapsed = MATCH_WIN_ANNOUNCE_TICKS - self.match_win_announce_ticks;
            let frame = match side {
                Side::Blue => 2 + elapsed,
                Side::Red => 98 + elapsed,
            };
            let sound_frame = match side {
                Side::Blue => 8,
                Side::Red => 105,
            };
            if frame == sound_frame {
                self.events.push(RoundEvent::RoundLostSound { side });
            }
        }
        if self.match_win_announce_ticks == 0 {
            self.start_next_round_intro();
            self.events.push(RoundEvent::RoundStart);
        }
        true
    }

    fn tick_final_win_announce(&mut self) -> bool {
        if self.final_win_announce_ticks == 0 {
            return false;
        }

        self.final_win_announce_ticks -= 1;
        if self.final_win_announce_ticks == 0 {
            self.final_win_announce_side = None;
        }
        true
    }

    fn tick_dying_balls(&mut self) {
        for ball in &mut self.dying_balls {
            ball.tick();
        }
        self.dying_balls.retain(|ball| !ball.is_finished());
    }

    #[expect(
        clippy::imprecise_flops,
        reason = "Preserve the SWF's Math.sqrt(vx*vx + vy*vy) normalization"
    )]
    pub fn try_fire(&mut self, side: Side) -> bool {
        let player = self.player(side);
        if player.stun_ticks > 0 || !player.gun_ready || player.energy_frame <= FIRE_THRESHOLD {
            return false;
        }

        let energy = f64::from(player.energy_frame);
        let mut vx = BALL_BASE_SPEED;
        let x = if side == Side::Blue {
            vx = -vx;
            X_MAX_BALL - 1.0
        } else {
            X_MIN_BALL + 1.0
        };
        let y = player.y;
        let vy = player.v * vx.abs() / 4.0;
        let speed = energy / 20.0;
        let length = (vx * vx + vy * vy).sqrt().max(f64::EPSILON);
        let ball = Ball {
            id: self.next_ball_id,
            x,
            y,
            prev_y: y,
            vx: vx / length * speed,
            vy: vy / length * speed,
            energy,
            color: side,
            age: 0,
        };
        self.next_ball_id += 1;
        self.balls.push(ball);
        self.player_mut(side).energy_frame = 1;
        self.events.push(RoundEvent::Shot { side });
        true
    }

    #[expect(
        clippy::suboptimal_flops,
        reason = "Preserve the SWF's explicit two-times-bound reflection arithmetic"
    )]
    fn move_balls(&mut self, moving_balls: usize) {
        let mut survivors = Vec::with_capacity(self.balls.len());
        let moving = core::mem::take(&mut self.balls);
        let mut round_cleared = false;
        for (index, mut ball) in moving.into_iter().enumerate() {
            if round_cleared {
                break;
            }
            if index >= moving_balls {
                survivors.push(ball);
                continue;
            }
            let slope_x = ball.x;
            ball.prev_y = ball.y;
            ball.x += ball.vx;
            ball.y += ball.vy;
            if ball.is_burning() {
                ball.vx += slope_x / SLOPE;
            }
            ball.age = ball.age.saturating_add(1);

            if ball.y < Y_MIN_BALL {
                ball.y = 2.0 * Y_MIN_BALL - ball.y;
                ball.vy = -ball.vy;
                self.events.push(RoundEvent::WallBounce);
            }
            if ball.y > Y_MAX_BALL {
                ball.y = 2.0 * Y_MAX_BALL - ball.y;
                ball.vy = -ball.vy;
                self.events.push(RoundEvent::WallBounce);
            }

            if ball.x < X_MIN_BALL {
                if !ball.is_burning() && self.paddle_intersects(Side::Red, ball.prev_y, ball.y) {
                    ball.x = 2.0 * X_MIN_BALL - ball.x;
                    ball.vx = reflect_velocity(ball.vx, self.settings.speed);
                    ball.vy += (self.red.v - ball.vy) / 2.0;
                    ball.color = Side::Red;
                    self.events.push(RoundEvent::PaddleHit { side: Side::Red });
                    survivors.push(ball);
                } else {
                    self.score_for(Side::Blue, ball);
                    round_cleared = self.round_was_cleared();
                }
            } else if ball.x > X_MAX_BALL {
                if !ball.is_burning() && self.paddle_intersects(Side::Blue, ball.prev_y, ball.y) {
                    ball.x = 2.0 * X_MAX_BALL - ball.x;
                    ball.vx = reflect_velocity(ball.vx, self.settings.speed);
                    ball.vy += (self.blue.v - ball.vy) / 2.0;
                    ball.color = Side::Blue;
                    self.events.push(RoundEvent::PaddleHit { side: Side::Blue });
                    survivors.push(ball);
                } else {
                    self.score_for(Side::Red, ball);
                    round_cleared = self.round_was_cleared();
                }
            } else {
                survivors.push(ball);
            }
        }
        self.balls = if round_cleared { Vec::new() } else { survivors };
    }

    fn paddle_intersects(&self, side: Side, prev_y: f64, y: f64) -> bool {
        let paddle = self.player(side);
        (prev_y - paddle.y).abs() <= PADDLE_SIZE || (y - paddle.y).abs() <= PADDLE_SIZE
    }

    #[expect(
        clippy::suboptimal_flops,
        reason = "Preserve the SWF's explicit vx*vx + vy*vy speed-bonus expression"
    )]
    fn score_for(&mut self, side: Side, ball: Ball) {
        let burning = ball.is_burning();
        self.dying_balls.push(DyingBall::from_scored_ball(ball));
        let speed_bonus = if self.settings.speed == SpeedMode::Fast {
            ((ball.vx * ball.vx + ball.vy * ball.vy) * VELOCITY_MULTI) as i32
        } else {
            0
        };
        let energy_score = (ball.energy / self.settings.speed.score_energy_multi()) as i32;
        let fire_bonus = if burning {
            (f64::from(FIRE_BONUS) * FIRE_BONUS_MULTI) as i32
        } else {
            0
        };
        let delta = energy_score + speed_bonus + fire_bonus;

        match side {
            Side::Blue => {
                self.blue_score += delta;
                if burning {
                    self.red.v = 0.0;
                    self.red.stun_ticks = PADDLE_STUN_TICKS;
                }
                if self.blue_score >= MATCH_SCORE {
                    self.events.push(RoundEvent::Score { side, burning });
                    self.record_match_win(side);
                    return;
                }
            },
            Side::Red => {
                self.red_score += delta;
                if burning {
                    self.blue.v = 0.0;
                    self.blue.stun_ticks = PADDLE_STUN_TICKS;
                }
                if self.red_score >= MATCH_SCORE {
                    self.events.push(RoundEvent::Score { side, burning });
                    self.record_match_win(side);
                    return;
                }
            },
        }
        self.events.push(RoundEvent::Score { side, burning });
    }

    fn record_match_win(&mut self, side: Side) {
        match side {
            Side::Blue => self.blue_matches += 1,
            Side::Red => self.red_matches += 1,
        }
        self.events.push(RoundEvent::MatchWin { side });
        self.clear_playfield();
        if self.blue_matches >= self.settings.wins_needed() {
            self.winner = Some(Side::Blue);
            self.start_final_win_announce(Side::Blue);
        } else if self.red_matches >= self.settings.wins_needed() {
            self.winner = Some(Side::Red);
            self.start_final_win_announce(Side::Red);
        } else {
            self.match_win_announce_ticks = MATCH_WIN_ANNOUNCE_TICKS;
            self.match_win_announce_side = Some(side);
            self.round_intro_ticks = 0;
            self.round_intro_visual_ticks = 0;
        }
    }

    fn start_final_win_announce(&mut self, side: Side) {
        self.match_win_announce_ticks = 0;
        self.match_win_announce_side = None;
        self.final_win_announce_ticks = final_win_announce_ticks(side);
        self.final_win_announce_side = Some(side);
        self.round_intro_ticks = 0;
        self.round_intro_visual_ticks = 0;
        self.events.push(RoundEvent::FinalMatchWinSound { side });
    }

    fn clear_playfield(&mut self) {
        self.balls.clear();
        self.next_ball_id = 10;
        self.blue.gun_ready = false;
        self.red.gun_ready = false;
    }

    fn start_next_round_intro(&mut self) {
        self.blue_score = 0;
        self.red_score = 0;
        self.blue.gun_ready = false;
        self.red.gun_ready = false;
        self.match_win_announce_side = None;
        self.round_intro_ticks = ROUND_INTRO_TICKS;
        self.round_intro_visual_ticks = ROUND_INTRO_VISUAL_TICKS;
    }

    fn round_was_cleared(&self) -> bool {
        self.events
            .iter()
            .any(|event| matches!(event, RoundEvent::MatchWin { .. }))
    }

    #[expect(
        clippy::imprecise_flops,
        reason = "Preserve the SWF's Math.sqrt(dx*dx + dy*dy) distance expression"
    )]
    fn apply_pair_gravity_and_merges(&mut self) {
        // Sprite 38 frame 2 keeps x/y/vx/vy/e locals for the current i ball,
        // records merged partner indices in delBall, then splices them after
        // the full scan. Clustered balls depend on that delayed deletion.
        let len = self.balls.len();
        let mut del_ball = Vec::new();
        let mut i = 0;
        while i < len {
            let mut scan = PairScan {
                x: self.balls[i].x,
                y: self.balls[i].y,
                vx: self.balls[i].vx,
                vy: self.balls[i].vy,
                energy: self.balls[i].energy,
            };
            let mut j = i + 1;
            while j < len {
                let dx = self.balls[j].x - scan.x;
                let dy = self.balls[j].y - scan.y;
                let distance = (dx * dx + dy * dy).sqrt().max(0.000_001);
                if distance < MERGE_DISTANCE {
                    let i_energy = self.balls[i].energy;
                    let j_energy = self.balls[j].energy;
                    if i_energy < j_energy {
                        self.balls[i].color = self.balls[j].color;
                    }
                    let total = (i_energy + j_energy).min(600.0);
                    let j_part = if total > 0.0 { j_energy / total } else { 0.0 };
                    let i_part = 1.0 - j_part;
                    scan.vx = i_part * scan.vx + j_part * self.balls[j].vx;
                    scan.vy = i_part * scan.vy + j_part * self.balls[j].vy;
                    self.balls[i].energy = total;
                    self.balls[j].energy = 0.0;
                    del_ball.push(j);
                    self.events.push(RoundEvent::Merge);
                } else {
                    self.apply_pair_gravity_to_j(i, j, dx, dy, distance, &mut scan);
                }
                j += 1;
            }

            self.balls[i].vx = scan.vx;
            self.balls[i].vy = scan.vy;
            i += 1;
        }

        for index in del_ball {
            if index < self.balls.len() {
                self.balls.remove(index);
            }
        }
    }

    fn apply_pair_gravity_to_j(
        &mut self,
        i: usize,
        j: usize,
        dx: f64,
        dy: f64,
        distance: f64,
        scan: &mut PairScan,
    ) {
        let gvx = dx / distance;
        let gvy = dy / distance;
        let gd = distance / self.settings.gravity.factor();
        let reverse = self
            .settings
            .polarisation
            .reverses_force(self.balls[i].color == self.balls[j].color);

        let j_energy = self.balls[j].energy;
        scan.vx += gravity_delta(gvx, gd, j_energy, reverse);
        scan.vy += gravity_delta(gvy, gd, j_energy, reverse);
        self.balls[j].vx -= gravity_delta(gvx, gd, scan.energy, reverse);
        self.balls[j].vy -= gravity_delta(gvy, gd, scan.energy, reverse);
    }
}

fn gravity_delta(unit: f64, distance_over_factor: f64, energy: f64, reverse: bool) -> f64 {
    if energy <= 0.0 {
        return 0.0;
    }

    let delta = unit / (distance_over_factor / (energy / ENERGY_FACTOR));
    if reverse { -delta } else { delta }
}

#[must_use]
pub fn reflect_velocity(value: f64, speed: SpeedMode) -> f64 {
    if value.abs() > BALL_MAX_SPEED {
        return if value < 0.0 {
            BALL_MAX_SPEED + 1.0
        } else {
            0.0 - (BALL_MAX_SPEED + 1.0)
        };
    }
    value * (0.0 - speed.reflect_mod())
}

#[must_use]
pub const fn world_to_stage(x: f64, y: f64) -> (f64, f64) {
    (WORLD_X + x, WORLD_Y + y)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 0.001,
            "expected {actual} to be within 0.001 of {expected}"
        );
    }

    fn scoring_ball(side: Side, energy: f64) -> Ball {
        Ball {
            id: 99,
            x: 0.0,
            y: 0.0,
            prev_y: 0.0,
            vx: 0.0,
            vy: 0.0,
            energy,
            color: side,
            age: 0,
        }
    }

    #[test]
    fn player_initial_y_matches_swf_sprite_140_paddle_placements() {
        assert_close(WORLD_Y, 214.45);
        assert_close(RIGHT_PADDLE_X, 241.25);
        assert_close(LEFT_PADDLE_X, -238.45);
        assert_close(BLUE_PADDLE_INITIAL_Y, -0.25);
        assert_close(RED_PADDLE_INITIAL_Y, -0.3);
        assert_close(initial_paddle_y(Side::Blue), -0.25);
        assert_close(initial_paddle_y(Side::Red), -0.3);

        let blue = Player::new(Side::Blue);
        let red = Player::new(Side::Red);
        assert_close(blue.y, -0.25);
        assert_close(red.y, -0.3);
        assert_close(world_to_stage(RIGHT_PADDLE_X, blue.y).1, 214.2);
        assert_close(world_to_stage(LEFT_PADDLE_X, red.y).1, 214.15);
    }

    #[test]
    fn shot_spawn_y_uses_paddle_clip_y() {
        let mut world = World::new(Settings::default());
        world.blue.gun_ready = true;
        world.red.gun_ready = true;

        assert!(world.try_fire(Side::Blue));
        assert_close(world.balls[0].y, BLUE_PADDLE_INITIAL_Y);

        assert!(world.try_fire(Side::Red));
        assert_close(world.balls[1].y, RED_PADDLE_INITIAL_Y);
    }

    #[test]
    fn tick_fire_order_matches_swf_sprite_132_pre_move_spawn() {
        let mut world = World::new(Settings::default());
        world.round_intro_ticks = 0;
        world.round_intro_visual_ticks = 0;
        world.blue.gun_ready = true;

        world.tick(Controls {
            blue_down: true,
            blue_fire: true,
            ..Controls::default()
        });

        assert_eq!(world.events, vec![RoundEvent::Shot { side: Side::Blue }]);
        assert_eq!(world.balls.len(), 1);
        assert_close(world.blue.v, 2.0);
        assert_close(world.blue.y, BLUE_PADDLE_INITIAL_Y + 2.0);

        let shot = world.balls[0];
        assert_close(shot.x, X_MAX_BALL - 1.0);
        assert_close(shot.y, BLUE_PADDLE_INITIAL_Y);
        assert_close(shot.prev_y, BLUE_PADDLE_INITIAL_Y);
        assert_close(shot.energy, f64::from(ENERGY_MAX_FRAME));
        assert_close(shot.vx, -8.944_271_909_999_16);
        assert_close(shot.vy, 4.472_135_954_999_58);

        world.tick(Controls::default());
        let moved = world.balls[0];
        assert_close(moved.x, shot.x + shot.vx);
        assert_close(moved.y, shot.y + shot.vy);
        assert_close(moved.prev_y, shot.y);
    }

    #[test]
    fn score_meter_frame_clamps_separately_from_round_win_threshold() {
        assert_eq!(score_meter_frame(-10), 1);
        assert_eq!(score_meter_frame(0), 1);
        assert_eq!(score_meter_frame(9), 1);
        assert_eq!(score_meter_frame(10), 2);
        assert_eq!(score_meter_frame(690), 70);
        assert_eq!(score_meter_frame(SCORE_METER_MAX), SCORE_METER_MAX_FRAME);
        assert_eq!(score_meter_frame(MATCH_SCORE), SCORE_METER_MAX_FRAME);
    }

    #[test]
    fn round_win_starts_at_710_not_score_meter_max_700() {
        let mut world = World::new(Settings::default());
        world.blue_score = SCORE_METER_MAX - 10;

        world.score_for(Side::Blue, scoring_ball(Side::Blue, 100.0));

        assert_eq!(world.blue_score, SCORE_METER_MAX);
        assert_eq!(score_meter_frame(world.blue_score), SCORE_METER_MAX_FRAME);
        assert_eq!(world.blue_matches, 0);
        assert_eq!(world.match_win_announce_ticks, 0);
        assert_eq!(
            world.events,
            vec![RoundEvent::Score {
                side: Side::Blue,
                burning: false,
            }]
        );

        world.events.clear();
        world.score_for(Side::Blue, scoring_ball(Side::Blue, 100.0));

        assert_eq!(world.blue_score, MATCH_SCORE);
        assert_eq!(score_meter_frame(world.blue_score), SCORE_METER_MAX_FRAME);
        assert_eq!(world.blue_matches, 1);
        assert_eq!(world.match_win_announce_ticks, MATCH_WIN_ANNOUNCE_TICKS);
        assert_eq!(world.match_win_announce_side, Some(Side::Blue));
        assert_eq!(
            world.events,
            vec![
                RoundEvent::Score {
                    side: Side::Blue,
                    burning: false,
                },
                RoundEvent::MatchWin { side: Side::Blue },
            ]
        );
    }

    #[test]
    fn round_intro_timeline_matches_swf_newgame_label() {
        let mut world = World::new(Settings::default());

        assert_eq!(world.round_intro_ticks, ROUND_INTRO_TICKS);
        assert_eq!(world.round_intro_visual_ticks, ROUND_INTRO_VISUAL_TICKS);
        assert!(!world.blue.gun_ready);
        assert!(!world.red.gun_ready);

        for _ in 0..(ROUND_INTRO_VISUAL_TICKS - ROUND_INTRO_SOUND_TICKS - 1) {
            world.tick(Controls::default());
            assert!(!world.events.contains(&RoundEvent::RoundIntroSound));
        }

        world.tick(Controls::default());
        assert_eq!(world.events, vec![RoundEvent::RoundIntroSound]);
        assert_eq!(world.round_intro_visual_ticks, ROUND_INTRO_SOUND_TICKS);
        assert!(!world.blue.gun_ready);
        assert!(!world.red.gun_ready);

        while world.round_intro_ticks > 1 {
            world.tick(Controls::default());
            assert!(!world.blue.gun_ready);
            assert!(!world.red.gun_ready);
        }

        world.tick(Controls::default());
        assert_eq!(world.round_intro_ticks, 0);
        assert_eq!(
            world.round_intro_visual_ticks,
            ROUND_INTRO_VISUAL_TICKS - ROUND_INTRO_TICKS
        );
        assert!(world.blue.gun_ready);
        assert!(world.red.gun_ready);
    }

    #[test]
    fn match_win_announcement_delays_score_reset_and_next_round() {
        let mut world = World::new(Settings::default());

        world.score_for(Side::Blue, scoring_ball(Side::Blue, 7_100.0));

        assert_eq!(world.blue_score, MATCH_SCORE);
        assert_eq!(world.blue_matches, 1);
        assert_eq!(world.winner, None);
        assert_eq!(world.match_win_announce_ticks, MATCH_WIN_ANNOUNCE_TICKS);
        assert_eq!(world.match_win_announce_side, Some(Side::Blue));
        assert_eq!(world.round_intro_ticks, 0);
        assert_eq!(world.round_intro_visual_ticks, 0);
        assert!(
            world
                .events
                .contains(&RoundEvent::MatchWin { side: Side::Blue })
        );

        for tick in 1..MATCH_WIN_ANNOUNCE_TICKS {
            world.tick(Controls::default());
            if tick == 6 {
                assert_eq!(
                    world.events,
                    vec![RoundEvent::RoundLostSound { side: Side::Blue }]
                );
            } else {
                assert!(
                    !world
                        .events
                        .contains(&RoundEvent::RoundLostSound { side: Side::Blue })
                );
            }
            assert_eq!(world.blue_score, MATCH_SCORE);
            assert_eq!(world.round_intro_ticks, 0);
        }

        world.tick(Controls::default());
        assert_eq!(world.events, vec![RoundEvent::RoundStart]);
        assert_eq!(world.blue_score, 0);
        assert_eq!(world.red_score, 0);
        assert_eq!(world.match_win_announce_ticks, 0);
        assert_eq!(world.match_win_announce_side, None);
        assert_eq!(world.round_intro_ticks, ROUND_INTRO_TICKS);
        assert_eq!(world.round_intro_visual_ticks, ROUND_INTRO_VISUAL_TICKS);
        assert!(!world.blue.gun_ready);
        assert!(!world.red.gun_ready);
    }

    #[test]
    fn final_match_win_uses_side_specific_sprite_162_duration() {
        let mut blue_world = World::new(Settings::default());
        blue_world.blue_matches = blue_world.settings.wins_needed() - 1;

        blue_world.score_for(Side::Blue, scoring_ball(Side::Blue, 7_100.0));

        assert_eq!(blue_world.winner, Some(Side::Blue));
        assert_eq!(blue_world.blue_matches, blue_world.settings.wins_needed());
        assert_eq!(
            blue_world.final_win_announce_ticks,
            BLUE_FINAL_WIN_ANNOUNCE_TICKS
        );
        assert_eq!(blue_world.final_win_announce_side, Some(Side::Blue));
        assert_eq!(
            blue_world.events,
            vec![
                RoundEvent::Score {
                    side: Side::Blue,
                    burning: true,
                },
                RoundEvent::MatchWin { side: Side::Blue },
                RoundEvent::FinalMatchWinSound { side: Side::Blue },
            ]
        );

        for _ in 0..BLUE_FINAL_WIN_ANNOUNCE_TICKS {
            blue_world.tick(Controls::default());
        }
        assert_eq!(blue_world.final_win_announce_ticks, 0);
        assert_eq!(blue_world.final_win_announce_side, None);

        let mut red_world = World::new(Settings::default());
        red_world.red_matches = red_world.settings.wins_needed() - 1;

        red_world.score_for(Side::Red, scoring_ball(Side::Red, 7_100.0));

        assert_eq!(red_world.winner, Some(Side::Red));
        assert_eq!(red_world.red_matches, red_world.settings.wins_needed());
        assert_eq!(
            red_world.final_win_announce_ticks,
            RED_FINAL_WIN_ANNOUNCE_TICKS
        );
        assert_eq!(red_world.final_win_announce_side, Some(Side::Red));

        for _ in 0..RED_FINAL_WIN_ANNOUNCE_TICKS {
            red_world.tick(Controls::default());
        }
        assert_eq!(red_world.final_win_announce_ticks, 0);
        assert_eq!(red_world.final_win_announce_side, None);
    }

    #[test]
    fn burning_scores_stun_defender_but_keep_swf_fire_bonus_typo() {
        let mut world = World::new(Settings::default());
        world.red.v = 8.0;

        world.score_for(Side::Blue, scoring_ball(Side::Blue, 600.0));

        assert_eq!(world.blue_score, 60);
        assert_eq!(world.red_score, 0);
        assert_close(world.red.v, 0.0);
        assert_eq!(world.red.stun_ticks, PADDLE_STUN_TICKS);
        assert_eq!(
            world.events,
            vec![RoundEvent::Score {
                side: Side::Blue,
                burning: true,
            }]
        );
    }

    #[test]
    fn scored_ball_death_clip_keeps_attach_depth() {
        let ball = Ball {
            id: 37,
            x: 1.0,
            y: 2.0,
            prev_y: 2.0,
            vx: 3.0,
            vy: 4.0,
            energy: 100.0,
            color: Side::Blue,
            age: 0,
        };

        let dying = DyingBall::from_scored_ball(ball);

        assert_eq!(dying.id, 37);
        assert_close(dying.x, 1.0);
        assert_close(dying.y, 2.0);
        assert_close(dying.vx, 3.0);
        assert_close(dying.vy, 4.0);
    }
}
