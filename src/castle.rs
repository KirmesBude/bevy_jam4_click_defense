use crate::attributes::{Health, Immortal};
use crate::hit_detection::HurtBoxBundle;
use crate::loading::TextureAssets;
use crate::physics::PhysicsCollisionBundle;
use crate::units::Faction;
use crate::GameState;
use bevy::prelude::*;
use bevy_xpbd_2d::components::{Collider, CollisionLayers, RigidBody};

pub struct CastlePlugin;

#[derive(Component)]
pub struct Castle;

#[derive(Component)]
pub struct MainCastle;

#[derive(Component)]
pub struct MainCastleHealthUI;

/// This plugin handles castle related stuff like health ui
/// Castle logic is only active during the State `GameState::Playing`
impl Plugin for CastlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (spawn_castle, spawn_health_ui))
            .add_systems(Update, (update_health).run_if(in_state(GameState::Playing)));
    }
}

fn spawn_castle(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(256.0, 256.0)),
                ..Default::default()
            },
            texture: textures.bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert((Castle, MainCastle))
        .insert(Immortal)
        .insert(Health::new(1000.0))
        .insert(PhysicsCollisionBundle {
            rigid_body: RigidBody::Static,
            collider: Collider::ball(128.0),
            ..Default::default()
        })
        .with_children(|children| {
            children.spawn(HurtBoxBundle {
                collider: Collider::ball(127.0),
                collisionlayers: CollisionLayers::new(
                    [Faction::Ally.hurt_layer()],
                    [Faction::Ally.opposite().hit_layer()],
                ),
                ..Default::default()
            });
        });
}

fn spawn_health_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "CastleHealth",
            TextStyle {
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }),
        MainCastleHealthUI,
    ));
}

fn update_health(
    castle_health: Query<&Health, (With<Castle>, With<MainCastle>)>,
    mut castle_health_ui: Query<&mut Text, With<MainCastleHealthUI>>,
) {
    let castle_health = castle_health.single();
    let mut castle_health_ui = castle_health_ui.single_mut();

    castle_health_ui.sections[0].value = format!("{}", castle_health);
}
