use bevy_rapier3d::prelude::{Dominance, KinematicCharacterController, RigidBody};

use crate::{ball::Ball, *};
pub struct OpponentPlugin;
impl Plugin for OpponentPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Opponent>()
            .add_system(spawn_opponent.in_schedule(OnEnter(GameState::Gameplay)))
            .add_system(controls.in_set(OnUpdate(GameState::Gameplay)));
    }
}
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Opponent;
fn spawn_opponent(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<GameAssets>,
) {
    commands
        .spawn(SpatialBundle {
            transform: Transform {
                translation: Vec3::new(12.5, 0.75, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(1.0, 1.5, 3.0),
            },
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(assets.wood.clone()),
                    perceptual_roughness: 1.0,
                    metallic: 1.0,
                    reflectance: 0.0,
                    ..Default::default()
                }),
                ..Default::default()
            });
        })
        .insert(Name::new("Opponent"))
        .insert(Opponent)
        .insert(RigidBody::KinematicPositionBased)
        .insert(Dominance::group(10))
        //.insert(GravityScale(0.0))
        .insert(Collider::cuboid(0.5, 0.4, 0.5))
        .insert(Restitution::coefficient(1.3))
        .insert(KinematicCharacterController::default());
}
fn controls(
    ball_data: Query<&Transform, With<Ball>>,
    mut opponents: Query<(&GlobalTransform, &mut KinematicCharacterController), With<Opponent>>,
    time: Res<Time>,
) {
    let (opponent_transform, mut controller) = opponents.single_mut();
    let speed = 10.0;
    let ball_transform = ball_data.get_single().unwrap();
    if (ball_transform.translation.z - opponent_transform.translation().z) > 1.0 {
        controller.translation = Some(Vec3::new(0.0, 0.0, speed * time.delta_seconds()));
    } else if (ball_transform.translation.z - opponent_transform.translation().z) < 1.0 {
        controller.translation = Some(Vec3::new(0.0, 0.0, -speed * time.delta_seconds()));
    }
}
