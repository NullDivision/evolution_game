use bevy::prelude::*;
use rand::Rng;

// Score component
#[derive(Component)]
struct Score(i32);

// Character component
#[derive(Component)]
struct Character;

fn game_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Player
    commands.spawn((
        bevy::sprite::MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(Color::RED.into()),
            ..default()
        },
        Character,
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
            Character,
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (game_startup, score_board_startup))
        .add_systems(
            FixedUpdate,
            |mut character_transforms: Query<&mut Transform, With<Character>>| {
                let mut rng = rand::thread_rng();

                for mut transform in character_transforms.iter_mut() {
                    // Move character around randomly
                    transform.translation.x = rng.gen_range(-1.0..1.0);
                }
            },
        )
        .run();
}
