use crate::{MAX_SCORE, MIN_SCORE, SearchCommand, SearchControl, SearchInfo};
use crossbeam_channel::{Receiver, Sender};
use shakmaty::{Chess, Move, Position};

mod negamax;

use negamax::NegaMax;

/// Executes search tasks.
pub struct Searcher {
    cmd_rx: Receiver<SearchCommand>,
    info_tx: Sender<SearchInfo>,
}

impl Searcher {
    pub fn new(cmd_rx: Receiver<SearchCommand>, info_tx: Sender<SearchInfo>) -> Self {
        Searcher { cmd_rx, info_tx }
    }

    /// Run the searcher
    pub fn run(mut self) {
        loop {
            match self.cmd_rx.recv() {
                Ok(SearchCommand::Start { position, control }) => self.search(position, control),
                Ok(SearchCommand::Stop) => (),
                Ok(SearchCommand::Quit) | Err(_) => break,
            }
        }
    }

    fn search(&mut self, position: Chess, control: SearchControl) {
        // Determine search constraints
        let (_max_depth, _time_limit) = match control {
            SearchControl::ToDepth(depth) => (depth, u64::MAX),
            SearchControl::TimeLimit(time_limit) => (u8::MAX, time_limit),
        };

        let legal_moves = position.legal_moves();

        // Initial values
        let mut best_move = legal_moves[0];
        let mut alpha = MIN_SCORE;
        let beta = MAX_SCORE;

        let mut negamax = NegaMax::new();

        for mv in position.legal_moves() {
            // Score this move (By searching)
            let new_position = position.clone().play(mv).unwrap();
            let score = -negamax.search(&new_position, _max_depth - 1, -beta, -alpha);

            // Update appropriately
            if score > alpha {
                alpha = score;
                best_move = mv;
            }
        }

        // It is necessary to send info at least once to En Croissant (the user interface) before outputting best move.
        self.send_info(0, vec![best_move], alpha, negamax.nodes_searched);

        // Output best move
        self.info_tx.send(SearchInfo::BestMove(best_move)).unwrap();
    }

    fn send_info(&self, depth: u8, pv: Vec<Move>, score: i32, nodes: u64) {
        self.info_tx
            .send(SearchInfo::Info {
                depth,
                pv,
                score,
                nodes,
            })
            .unwrap();
    }
}
