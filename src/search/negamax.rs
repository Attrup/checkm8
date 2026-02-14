use crate::{Score, scoring::evaluate};
use shakmaty::{Chess, Position};

pub struct NegaMax {
    pub nodes_searched: u64,
}

impl NegaMax {
    pub fn new() -> Self {
        Self { nodes_searched: 0 }
    }

    pub fn search(&mut self, position: &Chess, depth: u8, mut alpha: Score, beta: Score) -> Score {
        // Update state
        self.nodes_searched += 1;
        let legal_moves = position.legal_moves();

        // Depth limit reached / Terminal state
        if depth == 0 || legal_moves.is_empty() {
            return -evaluate(&position);
        }

        // Try each legal move
        for mv in legal_moves {
            let new_position = position.clone().play(mv).unwrap();

            // Recursive call - negate the score from opponent's perspective
            let score = -self.search(&new_position, depth - 1, -beta, -alpha);

            // Update alpha -> New best move
            if score > alpha {
                alpha = score;
            }

            // Beta cutoff -> Prune remaining moves!
            if alpha >= beta {
                return alpha;
            }
        }

        return alpha;
    }
}
