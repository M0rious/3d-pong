use bevy::pbr::NotShadowCaster;

use crate::*;
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Wall;
pub struct WallPlugin;
impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Wall>()
            .add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_walls))
            //.add_system(controls)
            ;
    }
}

fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, -8.0),
            ..default()
        })
        .with_children(|commands| {
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box {
                        min_x: -15.0,
                        max_x: 15.0,
                        min_y: 0.0,
                        max_y: 5.0,
                        min_z: 0.0,
                        max_z: 0.1,
                    })),
                    material: materials.add(Color::rgba(254.0, 0.0, 0.0, 255.0).into()),
                    ..default()
                })
                .insert(Name::new("Wall_Top"))
                .insert(Collider::cuboid(20.0, 10.0, 0.1))
                .insert(Restitution::coefficient(1.1))
                .insert(NotShadowCaster);
        });

    commands
        .spawn(SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, 8.0),
            ..default()
        })
        .with_children(|commands| {
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box {
                        min_x: -15.0,
                        max_x: 15.0,
                        min_y: 0.0,
                        max_y: 5.0,
                        min_z: 0.0,
                        max_z: 0.1,
                    })),
                    material: materials.add(Color::rgba(254.0, 0.0, 0.0, 255.0).into()),
                    ..default()
                })
                .insert(Name::new("Wall_Bottom"))
                .insert(Collider::cuboid(20.0, 10.0, 0.1))
                .insert(Restitution::coefficient(1.0))
                .insert(NotShadowCaster);
        });
}
