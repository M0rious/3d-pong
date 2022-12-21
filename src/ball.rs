use rand::Rng;

use crate::*;
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Ball;
pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ball>()
            .add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_ball))
            .add_system_set(SystemSet::on_update(GameState::Gameplay).with_system(ball_velocity))
            .add_system_set(SystemSet::on_update(GameState::Gameplay).with_system(detect_goal));
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
        .insert(Restitution::coefficient(1.3))
        //.insert(LockedAxes::TRANSLATION_LOCKED_Y)
        ;
}

fn ball_velocity(mut ball: Query<&mut Velocity, With<Ball>>) {
    let mut tr = ball.single_mut();
    //if tr.linvel.length() > 0.0 {
    //  let speed = 10.0;
    tr.linvel = adjust_vector(tr.linvel);
    //}
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

        let mut rng = rand::thread_rng();

        velocity.linvel.y = 0.0;
        velocity.linvel.x = rng.gen_range(-10.0..10.0);
        velocity.linvel.z = rng.gen_range(-10.0..10.0);
    }
}

fn adjust_vector(mut vector: Vec3) -> Vec3 {
    //vector = vector.normalize() * x;
    if vector.x > 0.0 && vector.x < 10.0 {
        vector.x = 10.0;
    } else if vector.x < 0.0 && vector.x > -10.0 {
        vector.x = -10.0;
    }
    //vector.y = 0.0;
    vector
}
