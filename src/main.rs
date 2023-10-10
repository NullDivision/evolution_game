use bevy::prelude::*;
use rand::Rng;

// Score component
#[derive(Component)]
struct Score(i32);

// Velocity component
#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
    x_max: f32,
    y_max: f32,
}

fn game_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Player
    commands.spawn((
        bevy::sprite::MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(12.).into()).into(),
            material: materials.add(Color::RED.into()),
            ..default()
        },
        Velocity {
            x: 0.,
            y: 0.,
            x_max: 1000.,
            y_max: 1000.,
        },
    ));

    // Other characters
    for i in 0..10 {
        let enemy_radius = 10.;

        commands.spawn((
            bevy::sprite::MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(enemy_radius).into()).into(),
                material: materials.add(Color::WHITE.into()),
                transform: Transform::from_translation(Vec3::new(
                    i as f32 * (enemy_radius + (enemy_radius / 2.)),
                    0.,
                    0.,
                )),
                ..default()
            },
            Velocity {
                x: 0.,
                y: 0.,
                x_max: 1000.,
                y_max: 1000.,
            },
        ));
    }
}

fn score_board_startup(mut commands: Commands) {
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

const DIRECTION_CHANGE_WEIGHT: f64 = 0.1;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (game_startup, score_board_startup))
        .add_systems(
            FixedUpdate,
            |mut character_transforms: Query<(&mut Transform, &mut Velocity)>,
             time_step: Res<FixedTime>| {
                let mut rng = rand::thread_rng();

                for (mut transform, mut velocity) in character_transforms.iter_mut() {
                    let x_velocity_offset = rng.gen_range(-velocity.x_max..=velocity.x_max);
                    let y_velocity_offset = rng.gen_range(-velocity.x_max..=velocity.y_max);

                    // Use weighted offset to determine direction
                    if rng.gen_bool(DIRECTION_CHANGE_WEIGHT) {
                        velocity.x = -x_velocity_offset;
                    } else {
                        velocity.x = x_velocity_offset;
                    }
                    if rng.gen_bool(DIRECTION_CHANGE_WEIGHT) {
                        velocity.y = -y_velocity_offset;
                    } else {
                        velocity.y = y_velocity_offset;
                    }

                    // Ensure velocity is within bounds while maintaining direction
                    if velocity.x.abs() >= velocity.x_max {
                        velocity.x = velocity.x_max * velocity.x.signum();
                    }
                    if velocity.y.abs() >= velocity.y_max {
                        velocity.y = velocity.y_max * velocity.y.signum();
                    }

                    // Move character around randomly
                    transform.translation.x += velocity.x * time_step.period.as_secs_f32();
                    transform.translation.y += velocity.y * time_step.period.as_secs_f32();
                }
            },
        )
        .run();
}
