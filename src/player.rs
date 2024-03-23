use std::collections::HashSet;

use crate::game::{ChineseChecker, Position};

use crossterm::style::Color;

#[derive(Debug)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub color: Color,
    pub triangle: u8,
}

impl Player {
    pub fn new(name: String, color: Color, triangle: u8) -> Self {
        Player {
            name,
            color,
            triangle,
        }
    }

    pub fn is_done(&self, cc: &ChineseChecker) -> bool {
        let state = cc.get_player_state(&self.name);
        state.positions == state.goal
    }

    fn evaluate(&self, mv: &Move, cc: &ChineseChecker) -> f32 {
        let to_tip = cc.tips[(self.triangle as usize + 3) % 6];
        let before = (to_tip - mv.from).norm();
        let after = (to_tip - mv.to).norm();
        before - after
    }

    pub fn select_move(&self, cc: &ChineseChecker) -> Option<Move> {
        let mut best_mv: Option<Move> = None;
        let mut best_score: f32 = -10.0;
        for mv in self.find_all_moves(cc) {
            let score = self.evaluate(&mv, cc);
            if score >= best_score {
                best_mv = Some(mv);
                best_score = score;
            }
        }
        best_mv
    }

    fn find_all_moves(&self, cc: &ChineseChecker) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        let mut dests: HashSet<(Position, Position)> = HashSet::new();
        for from in cc.get_player_state(&self.name).positions.iter() {
            // Get all crawls
            for to in cc.adjacent_unoccupied_positions(&from) {
                moves.push(Move { from: *from, to });
                dests.insert((*from, to));
            }
            // Get all jumps
            for to in cc.jumpable_positions(&from) {
                if !dests.contains(&(*from, to)) {
                    moves.push(Move { from: *from, to });
                    dests.insert((*from, to));
                }
            }
        }
        moves
    }
}
