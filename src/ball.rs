use bevy_rapier3d::prelude::{GravityScale, RigidBody, Velocity};
use rand::random;

use crate::*;
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Ball;
pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ball>()
            .add_system(spawn_ball.in_schedule(OnEnter(GameState::Gameplay)))
            .add_system(ball_velocity.in_set(OnUpdate(GameState::Gameplay)))
            .add_system(detect_goal.in_set(OnUpdate(GameState::Gameplay)));
    }
}

fn spawn_ball(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn(SpatialBundle {
            transform: Transform::from_xyz(0.0, 1.2, 0.0),
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: assets.ball_scene.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, -1.13, 0.0),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::ONE,
                },
                ..default()
            });
        })
        .insert(Name::new("Ball"))
        .insert(Ball)
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(1.0))
        .insert(Collider::ball(1.15))
        .insert(Velocity::linear(Vec3::new(-10.0, 0.0, 0.0)))
        .insert(Restitution::coefficient(1.3));
}

fn ball_velocity(mut ball: Query<&mut Velocity, With<Ball>>) {
    let mut tr = ball.single_mut();
    tr.linvel = adjust_vector(tr.linvel);
}

fn detect_goal(
    mut ball: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    mut goal_event_writer: EventWriter<crate::GoalEvent>,
) {
    let (mut position, mut velocity) = ball.single_mut();
    let x = position.translation.x;
    if !(-15.0..=15.0).contains(&x) {
        if x > 15.0 {
            goal_event_writer.send(GoalEvent { side: Side::Player });
        } else if x < -15.0 {
            goal_event_writer.send(GoalEvent { side: Side::CPU });
        }
        position.translation.x = 0.0;
        position.translation.y = 1.2;

        velocity.linvel.y = 0.0;
        velocity.linvel.x = random_number_in_range(-10.0, 10.0);
        velocity.linvel.z = random_number_in_range(-10.0, 10.0);
        velocity.angvel = Vec3::ZERO;
    }
}
fn random_number_in_range(min: f32, max: f32) -> f32 {
    random::<f32>() * (max - min) + min
}

fn adjust_vector(mut vector: Vec3) -> Vec3 {
    if vector.x > 0.0 && vector.x < 10.0 {
        vector.x = 10.0;
    } else if vector.x < 0.0 && vector.x > -10.0 {
        vector.x = -10.0;
    }
    vector
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    #[test]
    fn test_random_number_in_range() {
        for _ in 0..10000 {
            let number = super::random_number_in_range(2.0, 4.0);
            assert_eq!(number >= 2.0, true);
            assert_eq!(number <= 4.0, true);
        }
    }
}
