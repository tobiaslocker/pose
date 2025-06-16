use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use bevy_tokio_tasks::TokioTasksPlugin;
use bevy_tokio_tasks::TokioTasksRuntime;
use pose::character::Player;
use pose::realm::Choreography;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Playing,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AudioPlugin)
        .add_plugins(TokioTasksPlugin::default())
        .insert_state(GameState::Playing)
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Playing), Choreography::start)
        .add_systems(Update, Choreography::update)
        .add_systems(Update, Player::update)
        .run();
}

fn setup(mut commands: Commands, runtime: Res<TokioTasksRuntime>) {
    commands.spawn(Camera2d::default());
    let player = Player::new(&mut commands, &runtime);
    commands.insert_resource(player);
}
