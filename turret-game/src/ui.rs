use crate::Canvas;
use crate::builder::BuilderExt;
use bevy::color::palettes::css::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerResources {
    pub gold: u32,
    pub food: u32,
}

pub fn setup_player_resources(mut commands: Commands) {
    info!("For shits");
    commands.spawn(PlayerResources { gold: 0, food: 0 });
}

#[derive(Component)]
pub struct Ui;

pub fn init_ui(mut commands: Commands, canvas: Single<Entity, With<Canvas>>) {
    commands
        .entity(*canvas)
        .insert((Text::new(format!("Gold: {}, Food: {}", 0, 0)), Ui));
}

pub fn player_resources_ui(
    player_resources: Single<&PlayerResources>,
    mut player_resources_ui: Query<&mut Text, With<Ui>>,
) {
    // Implement UI for player resources

    for mut ui in player_resources_ui.iter_mut() {
        **ui = format!(
            "Gold: {}, Food: {}",
            player_resources.gold, player_resources.food
        );
    }
}

#[derive(Component)]
pub struct BuilderUi;

#[derive(Component)]
pub struct BuilderCanvas;

pub fn spawn_builder_ui(_: Trigger<Pointer<Released>>, mut commands: Commands) {
    let canvas_node = Node::builder()
        .width(Val::Percent(100.))
        .height(Val::Percent(100.))
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Center)
        .build();

    let builder_node = Node::builder()
        .width(Val::Percent(80.))
        .height(Val::Percent(80.))
        .flex_direction(FlexDirection::Column)
        .build();

    let title_bar = Node::builder()
        .width(Val::Percent(100.))
        .height(Val::Percent(5.))
        .build();

    let close_button = Node::builder()
        .width(Val::Percent(5.))
        .height(Val::Percent(100.))
        .left(Val::Percent(95.))
        .build();

    let button_node = Node::builder()
        .width(Val::Percent(50.))
        .height(Val::Percent(50.))
        .margin(UiRect {
            left: Val::Px(5.),
            right: Val::Px(5.),
            top: Val::Px(5.),
            bottom: Val::Px(5.),
        })
        .build();

    let text_node = Node::builder()
        .width(Val::Percent(100.))
        .height(Val::Percent(20.))
        .build();

    let canvas_bundle = (canvas_node, BackgroundColor(RED.into()), BuilderCanvas);
    let builder_bundle = (builder_node, BackgroundColor(Color::WHITE), BuilderUi);
    let button_bundle = (button_node, BackgroundColor(BLUE.into()));
    let title_bar_bundle = (title_bar, BackgroundColor(Color::BLACK));
    let close_button_bundle = (close_button, Button, BackgroundColor(GREEN.into()));
    let text_bundle = (
        text_node,
        BackgroundColor(Color::WHITE),
        Text::new("Farm"),
        TextColor(GOLD.into()),
    );

    let canvas = commands.spawn(canvas_bundle).id();
    let builder = commands.spawn(builder_bundle).insert(ChildOf(canvas)).id();
    let title_bar = commands
        .spawn(title_bar_bundle)
        .insert(ChildOf(builder))
        .id();
    let _close_button = commands
        .spawn(close_button_bundle)
        .observe(builder_menu_close_system)
        .insert(ChildOf(title_bar))
        .id();
    let button = commands.spawn(button_bundle).insert(ChildOf(builder)).id();
    let _text = commands.spawn(text_bundle).insert(ChildOf(button)).id();
}

pub fn builder_menu_close_system(
    _: Trigger<Pointer<Released>>,
    mut commands: Commands,
    builder_ui: Query<Entity, With<BuilderCanvas>>,
) {
    for entity in builder_ui.iter() {
        commands.entity(entity).despawn();
    }
}
