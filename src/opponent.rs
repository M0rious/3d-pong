use crate::*;
pub struct OpponentPlugin;
impl Plugin for OpponentPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Opponent>()
            .add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_opponent))
            .add_system_set(SystemSet::on_update(GameState::Gameplay).with_system(controls));
    }
}
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Opponent;
fn spawn_opponent(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
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
    keyboard: Res<Input<KeyCode>>,
    mut controllers: Query<&mut KinematicCharacterController, With<Opponent>>,
) {
    let mut controller = controllers.single_mut();

    let speed = 0.2;
    if keyboard.pressed(KeyCode::Up) {
        controller.translation = Some(Vec3::new(0.0, 0.0, -speed));
    }
    if keyboard.pressed(KeyCode::Down) {
        controller.translation = Some(Vec3::new(0.0, 0.0, speed));
    }
}
