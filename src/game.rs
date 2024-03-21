use std::{
    collections::{HashMap, HashSet},
    ops, vec,
};

use crossterm::style::Color;

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

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
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
        // 2   5
        // 4   3
        //   1
        if self.y < size {
            return Some(0);
        } else if self.x + self.y < 3 * size {
            return Some(2);
        } else if self.y - self.x > size {
            return Some(4);
        } else if self.y > 3 * size {
            return Some(1);
        } else if self.x + self.y > 7 * size {
            return Some(3);
        } else if self.x - self.y > 3 * size {
            return Some(5);
        }
        None
    }
}

#[derive(Debug)]
pub struct Node {
    neighbors: [Option<Position>; 6],
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
pub struct Player {
    pub name: String,
    pub color: Color,
}

impl Player {
    pub fn new(name: String, color: Color) -> Self {
        Player { name, color }
    }

    pub fn select_move(&self, cc: &ChineseChecker) -> Option<Move> {
        for choice in self.find_all_moves(cc) {
            // For the moment, just return the first possible move
            return Some(choice);
        }
        None
    }

    fn find_all_moves(&self, cc: &ChineseChecker) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        for from in cc
            .state
            .get(&self.name)
            .expect("Player is not found in the game.")
            .positions
            .iter()
        {
            for to in cc
                .nodes
                .get(from)
                .expect("Start position is not found in the game.")
                .neighbors
                .iter()
                .filter(|&to| to.is_some())
                .map(|&to| to.unwrap())
                .filter(|to| !cc.is_occupied(to))
            {
                moves.push(Move {
                    from: *from,
                    to,
                    path: vec![*from, to],
                })
            }
        }
        moves
    }
}

#[derive(Debug)]
pub struct PlayerState {
    pub positions: HashSet<Position>,
    target: HashSet<Position>,
}

#[derive(Debug)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub path: Vec<Position>,
}

#[derive(Debug)]
pub struct ChineseChecker {
    pub size: i16,
    pub nodes: HashMap<Position, Node>,
    pub state: HashMap<String, PlayerState>, // Positions each player holds
}

impl ChineseChecker {
    pub fn new(size: i16) -> ChineseChecker {
        // Currently only two players
        let mut b = ChineseChecker {
            size,
            nodes: HashMap::new(),
            state: HashMap::new(),
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

    pub fn play(&mut self, player: &Player, selected_move: &Move) {
        self.state
            .get_mut(&player.name)
            .expect("Player is not found in the game.")
            .positions
            .remove(&selected_move.from);
        self.state
            .get_mut(&player.name)
            .expect("Player is not found in the game.")
            .positions
            .insert(selected_move.to);
    }

    fn get_positions_in_triangle(&self, triangle: u8) -> HashSet<Position> {
        let positions: HashSet<Position> = self
            .nodes
            .iter()
            .filter(|&node| node.1.triangle == Some(triangle))
            .map(|node| *node.0)
            .collect();
        positions
    }

    pub fn add_player(&mut self, player: &Player) -> Result<(), &'static str> {
        let num_players = self.state.len() as u8;
        let mut target = num_players + 1;
        if num_players % 2 == 1 {
            target = num_players - 1;
        }
        if num_players >= 6 {
            return Err("The game cannot have more than 6 players.");
        }
        if self.state.contains_key(&player.name) {
            return Err("Player name already exists");
        }
        self.state.insert(
            String::from(&player.name),
            PlayerState {
                positions: self.get_positions_in_triangle(num_players),
                target: self.get_positions_in_triangle(target),
            },
        );
        Ok(())
    }

    fn is_occupied(&self, position: &Position) -> bool {
        for ps in self.state.values() {
            if ps.positions.contains(position) {
                return true;
            }
        }
        false
    }
}
