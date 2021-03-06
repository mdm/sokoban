use std::io::BufRead;

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

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

pub struct Level {
    data: Vec<Vec<Tile>>,
}

impl Level {
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

    pub fn move_pusher(&mut self, direction: Direction) -> Option<(usize, usize)> {
        let pusher = self.pusher();

        let pusher_destination = self.target(&pusher, &direction);
        match self.data[pusher_destination.1][pusher_destination.0].occupant {
            TileOccupant::None => {
                self.data[pusher_destination.1][pusher_destination.0].occupant = TileOccupant::Pusher;
                self.data[pusher.1][pusher.0].occupant = TileOccupant::None;
                Some(pusher_destination)
            }
            TileOccupant::Box => {
                let box_destination = self.target(&pusher_destination, &direction);
                match self.data[box_destination.1][box_destination.0].occupant {
                    TileOccupant::None => {
                        self.data[box_destination.1][box_destination.0].occupant = TileOccupant::Box;
                        self.data[pusher_destination.1][pusher_destination.0].occupant = TileOccupant::Pusher;
                        self.data[pusher.1][pusher.0].occupant = TileOccupant::None;
                        Some(pusher_destination)
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn filter<'a>(&'a self, f: impl Fn(&Tile) -> bool + 'a) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.data
        .iter()
        .enumerate()
        .map(|(y, row)|
            row.iter().enumerate().map(move |(x, tile)| (x, y, tile))
        )
        .flatten()
        .filter(move |(_, _, tile)| f(tile))
        .map(|(x, y, _)| (x, y))
    }

    fn target(&self, position: &(usize, usize), direction: &Direction) -> (usize, usize) {
        match direction {
            Direction::Up => (position.0, position.1 - 1),
            Direction::Right => (position.0 + 1, position.1),
            Direction::Down => (position.0, position.1 + 1),
            Direction::Left => (position.0 - 1, position.1),
        }
    }
}

pub struct LevelCollection {
    levels: Vec<Level>,
}

impl LevelCollection {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let mut lines = std::io::BufReader::new(file).lines();

        let mut collection = LevelCollection { levels: Vec::new() };
        let mut level = Level { data: Vec::new() };
        while let Some(Ok(line)) = lines.next() {
            if is_puzzle_line(&line) {
                let row = parse_row(&line);
                level.data.push(row);
            } else if !level.data.is_empty() {
                collection.levels.push(level);
                level = Level { data: Vec::new() };
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
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.levels.into_iter()
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
