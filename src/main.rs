use std::f32::consts::PI;

use ball::BallPlugin;
//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{app::AppExit, prelude::*};
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
// Not yet on bevy 0.10
// #[cfg(debug_assertions)]
// use bevy_inspector_egui_rapier::InspectableRapierPlugin;
#[cfg(debug_assertions)]
use bevy_rapier3d::render::RapierDebugRenderPlugin;

use bevy_rapier3d::prelude::{Collider, NoUserData, RapierPhysicsPlugin, Restitution};

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
//extern crate rand;
fn main() {
    let mut binding = App::new();
    let game = binding
        .insert_resource(ClearColor(Color::rgb(0.6, 0.6, 0.6)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WIDTH, HEIGHT).into(),
                title: "Pong".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .add_event::<GoalEvent>()
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(OpponentPlugin)
        .add_plugin(WallPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(ScoreboardPlugin)
        .insert_resource(Msaa::Sample4)
        .add_system(asset_loading.in_base_set(StartupSet::PreStartup))
        .add_startup_system(spawn_camera)
        .add_system(spawn_basic_scene.in_schedule(OnEnter(GameState::Gameplay)))
        .add_system(quit);
    #[cfg(debug_assertions)]
    // InspectableRapierPlugin is not yet upgraded to bevy 0.10
    //game.add_plugin(InspectableRapierPlugin)
    game.add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new());

    game.run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
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
        grass_texture: assets.load("textures/grass.tga"),
        grass_normal: assets.load("textures/grass_normal.tga"),
        wood: assets.load("textures/wood.tga"),
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
    pub grass_texture: Handle<Image>,
    pub grass_normal: Handle<Image>,
    pub wood: Handle<Image>,
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<GameAssets>,
    //mut windows: ResMut<Windows>,
) {
    // let window = windows.get_primary_mut().unwrap();
    // let (win_h, win_w) = (window.width(), window.height());
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 2.0,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
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
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 30.0,
                subdivisions: 1,
            })),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(assets.grass_texture.clone()),
                normal_map_texture: Some(assets.grass_normal.clone()),
                perceptual_roughness: 1.0,
                metallic: 1.0,
                reflectance: 0.0,
                ..Default::default()
            }),
            ..default()
        })
        .insert(Collider::cuboid(50.0, 0.1, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -0.05, 0.0)))
        .insert(Name::new("Ground"))
        .insert(Restitution::coefficient(0.0));
}

fn quit(keyboard: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
