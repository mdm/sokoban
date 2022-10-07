use bevy::prelude::*;

mod levels;

enum Sprite {
    BoxOnFloor,
    BoxOnGoal,
    Floor,
    Goal,
    Wall,
}

#[derive(Component)]
struct Position { x: usize, y: usize }

#[derive(Component)]
struct Tile { sprite: Sprite }

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut levels: ResMut<levels::LevelCollectionIter>,
    windows: Res<Windows>,
    camera: Query<&Transform, With<Camera>>,
) {
    let texture_handle = asset_server.load("sprites/kenney_sokobanpack/Tilesheet/sokoban_tilesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 13, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    println!("Spawning level");

    let tile_size = 64.0;

    if let Some(level) = levels.next() {
        for (x, y) in level.floors() {
            // println!("Floor ({}, {})", x, y);
            let x_f = (x as f32 - level.width() as f32 / 2.0 + 0.5) * tile_size;
            let y_f = (y as f32 - level.height() as f32 / 2.0 + 0.5) * tile_size;
            commands.spawn().insert(Tile { sprite: Sprite::Floor }).insert(Position { x, y });
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(89),
                transform: Transform { translation: Vec3::new(x_f, -y_f, 0.0), ..default() },
                ..default()
            });
        }
        for (x, y) in level.walls() {
            let x_f = (x as f32 - level.width() as f32 / 2.0 + 0.5) * tile_size;
            let y_f = (y as f32 - level.height() as f32 / 2.0 + 0.5) * tile_size;
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(97),
                transform: Transform { translation: Vec3::new(x_f, -y_f, 1.0), ..default() },
                ..default()
            });
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let levels = levels::LevelCollection::from_file("levels/Thinking-Rabbit-Original-Plus-Extra.txt")?;
    let levels = levels::LevelCollection::from_file("levels/single.txt")?;

    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(levels.into_iter())
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_level)
    .run();

    Ok(())
}
