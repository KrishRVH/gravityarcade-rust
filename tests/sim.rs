#![allow(
    clippy::expect_used,
    clippy::float_cmp,
    reason = "These parity tests assert exact recovered SWF constants and scenario invariants"
)]

use gravityarcade::sim::{
    BALL_CORE_RADIUS, BALL_DIE_TICKS, BLUE_FINAL_WIN_ANNOUNCE_TICKS, BURN_ENERGY_THRESHOLD, Ball,
    Controls, ENERGY_FACTOR, ENERGY_MAX_FRAME, GravityStrength, MATCH_WIN_ANNOUNCE_TICKS,
    MERGE_DISTANCE, PADDLE_SIZE, PADDLE_STUN_TICKS, Polarisation, RED_FINAL_WIN_ANNOUNCE_TICKS,
    ROUND_INTRO_SOUND_TICKS, ROUND_INTRO_TICKS, ROUND_INTRO_VISUAL_TICKS, RoundEvent, SLOPE,
    Settings, Side, SpeedMode, World, X_MAX_BALL, X_MIN_BALL, Y_MAX, Y_MIN, ball_die_visual,
    paddle_charge_visual, paddle_stun_color, reflect_velocity, score_meter_frame,
};

#[test]
fn reflect_velocity_matches_swf_clip_and_reverse() {
    assert_eq!(reflect_velocity(12.0, SpeedMode::Normal), -12.0);
    assert_eq!(reflect_velocity(-12.0, SpeedMode::Normal), 12.0);
    assert!((reflect_velocity(12.0, SpeedMode::Fast) + 13.8).abs() < 0.000_001);
    assert_eq!(reflect_velocity(120.0, SpeedMode::Normal), -71.0);
    assert_eq!(reflect_velocity(120.0, SpeedMode::Fast), -71.0);
    assert_eq!(reflect_velocity(-120.0, SpeedMode::Fast), 71.0);
}

#[test]
fn settings_defaults_and_cycles_match_swf_menu_frames() {
    let mut settings = Settings::default();
    assert_eq!(settings.matches, 3);
    assert_eq!(settings.polarisation, Polarisation::Neutral);
    assert_eq!(settings.gravity, GravityStrength::G2);
    assert_eq!(settings.speed, SpeedMode::Normal);

    settings.cycle_matches();
    assert_eq!(settings.matches, 5);
    settings.cycle_matches();
    assert_eq!(settings.matches, 7);
    settings.cycle_matches();
    assert_eq!(settings.matches, 1);
    settings.cycle_matches();
    assert_eq!(settings.matches, 3);

    assert_eq!(
        [
            Polarisation::Neutral.swf_value(),
            Polarisation::Neutral.next().swf_value(),
            Polarisation::Neutral.next().next().swf_value(),
            Polarisation::Neutral.next().next().next().swf_value(),
        ],
        [0, -1, 1, 2],
    );
    assert_eq!(GravityStrength::G2.next(), GravityStrength::G3);
    assert_eq!(SpeedMode::Normal.next(), SpeedMode::Fast);
}

#[test]
fn fired_blue_ball_spawns_at_right_and_moves_left() {
    let mut world = World::new(Settings::default());
    make_ready(&mut world);
    world.blue.y = 22.0;
    world.blue.v = 4.0;
    assert!(world.try_fire(Side::Blue));
    let ball = world.balls[0];
    assert_eq!(ball.x, X_MAX_BALL - 1.0);
    assert_eq!(ball.y, 22.0);
    assert_eq!(ball.energy, f64::from(ENERGY_MAX_FRAME));
    assert!(ball.vx < 0.0);
    assert!(ball.vy > 0.0);
    assert!((ball.vx.hypot(ball.vy) - 10.0).abs() < 0.000_001);
    assert_eq!(world.blue.energy_frame, 1);
}

#[test]
fn fired_red_ball_spawns_at_left_and_moves_right() {
    let mut world = World::new(Settings::default());
    make_ready(&mut world);
    world.red.y = -12.0;
    world.red.v = -2.0;
    assert!(world.try_fire(Side::Red));
    let ball = world.balls[0];
    assert_eq!(ball.x, X_MIN_BALL + 1.0);
    assert_eq!(ball.y, -12.0);
    assert!(ball.vx > 0.0);
    assert!(ball.vy < 0.0);
}

#[test]
fn tick_shot_uses_pre_move_paddle_y_and_waits_for_ball_move_frame() {
    let mut world = World::new(Settings::default());
    make_ready(&mut world);
    world.blue.y = 20.0;
    world.blue.v = 0.0;

    world.tick(Controls {
        blue_down: true,
        blue_fire: true,
        ..Controls::default()
    });

    assert_eq!(world.blue.y, 22.0);
    assert_eq!(world.blue.v, 2.0);
    assert_eq!(world.balls.len(), 1);
    let ball = world.balls[0];
    assert_eq!(ball.x, X_MAX_BALL - 1.0);
    assert_eq!(ball.y, 20.0);
    assert_eq!(ball.prev_y, 20.0);
    assert_eq!(ball.age, 0);
    assert!(ball.vx < 0.0);
    assert!(ball.vy > 0.0);
    assert!(
        world
            .events
            .contains(&RoundEvent::Shot { side: Side::Blue })
    );

    world.tick(Controls::default());

    assert!(world.balls[0].x < X_MAX_BALL - 1.0);
    assert!(world.balls[0].y > 20.0);
    assert_eq!(world.balls[0].age, 1);
}

#[test]
fn shot_requires_charged_energy_frame() {
    let mut world = World::new(Settings::default());
    make_ready(&mut world);
    world.blue.energy_frame = 20;
    assert!(!world.try_fire(Side::Blue));
    world.blue.energy_frame = 21;
    assert!(world.try_fire(Side::Blue));
}

#[test]
fn stunned_player_cannot_fire_until_paralised_clip_returns_to_move() {
    let mut world = World::new(Settings::default());
    make_ready(&mut world);
    world.red.stun_ticks = PADDLE_STUN_TICKS;

    assert!(!world.try_fire(Side::Red));
    assert!(world.balls.is_empty());

    for _ in 0..PADDLE_STUN_TICKS {
        world.tick(Controls {
            red_fire: true,
            ..Controls::default()
        });
    }

    assert!(world.balls.is_empty());
    assert_eq!(world.red.stun_ticks, 0);
    assert!(world.try_fire(Side::Red));
}

#[test]
fn stunned_paddle_charge_clip_keeps_advancing_without_firing() {
    let mut world = World::new(Settings::default());
    make_ready(&mut world);
    world.red.y = 14.0;
    world.red.energy_frame = 1;
    world.red.stun_ticks = PADDLE_STUN_TICKS;

    world.tick(Controls {
        red_down: true,
        red_fire: true,
        ..Controls::default()
    });

    assert_eq!(world.red.energy_frame, 2);
    assert_eq!(world.red.stun_ticks, PADDLE_STUN_TICKS - 1);
    assert_eq!(world.red.y, 14.0);
    assert_eq!(world.red.v, 0.0);
    assert!(world.balls.is_empty());
}

#[test]
fn swept_paddle_collision_catches_crossing_ball() {
    let mut world = World::new(Settings::default());
    world.blue.y = 0.0;
    world.balls.push(Ball {
        id: 1,
        x: X_MAX_BALL - 0.5,
        y: 0.0,
        prev_y: 0.0,
        vx: 3.0,
        vy: PADDLE_SIZE + 10.0,
        energy: 40.0,
        color: Side::Red,
        age: 10,
    });
    world.tick(Controls::default());
    assert_eq!(world.balls.len(), 1);
    assert!(world.balls[0].vx < 0.0);
}

#[test]
fn paddle_collision_includes_exact_swf_paddle_size_boundary() {
    let mut edge_hit = World::new(Settings::default());
    edge_hit.blue.y = 0.0;
    edge_hit.balls.push(Ball {
        id: 1,
        x: X_MAX_BALL + 0.1,
        y: PADDLE_SIZE,
        prev_y: PADDLE_SIZE,
        vx: 0.0,
        vy: 0.0,
        energy: 40.0,
        color: Side::Red,
        age: 10,
    });

    edge_hit.tick(Controls::default());

    assert_eq!(edge_hit.balls.len(), 1);
    assert_eq!(edge_hit.red_score, 0);
    assert!(
        edge_hit
            .events
            .contains(&RoundEvent::PaddleHit { side: Side::Blue })
    );

    let mut just_outside = World::new(Settings::default());
    just_outside.blue.y = 0.0;
    just_outside.balls.push(Ball {
        id: 1,
        x: X_MAX_BALL + 0.1,
        y: PADDLE_SIZE + 0.001,
        prev_y: PADDLE_SIZE + 0.001,
        vx: 0.0,
        vy: 0.0,
        energy: 40.0,
        color: Side::Red,
        age: 10,
    });

    just_outside.tick(Controls::default());

    assert!(just_outside.balls.is_empty());
    assert_eq!(just_outside.red_score, 4);
    assert!(just_outside.events.contains(&RoundEvent::Score {
        side: Side::Red,
        burning: false,
    }));
}

#[test]
fn paddle_hits_recolor_ball_and_mix_vertical_velocity_for_both_sides() {
    let mut blue_hit = World::new(Settings::default());
    blue_hit.blue.y = 0.0;
    blue_hit.blue.v = 10.0;
    blue_hit.balls.push(Ball {
        id: 1,
        x: X_MAX_BALL + 0.1,
        y: 8.0,
        prev_y: 8.0,
        vx: 2.0,
        vy: -4.0,
        energy: 40.0,
        color: Side::Red,
        age: 10,
    });

    blue_hit.tick(Controls::default());

    assert_eq!(blue_hit.balls.len(), 1);
    assert_eq!(blue_hit.balls[0].color, Side::Blue);
    assert!((blue_hit.balls[0].x - 232.9).abs() < 0.000_001);
    assert_eq!(blue_hit.balls[0].vx, -2.0);
    assert_eq!(blue_hit.balls[0].vy, 2.0);
    assert!(
        blue_hit
            .events
            .contains(&RoundEvent::PaddleHit { side: Side::Blue })
    );

    let mut red_hit = World::new(Settings::default());
    red_hit.red.y = 0.0;
    red_hit.red.v = -10.0;
    red_hit.balls.push(Ball {
        id: 2,
        x: X_MIN_BALL - 0.1,
        y: -8.0,
        prev_y: -8.0,
        vx: -2.0,
        vy: 4.0,
        energy: 40.0,
        color: Side::Blue,
        age: 10,
    });

    red_hit.tick(Controls::default());

    assert_eq!(red_hit.balls.len(), 1);
    assert_eq!(red_hit.balls[0].color, Side::Red);
    assert!((red_hit.balls[0].x + 232.9).abs() < 0.000_001);
    assert_eq!(red_hit.balls[0].vx, 2.0);
    assert_eq!(red_hit.balls[0].vy, -2.0);
    assert!(
        red_hit
            .events
            .contains(&RoundEvent::PaddleHit { side: Side::Red })
    );
}

#[test]
fn burning_ball_pierces_intersecting_paddle_and_scores() {
    let mut world = World::new(Settings::default());
    world.red.y = 0.0;
    world.balls.push(Ball {
        id: 1,
        x: X_MIN_BALL - 1.0,
        y: 0.0,
        prev_y: 0.0,
        vx: -1.0,
        vy: 0.0,
        energy: 600.0,
        color: Side::Blue,
        age: 10,
    });

    world.tick(Controls::default());

    assert!(world.balls.is_empty());
    assert_eq!(world.blue_score, 60);
    assert_eq!(world.red.stun_ticks, PADDLE_STUN_TICKS);
    assert!(
        !world
            .events
            .contains(&RoundEvent::PaddleHit { side: Side::Red })
    );
    assert!(world.events.contains(&RoundEvent::Score {
        side: Side::Blue,
        burning: true,
    }));
}

#[test]
fn energy_recharges_one_frame_per_tick() {
    let mut world = World::new(Settings::default());
    make_ready(&mut world);
    assert!(world.try_fire(Side::Blue));
    assert_eq!(world.blue.energy_frame, 1);
    world.tick(Controls::default());
    assert_eq!(world.blue.energy_frame, 2);
    world.blue.energy_frame = ENERGY_MAX_FRAME - 1;
    world.tick(Controls::default());
    assert_eq!(world.blue.energy_frame, ENERGY_MAX_FRAME);
    world.blue.energy_frame = ENERGY_MAX_FRAME;
    world.tick(Controls::default());
    assert_eq!(world.blue.energy_frame, ENERGY_MAX_FRAME);
}

#[test]
fn paddle_bounds_zero_velocity_like_swf_ty_clamp() {
    let mut world = World::new(Settings::default());
    world.blue.y = Y_MAX - 1.0;
    world.blue.v = 5.0;
    world.red.y = Y_MIN + 1.0;
    world.red.v = -5.0;

    world.tick(Controls {
        blue_down: true,
        red_up: true,
        ..Controls::default()
    });

    assert_eq!(world.blue.y, Y_MAX);
    assert_eq!(world.blue.v, 0.0);
    assert_eq!(world.red.y, Y_MIN);
    assert_eq!(world.red.v, 0.0);
}

#[test]
fn only_burning_balls_receive_swf_playfield_slope() {
    let mut normal = World::new(Settings::default());
    normal.balls.push(Ball {
        id: 1,
        x: 100.0,
        y: 0.0,
        prev_y: 0.0,
        vx: 5.0,
        vy: 0.0,
        energy: 200.0,
        color: Side::Blue,
        age: 0,
    });

    normal.tick(Controls::default());

    assert_eq!(normal.balls[0].x, 105.0);
    assert_eq!(normal.balls[0].vx, 5.0);

    let mut burning = World::new(Settings::default());
    burning.balls.push(Ball {
        energy: BURN_ENERGY_THRESHOLD + 1.0,
        ..normal.balls[0]
    });

    burning.tick(Controls::default());

    assert_eq!(burning.balls[0].x, 110.0);
    assert!((burning.balls[0].vx - (5.0 + 105.0 / SLOPE)).abs() < 0.000_001);
}

#[test]
fn burn_state_matches_swf_set_energy_threshold() {
    let cold = Ball {
        id: 1,
        x: 0.0,
        y: 0.0,
        prev_y: 0.0,
        vx: 0.0,
        vy: 0.0,
        energy: BURN_ENERGY_THRESHOLD,
        color: Side::Blue,
        age: 0,
    };
    let burning = Ball {
        energy: BURN_ENERGY_THRESHOLD + 1.0,
        ..cold
    };

    assert!(!cold.is_burning());
    assert!(burning.is_burning());
}

#[test]
fn ball_visual_scale_uses_swf_set_energy_and_core_shape_bounds() {
    let tiny = Ball {
        id: 1,
        x: 0.0,
        y: 0.0,
        prev_y: 0.0,
        vx: 0.0,
        vy: 0.0,
        energy: 0.0,
        color: Side::Blue,
        age: 0,
    };
    let capped = Ball {
        energy: BURN_ENERGY_THRESHOLD,
        ..tiny
    };
    let burning = Ball {
        energy: BURN_ENERGY_THRESHOLD + 40.0,
        ..tiny
    };

    assert!((tiny.visual_scale() - 0.30).abs() < 0.000_001);
    assert!(BALL_CORE_RADIUS.mul_add(-0.30, tiny.visual_radius()).abs() < 0.000_001);
    assert!((capped.visual_scale() - 1.0).abs() < 0.000_001);
    assert!((capped.visual_radius() - BALL_CORE_RADIUS).abs() < 0.000_001);
    assert!((burning.visual_scale() - 1.0).abs() < 0.000_001);
    assert!((burning.visual_radius() - BALL_CORE_RADIUS).abs() < 0.000_001);
}

#[test]
fn scored_ball_die_visual_uses_swf_sprite15_tween() {
    let start = ball_die_visual(0);
    let carried_alpha = ball_die_visual(2);
    let bright = ball_die_visual(3);
    let tail = ball_die_visual(BALL_DIE_TICKS - 1);
    let clamped = ball_die_visual(999);

    assert!((start.scale - 0.586_944_580_078_125).abs() < 0.000_001);
    assert_eq!(start.alpha, 0.0);
    assert!((carried_alpha.scale - 0.668_563_842_773_437_5).abs() < 0.000_001);
    assert_eq!(carried_alpha.alpha, 95.0 / 256.0);
    assert!((bright.scale - 0.794_235_229_492_187_5).abs() < 0.000_001);
    assert_eq!(bright.alpha, 224.0 / 256.0);
    assert!((tail.scale - 1.673_904_418_945_312_5).abs() < 0.000_001);
    assert_eq!(tail.alpha, 0.0);
    assert_eq!(clamped, tail);
}

#[test]
fn paddle_charge_visual_uses_swf_tween_tables() {
    let blue_start = paddle_charge_visual(Side::Blue, 1);
    let blue_carried = paddle_charge_visual(Side::Blue, 61);
    let blue_end = paddle_charge_visual(Side::Blue, ENERGY_MAX_FRAME);
    let red_mid = paddle_charge_visual(Side::Red, 40);
    let red_end = paddle_charge_visual(Side::Red, ENERGY_MAX_FRAME + 20);

    assert!((blue_start.sx - 0.531_463_623_046_875).abs() < 0.000_001);
    assert!((blue_start.sy - 0.752_914_428_710_937_5).abs() < 0.000_001);
    assert_eq!(blue_start.color, (221, 236, 255));
    assert_eq!(blue_start.ready_flash, None);
    assert_eq!(blue_carried.color, (244, 238, 255));
    assert_eq!(blue_end.color, (68, 255, 251));
    let blue_ready = blue_end
        .ready_flash
        .expect("frame 200 should place the SWF ready overlay");
    assert!((blue_ready.scale - 1.0).abs() < 0.000_001);
    assert!((blue_ready.alpha - 1.0).abs() < 0.000_001);
    assert!((red_mid.sx - 0.916_763_305_664_062_5).abs() < 0.000_001);
    assert!((red_mid.sy - 0.956_100_463_867_187_5).abs() < 0.000_001);
    assert_eq!(red_mid.color, (255, 241, 201));
    assert_eq!(red_end.color, (255, 124, 34));
}

#[test]
fn paddle_stun_color_follows_swf_paralised_frame_sequence() {
    assert_eq!(paddle_stun_color(0), None);
    assert_eq!(paddle_stun_color(PADDLE_STUN_TICKS), Some((255, 0, 0)));
    assert_eq!(
        paddle_stun_color(PADDLE_STUN_TICKS - 1),
        Some((238, 255, 68))
    );
    assert_eq!(
        paddle_stun_color(PADDLE_STUN_TICKS - 2),
        Some((255, 255, 255))
    );
    assert_eq!(paddle_stun_color(1), Some((255, 255, 255)));
}

#[test]
fn score_meter_frame_matches_swf_int_divide_mapping() {
    assert_eq!(score_meter_frame(-10), 1);
    assert_eq!(score_meter_frame(0), 1);
    assert_eq!(score_meter_frame(4), 1);
    assert_eq!(score_meter_frame(10), 2);
    assert_eq!(score_meter_frame(699), 70);
    assert_eq!(score_meter_frame(700), 71);
    assert_eq!(score_meter_frame(710), 71);
}

#[test]
fn normal_goal_scores_energy_only_and_does_not_stun() {
    let mut world = World::new(Settings::default());
    world.red.v = 5.0;
    world.balls.push(Ball {
        id: 1,
        x: X_MIN_BALL - 1.0,
        y: 200.0,
        prev_y: 200.0,
        vx: 0.0,
        vy: 0.0,
        energy: 40.0,
        color: Side::Blue,
        age: 3,
    });

    world.tick(Controls::default());

    assert_eq!(world.blue_score, 4);
    assert_eq!(world.red.v, 4.0);
    assert_eq!(world.red.stun_ticks, 0);
    assert!(world.events.contains(&RoundEvent::Score {
        side: Side::Blue,
        burning: false,
    }));
}

#[test]
fn scored_ball_leaves_registry_but_keeps_swf_die_display() {
    let mut world = World::new(Settings::default());
    world.balls.push(Ball {
        id: 7,
        x: X_MIN_BALL - 1.0,
        y: 100.0,
        prev_y: 100.0,
        vx: -2.0,
        vy: 8.0,
        energy: 40.0,
        color: Side::Blue,
        age: 3,
    });

    world.tick(Controls::default());

    assert!(world.balls.is_empty());
    assert_eq!(world.dying_balls.len(), 1);
    let dying = world.dying_balls[0];
    assert_eq!(dying.x, X_MIN_BALL - 3.0);
    assert_eq!(dying.y, 108.0);
    assert_eq!(dying.vx, -2.0);
    assert_eq!(dying.vy, 8.0);
    assert_eq!(dying.energy, 40.0);
    assert_eq!(dying.color, Side::Blue);
    assert!(!dying.burning);
    assert_eq!(dying.age, 0);
    assert_eq!(dying.visual(), ball_die_visual(0));

    world.tick(Controls::default());

    let dying = world.dying_balls[0];
    assert_eq!(dying.x, X_MIN_BALL - 5.0);
    assert_eq!(dying.y, 116.0);
    assert_eq!(dying.vx, -1.0);
    assert_eq!(dying.vy, 4.0);
    assert_eq!(dying.age, 1);

    for _ in 1..BALL_DIE_TICKS {
        world.tick(Controls::default());
    }

    assert!(world.dying_balls.is_empty());
}

#[test]
fn burning_goal_paralyses_without_firebonus_points() {
    let mut world = World::new(Settings::default());
    world.red.v = 5.0;
    world.balls.push(Ball {
        id: 1,
        x: X_MIN_BALL - 1.0,
        y: 200.0,
        prev_y: 200.0,
        vx: 0.0,
        vy: 0.0,
        energy: 600.0,
        color: Side::Blue,
        age: 3,
    });

    world.tick(Controls::default());

    assert_eq!(world.blue_score, 60);
    assert_eq!(world.red.v, 0.0);
    assert_eq!(world.red.stun_ticks, PADDLE_STUN_TICKS);
    assert!(world.events.contains(&RoundEvent::Score {
        side: Side::Blue,
        burning: true,
    }));
}

#[test]
fn burning_goal_disables_paddle_for_the_swf_paralised_span() {
    let mut world = World::new(Settings::default());
    world.red.y = 12.0;
    world.balls.push(Ball {
        id: 1,
        x: X_MIN_BALL - 1.0,
        y: 200.0,
        prev_y: 200.0,
        vx: 0.0,
        vy: 0.0,
        energy: 600.0,
        color: Side::Blue,
        age: 3,
    });

    world.tick(Controls::default());

    let stunned_y = world.red.y;
    for expected_after_tick in (0..PADDLE_STUN_TICKS).rev() {
        world.tick(Controls {
            red_down: true,
            ..Controls::default()
        });
        assert_eq!(world.red.stun_ticks, expected_after_tick);
        assert_eq!(world.red.y, stunned_y);
        assert_eq!(world.red.v, 0.0);
    }

    world.tick(Controls {
        red_down: true,
        ..Controls::default()
    });

    assert_eq!(world.red.stun_ticks, 0);
    assert_eq!(world.red.v, 2.0);
    assert_eq!(world.red.y, stunned_y + 2.0);
}

#[test]
fn nearby_balls_merge_energy_and_velocity() {
    let mut world = World::new(Settings::default());
    world.balls.push(Ball {
        id: 1,
        x: 0.0,
        y: 0.0,
        prev_y: 0.0,
        vx: 2.0,
        vy: 0.0,
        energy: 100.0,
        color: Side::Blue,
        age: 0,
    });
    world.balls.push(Ball {
        id: 2,
        x: 10.0,
        y: 0.0,
        prev_y: 0.0,
        vx: 0.0,
        vy: 4.0,
        energy: 100.0,
        color: Side::Red,
        age: 0,
    });

    world.tick(Controls::default());

    assert_eq!(world.balls.len(), 1);
    assert_eq!(world.balls[0].energy, 200.0);
    assert!((world.balls[0].vx - 1.0).abs() < 0.000_001);
    assert!((world.balls[0].vy - 2.0).abs() < 0.000_001);
    assert_eq!(world.balls[0].color, Side::Blue);
}

#[test]
fn merged_ball_takes_partner_color_only_when_partner_has_more_energy() {
    let mut partner_larger = World::new(Settings::default());
    partner_larger.balls.push(Ball {
        id: 1,
        x: 0.0,
        y: 0.0,
        prev_y: 0.0,
        vx: 3.0,
        vy: 0.0,
        energy: 100.0,
        color: Side::Blue,
        age: 0,
    });
    partner_larger.balls.push(Ball {
        id: 2,
        x: 10.0,
        y: 0.0,
        prev_y: 0.0,
        vx: 0.0,
        vy: 6.0,
        energy: 200.0,
        color: Side::Red,
        age: 0,
    });

    partner_larger.tick(Controls::default());

    assert_eq!(partner_larger.balls.len(), 1);
    assert_eq!(partner_larger.balls[0].energy, 300.0);
    assert_eq!(partner_larger.balls[0].color, Side::Red);
    assert!((partner_larger.balls[0].vx - 1.0).abs() < 0.000_001);
    assert!((partner_larger.balls[0].vy - 4.0).abs() < 0.000_001);

    let mut first_larger = World::new(Settings::default());
    first_larger.balls.push(Ball {
        id: 1,
        x: 0.0,
        y: 0.0,
        prev_y: 0.0,
        vx: 3.0,
        vy: 0.0,
        energy: 200.0,
        color: Side::Blue,
        age: 0,
    });
    first_larger.balls.push(Ball {
        id: 2,
        x: 10.0,
        y: 0.0,
        prev_y: 0.0,
        vx: 0.0,
        vy: 6.0,
        energy: 100.0,
        color: Side::Red,
        age: 0,
    });

    first_larger.tick(Controls::default());

    assert_eq!(first_larger.balls.len(), 1);
    assert_eq!(first_larger.balls[0].energy, 300.0);
    assert_eq!(first_larger.balls[0].color, Side::Blue);
    assert!((first_larger.balls[0].vx - 2.0).abs() < 0.000_001);
    assert!((first_larger.balls[0].vy - 2.0).abs() < 0.000_001);
}

#[test]
fn merged_survivor_uses_original_loop_energy_for_later_pair_gravity() {
    let mut world = World::new(Settings::default());
    world
        .balls
        .push(test_ball(1, 0.0, 0.0, 0.0, 100.0, Side::Blue));
    world
        .balls
        .push(test_ball(2, 10.0, 0.0, 0.0, 100.0, Side::Red));
    world
        .balls
        .push(test_ball(3, 30.0, 0.0, 0.0, 100.0, Side::Red));

    world.tick(Controls::default());

    assert_eq!(
        world.balls.iter().map(|ball| ball.id).collect::<Vec<_>>(),
        vec![1, 3],
    );
    assert_eq!(world.balls[0].energy, 200.0);
    assert_eq!(world.balls[1].energy, 100.0);
    let gravity_from_original_i_energy =
        (100.0 / ENERGY_FACTOR) / (30.0 / GravityStrength::G2.factor());
    assert!((world.balls[1].vx - (0.0 - gravity_from_original_i_energy)).abs() < 0.000_001);
}

#[test]
fn merged_partner_can_still_merge_later_before_swf_delball_splice() {
    let mut world = World::new(Settings::default());
    world
        .balls
        .push(test_ball(1, 0.0, 0.0, 0.0, 100.0, Side::Blue));
    world
        .balls
        .push(test_ball(2, 10.0, 0.0, 0.0, 100.0, Side::Red));
    world
        .balls
        .push(test_ball(3, 20.0, 0.0, 0.0, 100.0, Side::Red));

    world.tick(Controls::default());

    assert_eq!(
        world.balls.iter().map(|ball| ball.id).collect::<Vec<_>>(),
        vec![1, 3],
    );
    assert_eq!(world.balls[0].energy, 200.0);
    assert_eq!(world.balls[1].energy, 0.0);
}

#[test]
fn merge_distance_is_strict_and_energy_caps_at_swf_limit() {
    let mut exact_distance = World::new(Settings::default());
    exact_distance
        .balls
        .push(test_ball(1, 0.0, 0.0, 0.0, 500.0, Side::Blue));
    exact_distance
        .balls
        .push(test_ball(2, MERGE_DISTANCE, 0.0, 0.0, 200.0, Side::Red));

    exact_distance.tick(Controls::default());

    assert_eq!(exact_distance.balls.len(), 2);
    assert_eq!(exact_distance.balls[0].energy, 500.0);
    assert_eq!(exact_distance.balls[1].energy, 200.0);
    assert!(!exact_distance.events.contains(&RoundEvent::Merge));

    let mut just_inside = World::new(Settings::default());
    just_inside
        .balls
        .push(test_ball(1, 0.0, 0.0, 0.0, 500.0, Side::Blue));
    just_inside.balls.push(test_ball(
        2,
        MERGE_DISTANCE - 0.001,
        0.0,
        0.0,
        200.0,
        Side::Red,
    ));

    just_inside.tick(Controls::default());

    assert_eq!(just_inside.balls.len(), 1);
    assert_eq!(just_inside.balls[0].energy, 600.0);
    assert_eq!(just_inside.balls[0].color, Side::Blue);
    assert!(just_inside.events.contains(&RoundEvent::Merge));
}

#[test]
fn gravity_strength_changes_pair_acceleration() {
    let mut weak = World::new(Settings::default());
    weak.settings.gravity = gravityarcade::sim::GravityStrength::G1;
    weak.balls.push(Ball {
        id: 1,
        x: -50.0,
        y: 0.0,
        prev_y: 0.0,
        vx: 0.0,
        vy: 0.0,
        energy: 100.0,
        color: Side::Blue,
        age: 0,
    });
    weak.balls.push(Ball {
        id: 2,
        x: 50.0,
        y: 0.0,
        prev_y: 0.0,
        vx: 0.0,
        vy: 0.0,
        energy: 100.0,
        color: Side::Red,
        age: 0,
    });
    weak.tick(Controls::default());
    let g1_vx = weak.balls[0].vx;

    let mut strong = World::new(Settings::default());
    strong.settings.gravity = gravityarcade::sim::GravityStrength::G5;
    strong.balls = weak
        .balls
        .iter()
        .map(|ball| Ball {
            vx: 0.0,
            vy: 0.0,
            ..*ball
        })
        .collect();
    strong.tick(Controls::default());

    assert!(strong.balls[0].vx > g1_vx);
}

#[test]
fn polarisation_modes_flip_pair_gravity_by_color_like_swf_values() {
    let force_x_for = |polarisation: Polarisation, partner_color: Side| {
        let mut world = World::new(Settings {
            polarisation,
            ..Settings::default()
        });
        world
            .balls
            .push(test_ball(1, -50.0, 0.0, 0.0, 100.0, Side::Blue));
        world
            .balls
            .push(test_ball(2, 50.0, 0.0, 0.0, 100.0, partner_color));

        world.tick(Controls::default());

        (world.balls[0].vx, world.balls[1].vx)
    };

    let neutral_same = force_x_for(Polarisation::Neutral, Side::Blue);
    assert!(neutral_same.0 > 0.0);
    assert!(neutral_same.1 < 0.0);

    let same_repels_same = force_x_for(Polarisation::SameRepels, Side::Blue);
    assert!(same_repels_same.0 < 0.0);
    assert!(same_repels_same.1 > 0.0);

    let same_repels_opposite = force_x_for(Polarisation::SameRepels, Side::Red);
    assert!(same_repels_opposite.0 > 0.0);
    assert!(same_repels_opposite.1 < 0.0);

    let opposite_repels_opposite = force_x_for(Polarisation::OppositeRepels, Side::Red);
    assert!(opposite_repels_opposite.0 < 0.0);
    assert!(opposite_repels_opposite.1 > 0.0);

    let all_repels_same = force_x_for(Polarisation::AllRepel, Side::Blue);
    assert!(all_repels_same.0 < 0.0);
    assert!(all_repels_same.1 > 0.0);
}

#[test]
fn score_700_fills_meter_without_winning_round() {
    let mut world = World::new(Settings::default());
    world.blue_score = 696;
    world.balls.push(Ball {
        id: 1,
        x: X_MIN_BALL - 1.0,
        y: 200.0,
        prev_y: 200.0,
        vx: -1.0,
        vy: 0.0,
        energy: 40.0,
        color: Side::Blue,
        age: 1,
    });

    world.tick(Controls::default());

    assert_eq!(world.blue_score, 700);
    assert_eq!(world.blue_matches, 0);
    assert_eq!(world.winner, None);
    assert!(world.events.contains(&RoundEvent::Score {
        side: Side::Blue,
        burning: false,
    }));
    assert!(
        !world
            .events
            .contains(&RoundEvent::MatchWin { side: Side::Blue })
    );
}

#[test]
fn match_score_clears_round_and_records_winner() {
    let mut world = World::new(Settings {
        matches: 1,
        ..Settings::default()
    });
    make_ready(&mut world);
    world.blue_score = 706;
    world.balls.push(Ball {
        id: 1,
        x: X_MIN_BALL - 1.0,
        y: 200.0,
        prev_y: 200.0,
        vx: -1.0,
        vy: 0.0,
        energy: 40.0,
        color: Side::Blue,
        age: 1,
    });
    world.tick(Controls::default());

    assert_eq!(world.blue_matches, 1);
    assert_eq!(world.winner, Some(Side::Blue));
    assert_eq!(world.blue_score, 710);
    assert_eq!(world.match_win_announce_ticks, 0);
    assert_eq!(
        world.final_win_announce_ticks,
        BLUE_FINAL_WIN_ANNOUNCE_TICKS
    );
    assert_eq!(world.final_win_announce_side, Some(Side::Blue));
    assert!(world.balls.is_empty());
    assert!(
        world
            .events
            .contains(&RoundEvent::FinalMatchWinSound { side: Side::Blue })
    );
}

#[test]
fn final_match_win_announces_then_returns_to_menu_state_without_new_round() {
    let mut world = World::new(Settings {
        matches: 1,
        ..Settings::default()
    });
    make_ready(&mut world);
    world.blue_score = 706;
    world.balls.push(Ball {
        id: 1,
        x: X_MIN_BALL - 1.0,
        y: 200.0,
        prev_y: 200.0,
        vx: -1.0,
        vy: 0.0,
        energy: 40.0,
        color: Side::Blue,
        age: 1,
    });
    world.tick(Controls::default());

    for _ in 0..BLUE_FINAL_WIN_ANNOUNCE_TICKS {
        world.tick(Controls::default());
    }

    assert_eq!(world.winner, Some(Side::Blue));
    assert_eq!(world.final_win_announce_ticks, 0);
    assert_eq!(world.final_win_announce_side, None);
    assert_eq!(world.match_win_announce_ticks, 0);
    assert_eq!(world.blue_score, 710);
    assert_eq!(world.red_score, 0);
    assert_eq!(world.round_intro_ticks, 0);
    assert!(!world.blue.gun_ready);
    assert!(!world.red.gun_ready);
    assert!(!world.events.contains(&RoundEvent::RoundStart));
}

#[test]
fn red_final_match_uses_redfinal_swf_timeline_length() {
    let mut world = World::new(Settings {
        matches: 1,
        ..Settings::default()
    });
    make_ready(&mut world);
    world.red_score = 706;
    world.balls.push(Ball {
        id: 1,
        x: X_MAX_BALL + 1.0,
        y: 200.0,
        prev_y: 200.0,
        vx: 1.0,
        vy: 0.0,
        energy: 40.0,
        color: Side::Red,
        age: 1,
    });
    world.tick(Controls::default());

    assert_eq!(world.red_matches, 1);
    assert_eq!(world.winner, Some(Side::Red));
    assert_eq!(world.final_win_announce_ticks, RED_FINAL_WIN_ANNOUNCE_TICKS);
    assert_eq!(world.final_win_announce_side, Some(Side::Red));
    assert!(
        world
            .events
            .contains(&RoundEvent::FinalMatchWinSound { side: Side::Red })
    );
}

#[test]
fn non_final_round_win_announces_before_restarting_intro_gate() {
    let mut world = World::new(Settings {
        matches: 3,
        ..Settings::default()
    });
    make_ready(&mut world);
    world.blue_score = 706;
    world.blue.energy_frame = 10;
    world.red.energy_frame = 30;
    world.next_ball_id = 42;
    world.balls.push(Ball {
        id: 1,
        x: X_MIN_BALL - 1.0,
        y: 200.0,
        prev_y: 200.0,
        vx: -1.0,
        vy: 0.0,
        energy: 40.0,
        color: Side::Blue,
        age: 1,
    });
    world.tick(Controls::default());

    assert_eq!(world.blue_matches, 1);
    assert_eq!(world.winner, None);
    assert_eq!(world.blue_score, 710);
    assert_eq!(world.blue.energy_frame, 11);
    assert_eq!(world.red.energy_frame, 31);
    assert_eq!(world.round_intro_ticks, 0);
    assert_eq!(world.match_win_announce_ticks, MATCH_WIN_ANNOUNCE_TICKS);
    assert_eq!(world.match_win_announce_side, Some(Side::Blue));
    assert!(!world.blue.gun_ready);
    assert!(!world.red.gun_ready);
    assert!(world.balls.is_empty());
    assert_eq!(world.next_ball_id, 10);
    assert!(
        !world
            .events
            .contains(&RoundEvent::RoundLostSound { side: Side::Blue })
    );

    let score_idx = world
        .events
        .iter()
        .position(|event| {
            matches!(
                event,
                RoundEvent::Score {
                    side: Side::Blue,
                    burning: false
                }
            )
        })
        .expect("match-winning score event should be emitted");
    let win_idx = world
        .events
        .iter()
        .position(|event| matches!(event, RoundEvent::MatchWin { side: Side::Blue }))
        .expect("match win event should be emitted");
    assert!(score_idx < win_idx);

    for _ in 0..5 {
        world.tick(Controls::default());
        assert!(
            !world
                .events
                .contains(&RoundEvent::RoundLostSound { side: Side::Blue })
        );
    }

    world.tick(Controls::default());
    assert_eq!(world.match_win_announce_ticks, MATCH_WIN_ANNOUNCE_TICKS - 6);
    assert!(
        world
            .events
            .contains(&RoundEvent::RoundLostSound { side: Side::Blue })
    );

    for _ in 0..(MATCH_WIN_ANNOUNCE_TICKS - 6) {
        world.tick(Controls::default());
    }

    assert_eq!(world.blue_score, 0);
    assert_eq!(world.red_score, 0);
    assert_eq!(world.match_win_announce_ticks, 0);
    assert_eq!(world.match_win_announce_side, None);
    assert_eq!(world.round_intro_ticks, ROUND_INTRO_TICKS);
    assert_eq!(world.blue.energy_frame, 107);
    assert_eq!(world.red.energy_frame, 127);
    assert!(world.events.contains(&RoundEvent::RoundStart));
    assert!(!world.blue.gun_ready);
    assert!(!world.red.gun_ready);

    for _ in 0..ROUND_INTRO_TICKS {
        world.tick(Controls::default());
    }

    assert_eq!(world.round_intro_ticks, 0);
    assert_eq!(world.blue.energy_frame, 167);
    assert_eq!(world.red.energy_frame, 187);
    assert!(world.blue.gun_ready);
    assert!(world.red.gun_ready);
}

#[test]
fn red_round_lost_sound_uses_swf_sprite162_frame_105() {
    let mut world = World::new(Settings {
        matches: 3,
        ..Settings::default()
    });
    make_ready(&mut world);
    world.red_score = 706;
    world.balls.push(Ball {
        id: 1,
        x: X_MAX_BALL + 1.0,
        y: 200.0,
        prev_y: 200.0,
        vx: 1.0,
        vy: 0.0,
        energy: 40.0,
        color: Side::Red,
        age: 1,
    });
    world.tick(Controls::default());

    assert_eq!(world.match_win_announce_ticks, MATCH_WIN_ANNOUNCE_TICKS);
    assert_eq!(world.match_win_announce_side, Some(Side::Red));
    assert!(
        !world
            .events
            .contains(&RoundEvent::RoundLostSound { side: Side::Red })
    );

    for _ in 0..6 {
        world.tick(Controls::default());
        assert!(
            !world
                .events
                .contains(&RoundEvent::RoundLostSound { side: Side::Red })
        );
    }

    world.tick(Controls::default());

    assert_eq!(world.match_win_announce_ticks, MATCH_WIN_ANNOUNCE_TICKS - 7);
    assert!(
        world
            .events
            .contains(&RoundEvent::RoundLostSound { side: Side::Red })
    );
}

#[test]
fn round_intro_disables_guns_until_swf_ready_frame() {
    let mut world = World::new(Settings::default());
    assert_eq!(world.round_intro_ticks, ROUND_INTRO_TICKS);
    assert_eq!(world.round_intro_visual_ticks, ROUND_INTRO_VISUAL_TICKS);
    assert!(!world.blue.gun_ready);
    assert!(!world.red.gun_ready);
    assert!(!world.try_fire(Side::Blue));

    for _ in 0..(ROUND_INTRO_VISUAL_TICKS - ROUND_INTRO_SOUND_TICKS - 1) {
        world.tick(Controls::default());
        assert!(!world.events.contains(&RoundEvent::RoundIntroSound));
    }

    world.tick(Controls::default());
    assert!(world.events.contains(&RoundEvent::RoundIntroSound));

    let remaining_ready_ticks = world.round_intro_ticks;
    assert_eq!(
        remaining_ready_ticks,
        ROUND_INTRO_TICKS - (ROUND_INTRO_VISUAL_TICKS - ROUND_INTRO_SOUND_TICKS)
    );

    for _ in 0..remaining_ready_ticks {
        world.tick(Controls::default());
    }

    assert_eq!(world.round_intro_ticks, 0);
    assert_eq!(
        world.round_intro_visual_ticks,
        ROUND_INTRO_VISUAL_TICKS - ROUND_INTRO_TICKS
    );
    assert!(world.blue.gun_ready);
    assert!(world.red.gun_ready);
    assert!(world.try_fire(Side::Blue));

    for _ in 0..(ROUND_INTRO_VISUAL_TICKS - ROUND_INTRO_TICKS) {
        world.tick(Controls::default());
    }

    assert_eq!(world.round_intro_visual_ticks, 0);
}

#[test]
fn held_fire_during_round_intro_waits_one_tick_after_swf_gunready_frame() {
    let mut world = World::new(Settings::default());
    let fire = Controls {
        blue_fire: true,
        ..Controls::default()
    };

    for _ in 0..(ROUND_INTRO_TICKS - 1) {
        world.tick(fire);
        assert!(world.balls.is_empty());
        assert!(!world.blue.gun_ready);
    }

    assert_eq!(world.round_intro_ticks, 1);
    world.tick(fire);
    assert_eq!(world.round_intro_ticks, 0);
    assert!(world.blue.gun_ready);
    assert!(world.balls.is_empty());
    assert!(
        !world
            .events
            .contains(&RoundEvent::Shot { side: Side::Blue })
    );

    world.tick(fire);
    assert_eq!(world.balls.len(), 1);
    assert!(
        world
            .events
            .contains(&RoundEvent::Shot { side: Side::Blue })
    );
}

fn make_ready(world: &mut World) {
    world.round_intro_ticks = 0;
    world.round_intro_visual_ticks = 0;
    world.blue.gun_ready = true;
    world.red.gun_ready = true;
}

fn test_ball(id: u32, x: f64, y: f64, vx: f64, energy: f64, color: Side) -> Ball {
    Ball {
        id,
        x,
        y,
        prev_y: y,
        vx,
        vy: 0.0,
        energy,
        color,
        age: 0,
    }
}
