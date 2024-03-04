use crate::state::AppState;
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

pub struct MutationsMenuPlugin;

fn startup_trait_card(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                width: Val::Vw(100.),
                ..default()
            },
            ..default()
        })
        .with_children(|node_parent| {
            node_parent
                .spawn((
                    ButtonBundle {
                        background_color: BackgroundColor(Color::BLUE),
                        style: Style {
                            align_items: AlignItems::Center,
                            display: Display::Flex,
                            height: Val::Vh(5.),
                            justify_content: JustifyContent::Center,
                            width: Val::Percent(80.),
                            ..default()
                        },
                        ..default()
                    },
                    MutationsMenu,
                    Mutation::ControlledMovement,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            justify: JustifyText::Center,
                            sections: vec![TextSection::new(
                                format!("Controlled Movement"),
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

impl Plugin for MutationsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), startup_trait_card)
            .add_systems(Update, handle_mouse_input.run_if(in_state(AppState::Menu)))
            .add_systems(OnExit(AppState::Menu), destroy_trait_card);
    }
}
