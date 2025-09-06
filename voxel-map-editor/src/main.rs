mod map;
mod menu;

use crate::menu::MenuPlugin;
use bevy::prelude::*;
use bevy_ui_text_input::TextInputPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MenuPlugin, TextInputPlugin))
        .init_state::<AppState>()
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_state)
        .run();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    Generating,
    InApp,
}

fn setup_state(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::Menu);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}
