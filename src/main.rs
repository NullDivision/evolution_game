use bevy::{
    prelude::{default, Camera2dBundle, Commands, Component, Query},
    text::{Text, Text2dBundle, TextSection, TextStyle},
};

// Score component
#[derive(Component)]
struct Score(i32);

fn main() {
    bevy::app::App::new()
        .add_plugins(bevy::DefaultPlugins)
        .add_systems(bevy::app::Startup, |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
            commands.spawn((
                Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            format!("Score: {}", 0),
                            TextStyle { ..default() },
                        )],
                        ..default()
                    },
                    ..default()
                },
                Score(0),
            ));
        })
        .add_systems(
            bevy::app::FixedUpdate,
            |mut score: Query<&mut Score>, mut text: Query<&mut Text>| {
                score.single_mut().0 += 1;
                text.single_mut().sections[0].value = format!("Score: {}", score.single().0);
            },
        )
        .run();
}
