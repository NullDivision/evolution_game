mod movement;
mod mutations;
mod state;

use bevy::{
    math::primitives::Circle,
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use movement::*;
use mutations::*;
use rand::{Rng, rngs::ThreadRng};
use state::*;

const MAX_ENTITIES: usize = 500;

#[derive(Component)]
struct Weight(f32);

#[derive(Bundle)]
struct Creature {
    movement: Movement,
    sprite: MaterialMesh2dBundle<ColorMaterial>,
    weight: Weight,
}

fn get_offscreen_render_location(viewport: Rect, range: &mut ThreadRng) -> Vec3 {
    let mut entity_location = Vec3::default();

    entity_location.x = range.gen_range(viewport.min.x..=viewport.max.x);
    entity_location.y = range.gen_range(viewport.min.y..=viewport.max.y);

    entity_location
}

// Startup functions
fn startup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    let mut rng = rand::thread_rng();
    let main_window = window.single();
    let window_half_width = main_window.width() / 2.;
    let window_half_height = main_window.height() / 2.;

    commands.spawn(Camera2dBundle::default());

    let player_radius = 1.;

    // Player
    commands.spawn((
        Creature {
            movement: build_movement(),
            sprite:  MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(10.)).into(),
                material: materials.add(Color::RED),
                ..default()


            },
            weight: Weight(player_radius),
        },
        Mutations {
            controlled_movement: false,
        },
    ));

    // Other characters
    for _i in 0..MAX_ENTITIES {
        let enemy_radius = 1.;

        commands.spawn((
            Creature {
                movement: build_movement(),
                sprite: MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(10.)).into(),
                    material: materials.add(Color::WHITE),
                    // Randomly position element in view
                    transform: Transform::from_translation(Vec3::new(
                            rng.gen_range(-window_half_width..=window_half_width),
                            rng.gen_range(-window_half_height..=window_half_height),
                            0.,
                            )),
                            ..default()
                },
                weight: Weight(enemy_radius),
            },
        ));
    }
}

// Update functions
fn update_inert_jitter_velocity(mut character_movement: Query<&mut Movement, Without<Mutations>>) {
    let mut rng = rand::thread_rng();

    for mut movement in character_movement.iter_mut() {
        calculate_jitter(&mut rng, &mut movement);
    }
}

fn update_entity_movement(
    mut character_transforms: Query<(&mut Transform, &mut Movement)>,
    time_step: Res<Time>,
) {
    for (mut transform, velocity) in character_transforms.iter_mut() {
        // Move character around randomly
        transform.translation.x += velocity.velocity_x * time_step.elapsed_seconds();
        transform.translation.y += velocity.velocity_y * time_step.elapsed_seconds();
    }
}

fn update_keyboard_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mutations: Query<&mut Mutations>,
    velocity: Query<&mut Movement, With<Mutations>>,
) {
    match mutations.get_single() {
        Ok(player_mutations) => {
            if !player_mutations.controlled_movement {
                return;
            }

            build_keyboard_movement(keyboard_input, velocity);
        }
        Err(_) => {
            println!("No entity with mutations found");
            return;
        }
    }
}

fn update_mutant_jitter_velocity(mut character_movement: Query<(&mut Movement, &Mutations)>) {
    let mut rng = rand::thread_rng();

    for (mut movement, entity_mutations) in character_movement.iter_mut() {
        if entity_mutations.controlled_movement {
            continue;
        }

        calculate_jitter(&mut rng, &mut movement);
    }
}

fn handle_collisions(
    mut player: Query<(&mut Transform, &mut Weight), With<Mutations>>,
    mut npcs: Query<(&mut Transform, &Weight), (With<Movement>, Without<Mutations>)>,
    projection: Query<&OrthographicProjection, With<Camera>>,
) {
    let (mut player_transform, mut player_weight) = player.single_mut();
    let mut rng = rand::thread_rng();

    for (mut npc_transform, npc_weight) in npcs.iter_mut() {
        let distance = player_transform
            .translation
            .distance(npc_transform.translation);

        if distance < (10. * player_weight.0) {
            // Player weight goes up based on enemy weight consumed
            player_weight.0 += npc_weight.0 / player_weight.0;
            player_transform.scale = Transform::from_scale(Vec3::splat(player_weight.0)).scale;

            // Move entity off screen
            npc_transform.translation = get_offscreen_render_location(projection.single().area, &mut rng);
        }
    }
}

fn update_camera_position(
    mut camera: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
    player: Query<(&Weight, &Transform), (With<Mutations>, Without<Camera>)>
) {
    let (player_weight, player_transform) = player.single();

    let (mut projection, mut camera_transform) = camera.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
    projection.scale = player_weight.0 * 0.5;
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MutationsMenuPlugin))
        .init_state::<AppState>()
        .add_systems(Startup, startup_game)
        .add_systems(
            FixedUpdate,
            (
                update_inert_jitter_velocity,
                update_mutant_jitter_velocity,
                update_keyboard_movement,
                update_entity_movement,
                update_camera_position,
                handle_collisions,
            ),
        )
        .run();
}
