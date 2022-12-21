use crate::*;
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_player))
            .add_system_set(SystemSet::on_update(GameState::Gameplay).with_system(controls));
    }
}
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Player;
fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands
        .spawn(SpatialBundle {
            transform: Transform {
                translation: Vec3::new(-12.5, 0.75, 0.0),
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
        .insert(Name::new("Player"))
        .insert(Player)
        .insert(RigidBody::KinematicPositionBased)
        .insert(Dominance::group(10))
        //.insert(GravityScale(0.0))
        .insert(Collider::cuboid(0.5, 0.4, 0.5))
        .insert(Restitution::coefficient(1.3))
        .insert(KinematicCharacterController::default());
}

fn controls(
    keyboard: Res<Input<KeyCode>>,
    mut controllers: Query<&mut KinematicCharacterController, With<Player>>,
) {
    let mut controller = controllers.single_mut();

    let speed = 0.2;
    if keyboard.pressed(KeyCode::W) {
        controller.translation = Some(Vec3::new(0.0, 0.0, -speed));
    }
    if keyboard.pressed(KeyCode::S) {
        controller.translation = Some(Vec3::new(0.0, 0.0, speed));
    }
}
