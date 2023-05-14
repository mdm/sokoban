use std::io::BufRead;

use anyhow::Result;
use bevy::prelude::Resource;

enum TileKind {
    Outside,
    Floor,
    Goal,
}

enum TileOccupant {
    None,
    Box,
    Pusher,
    Wall,
}

struct Tile {
    kind: TileKind,
    occupant: TileOccupant,
}

#[derive(Resource)]
pub struct Level {
    width: usize,
    data: Vec<Vec<Tile>>,
}

impl Level {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn walls(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.filter(|tile| matches!(tile.occupant, TileOccupant::Wall))
    }

    pub fn floors(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.filter(|tile| matches!(tile.kind, TileKind::Floor))
    }

    pub fn goals(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.filter(|tile| matches!(tile.kind, TileKind::Goal))
    }

    pub fn boxes(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.filter(|tile| matches!(tile.occupant, TileOccupant::Box))
    }

    pub fn pusher(&self) -> (usize, usize) {
        self.filter(|tile| matches!(tile.occupant, TileOccupant::Pusher))
        .next().unwrap()
    }

    fn filter<'a>(&'a self, f: impl Fn(&Tile) -> bool + 'a) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.data
        .iter()
        .enumerate()
        .flat_map(|(y, row)|
            row.iter().enumerate().map(move |(x, tile)| (x, y, tile))
        )
        .filter(move |(_, _, tile)| f(tile))
        .map(|(x, y, _)| (x, y))
    }
}

pub struct LevelCollection {
    levels: Vec<Level>,
}

impl LevelCollection {
    pub fn from_file(path: &str) -> Result<Self> {
        let file = std::fs::File::open(path)?;
        let mut lines = std::io::BufReader::new(file).lines();

        let mut collection = LevelCollection { levels: Vec::new() };
        let mut level = Level { width: 0, data: Vec::new() };
        while let Some(Ok(line)) = lines.next() {
            if is_puzzle_line(&line) {
                let row = parse_row(&line);
                if row.len() > level.width {
                    level.width = row.len();
                }
                level.data.push(row);
            } else if !level.data.is_empty() {
                collection.levels.push(level);
                level = Level { width: 0, data: Vec::new() };
            }
        }

        if !level.data.is_empty() {
            collection.levels.push(level);
        }

        Ok(collection)
    }
}

impl IntoIterator for LevelCollection {
    type Item = Level;
    type IntoIter = LevelCollectionIter;

    fn into_iter(self) -> Self::IntoIter {
        LevelCollectionIter {
            inner_iter: self.levels.into_iter(),
        }
    }
}

#[derive(Resource)]
pub struct LevelCollectionIter {
    inner_iter: std::vec::IntoIter<Level>,
}

impl Iterator for LevelCollectionIter {
    type Item = Level;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next()
    }
}

fn is_puzzle_line(line: &str) -> bool {
    line.chars().filter(|char| *char == '#').count() >= 2
}

fn parse_row(line: &str) -> Vec<Tile> {
    let mut row = Vec::new();
    let mut inside = false;
    for char in line.chars() {
        match char {
            '#' => {
                row.push(Tile { kind: TileKind::Floor, occupant: TileOccupant::Wall });
                inside = true;
            }
            'p' | '@' => {
                row.push(Tile { kind: TileKind::Floor, occupant: TileOccupant::Pusher });
            }
            'P' | '+' => {
                row.push(Tile { kind: TileKind::Goal, occupant: TileOccupant::Pusher });
            }
            'b' | '$' => {
                row.push(Tile { kind: TileKind::Floor, occupant: TileOccupant::Box });
            }
            'B' | '*' => {
                row.push(Tile { kind: TileKind::Goal, occupant: TileOccupant::Box });
            }
            '.' => {
                row.push(Tile { kind: TileKind::Goal, occupant: TileOccupant::None });
            }
            '-' | '_' => {
                if inside {
                    row.push(Tile { kind: TileKind::Floor, occupant: TileOccupant::None });
                } else {
                    row.push(Tile { kind: TileKind::Outside, occupant: TileOccupant::None });
                }
            }
            _ => {}
        }
    }

    row
}
