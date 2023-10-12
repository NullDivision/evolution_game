use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use rand::Rng;

// Score component
#[derive(Component)]
struct Score(i32);

// Velocity component
#[derive(Component)]
struct Movement {
    signum_x: i8,
    signum_y: i8,
    velocity_x: f32,
    velocity_y: f32,
    velocity_x_max: f32,
    velocity_y_max: f32,
}

const MAX_ENTITIES: usize = 500;
const MAX_VELOCITY: f32 = 10.;

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

    // Player
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(12.).into()).into(),
            material: materials.add(Color::RED.into()),
            ..default()
        },
        Movement {
            signum_x: 1,
            signum_y: 1,
            velocity_x: 0.,
            velocity_y: 0.,
            velocity_x_max: MAX_VELOCITY,
            velocity_y_max: MAX_VELOCITY,
        },
        mutations::Mutations {
            controlled_movement: false,
        },
    ));

    // Other characters
    for _i in 0..MAX_ENTITIES {
        let enemy_radius = 10.;

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(enemy_radius).into()).into(),
                material: materials.add(Color::WHITE.into()),
                // Randomly position element in view
                transform: Transform::from_translation(Vec3::new(
                    rng.gen_range(-window_half_width..=window_half_width),
                    rng.gen_range(-window_half_height..=window_half_height),
                    0.,
                )),
                ..default()
            },
            Movement {
                signum_x: 1,
                signum_y: 1,
                velocity_x: 0.,
                velocity_y: 0.,
                velocity_x_max: MAX_VELOCITY,
                velocity_y_max: MAX_VELOCITY,
            },
        ));
    }
}

fn startup_score_board(mut commands: Commands) {
    commands.spawn((
        TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    format!("Score: {}", 0),
                    TextStyle { ..default() },
                )],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(5.),
                top: Val::Px(5.),
                ..default()
            },
            ..default()
        },
        Score(0),
    ));
}

const DIRECTION_CHANGE_WEIGHT: f64 = 0.05;

// Update functions
fn update_movement(mut character_movement: Query<&mut Movement>) {
    let mut rng = rand::thread_rng();

    for mut movement in character_movement.iter_mut() {
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

        // Ensure velocity is within bounds while maintaining direction
        if movement.velocity_x.abs() >= movement.velocity_x_max {
            movement.velocity_x = movement.velocity_x_max * movement.velocity_x.signum();
        }
        if movement.velocity_y.abs() >= movement.velocity_y_max {
            movement.velocity_y = movement.velocity_y_max * movement.velocity_y.signum();
        }
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

mod app_state {
    use bevy::prelude::*;

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
    pub enum AppState {
        #[default]
        Menu,
        Game,
    }
}

mod mutations {
    use crate::app_state::AppState;
    use bevy::prelude::*;

    #[derive(Component)]
    struct MutationsMenu;

    #[derive(Component)]
    pub struct Mutations {
        pub controlled_movement: bool,
    }

    #[derive(Component, Debug)]
    enum Mutation {
        ControlledMovement,
    }

    pub struct MenuPlugin;

    fn startup_trait_card(mut commands: Commands) {
        commands
            .spawn((
                ButtonBundle {
                    background_color: BackgroundColor(Color::BLUE),
                    ..default()
                },
                MutationsMenu,
                Mutation::ControlledMovement,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            format!("Trait"),
                            TextStyle {
                                color: Color::WHITE,
                                ..default()
                            },
                        )],
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                    ..default()
                });
            });
    }

    fn handle_mouse_input(
        button_interaction: Query<(&Interaction, &Mutation), (Changed<Interaction>, With<Button>)>,
        mut entity_mutations: Query<&mut Mutations>,
        mut next_app_state: ResMut<NextState<AppState>>,
    ) {
        for (interaction, mutation) in button_interaction.iter() {
            println!("Interaction: {:?}, Mutation: {:?}", interaction, mutation);

            match *interaction {
                Interaction::Pressed => match mutation {
                    Mutation::ControlledMovement => {
                        println!("Selecting Controlled movement");
                        entity_mutations
                            .get_single_mut()
                            .unwrap()
                            .controlled_movement = true;
                        next_app_state.set(AppState::Game);
                    }
                },
                _ => {}
            }
        }
    }

    fn destroy_trait_card(mut commands: Commands, trait_menu: Query<Entity, With<MutationsMenu>>) {
        for entity in trait_menu.iter() {
            println!("Destroying trait card");
            commands.entity(entity).despawn_recursive();
        }
    }

    impl Plugin for MenuPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(AppState::Menu), startup_trait_card)
                .add_systems(Update, handle_mouse_input.run_if(in_state(AppState::Menu)))
                .add_systems(OnExit(AppState::Menu), destroy_trait_card);
        }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, mutations::MenuPlugin))
        .add_state::<crate::app_state::AppState>()
        .add_systems(Startup, (startup_game, startup_score_board))
        .add_systems(FixedUpdate, (update_movement, update_entity_movement))
        .run();
}
