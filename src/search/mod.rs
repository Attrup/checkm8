use crate::{MAX_SCORE, MIN_SCORE, SearchCommand, SearchControl, SearchInfo};
use crossbeam_channel::{Receiver, Sender};
use shakmaty::{Chess, Move, Position};
use std::time::{Duration, Instant};

mod negamax;

use negamax::NegaMax;

pub struct Timer {
    time_limit: Duration,
    start_time: Instant,
}

impl Timer {
    pub fn new(time_limit: Duration) -> Self {
        Timer {
            time_limit,
            start_time: Instant::now(),
        }
    }

    pub fn limit_exceeded(&self) -> bool {
        self.start_time.elapsed() >= self.time_limit
    }
}

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
        let (max_depth, time_limit) = match control {
            SearchControl::ToDepth(depth) => (depth, Duration::from_millis(u64::MAX)),
            SearchControl::TimeLimit(time_limit) => (u8::MAX, Duration::from_millis(time_limit)),
        };
        // Log start time:
        let timer = Timer::new(time_limit);

        // Initial values
        let mut selected_move = position.legal_moves()[0];
        let mut running_depth: u8 = 1;

        while running_depth <= max_depth && !timer.limit_exceeded() {
            // Init alpha beta
            let mut alpha = MIN_SCORE;
            let beta = MAX_SCORE;
            let mut best_move = position.legal_moves()[0];

            // Init searcher
            let mut negamax = NegaMax::new();

            for mv in position.legal_moves() {
                // Score this move (By searching)
                let new_position = position.clone().play(mv).unwrap();

                let score =
                    match negamax.search(&new_position, running_depth - 1, -beta, -alpha, &timer) {
                        Some(score) => -score,
                        None => break,
                    };

                // Update appropriately
                if score > alpha {
                    alpha = score;
                    best_move = mv;
                }
            }

            if !timer.limit_exceeded() {
                // Send info
                self.send_info(
                    running_depth,
                    vec![selected_move],
                    alpha,
                    negamax.nodes_searched,
                );

                // Update best move
                selected_move = best_move;
            }

            // Run another depth if we can!
            running_depth += 1;
        }

        // Output best move
        self.info_tx
            .send(SearchInfo::BestMove(selected_move))
            .unwrap();
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
