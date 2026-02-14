use crate::{MIN_SCORE, Score, scoring::evaluate};
use shakmaty::{Chess, Position};

pub struct NegaMax {
    pub nodes_searched: u64,
}

impl NegaMax {
    pub fn new() -> Self {
        Self { nodes_searched: 0 }
    }

    // NegaMax Search
    pub fn search(&mut self, position: &Chess, depth: u8) -> Option<Score> {
        // Update state
        self.nodes_searched += 1;
        let legal_moves = position.legal_moves();

        // Depth limit reached / Terminal state
        if depth == 0 || legal_moves.is_empty() {
            return None;
        }

        // Base values to search from
        let mut best_score = MIN_SCORE;

        // Try each legal move
        for mv in legal_moves {
            let new_position = position.clone().play(mv).unwrap();

            // Recursive call - negate the score from opponent's perspective
            let score = if let Some(child_score) = self.search(&new_position, depth - 1) {
                -child_score
            } else {
                // If child returns None (terminal position / depth limit reached), evaluate it
                -evaluate(&new_position)
            };

            if score > best_score {
                best_score = score;
            }
        }

        return Some(best_score);
    }
}
