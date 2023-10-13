use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

use crate::mutations::Mutations;

// Velocity component
#[derive(Component, Debug)]
pub struct Movement {
    pub signum_x: i8,
    pub signum_y: i8,
    pub velocity_x: f32,
    pub velocity_y: f32,
    velocity_x_max: f32,
    velocity_y_max: f32,
}

const MAX_VELOCITY: f32 = 10.;
const DIRECTION_CHANGE_WEIGHT: f64 = 0.03;
const DECELERATION: f32 = 0.5;

fn apply_deceleration(velocity: &mut Movement) {
    if velocity.velocity_x > 0. {
        velocity.velocity_x -= DECELERATION;
    } else if velocity.velocity_x < 0. {
        velocity.velocity_x += DECELERATION;
    }

    if velocity.velocity_y > 0. {
        velocity.velocity_y -= DECELERATION;
    } else if velocity.velocity_y < 0. {
        velocity.velocity_y += DECELERATION;
    }
}

pub fn build_keyboard_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut velocity: Query<&mut Movement, With<Mutations>>,
) {
    let mut player_velocity = velocity.get_single_mut().unwrap();

    if keyboard_input.pressed(KeyCode::Left) {
        player_velocity.velocity_x -= 1.;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        player_velocity.velocity_x += 1.;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        player_velocity.velocity_y += 1.;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        player_velocity.velocity_y -= 1.;
    }

    apply_deceleration(&mut player_velocity);
}

pub fn build_movement() -> Movement {
    Movement {
        signum_x: 1,
        signum_y: 1,
        velocity_x: 0.,
        velocity_y: 0.,
        velocity_x_max: MAX_VELOCITY,
        velocity_y_max: MAX_VELOCITY,
    }
}

fn clamp_velocity(movement: &mut Movement) {
    if movement.velocity_x.abs() >= movement.velocity_x_max {
        movement.velocity_x = movement.velocity_x_max * movement.velocity_x.signum();
    }
    if movement.velocity_y.abs() >= movement.velocity_y_max {
        movement.velocity_y = movement.velocity_y_max * movement.velocity_y.signum();
    }
}

pub fn calculate_jitter(rng: &mut ThreadRng, movement: &mut Movement) {
    let x_velocity_offset = rng.gen_range(0.0..=movement.velocity_x_max);
    let y_velocity_offset = rng.gen_range(0.0..=movement.velocity_y_max);

    // Use weighted offset to determine direction
    if rng.gen_bool(DIRECTION_CHANGE_WEIGHT) {
        movement.signum_x *= -1;
    }
    if rng.gen_bool(DIRECTION_CHANGE_WEIGHT) {
        movement.signum_y *= -1;
    }

    // Add speed
    movement.velocity_x += x_velocity_offset * movement.signum_x as f32;
    movement.velocity_y += y_velocity_offset * movement.signum_y as f32;

    clamp_velocity(movement);
}
