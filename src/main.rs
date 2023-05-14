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
    destination: Position,
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
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &Position, Option<&Movable>), With<Pusher>>,
) {
    if let Ok((pusher, position, movable)) = query.get_single_mut() {
        if movable.is_some() {
            return;
        }

        if keys.pressed(KeyCode::Left) {
            commands.entity(pusher).insert(Movable {
                destination: Position {
                    x: position.x - 1,
                    y: position.y,
                },
            });
        }

        if keys.pressed(KeyCode::Right) {
            commands.entity(pusher).insert(Movable {
                destination: Position {
                    x: position.x + 1,
                    y: position.y,
                },
            });
        }

        if keys.pressed(KeyCode::Up) {
            commands.entity(pusher).insert(Movable {
                destination: Position {
                    x: position.x,
                    y: position.y - 1,
                },
            });
        }

        if keys.pressed(KeyCode::Down) {
            commands.entity(pusher).insert(Movable {
                destination: Position {
                    x: position.x,
                    y: position.y + 1,
                },
            });
        }
    }
}

fn move_movable(
    mut commands: Commands,
    time: Res<Time>,
    level: Res<Level>,
    mut query: Query<(Entity, &mut Movable, &mut Position, &mut Transform)>,
) {
    let tile_size = 64.0;
    let tiles_per_second = 2.0;

    for (entity, movable, mut position, mut transform) in query.iter_mut() {
        let mut stop = false;
        let x_f = (movable.destination.x as f32 - level.width() as f32 / 2.0 + 0.5) * tile_size;
        let y_f = (movable.destination.y as f32 - level.height() as f32 / 2.0 + 0.5) * tile_size;

        if movable.destination.x < position.x {
            transform.translation.x -= tile_size * tiles_per_second * time.delta_seconds();

            if transform.translation.x < x_f {
                transform.translation.x = x_f;
                stop = true;
            }
        }

        if movable.destination.x > position.x {
            transform.translation.x += tile_size * tiles_per_second * time.delta_seconds();

            if transform.translation.x > x_f {
                transform.translation.x = x_f;
                stop = true;
            }
        }

        if movable.destination.y < position.y {
            transform.translation.y += tile_size * tiles_per_second * time.delta_seconds();

            if transform.translation.y > -y_f {
                transform.translation.y = -y_f;
                stop = true;
            }
        }

        if movable.destination.y > position.y {
            transform.translation.y -= tile_size * tiles_per_second * time.delta_seconds();

            if transform.translation.y < -y_f {
                transform.translation.y = -y_f;
                stop = true;
            }
        }

        if stop {
            position.x = movable.destination.x;
            position.y = movable.destination.y;
            commands.entity(entity).remove::<Movable>();
        }
    }
}

fn stop_pusher(
    mut commands: Commands,
    level: Res<Level>,
    mut pusher_query: Query<(Entity, &Position, &Movable, &mut Transform), With<Pusher>>,
    mut walls_query: Query<&Position, With<Wall>>,
) {
    if let Ok((pusher, position, movable, mut transform)) = pusher_query.get_single_mut() {
        for wall in walls_query.iter_mut() {
            if movable.destination.x == wall.x && movable.destination.y == wall.y {
                let tile_size = 64.0;
                let x_f = (position.x as f32 - level.width() as f32 / 2.0 + 0.5) * tile_size;
                let y_f = (position.y as f32 - level.height() as f32 / 2.0 + 0.5) * tile_size;

                transform.translation.x = x_f;
                transform.translation.y = -y_f;
                commands.entity(pusher).remove::<Movable>();
                break;
            }
        }
    }
}

fn push_box(
    mut commands: Commands,
    pusher_query: Query<(&Position, &Movable), With<Pusher>>,
    mut boxes_query: Query<(Entity, &Position), With<Box>>,
) {
    if let Ok((pusher_position, movable)) = pusher_query.get_single() {
        for (box_entity, box_position) in boxes_query.iter_mut() {
            if movable.destination.x == box_position.x && movable.destination.y == box_position.y {
                commands.entity(box_entity).insert(Movable {
                    destination: Position {
                        x: movable.destination.x - pusher_position.x + box_position.x,
                        y: movable.destination.y - pusher_position.y + box_position.y,
                    },
                });
            }
        }
    }
}

fn stop_box(
    mut commands: Commands,
    level: Res<Level>,
    mut movables_query: Query<(Entity, &Position, &mut Transform), With<Movable>>,
    mut pushed_box_query: Query<&Movable, With<Box>>,
    mut boxes_query: Query<&Position, With<Box>>,
    mut walls_query: Query<&Position, With<Wall>>,
) {
    if let Ok(box_movable) = pushed_box_query.get_single_mut() {
        let mut stop = false;
        for wall in walls_query.iter_mut() {
            if box_movable.destination.x == wall.x && box_movable.destination.y == wall.y {
                stop = true;
                break;
            }
        }

        for other_box in boxes_query.iter_mut() {
            if box_movable.destination.x == other_box.x && box_movable.destination.y == other_box.y
            {
                stop = true;
                break;
            }
        }

        if stop {
            for (entity, position, mut transform) in movables_query.iter_mut() {
                let tile_size = 64.0;
                let x_f = (position.x as f32 - level.width() as f32 / 2.0 + 0.5) * tile_size;
                let y_f = (position.y as f32 - level.height() as f32 / 2.0 + 0.5) * tile_size;

                transform.translation.x = x_f;
                transform.translation.y = -y_f;
                commands.entity(entity).remove::<Movable>();
            }
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum SokobanSystemSet {
    Stop,
}

fn main() -> Result<()> {
    // let levels = levels::LevelCollection::from_file("levels/Thinking-Rabbit-Original-Plus-Extra.txt")?;
    let levels = levels::LevelCollection::from_file("levels/single.txt")?;

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(levels.into_iter())
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_level)
        .add_systems(
            (
                handle_keyboard_input,
                apply_system_buffers,
                push_box,
                apply_system_buffers,
            )
                .chain()
                .before(SokobanSystemSet::Stop),
        )
        .add_system(stop_pusher.in_set(SokobanSystemSet::Stop))
        .add_system(stop_box.in_set(SokobanSystemSet::Stop))
        .add_systems(
            (apply_system_buffers, move_movable)
                .chain()
                .after(SokobanSystemSet::Stop),
        )
        .add_system(bevy::window::close_on_esc)
        .run();

    Ok(())
}
