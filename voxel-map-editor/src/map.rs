use crate::AppState;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Generating), generate_map);
    }
}

#[derive(Clone, Copy)]
struct Dimensions {
    width: u32,
    length: u32,
    height: u32,
}

impl Dimensions {
    fn magnitude(&self) -> u32 {
        self.width * self.length * self.height
    }
}

impl From<(u32, u32, u32)> for Dimensions {
    fn from((width, length, height): (u32, u32, u32)) -> Self {
        Dimensions {
            width,
            length,
            height,
        }
    }
}

#[derive(Default)]
pub struct MapBuilder {
    width: u32,
    length: u32,
    height: u32,
}

impl MapBuilder {
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn length(mut self, length: u32) -> Self {
        self.length = length;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn build(self) -> Map {
        let nodes: Vec<Tile> = (0..self.height)
            .flat_map(move |z| {
                (0..self.length).flat_map(move |y| {
                    (0..self.width).map(move |x| {
                        Tile::new(
                            x as f32 - self.width as f32 / 2.0,
                            y as f32 - self.width as f32 / 2.0,
                            z as f32,
                        )
                    })
                })
            })
            .collect();

        Map {
            dimensions: Dimensions {
                width: self.width,
                length: self.length,
                height: self.height,
            },
            nodes,
        }
    }
}

#[derive(Component, Clone, Copy)]
struct Tile {
    x: f32,
    y: f32,
    z: f32,
}

impl From<Tile> for (f32, f32, f32) {
    fn from(tile: Tile) -> Self {
        (tile.x, tile.y, tile.z)
    }
}

impl Tile {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Tile { x, y, z }
    }
}

#[derive(Clone)]
pub struct Map {
    dimensions: Dimensions,
    nodes: Vec<Tile>,
}

impl Map {
    pub fn builder() -> MapBuilder {
        MapBuilder::default()
    }
}

#[derive(Event)]
pub struct GenerateMapEvent(pub Map);

impl GenerateMapEvent {
    fn map(&self) -> Map {
        self.0.clone()
    }
}

fn generate_map(
    mut event_reader: EventReader<GenerateMapEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in event_reader.read() {
        for tile in event.map().nodes.into_iter() {
            // Process each tile here
            let (x, y, z): (f32, f32, f32) = tile.into();
            commands.spawn((
                Mesh3d(asset_server.load("frame.obj")),
                MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
                Transform::from_translation(Vec3::new(x, y, z)),
            ));
        }
    }
}
