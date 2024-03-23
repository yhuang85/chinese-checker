use crate::player::{Move, Player};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops,
};

// We use a coordinate system that is compatible with
// the natural orientation of a terminal. Namely, the
// origin (0, 0) is the upper-left corner of the screen
// x in the row direction and y in the column direction
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<(i16, i16)> for Position {
    fn from(value: (i16, i16)) -> Self {
        Position {
            x: value.0,
            y: value.1,
        }
    }
}

impl Position {
    fn validate(self, size: i16) -> Option<Self> {
        // Return Self if the position is valid and None otherwise
        if (self.y >= size && self.y - self.x <= size && self.x + self.y <= 7 * size)
            || (self.y <= 3 * size && self.x + self.y >= 3 * size && self.x - self.y <= 3 * size)
        {
            return Some(self);
        }
        None
    }

    fn in_triangle(&self, size: i16) -> Option<u8> {
        // Return the index of the triangle if the position is within one and None otherwise
        // The triangles are ordered as follows
        //   0
        // 1   5
        // 2   4
        //   3
        if self.y < size {
            return Some(0);
        } else if self.x + self.y < 3 * size {
            return Some(1);
        } else if self.y - self.x > size {
            return Some(2);
        } else if self.y > 3 * size {
            return Some(3);
        } else if self.x + self.y > 7 * size {
            return Some(4);
        } else if self.x - self.y > 3 * size {
            return Some(5);
        }
        None
    }

    pub fn norm(&self) -> f32 {
        (self.x as f32 * self.x as f32 / 3.0 + self.y as f32 * self.y as f32).sqrt()
    }
}

#[derive(Debug)]
pub struct Node {
    pub neighbors: [Option<Position>; 6],
    triangle: Option<u8>, // 0..6
}

impl Node {
    fn new(position: Position, size: i16) -> Self {
        Node {
            neighbors: [
                (position + Position::from((-1, -1))).validate(size),
                (position + Position::from((-2, 0))).validate(size),
                (position + Position::from((-1, 1))).validate(size),
                (position + Position::from((1, 1))).validate(size),
                (position + Position::from((2, 0))).validate(size),
                (position + Position::from((1, -1))).validate(size),
            ],
            triangle: position.in_triangle(size),
        }
    }
}

#[derive(Debug)]
pub struct PlayerState {
    pub positions: HashSet<Position>,
    pub goal: HashSet<Position>,
}

#[derive(Debug)]
pub struct ChineseChecker {
    pub size: i16,
    pub nodes: HashMap<Position, Node>,
    pub state: HashMap<String, PlayerState>, // Positions each player holds
    pub tips: [Position; 6],                 // The six tips of the boards
}

impl ChineseChecker {
    pub fn new(size: i16) -> ChineseChecker {
        // Currently only two players
        let mut b = ChineseChecker {
            size,
            nodes: HashMap::new(),
            state: HashMap::new(),
            tips: [
                Position::from((3 * size, 0)),
                Position::from((0, size)),
                Position::from((0, 3 * size)),
                Position::from((3 * size, 4 * size)),
                Position::from((6 * size, 3 * size)),
                Position::from((6 * size, size)),
            ],
        };
        let mut x: i16;
        let mut index: i16;
        for y in 0..=4 * size as i16 {
            if y < size || (y >= 2 * size && y <= 3 * size) {
                x = 3 * size - y;
                index = y;
            } else {
                x = y - size;
                index = 4 * size - y;
            }
            for i in 0..=index {
                let position = Position::from((x + 2 * i, y));
                b.nodes.insert(position, Node::new(position, size));
            }
        }
        b
    }

    pub fn make_move(&mut self, player: &Player, mv: &Move) {
        let positions = &mut self
            .state
            .get_mut(&player.name)
            .expect("Player is not found in the game.")
            .positions;

        positions.remove(&mv.from);
        positions.insert(mv.to);
    }

    fn get_positions_in_triangle(&self, triangle: u8) -> HashSet<Position> {
        self.nodes
            .iter()
            .filter(|&node| node.1.triangle == Some(triangle))
            .map(|node| *node.0)
            .collect::<HashSet<Position>>()
    }

    pub fn add_player(&mut self, player: &Player) {
        if self.state.len() >= 6 {
            panic!("The game cannot have more than 6 players.");
        }
        if self.state.contains_key(&player.name) {
            panic!("Player {} already exists in the game.", player.name);
        }
        self.state.insert(
            String::from(&player.name),
            PlayerState {
                positions: self.get_positions_in_triangle(player.triangle),
                goal: self.get_positions_in_triangle((player.triangle + 3) % 6),
            },
        );
    }

    pub fn is_occupied(&self, position: &Position) -> bool {
        for ps in self.state.values() {
            if ps.positions.contains(position) {
                return true;
            }
        }
        false
    }

    fn get_all_neighbors(&self, position: &Position) -> [Option<Position>; 6] {
        self.nodes
            .get(position)
            .expect(&format!(
                "Position {:?} is not found on the board.",
                position
            ))
            .neighbors
    }

    fn get_valid_neighbors(&self, position: &Position) -> Vec<Position> {
        self.nodes
            .get(position)
            .expect(&format!(
                "Position {:?} is not found on the board.",
                position
            ))
            .neighbors
            .iter()
            .filter(|&p| p.is_some())
            .map(|p| p.unwrap())
            .collect::<Vec<Position>>()
    }

    pub fn adjacent_unoccupied_positions(&self, position: &Position) -> HashSet<Position> {
        self.get_valid_neighbors(position)
            .iter()
            .filter(|&p| !self.is_occupied(p))
            .map(|p| *p)
            .collect::<HashSet<Position>>()
    }

    fn opposite(&self, center: &Position, leg: &Position) -> Option<Position> {
        let index = self
            .get_all_neighbors(center)
            .iter()
            .position(|&p| p == Some(*leg))
            .expect(&format!("{:?} is not a neighbor of {:?}.", leg, center));
        self.get_all_neighbors(center)[(index + 3) % 6]
    }

    pub fn jumpable_positions(&self, position: &Position) -> HashSet<Position> {
        let mut result = HashSet::new();
        let mut to_positions = self.one_jumpable_positions(position);
        loop {
            if to_positions.is_empty() {
                break;
            }
            for p in to_positions.iter() {
                result.insert(*p);
            }
            to_positions = to_positions
                .iter()
                .map(|p| self.one_jumpable_positions(&p))
                .flatten()
                .filter(|p| !result.contains(p))
                .collect();
        }
        result
    }

    fn one_jumpable_positions(&self, position: &Position) -> HashSet<Position> {
        self.get_valid_neighbors(position)
            .iter()
            .filter(|&p| self.is_occupied(p))
            .map(|p| self.opposite(p, position))
            .filter(|p| p.is_some_and(|p| !self.is_occupied(&p)))
            .map(|p| p.unwrap())
            .collect::<HashSet<Position>>()
    }

    pub fn get_player_state(&self, name: &str) -> &PlayerState {
        &self
            .state
            .get(name)
            .expect(&format!("Player {name} is not found in the game."))
    }
}
