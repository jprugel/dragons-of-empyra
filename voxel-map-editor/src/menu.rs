use crate::AppState;
use crate::map::GenerateMapEvent;
use crate::map::Map;
use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy_builder::BuilderExt;
use bevy_ui_text_input::actions::TextInputAction;
use bevy_ui_text_input::{
    TextInputFilter, TextInputMode, TextInputNode, TextInputQueue, TextSubmitEvent,
};
use std::collections::{HashMap, hash_map::Values};

#[derive(Debug)]
enum Dimension {
    Length(u32),
    Width(u32),
    Height(u32),
}
#[derive(Resource, Debug, Default)]
struct InputMap(HashMap<Entity, Dimension>);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_systems(OnEnter(AppState::Menu), setup_main_menu)
            .init_resource::<Dimensions>()
            .init_resource::<InputMap>()
            .add_event::<GenerateMapEvent>()
            .add_event::<SubmitDimensionsEvent>()
            .add_systems(
                Update,
                main_menu_system.run_if(in_state(MenuState::MainMenu)),
            )
            .add_systems(
                Update,
                back_button_system.run_if(in_state(MenuState::Options)),
            )
            .add_systems(OnEnter(MenuState::MainMenu), setup_main_menu)
            .add_systems(OnEnter(MenuState::Options), setup_options_menu)
            .add_systems(OnExit(MenuState::MainMenu), menu_cleanup)
            //.add_systems(Update, debug_dimensions)
            .add_systems(Update, submit_button_system)
            .add_systems(Update, on_submit_button_system)
            .add_systems(Update, on_submit_dimensions)
            .add_systems(OnExit(AppState::Menu), menu_cleanup)
            .add_systems(OnExit(MenuState::Options), menu_cleanup);
    }
}

fn debug_dimensions(mut dimensions: ResMut<InputMap>) {
    info!("{:?}", dimensions);
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum MenuState {
    #[default]
    MainMenu,
    Settings,
    Options,
}

#[derive(Resource, Debug, Default)]
struct Dimensions {
    width: u32,
    length: u32,
    height: u32,
}

#[derive(Component)]
struct SubmitDimensions;

#[derive(Component)]
struct MenuCanvas;

#[derive(Component)]
struct OptionsMenu;

#[derive(Component)]
struct WidthInput;

#[derive(Component)]
struct LengthInput;

#[derive(Component)]
struct HeightInput;

const MARGIN: UiRect = UiRect::all(Val::Px(10.0));
const BORDER: UiRect = UiRect::all(Val::Px(5.0));

fn setup_options_menu(mut commands: Commands, mut input_map: ResMut<InputMap>) {
    let canvas_node = Node::builder()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .flex_direction(FlexDirection::Column)
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .build();

    let label_node = Node::builder()
        .width(Val::Percent(30.0))
        .height(Val::Percent(100.0))
        .build();

    let input_node = Node::builder()
        .width(Val::Percent(70.0))
        .height(Val::Percent(100.0))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .build();

    let dimension_node = Node::builder()
        .width(Val::Percent(30.0))
        .height(Val::Percent(10.0))
        .margin(MARGIN)
        .border(BORDER)
        .build();

    let button_node = Node::builder()
        .width(Val::Percent(30.0))
        .height(Val::Percent(10.0))
        .margin(MARGIN)
        .border(BORDER)
        .build();

    let canvas_bundle = (
        canvas_node,
        MenuCanvas,
        BackgroundColor(DARK_SLATE_GRAY.into()),
    );

    let width_input_bundle = (
        input_node.clone(),
        WidthInput,
        TextInputNode {
            clear_on_submit: false,
            mode: TextInputMode::SingleLine,
            filter: Some(TextInputFilter::Integer),
            max_chars: Some(5),
            justification: JustifyText::Center,
            ..default()
        },
        BackgroundColor(RED.into()),
    );
    let length_input_bundle = (
        input_node.clone(),
        LengthInput,
        TextInputNode {
            mode: TextInputMode::SingleLine,
            clear_on_submit: false,
            filter: Some(TextInputFilter::Integer),
            max_chars: Some(5),
            justification: JustifyText::Center,
            ..default()
        },
        BackgroundColor(RED.into()),
    );
    let height_input_bundle = (
        input_node,
        HeightInput,
        TextInputNode {
            mode: TextInputMode::SingleLine,
            clear_on_submit: false,
            filter: Some(TextInputFilter::Integer),
            max_chars: Some(5),
            justification: JustifyText::Center,
            ..default()
        },
        BackgroundColor(RED.into()),
    );
    let label_bundle = (
        label_node,
        TextFont {
            font_size: 32.,
            ..default()
        },
    );
    let dimension_bundle = (
        dimension_node.clone(),
        BorderColor(DARK_GRAY.into()),
        BackgroundColor(DARK_CYAN.into()),
    );
    let submit_button_bundle = (
        button_node.clone(),
        Button,
        SubmitDimensions,
        BackgroundColor(DARK_CYAN.into()),
        Text::new("Submit"),
    );
    let back_button_bundle = (
        button_node,
        OptionsMenu,
        Button,
        BackgroundColor(DARK_CYAN.into()),
        Text::new("Back"),
    );

    let canvas = commands.spawn(canvas_bundle).id();
    let width = commands
        .spawn(dimension_bundle.clone())
        .insert(ChildOf(canvas))
        .id();
    let _width_label = commands
        .spawn(label_bundle.clone())
        .insert((ChildOf(width), Text::new("Width")));
    let width_input = commands
        .spawn(width_input_bundle)
        .insert(ChildOf(width))
        .id();
    let length = commands
        .spawn(dimension_bundle.clone())
        .insert(ChildOf(canvas))
        .id();
    let _length_label = commands
        .spawn(label_bundle.clone())
        .insert((ChildOf(length), Text::new("Length")));
    let length_input = commands
        .spawn(length_input_bundle)
        .insert(ChildOf(length))
        .id();

    let height = commands
        .spawn(dimension_bundle.clone())
        .insert(ChildOf(canvas))
        .id();
    let _height_label = commands
        .spawn(label_bundle)
        .insert((ChildOf(height), Text::new("Height")));
    let height_input = commands
        .spawn(height_input_bundle)
        .insert(ChildOf(height))
        .id();
    let _submit_button = commands.spawn(submit_button_bundle).insert(ChildOf(canvas));
    let _back_button = commands.spawn(back_button_bundle).insert(ChildOf(canvas));
    input_map.0.insert(width_input, Dimension::Width(0));
    input_map.0.insert(length_input, Dimension::Length(0));
    input_map.0.insert(height_input, Dimension::Height(0));
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
        .border(BORDER)
        .margin(MARGIN)
        .build();

    let settings_button = Node::builder()
        .width(Val::Percent(30.0))
        .height(Val::Percent(10.0))
        .border(BORDER)
        .margin(MARGIN)
        .build();

    let canvas_bundle = (
        canvas_node,
        MenuCanvas,
        BackgroundColor(DARK_SLATE_GRAY.into()),
    );
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

fn submit_button_system(
    interactions: Query<&mut Interaction, With<SubmitDimensions>>,
    mut queue: Query<&mut TextInputQueue>,
) {
    for interaction in &interactions {
        match *interaction {
            Interaction::Pressed => {
                for mut item in queue.iter_mut() {
                    item.add(TextInputAction::Submit);
                }
            }
            _ => {}
        }
    }
}

#[derive(Event)]
struct SubmitDimensionsEvent;

fn on_submit_button_system(
    mut text_submit: EventReader<TextSubmitEvent>,
    mut input_map: ResMut<InputMap>,
    mut event_writer: EventWriter<GenerateMapEvent>,
) {
    for event in text_submit.read() {
        let value: &mut Dimension = input_map.0.get_mut(&event.entity).unwrap();
        match value {
            Dimension::Width(width) => {
                *width = event.text.parse().unwrap_or(0);
            }
            Dimension::Length(length) => {
                *length = event.text.parse().unwrap_or(0);
            }
            Dimension::Height(height) => {
                *height = event.text.parse().unwrap_or(0);
            }
        }
        if input_map.0.values().all(|x| match x {
            Dimension::Width(width) => *width >= 1,
            Dimension::Length(length) => *length >= 1,
            Dimension::Height(height) => *height >= 1,
        }) {
            info!("All dimensions are valid");
            // Extract values from the HashMap
            let mut width_val = 0;
            let mut length_val = 0;
            let mut height_val = 0;

            for dimension in input_map.0.values() {
                match dimension {
                    Dimension::Width(w) => width_val = *w,
                    Dimension::Length(l) => length_val = *l,
                    Dimension::Height(h) => height_val = *h,
                }
            }

            event_writer.write(GenerateMapEvent(
                Map::builder()
                    .width(width_val)
                    .length(length_val)
                    .height(height_val)
                    .build(),
            ));
        }
    }
}

fn on_submit_dimensions(
    mut event_reader: EventReader<GenerateMapEvent>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for _ in event_reader.read() {
        // Handle the event
        app_state.set(AppState::Generating);
    }
}

fn back_button_system(
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
