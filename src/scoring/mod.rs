use crate::{MATE_SCORE, Score};
use shakmaty::{Chess, Position, Role};

pub fn evaluate(position: &Chess) -> Score {
    // Die
    if position.is_checkmate() {
        return MATE_SCORE;
    }

    // Draw
    if position.is_stalemate() || position.is_insufficient_material() {
        return 0;
    }

    // Score each piece
    let mut score = 0;
    for (_square, piece) in position.board() {
        let piece_val = match piece.role {
            Role::Pawn => 100,
            Role::Knight => 320,
            Role::Bishop => 330,
            Role::Rook => 500,
            Role::Queen => 900,
            Role::King => 10000,
        };

        if piece.color == position.turn() {
            score += piece_val;
        } else {
            score -= piece_val;
        }
    }

    return score;
}
