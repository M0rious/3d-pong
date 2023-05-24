use crate::*;

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_scoreboard.in_schedule(OnEnter(GameState::Gameplay)))
            .add_system(goal_handler.in_set(OnUpdate(GameState::Gameplay)));
    }
}

#[derive(Component)]
struct Points {
    pub player: i32,
    pub cpu: i32,
}

#[derive(Component)]
struct PointsUI;

fn spawn_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Points { cpu: 0, player: 0 });
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        })
        .with_children(|commands| {
            commands
                .spawn(TextBundle {
                    style: Style {
                        margin: UiRect::all(Val::Percent(3.0)),

                        ..default()
                    },
                    text: Text::from_section(
                        format!("Player {}  -   CPU {}", 0, 0),
                        TextStyle {
                            font: asset_server.load("FiraSans-Bold.ttf"),
                            font_size: 96.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                })
                .insert(PointsUI);
        });
}
fn goal_handler(
    mut points: Query<&mut Points>,
    mut goal_event_reader: EventReader<crate::GoalEvent>,
    mut text: Query<&mut Text, With<PointsUI>>,
) {
    let mut points = points.single_mut();
    for x in goal_event_reader.iter() {
        match x.side {
            Side::Player => points.player += 1,
            Side::CPU => points.cpu += 1,
        }
    }
    let mut text = text.single_mut();
    *text = Text::from_section(
        format!("Player {}  -   CPU {}", points.player, points.cpu),
        text.sections[0].style.clone(),
    );
}
