use crate::{
    castle::{
        spawner::Wave, upgrade::SpawnCooldownReduction, AllyCastle, Castle, EnemyCastle, Gold,
        QueueAllyUnit, SpawnQueue,
    },
    common::{attributes::Health, Faction},
    loading::UiAssets,
    units::{
        upgrade::{AttackCooldownUpgrade, ShieldUpgrade},
        UnitKind,
    },
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
            (setup_game_ui, setup_resource_ui, setup_health_ui),
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
                update_soldier_shield_button,
                update_soldier_attackspeed_button,
                click_soldier_shield_button,
                click_soldier_attackspeed_button,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Debug, Component)]
struct SpawnButton(pub UnitKind);

#[derive(Debug, Default, Component)]
struct SpawnButtonText;

fn setup_game_ui(mut commands: Commands, ui_assets: Res<UiAssets>) {
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
                            format!("{}", 0),
                            TextStyle {
                                font_size: 30.0,
                                color: Color::rgb(0.0, 0.0, 0.0),
                                ..default()
                            },
                        ))
                        .insert(SpawnButtonText);
                    parent.spawn(TextBundle::from_section(
                        format!("{}", UnitKind::Soldier.cost()),
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
                                color: Color::rgb(0.0, 0.0, 0.0),
                                ..default()
                            },
                        ))
                        .insert(SpawnCooldownReductionButtonLevelText);
                    parent
                        .spawn(TextBundle::from_section(
                            format!("{}", 0),
                            TextStyle {
                                font_size: 30.0,
                                color: Color::rgb(0.9, 0.9, 0.0),
                                ..default()
                            },
                        ))
                        .insert(SpawnCooldownReductionButtonCostText);
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
                        image: ui_assets.soldier_shield.clone().into(),
                        ..Default::default()
                    },
                    SoldierShieldButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            format!("{}", 0),
                            TextStyle {
                                font_size: 30.0,
                                color: Color::rgb(0.0, 0.0, 0.0),
                                ..default()
                            },
                        ),
                        SoldierShieldButtonLevelText,
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            format!("{}", 0),
                            TextStyle {
                                font_size: 30.0,
                                color: Color::rgb(0.9, 0.9, 0.0),
                                ..default()
                            },
                        ),
                        SoldierShieldButtonCostText,
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
                        image: ui_assets.soldier_attackspeed.clone().into(),
                        ..Default::default()
                    },
                    SoldierAttackspeedButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            format!("{}", 0),
                            TextStyle {
                                font_size: 30.0,
                                color: Color::rgb(0.0, 0.0, 0.0),
                                ..default()
                            },
                        ),
                        SoldierAttackspeedButtonLevelText,
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            format!("{}", 0),
                            TextStyle {
                                font_size: 30.0,
                                color: Color::rgb(0.9, 0.9, 0.0),
                                ..default()
                            },
                        ),
                        SoldierAttackspeedButtonCostText,
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

fn update_spawn_button_text(
    mut query: Query<&mut Text, With<SpawnButtonText>>,
    ally_castle: Res<AllyCastle>,
    spawn_queues: Query<&SpawnQueue, With<Castle>>,
) {
    if let Some(entity) = ally_castle.0 {
        if let Ok(spawn_queue) = spawn_queues.get(entity) {
            for mut text in &mut query {
                let value = spawn_queue
                    .units
                    .iter()
                    .filter(|x| matches!(x, UnitKind::Soldier))
                    .count();
                text.sections[0].value = format!("{}", value);
            }
        }
    }
}

fn click_spawn_cooldown_reduction_button(
    mut interaction_query: Query<
        (&Interaction, &SpawnCooldownReductionButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut spawn_cooldown_reduction: Query<&mut SpawnCooldownReduction>,
    ally_castle: Res<AllyCastle>,
    mut gold: ResMut<Gold>,
) {
    for (interaction, _spawn_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(entity) = ally_castle.0 {
                    if let Ok(mut spawn_cooldown_reduction) =
                        spawn_cooldown_reduction.get_mut(entity)
                    {
                        let cost = spawn_cooldown_reduction.cost();
                        if gold.0 >= cost && spawn_cooldown_reduction.level_up() {
                            gold.0 -= cost;
                        }
                    }
                }
            }
            Interaction::Hovered => { /* TODO; Color shaded */ }
            Interaction::None => { /* TODO; Color normal */ }
        }
    }
}

fn update_spawn_cooldown_reduction_button(
    mut leveltext: Query<
        &mut Text,
        (
            With<SpawnCooldownReductionButtonLevelText>,
            Without<SpawnCooldownReductionButtonCostText>,
        ),
    >,
    mut costtext: Query<
        &mut Text,
        (
            With<SpawnCooldownReductionButtonCostText>,
            Without<SpawnCooldownReductionButtonLevelText>,
        ),
    >,
    spawn_cooldown_reduction: Query<&SpawnCooldownReduction>,
    ally_castle: Res<AllyCastle>,
) {
    if let Some(entity) = ally_castle.0 {
        if let Ok(spawn_cooldown_reduction) = spawn_cooldown_reduction.get(entity) {
            for mut text in &mut leveltext {
                text.sections[0].value = format!("{}", spawn_cooldown_reduction.level());
            }

            for mut text in &mut costtext {
                text.sections[0].value = format!("{}", spawn_cooldown_reduction.cost());
            }
        }
    }
}

#[derive(Debug, Component)]
struct SpawnCooldownReductionButton;

#[derive(Debug, Default, Component)]
struct SpawnCooldownReductionButtonLevelText;

#[derive(Debug, Default, Component)]
struct SpawnCooldownReductionButtonCostText;

/* Gold, Wave */
fn setup_resource_ui(mut commands: Commands, gold: Res<Gold>, wave: Res<Wave>) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(98.0),
                height: Val::Percent(5.0),
                left: Val::Percent(1.0),
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

/* Castle Health */
fn setup_health_ui(mut commands: Commands) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(98.0),
                height: Val::Percent(5.0),
                left: Val::Percent(1.0),
                top: Val::Percent(95.0),
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
                    format!("{}", 0),
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.0, 0.9),
                        ..default()
                    },
                ),
                CastleHealthUi(Faction::Ally),
            ));
            children.spawn((
                TextBundle::from_section(
                    format!("{}", 0),
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.0, 0.9),
                        ..default()
                    },
                ),
                CastleHealthUi(Faction::Enemy),
            ));
        });
}

#[derive(Debug, Default, Component)]
struct GoldUi;

#[derive(Debug, Default, Component)]
struct WaveUi;

#[derive(Debug, Component)]
struct CastleHealthUi(Faction);

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
    mut castle_health_uis: Query<(&mut Text, &CastleHealthUi)>,
    ally_castle: Res<AllyCastle>,
    enemy_castle: Res<EnemyCastle>,
    castle_healths: Query<&Health, With<Castle>>,
) {
    for (mut text, castle_health_ui) in &mut castle_health_uis {
        if let Some(entity) = match castle_health_ui.0 {
            Faction::Ally => ally_castle.0,
            Faction::Enemy => enemy_castle.0,
        } {
            if let Ok(health) = castle_healths.get(entity) {
                text.sections[0].value = format!("{}", health);
            }
        }
    }
}

#[derive(Debug, Component)]
struct SoldierShieldButton;

#[derive(Debug, Default, Component)]
struct SoldierShieldButtonLevelText;

#[derive(Debug, Default, Component)]
struct SoldierShieldButtonCostText;

fn click_soldier_shield_button(
    mut interaction_query: Query<
        (&Interaction, &SoldierShieldButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut soldier_shields: Query<&mut ShieldUpgrade, With<Castle>>,
    ally_castle: Res<AllyCastle>,
    mut gold: ResMut<Gold>,
) {
    for (interaction, _spawn_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(entity) = ally_castle.0 {
                    if let Ok(mut soldier_shield) = soldier_shields.get_mut(entity) {
                        let cost = soldier_shield.cost();
                        if gold.0 >= cost && soldier_shield.level_up() {
                            gold.0 -= cost;
                        }
                    }
                }
            }
            Interaction::Hovered => { /* TODO; Color shaded */ }
            Interaction::None => { /* TODO; Color normal */ }
        }
    }
}

fn update_soldier_shield_button(
    mut leveltext: Query<
        &mut Text,
        (
            With<SoldierShieldButtonLevelText>,
            Without<SoldierShieldButtonCostText>,
        ),
    >,
    mut costtext: Query<
        &mut Text,
        (
            With<SoldierShieldButtonCostText>,
            Without<SoldierShieldButtonLevelText>,
        ),
    >,
    soldier_shields: Query<&ShieldUpgrade, With<Castle>>,
    ally_castle: Res<AllyCastle>,
) {
    if let Some(entity) = ally_castle.0 {
        if let Ok(soldier_shield) = soldier_shields.get(entity) {
            for mut text in &mut leveltext {
                text.sections[0].value = format!("{}", soldier_shield.level());
            }

            for mut text in &mut costtext {
                text.sections[0].value = format!("{}", soldier_shield.cost());
            }
        }
    }
}

#[derive(Debug, Component)]
struct SoldierAttackspeedButton;

#[derive(Debug, Default, Component)]
struct SoldierAttackspeedButtonLevelText;

#[derive(Debug, Default, Component)]
struct SoldierAttackspeedButtonCostText;

fn click_soldier_attackspeed_button(
    mut interaction_query: Query<
        (&Interaction, &SoldierAttackspeedButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut soldier_attackspeeds: Query<&mut AttackCooldownUpgrade, With<Castle>>,
    ally_castle: Res<AllyCastle>,
    mut gold: ResMut<Gold>,
) {
    for (interaction, _spawn_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(entity) = ally_castle.0 {
                    if let Ok(mut soldier_attackspeed) = soldier_attackspeeds.get_mut(entity) {
                        let cost = soldier_attackspeed.cost();
                        if gold.0 >= cost && soldier_attackspeed.level_up() {
                            gold.0 -= cost;
                        }
                    }
                }
            }
            Interaction::Hovered => { /* TODO; Color shaded */ }
            Interaction::None => { /* TODO; Color normal */ }
        }
    }
}

fn update_soldier_attackspeed_button(
    mut leveltext: Query<
        &mut Text,
        (
            With<SoldierAttackspeedButtonLevelText>,
            Without<SoldierAttackspeedButtonCostText>,
        ),
    >,
    mut costtext: Query<
        &mut Text,
        (
            With<SoldierAttackspeedButtonCostText>,
            Without<SoldierAttackspeedButtonLevelText>,
        ),
    >,
    soldier_attackspeeds: Query<&AttackCooldownUpgrade, With<Castle>>,
    ally_castle: Res<AllyCastle>,
) {
    if let Some(entity) = ally_castle.0 {
        if let Ok(soldier_attackspeed) = soldier_attackspeeds.get(entity) {
            for mut text in &mut leveltext {
                text.sections[0].value = format!("{}", soldier_attackspeed.level());
            }

            for mut text in &mut costtext {
                text.sections[0].value = format!("{}", soldier_attackspeed.cost());
            }
        }
    }
}
