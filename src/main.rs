use ::anyhow::Result;
use bevy::prelude::*;

use crate::levels::Level;

mod levels;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Movable {
    destination: Option<Position>,
}

#[derive(Component)]
struct Pusher;

#[derive(Component)]
struct Box;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Floor;

#[derive(Component)]
struct Goal;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut levels: ResMut<levels::LevelCollectionIter>,
    camera: Query<&Transform, With<Camera>>,
) {
    let texture_handle =
        asset_server.load("sprites/kenney_sokobanpack/Tilesheet/sokoban_tilesheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 13, 8, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    println!("Spawning level");

    let tile_size = 64.0;

    if let Some(level) = levels.next() {
        for (x, y) in level.floors() {
            // println!("Floor ({}, {})", x, y);
            let x_f = (x as f32 - level.width() as f32 / 2.0 + 0.5) * tile_size;
            let y_f = (y as f32 - level.height() as f32 / 2.0 + 0.5) * tile_size;
            commands.spawn((
                Floor,
                Position {
                    x: x as i32,
                    y: y as i32,
                },
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(89),
                    transform: Transform {
                        translation: Vec3::new(x_f, -y_f, 0.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
        for (x, y) in level.walls() {
            let x_f = (x as f32 - level.width() as f32 / 2.0 + 0.5) * tile_size;
            let y_f = (y as f32 - level.height() as f32 / 2.0 + 0.5) * tile_size;
            commands.spawn((
                Wall,
                Position {
                    x: x as i32,
                    y: y as i32,
                },
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(97),
                    transform: Transform {
                        translation: Vec3::new(x_f, -y_f, 1.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
        for (x, y) in level.goals() {
            let x_f = (x as f32 - level.width() as f32 / 2.0 + 0.5) * tile_size;
            let y_f = (y as f32 - level.height() as f32 / 2.0 + 0.5) * tile_size;
            commands.spawn((
                Goal,
                Position {
                    x: x as i32,
                    y: y as i32,
                },
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(102),
                    transform: Transform {
                        translation: Vec3::new(x_f, -y_f, 1.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
        for (x, y) in level.boxes() {
            let x_f = (x as f32 - level.width() as f32 / 2.0 + 0.5) * tile_size;
            let y_f = (y as f32 - level.height() as f32 / 2.0 + 0.5) * tile_size;
            commands.spawn((
                Box,
                Position {
                    x: x as i32,
                    y: y as i32,
                },
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(6),
                    transform: Transform {
                        translation: Vec3::new(x_f, -y_f, 1.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
        let (x, y) = level.pusher();
        let x_f = (x as f32 - level.width() as f32 / 2.0 + 0.5) * tile_size;
        let y_f = (y as f32 - level.height() as f32 / 2.0 + 0.5) * tile_size;
        commands.spawn((
            Pusher,
            Position {
                x: x as i32,
                y: y as i32,
            },
            Movable { destination: None },
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(52),
                transform: Transform {
                    translation: Vec3::new(x_f, -y_f, 1.0),
                    ..default()
                },
                ..default()
            },
        ));

        commands.insert_resource(level);
    }
}

fn handle_keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&Position, &mut Movable), With<Pusher>>,
) {
    if let Ok((position, mut movable)) = query.get_single_mut() {
        if movable.destination.is_some() {
            return;
        }

        if keys.pressed(KeyCode::Left) {
            movable.destination = Some(Position {
                x: position.x - 1,
                y: position.y,
            });
        }

        if keys.pressed(KeyCode::Right) {
            movable.destination = Some(Position {
                x: position.x + 1,
                y: position.y,
            });
        }

        if keys.pressed(KeyCode::Up) {
            movable.destination = Some(Position {
                x: position.x,
                y: position.y - 1,
            });
        }

        if keys.pressed(KeyCode::Down) {
            movable.destination = Some(Position {
                x: position.x,
                y: position.y + 1,
            });
        }
    }
}

fn move_movable(
    time: Res<Time>,
    level: Res<Level>,
    mut query: Query<(&mut Movable, &mut Position, &mut Transform)>,
) {
    let tile_size = 64.0;
    let tiles_per_second = 2.0;

    for (mut movable, mut position, mut transform) in query.iter_mut() {
        let mut stop = false;
        if let Some(destination) = &movable.destination {
            let x_f = (destination.x as f32 - level.width() as f32 / 2.0 + 0.5) * tile_size;
            let y_f = (destination.y as f32 - level.height() as f32 / 2.0 + 0.5) * tile_size;

            if destination.x < position.x {
                transform.translation.x -= tile_size * tiles_per_second * time.delta_seconds();

                if transform.translation.x < x_f {
                    transform.translation.x = x_f;
                    stop = true;    
                }
            }

            if destination.x > position.x {
                transform.translation.x += tile_size * tiles_per_second * time.delta_seconds();

                if transform.translation.x > x_f {
                    transform.translation.x = x_f;
                    stop = true;    
                }
            }

            if destination.y < position.y {
                transform.translation.y += tile_size * tiles_per_second * time.delta_seconds();


                if transform.translation.y > -y_f {
                    transform.translation.y = -y_f;
                    stop = true;    
                }
            }

            if destination.y > position.y {
                transform.translation.y -= tile_size * tiles_per_second * time.delta_seconds();


                if transform.translation.y < -y_f {
                    transform.translation.y = -y_f;
                    stop = true;    
                }
            }
        }

        if stop {
            if let Some(destination) = movable.destination.take() {
                position.x = destination.x;
                position.y = destination.y;
            }
        }
    }
}

fn main() -> Result<()> {
    // let levels = levels::LevelCollection::from_file("levels/Thinking-Rabbit-Original-Plus-Extra.txt")?;
    let levels = levels::LevelCollection::from_file("levels/single.txt")?;

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(levels.into_iter())
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_level)
        .add_system(handle_keyboard_input)
        .add_system(move_movable)
        .run();

    Ok(())
}
