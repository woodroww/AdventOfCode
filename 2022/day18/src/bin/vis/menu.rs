use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{BoulderEmpty, MattsAxis, TouchingCube, VisState, BoulderCube, BoulderAssets};

pub struct MenuPlugin;

#[derive(Component, Inspectable)]
pub struct MenuRoot;

#[derive(Component, Inspectable)]
pub struct EmptiesButton;

#[derive(Component, Inspectable)]
pub struct TouchingButton;

#[derive(Component, Inspectable)]
pub struct AxisButton;

#[derive(Component, Inspectable)]
pub struct ShowUpButton;

#[derive(Component, Inspectable)]
pub struct ShowDownButton;

const NORMAL_BUTTON: Color = Color::rgb(0.0, 0.0, 50.0 / 255.0);
const HOVERED_BUTTON: Color = Color::rgb(25.0 / 255.0, 25.0 / 255.0, 50.0 / 255.0);
const PRESSED_BUTTON: Color = Color::rgb(50.0 / 255.0, 50.0 / 255.0, 50.0 / 255.0);

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(VisState::Menu).with_system(spawn_main_menu))
            .add_system_set(
                SystemSet::on_update(VisState::Menu)
                    .with_system(button_system)
                    .with_system(empties_button_press)
                    .with_system(touching_button_press)
                    .with_system(axis_button_press)
                    .with_system(show_up_button_press)
                    .with_system(show_down_button_press)
            );
    }
}

fn show_down_button_press(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ShowDownButton>)>,
    mut cube_query: Query<(&mut Visibility, &BoulderCube)>,
    mut assets: ResMut<BoulderAssets>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if assets.show_level > 0 {
                    assets.show_level -= 1;
                }
                for (mut visibility, cube) in cube_query.iter_mut() {
                    if cube.location.y > assets.show_level {
                        visibility.is_visible = false;
                    } else {
                        visibility.is_visible = true;
                    }
                }
            }
            _ => {}
        }
    }
}

fn show_up_button_press(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ShowUpButton>)>,
    mut cube_query: Query<(&mut Visibility, &BoulderCube)>,
    mut assets: ResMut<BoulderAssets>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if assets.show_level < assets.max_show_y {
                    assets.show_level += 1;
                }
                for (mut visibility, cube) in cube_query.iter_mut() {
                    if cube.location.y > assets.show_level {
                        visibility.is_visible = false;
                    } else {
                        visibility.is_visible = true;
                    }
                }
            }
            _ => {}
        }
    }
}

fn axis_button_press(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<AxisButton>)>,
    mut axis_query: Query<&mut Visibility, With<MattsAxis>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                for mut visibility in axis_query.iter_mut() {
                    visibility.toggle();
                }
            }
            _ => {}
        }
    }
}

fn touching_button_press(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<TouchingButton>)>,
    mut touching_cube_query: Query<&mut Visibility, With<TouchingCube>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                for mut visibility in touching_cube_query.iter_mut() {
                    visibility.toggle();
                }
            }
            _ => {}
        }
    }
}

fn empties_button_press(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<EmptiesButton>)>,
    mut empty_cube_query: Query<&mut Visibility, With<BoulderEmpty>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                for mut visibility in empty_cube_query.iter_mut() {
                    visibility.toggle();
                }
            }
            _ => {}
        }
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn spawn_button(
    commands: &mut Commands,
    asset_server: &AssetServer,
    text: &str,
    color: Color,
) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(10.0), Val::Percent(15.0)),
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                //margin: UiRect::all(Val::Percent(5.0)),
                min_size: Size::new(Val::Px(100.0), Val::Px(30.0)),
                max_size: Size::new(Val::Px(100.0), Val::Px(30.0)),
                ..default()
            },
            background_color: color.into(),
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    //margin: UiRect::all(Val::Percent(3.0)),
                    ..default()
                },
                text: Text::from_section(
                    text.to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/RobotoMono-VariableFont_wght.ttf"),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
        })
        .id()
}

fn spawn_layer_button(commands: &mut Commands, asset_server: &AssetServer, text: &str) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(10.0), Val::Percent(15.0)),
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                //margin: UiRect::all(Val::Percent(5.0)),
                min_size: Size::new(Val::Px(30.0), Val::Px(30.0)),
                max_size: Size::new(Val::Px(30.0), Val::Px(30.0)),
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    min_size: Size::new(Val::Px(30.0), Val::Px(30.0)),
                    max_size: Size::new(Val::Px(30.0), Val::Px(30.0)),
                    //margin: UiRect::all(Val::Percent(3.0)),
                    ..default()
                },
                text: Text::from_section(
                    text.to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/RobotoMono-VariableFont_wght.ttf"),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
        })
        .id()
}

fn spawn_level_ui(mut commands: &mut Commands, asset_server: &AssetServer) -> Entity {
    let up_button = spawn_layer_button(&mut commands, &asset_server, "U");
    commands
        .entity(up_button)
        .insert(Name::new("LayerUpButton"))
        .insert(ShowUpButton);
    let down_button = spawn_layer_button(&mut commands, &asset_server, "D");
    commands
        .entity(down_button)
        .insert(Name::new("LayerDownButton"))
        .insert(ShowDownButton);

    let layer_text = commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                //margin: UiRect::all(Val::Percent(3.0)),
                min_size: Size::new(Val::Px(30.0), Val::Px(30.0)),
                max_size: Size::new(Val::Px(30.0), Val::Px(30.0)),
                ..default()
            },
            text: Text::from_section(
                "0".to_string(),
                TextStyle {
                    font: asset_server.load("fonts/RobotoMono-VariableFont_wght.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ),
            ..default()
        })
        .id();
    commands
        .entity(layer_text)
        .insert(Name::new("LavaLayerText"));

    commands
        .spawn(NodeBundle {
            style: Style {
                //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        })
        .add_child(down_button)
        .add_child(layer_text)
        .add_child(up_button)
        .id()
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let empties_button = spawn_button(
        &mut commands,
        &asset_server,
        "Empties",
        Color::rgb_u8(50, 0, 0),
    );
    commands
        .entity(empties_button)
        .insert(EmptiesButton)
        .insert(Name::new("EmptiesButton"));

    let touching_button = spawn_button(
        &mut commands,
        &asset_server,
        "Touching",
        Color::rgb_u8(0, 0, 50),
    );
    commands
        .entity(touching_button)
        .insert(TouchingButton)
        .insert(Name::new("TouchingButton"));

    let axis_button = spawn_button(
        &mut commands,
        &asset_server,
        "Axis",
        Color::rgb_u8(0, 0, 50),
    );
    commands
        .entity(axis_button)
        .insert(AxisButton)
        .insert(Name::new("AxisButton"));

    let layers_buttons = spawn_level_ui(&mut commands, &asset_server);
    commands.entity(layers_buttons);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(MenuRoot)
        .with_children(|commands| {
            commands
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::FlexEnd,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|commands| {
                    commands
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(10.0), Val::Percent(100.0)),
                                justify_content: JustifyContent::FlexStart,
                                flex_direction: FlexDirection::Column,
                                margin: UiRect::all(Val::Percent(3.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .add_child(empties_button)
                        .add_child(axis_button)
                        .add_child(touching_button)
                        .add_child(layers_buttons);
                });
        });
}
