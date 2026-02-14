use std::i32;

pub mod bot;
pub mod scoring;
pub mod search;

// Common types
pub type Score = i32;

// Parameters
const SEARCH_TIME_MS: u64 = 2000;

const MATE_SCORE: Score = i32::MIN + 1;
const MIN_SCORE: Score = i32::MIN;
const MAX_SCORE: Score = i32::MAX;

/// Instructions for the search thread
pub enum SearchControl {
    // Search to a given depth
    ToDepth(u8),
    // Search for a approximate duration (in milliseconds)
    TimeLimit(u64),
}

/// Instructions for the search thread
pub enum SearchCommand {
    Start {
        position: shakmaty::Chess,
        control: SearchControl,
    },
    Stop,
    Quit,
}

/// Search information to be logged
pub enum SearchInfo {
    BestMove(shakmaty::Move),
    Info {
        depth: u8,
        pv: Vec<shakmaty::Move>,
        score: i32,
        nodes: u64,
    },
}
