use crate::*;
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_startup_system(spawn_player)
            .add_system(controls);
    }
}
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Player;
fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform {
                translation: Vec3::new(-12.5, 0.0, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(1.0, 3.0, 3.0),
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player)
        .insert(RigidBody::Fixed)
        .insert(GravityScale(0.0))
        .insert(Collider::cuboid(0.7, 0.7, 0.7));
}

fn controls(
    keyboard: Res<Input<KeyCode>>,
    mut player: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    let (_, mut tr) = player.single_mut();

    let speed = 8.0;
    if keyboard.pressed(KeyCode::W) {
        tr.translation.z -= speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        tr.translation.z += speed * time.delta_seconds();
    }
}
