use std::f32::consts::PI;

use ball::BallPlugin;
//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{app::AppExit, prelude::*, render::render_resource::ShaderStage};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_rapier3d::prelude::*;
use main_menu::MainMenuPlugin;
use opponent::OpponentPlugin;
use player::PlayerPlugin;
use scoreboard::ScoreboardPlugin;
use walls::WallPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

mod ball;
mod main_menu;
mod opponent;
mod player;
mod scoreboard;
mod walls;
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.6, 0.6, 0.6)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                height: HEIGHT,
                width: WIDTH,
                title: "Pong".to_string(),
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_event::<GoalEvent>()
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(InspectableRapierPlugin)
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(OpponentPlugin)
        .add_plugin(WallPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(ScoreboardPlugin)
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
        .add_startup_system(spawn_camera)
        .add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_basic_scene))
        .add_state(GameState::MainMenu)
        .add_system(quit)
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    Gameplay,
}

pub struct GoalEvent {
    pub side: Side,
}
pub enum Side {
    Player,
    CPU,
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        ball_scene: assets.load("ball.glb#Scene0"),
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 0.0).looking_at(Vec3::ZERO, Vec3::NEG_Z),
        ..default()
    });
}

#[derive(Resource)]
pub struct GameAssets {
    pub ball_scene: Handle<Scene>,
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 2.0,
    });

    const HALF_SIZE: f32 = 20.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-7.0, 0.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.) * Quat::from_rotation_y(0.5),
            ..default()
        },
        ..default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Collider::cuboid(50.0, 0.1, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -0.05, 0.0)))
        .insert(Name::new("Ground"))
        .insert(Restitution::coefficient(1.0));
}

fn quit(keyboard: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
