use crate::*;

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_scoreboard))
            .add_system_set(SystemSet::on_update(GameState::Gameplay).with_system(goal_handler));
    }
}

fn spawn_scoreboard() {}
fn goal_handler(mut goal_event_reader: EventReader<crate::GoalEvent>) {
    for x in goal_event_reader.iter() {
        match x.side {
            Side::Player => info!("Player Goal!"),
            Side::CPU => info!("CPU Goal!"),
        }
    }
}
