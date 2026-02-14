use crate::{MATE_SCORE, Score};
use shakmaty::{Chess, Color, Move, Position, Role};

// Constants
const CAPTURE_BASE: i32 = 1_000_000;
const PROMOTION_BASE: i32 = 100_000;
const CHECK_BONUS: i32 = 5_000;
const QUIET_BASE: i32 = 0;

// Mobility weighting
const MOBILITY_FACTOR: Score = 2;

pub fn evaluate(position: &Chess, depth: u8) -> Score {
    // Death -> Score quicker checkmates higher
    if position.is_checkmate() {
        return MATE_SCORE + depth as i32 * 5;
    }

    // Draw
    if position.is_stalemate() || position.is_insufficient_material() {
        return 0;
    }

    // Determine phase
    let phase = game_phase(position);
    let max_phase = 24;

    // Score each piece
    let mut score = 0;

    for (square, piece) in position.board() {
        let idx = match piece.color {
            Color::White => square as usize,
            Color::Black => mirror_idx(square as usize),
        };

        // Material term
        let mat = piece_value(piece.role);

        // PST blended term (midgame/endgame)
        let pst = match piece.role {
            Role::Pawn => blend_pst(PAWN_PST_MG[idx], PAWN_PST_EG[idx], phase, max_phase),
            Role::Knight => blend_pst(KNIGHT_PST_MG[idx], KNIGHT_PST_EG[idx], phase, max_phase),
            Role::Bishop => blend_pst(BISHOP_PST_MG[idx], BISHOP_PST_EG[idx], phase, max_phase),
            Role::Rook => blend_pst(ROOK_PST_MG[idx], ROOK_PST_EG[idx], phase, max_phase),
            Role::Queen => blend_pst(QUEEN_PST_MG[idx], QUEEN_PST_EG[idx], phase, max_phase),
            Role::King => blend_pst(KING_PST_MG[idx], KING_PST_EG[idx], phase, max_phase),
        };

        // Add score
        if piece.color == position.turn() {
            score += mat + pst;
        } else {
            score -= mat + pst;
        }
    }

    // Mobility bonus
    score += position.legal_moves().len() as Score * MOBILITY_FACTOR;

    return score;
}

/// Best scoring move = lowest value to sort to start!
pub fn score_move(position: &Chess, mv: &Move) -> Score {
    // Determine who's the attacker
    let attacker = mv.role();

    // Captures
    if let Some(victim) = mv.capture() {
        let victim_value = piece_value(victim);
        let attacker_value = piece_value(attacker);

        let mvv_lva_score = victim_value * 10 - attacker_value;

        // Ensure captures outscore any non-capture
        return -(CAPTURE_BASE + mvv_lva_score);
    }

    // Promotions
    if let Some(promotion) = mv.promotion() {
        let promotion_val = piece_value(promotion);
        return -(PROMOTION_BASE + promotion_val);
    }

    // Checks
    if position.clone().play(*mv).unwrap().is_check() {
        return -CHECK_BONUS;
    }

    // Quiet
    return -QUIET_BASE;
}

fn piece_value(role: Role) -> Score {
    return match role {
        Role::Pawn => 100,
        Role::Knight => 320,
        Role::Bishop => 330,
        Role::Rook => 500,
        Role::Queen => 900,
        Role::King => 10000,
    };
}

#[inline]
fn mirror_idx(idx: usize) -> usize {
    let file = idx % 8;
    let rank = idx / 8;
    let mirrored_rank = 7 - rank;
    mirrored_rank * 8 + file
}

fn game_phase(position: &Chess) -> i32 {
    let mut phase = 0;
    for (_, piece) in position.board() {
        phase += match piece.role {
            Role::Pawn => 0,
            Role::King => 0,
            Role::Knight => 1,
            Role::Bishop => 1,
            Role::Rook => 2,
            Role::Queen => 4,
        };
    }

    return phase;
}

fn blend_pst(mg: Score, eg: Score, phase: i32, max_phase: i32) -> Score {
    return (mg * phase + eg * (max_phase - phase)) / max_phase;
}

// --- PAWN PSTs ---

const PAWN_PST_MG: [Score; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, -20, -20, 10, 10, 5, 5, -5, -10, 0, 0, -10, -5, 5, 0, 0, 0,
    20, 20, 0, 0, 0, 5, 5, 10, 25, 25, 10, 5, 5, 10, 10, 20, 30, 30, 20, 10, 10, 50, 50, 50, 50,
    50, 50, 50, 50, 0, 0, 0, 0, 0, 0, 0, 0,
];

const PAWN_PST_EG: [Score; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 10, 15, 15, -10, -10, 15, 15, 10, 10, 0, -5, 5, 5, -5, 0, 10, 0, 5, 10,
    25, 25, 10, 5, 0, 10, 10, 20, 35, 35, 20, 10, 10, 15, 15, 25, 35, 35, 25, 15, 15, 60, 60, 60,
    60, 60, 60, 60, 60, 0, 0, 0, 0, 0, 0, 0, 0,
];

// --- KNIGHT PSTs ---

const KNIGHT_PST_MG: [Score; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15, 10,
    0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15, 15, 10,
    5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
];

const KNIGHT_PST_EG: [Score; 64] = [
    -40, -30, -25, -25, -25, -25, -30, -40, -30, -15, -5, -5, -5, -5, -15, -30, -25, -5, 5, 10, 10,
    5, -5, -25, -25, 0, 10, 15, 15, 10, 0, -25, -25, -5, 10, 15, 15, 10, -5, -25, -25, 0, 5, 10,
    10, 5, 0, -25, -30, -15, -5, 0, 0, -5, -15, -30, -40, -30, -25, -25, -25, -25, -30, -40,
];

// --- BISHOP PSTs ---

const BISHOP_PST_MG: [Score; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5, 0,
    -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10, 10, 10,
    -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
];

const BISHOP_PST_EG: [Score; 64] = [
    -15, -5, -5, -5, -5, -5, -5, -15, -5, 5, 5, 5, 5, 5, 5, -5, -5, 5, 10, 15, 15, 10, 5, -5, -5,
    10, 10, 15, 15, 10, 10, -5, -5, 5, 15, 15, 15, 15, 5, -5, -5, 10, 15, 15, 15, 15, 10, -5, -5,
    5, 5, 5, 5, 5, 5, -5, -15, -5, -5, -5, -5, -5, -5, -15,
];

// --- ROOK PSTs ---

const ROOK_PST_MG: [Score; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
    0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0,
    0, 5, 5, 0, 0, 0,
];

const ROOK_PST_EG: [Score; 64] = [
    0, 0, 5, 10, 10, 5, 0, 0, 5, 10, 15, 15, 15, 15, 10, 5, 0, 5, 10, 15, 15, 10, 5, 0, 0, 5, 10,
    15, 15, 10, 5, 0, 0, 5, 10, 15, 15, 10, 5, 0, 0, 5, 10, 15, 15, 10, 5, 0, -5, 0, 5, 10, 10, 5,
    0, -5, 0, 0, 0, 5, 5, 0, 0, 0,
];

// --- QUEEN PSTs ---

const QUEEN_PST_MG: [Score; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0, -10,
    -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0, 5, 0, 0,
    0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
];

const QUEEN_PST_EG: [Score; 64] = [
    -15, -10, -5, -5, -5, -5, -10, -15, -10, 0, 0, 0, 0, 0, 0, -10, -5, 0, 5, 10, 10, 5, 0, -5, -5,
    0, 10, 15, 15, 10, 0, -5, -5, 0, 10, 15, 15, 10, 0, -5, -5, 0, 5, 10, 10, 5, 0, -5, -10, 0, 0,
    5, 5, 0, 0, -10, -15, -10, -5, -5, -5, -5, -10, -15,
];

// --- KING PSTs ---

const KING_PST_MG: [Score; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40,
    -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40, -40, -30,
    -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0,
    10, 30, 20,
];

const KING_PST_EG: [Score; 64] = [
    -50, -40, -30, -20, -20, -30, -40, -50, -30, -20, -10, 0, 0, -10, -20, -30, -30, -10, 20, 30,
    30, 20, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30,
    -10, 20, 30, 30, 20, -10, -30, -30, -20, -10, 0, 0, -10, -20, -30, -50, -40, -30, -20, -20,
    -30, -40, -50,
];
