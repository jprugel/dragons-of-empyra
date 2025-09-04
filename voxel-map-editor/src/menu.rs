use crate::AppState;
use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy_builder::BuilderExt;
use bevy_ui_text_input::{TextInputFilter, TextInputMode, TextInputNode};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_systems(OnEnter(AppState::Menu), setup_main_menu)
            .add_systems(
                Update,
                main_menu_system.run_if(in_state(MenuState::MainMenu)),
            )
            .add_systems(
                Update,
                options_menu_system.run_if(in_state(MenuState::Options)),
            )
            .add_systems(OnEnter(MenuState::MainMenu), setup_main_menu)
            .add_systems(OnEnter(MenuState::Options), setup_options_menu)
            .add_systems(OnExit(MenuState::MainMenu), menu_cleanup)
            .add_systems(OnExit(MenuState::Options), menu_cleanup);
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum MenuState {
    #[default]
    MainMenu,
    Settings,
    Options,
}

#[derive(Component)]
struct MenuCanvas;

#[derive(Component)]
struct OptionsMenu;

fn setup_options_menu(mut commands: Commands) {
    let canvas_node = Node::builder()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .flex_direction(FlexDirection::Column)
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .build();

    let x_input_node = Node::builder()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .build();

    let y_input_node = Node::builder()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .build();

    let z_input_node = Node::builder()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .build();

    let x_node = Node::builder()
        .width(Val::Percent(30.0))
        .height(Val::Percent(10.0))
        .margin(UiRect::all(Val::Px(20.0)))
        .border(UiRect::all(Val::Px(10.0)))
        .build();

    let y_node = Node::builder()
        .width(Val::Percent(30.0))
        .height(Val::Percent(10.0))
        .margin(UiRect::all(Val::Px(20.0)))
        .border(UiRect::all(Val::Px(10.0)))
        .build();

    let z_node = Node::builder()
        .width(Val::Percent(30.0))
        .height(Val::Percent(10.0))
        .margin(UiRect::all(Val::Px(20.0)))
        .border(UiRect::all(Val::Px(10.0)))
        .build();

    let back_button = Node::builder()
        .width(Val::Percent(30.0))
        .height(Val::Percent(10.0))
        .margin(UiRect::all(Val::Px(20.0)))
        .border(UiRect::all(Val::Px(10.0)))
        .build();

    let canvas_bundle = (
        canvas_node,
        MenuCanvas,
        BackgroundColor(DARK_SLATE_GRAY.into()),
    );
    let x_input_bundle = (
        x_input_node,
        TextInputNode {
            mode: TextInputMode::SingleLine,
            filter: Some(TextInputFilter::Integer),
            max_chars: Some(5),
            justification: JustifyText::Center,
            ..default()
        },
    );
    let x_bundle = (
        x_node,
        BorderColor(DARK_GRAY.into()),
        BackgroundColor(DARK_CYAN.into()),
    );
    let y_bundle = (
        y_node,
        TextInputNode::default(),
        BorderColor(DARK_GRAY.into()),
        BackgroundColor(DARK_CYAN.into()),
    );
    let z_bundle = (
        z_node,
        TextInputNode::default(),
        BorderColor(DARK_GRAY.into()),
        BackgroundColor(DARK_CYAN.into()),
    );
    let back_button_bundle = (
        back_button,
        OptionsMenu,
        Button,
        BackgroundColor(DARK_CYAN.into()),
        Text::new("Back"),
    );

    let canvas = commands.spawn(canvas_bundle).id();
    let x_input = commands.spawn(x_bundle).insert(ChildOf(canvas)).id();
    let _x_input = commands.spawn(x_input_bundle).insert(ChildOf(x_input));
    let _y_input = commands.spawn(y_bundle).insert(ChildOf(canvas));
    let _z_input = commands.spawn(z_bundle).insert(ChildOf(canvas));
    let _back_button = commands.spawn(back_button_bundle).insert(ChildOf(canvas));
}

#[derive(Component)]
struct StartButton;

fn setup_main_menu(mut commands: Commands) {
    let canvas_node = Node::builder()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .flex_direction(FlexDirection::Column)
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .build();

    let start_button = Node::builder()
        .width(Val::Percent(30.0))
        .height(Val::Percent(10.0))
        .border(UiRect::all(Val::Px(2.0)))
        .margin(UiRect::all(Val::Px(20.0)))
        .build();

    let settings_button = Node::builder()
        .width(Val::Percent(30.0))
        .height(Val::Percent(10.0))
        .border(UiRect::all(Val::Px(2.0)))
        .margin(UiRect::all(Val::Px(20.0)))
        .build();

    let canvas_bundle = (canvas_node, BackgroundColor(DARK_SLATE_GRAY.into()));
    let start_button_bundle = (
        start_button,
        StartButton,
        Text::new("Start"),
        Button,
        BackgroundColor(DARK_CYAN.into()),
        BorderColor(DARK_GRAY.into()),
        BorderRadius::all(Val::Px(2.0)),
    );
    let settings_button_bundle = (
        settings_button,
        Text::new("Settings"),
        Button,
        BackgroundColor(DARK_CYAN.into()),
        BorderColor(DARK_GRAY.into()),
    );

    let canvas = commands.spawn(canvas_bundle).id();
    let _start_button = commands
        .spawn(start_button_bundle)
        .insert(ChildOf(canvas))
        .id();
    let _settings_button = commands
        .spawn(settings_button_bundle)
        .insert(ChildOf(canvas))
        .id();
}

fn main_menu_system(
    mut menu_state: ResMut<NextState<MenuState>>,
    interactions: Query<&mut Interaction, With<StartButton>>,
) {
    for interaction in &interactions {
        match *interaction {
            Interaction::Pressed => {
                menu_state.set(MenuState::Options);
            }
            _ => {}
        }
    }
}

fn options_menu_system(
    mut menu_state: ResMut<NextState<MenuState>>,
    interactions: Query<&mut Interaction, With<OptionsMenu>>,
) {
    for interaction in &interactions {
        match *interaction {
            Interaction::Pressed => {
                menu_state.set(MenuState::MainMenu);
            }
            _ => {}
        }
    }
}

fn menu_cleanup(menu: Query<Entity, With<MenuCanvas>>, mut commands: Commands) {
    for entity in &menu {
        commands.entity(entity).despawn();
    }
}
