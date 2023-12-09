use crate::{
    castle::{
        spawner::Wave, upgrade::SpawnCooldownReduction, AllyCastle, Castle, Gold, QueueAllyUnit,
    },
    common::attributes::Health,
    loading::UiAssets,
    units::UnitKind,
    GameState,
};
use bevy::prelude::*;

pub struct GameUiPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (setup_game_ui, setup_resource_ui),
        )
        .add_systems(
            Update,
            (
                click_spawn_button,
                update_spawn_button_text,
                click_spawn_cooldown_reduction_button,
                update_spawn_cooldown_reduction_button,
                update_gold_ui,
                update_wave_ui,
                update_castle_health_ui,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Debug, Component)]
struct SpawnButton(pub UnitKind);

#[derive(Debug, Default, Component)]
struct SpawnButtonText;

fn setup_game_ui(mut commands: Commands, gold: Res<Gold>, ui_assets: Res<UiAssets>) {
    info!("game_ui");
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(20.0),
                height: Val::Percent(100.0),
                left: Val::Percent(1.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        },)) /* TODO: Another NodeBundle for FlexDirection Row */
        .with_children(|children| {
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(128.0),
                            height: Val::Px(128.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::FlexStart,
                            ..Default::default()
                        },
                        image: ui_assets.soldier_button.clone().into(),
                        ..Default::default()
                    },
                    SpawnButton(UnitKind::Soldier),
                ))
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section(
                            format!("{}", gold.0),
                            TextStyle {
                                font_size: 30.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ))
                        .insert(SpawnButtonText);
                    parent.spawn(TextBundle::from_section(
                        format!("{}", 1),
                        TextStyle {
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.0),
                            ..default()
                        },
                    ));
                });

            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(128.0),
                            height: Val::Px(128.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::FlexStart,
                            ..Default::default()
                        },
                        image: ui_assets.tech_castle_button.clone().into(),
                        ..Default::default()
                    },
                    SpawnCooldownReductionButton,
                ))
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section(
                            format!("{}", 0),
                            TextStyle {
                                font_size: 30.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ))
                        .insert(SpawnCooldownReductionButtonText);
                    parent.spawn(TextBundle::from_section(
                        format!("{}", 5),
                        TextStyle {
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.0),
                            ..default()
                        },
                    ));
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

fn update_spawn_button_text(mut query: Query<&mut Text, With<SpawnButtonText>>, gold: Res<Gold>) {
    for mut text in &mut query {
        text.sections[0].value = format!("{}", gold.0);
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
                text.sections[0].value = format!("{}", spawn_cooldown_reduction.level);
            }
        }
    }
}

#[derive(Debug, Component)]
struct SpawnCooldownReductionButton;

#[derive(Debug, Default, Component)]
struct SpawnCooldownReductionButtonText;

/* Castle Health, Gold, Wave */
fn setup_resource_ui(mut commands: Commands, gold: Res<Gold>, wave: Res<Wave>) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        },))
        .with_children(|children| {
            children.spawn((
                TextBundle::from_section(
                    format!("{}", gold.0),
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.0),
                        ..default()
                    },
                ),
                GoldUi,
            ));
            children.spawn((
                TextBundle::from_section(
                    format!("CastleHealth:{}", 0),
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.0, 0.9),
                        ..default()
                    },
                ),
                CastleHealthUi,
            ));
            children.spawn((
                TextBundle::from_section(
                    format!("Wave:{}", wave.level),
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.0, 0.9, 0.9),
                        ..default()
                    },
                ),
                WaveUi,
            ));
        });
}

#[derive(Debug, Default, Component)]
struct GoldUi;

#[derive(Debug, Default, Component)]
struct WaveUi;

#[derive(Debug, Default, Component)]
struct CastleHealthUi;

fn update_gold_ui(mut gold_uis: Query<&mut Text, With<GoldUi>>, gold: Res<Gold>) {
    for mut text in &mut gold_uis {
        text.sections[0].value = format!("{}", gold.0);
    }
}

fn update_wave_ui(mut wave_uis: Query<&mut Text, With<WaveUi>>, wave: Res<Wave>) {
    for mut text in &mut wave_uis {
        text.sections[0].value = format!("Wave:{}", wave.level);
    }
}

fn update_castle_health_ui(
    mut castle_health_uis: Query<&mut Text, With<CastleHealthUi>>,
    ally_castle: Res<AllyCastle>,
    castle_healths: Query<&Health, With<Castle>>,
) {
    if let Some(entity) = ally_castle.0 {
        if let Ok(health) = castle_healths.get(entity) {
            for mut text in &mut castle_health_uis {
                text.sections[0].value = format!("{}", health);
            }
        }
    }
}
