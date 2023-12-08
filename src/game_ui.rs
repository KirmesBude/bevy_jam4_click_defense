use crate::{
    actions::QueueAllyUnit,
    castle::{AllyCastle, UnitPoints},
    techtree::SpawnCooldownReduction,
    units::UnitKind,
    GameState,
};
use bevy::prelude::*;

pub struct GameUiPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_game_ui)
            .add_systems(
                Update,
                (
                    click_spawn_button,
                    update_spawn_button_text,
                    click_spawn_cooldown_reduction_button,
                    update_spawn_cooldown_reduction_button,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Debug, Component)]
struct SpawnButton(pub UnitKind);

#[derive(Debug, Default, Component)]
struct SpawnButtonText;

fn setup_game_ui(mut commands: Commands, unit_points: Res<UnitPoints>) {
    info!("game_ui");
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },)) /* TODO: Another NodeBundle for FlexDirection Row */
        .with_children(|children| {
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: Color::BLUE.into(),
                        ..Default::default()
                    },
                    SpawnButton(UnitKind::Soldier),
                ))
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section(
                            format!("Soldier ({})", unit_points.0),
                            TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ))
                        .insert(SpawnButtonText);
                });

            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: Color::GREEN.into(),
                        ..Default::default()
                    },
                    SpawnCooldownReductionButton,
                ))
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section(
                            format!("Tech cd ({})", 0),
                            TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ))
                        .insert(SpawnCooldownReductionButtonText);
                });
        });
}

fn click_spawn_button(
    mut interaction_query: Query<
        (&Interaction, &SpawnButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut queueallyunit_evw: EventWriter<QueueAllyUnit>,
) {
    for (interaction, spawn_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                queueallyunit_evw.send(QueueAllyUnit {
                    kind: spawn_button.0,
                });
            }
            Interaction::Hovered => { /* TODO; Color shaded */ }
            Interaction::None => { /* TODO; Color normal */ }
        }
    }
}

fn update_spawn_button_text(
    mut query: Query<&mut Text, With<SpawnButtonText>>,
    unit_points: Res<UnitPoints>,
) {
    for mut text in &mut query {
        text.sections[0].value = format!("Soldier ({})", unit_points.0);
    }
}

fn click_spawn_cooldown_reduction_button(
    mut interaction_query: Query<
        (&Interaction, &SpawnCooldownReductionButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut spawn_cooldown_reduction: Query<&mut SpawnCooldownReduction>,
    ally_castle: Res<AllyCastle>,
) {
    for (interaction, _spawn_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(entity) = ally_castle.0 {
                    if let Ok(mut spawn_cooldown_reduction) =
                        spawn_cooldown_reduction.get_mut(entity)
                    {
                        spawn_cooldown_reduction.level += 1;
                    }
                }
            }
            Interaction::Hovered => { /* TODO; Color shaded */ }
            Interaction::None => { /* TODO; Color normal */ }
        }
    }
}

fn update_spawn_cooldown_reduction_button(
    mut query: Query<&mut Text, With<SpawnCooldownReductionButtonText>>,
    spawn_cooldown_reduction: Query<&SpawnCooldownReduction>,
    ally_castle: Res<AllyCastle>,
) {
    for mut text in &mut query {
        if let Some(entity) = ally_castle.0 {
            if let Ok(spawn_cooldown_reduction) = spawn_cooldown_reduction.get(entity) {
                text.sections[0].value = format!("Tech cd ({})", spawn_cooldown_reduction.level);
            }
        }
    }
}

#[derive(Debug, Component)]
struct SpawnCooldownReductionButton;

#[derive(Debug, Default, Component)]
struct SpawnCooldownReductionButtonText;
