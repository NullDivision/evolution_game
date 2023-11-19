mod movement;
mod mutations;
mod state;

use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use movement::*;
use mutations::*;
use rand::Rng;
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
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(Color::RED.into()),
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
                    mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                    material: materials.add(Color::WHITE.into()),
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
    time_step: Res<FixedTime>,
) {
    for (mut transform, velocity) in character_transforms.iter_mut() {
        // Move character around randomly
        transform.translation.x += velocity.velocity_x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.velocity_y * time_step.period.as_secs_f32();
    }
}

fn update_keyboard_movement(
    keyboard_input: Res<Input<KeyCode>>,
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

fn detect_collisions(
    mut commands: Commands,
    mut player: Query<(&mut Transform, &mut Weight), With<Mutations>>,
    npcs: Query<(&Transform, &Weight, Entity), (&Movement, Without<Mutations>)>,
) {
    let (mut player_transform, mut player_weight) = player.single_mut();

    for (npc_transform, npc_weight, npc) in npcs.iter() {
        let distance = player_transform
            .translation
            .distance(npc_transform.translation);

        println!("{:?}", player_transform.scale);
        if distance < (10. * player_weight.0) {
            // Player weight goes up based on enemy weight consumed
            player_weight.0 += npc_weight.0 / player_weight.0; 
            player_transform.scale = Transform::from_scale(Vec3::splat(player_weight.0)).scale;

            commands.entity(npc).despawn();
        }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MutationsMenuPlugin))
        .add_state::<AppState>()
        .add_systems(Startup, startup_game)
        .add_systems(
            FixedUpdate,
            (
                update_inert_jitter_velocity,
                update_mutant_jitter_velocity,
                update_keyboard_movement,
                update_entity_movement,
                detect_collisions,
            ),
        )
        .run();
}
