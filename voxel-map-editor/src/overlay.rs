use bevy::prelude::*;
use bevy_builder::*;

struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_overlay);
    }
}

/// This system will be an overlay similar to the one in magicka voxel editor.
/// Things we need:
/// - Map Dimension Input
/// - Tile Dimension Input
/// - Navbar for importing tiles or old maps.
fn setup_overlay(mut commands: Commands) {
    let canvas_node = Node::builder()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .build();

    let bar_node = Node::builder()
        .width(Val::Percent(100.0))
        .height(Val::Percent(5.0))
        .build();

    let canvas_bundle = (canvas_node);

    let canvas = commands.spawn(canvas_bundle).id();

    let bar = commands.spawn(bar_node).insert(ChildOf(canvas)).id();
}
