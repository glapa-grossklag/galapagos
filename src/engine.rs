const KING: i32 = 10000;
const PAWN: i32     = 100;
const KNIGHT: i32   = 300;
const BISHOP: i32   = 300;
const ROOK: i32     = 500;
const QUEEN: i32    = 900;


/// Calculates the static value of the given board from
/// the point of view of the side to move
pub fn evaluate(board: &chess::Board) -> i32 {
    // Check for checkmate or stalemate
    if board.status() == chess::BoardStatus::Checkmate {
        return -KING;
    } else if board.status() == chess::BoardStatus::Stalemate {
        return 0;
    }

    let mut total: i32 = 0;
    let us = board.color_combined(board.side_to_move());
    let them = board.color_combined(!board.side_to_move());

    // Pawns
    total += PAWN * (us & board.pieces(chess::Piece::Pawn)).popcnt() as i32;
    total -= PAWN * (them & board.pieces(chess::Piece::Pawn)).popcnt() as i32;

    // Knights
    total += KNIGHT * (us & board.pieces(chess::Piece::Knight)).popcnt() as i32;
    total -= KNIGHT * (them & board.pieces(chess::Piece::Knight)).popcnt() as i32;

    // Bishops
    total += BISHOP * (us & board.pieces(chess::Piece::Bishop)).popcnt() as i32;
    total -= BISHOP * (them & board.pieces(chess::Piece::Bishop)).popcnt() as i32;

    // Rooks
    total += ROOK * (us & board.pieces(chess::Piece::Rook)).popcnt() as i32;
    total -= ROOK * (them & board.pieces(chess::Piece::Rook)).popcnt() as i32;

    // Queens
    total += QUEEN * (us & board.pieces(chess::Piece::Queen)).popcnt() as i32;
    total -= QUEEN * (them & board.pieces(chess::Piece::Queen)).popcnt() as i32;

    return total;
}

fn search(
    board: chess::Board,
    depth: i32,
    mut alpha: i32,
    beta: i32
    ) -> i32 {
    // Terminal node
    if depth == 0 {
        return evaluate(&board);
    }

    // Check captures
    let mut moves = chess::MoveGen::new_legal(&board);
    moves.set_iterator_mask(*board.color_combined(!board.side_to_move()));
    for child in &mut moves {
        let evaluation = -search(board.make_move_new(child), depth-1, -beta, -alpha);
        if evaluation >= beta {
            return beta;
        }
        if evaluation > alpha {
            alpha = evaluation;
        }
    }

    // Check non-captures
    moves.set_iterator_mask(!chess::EMPTY);
    for child in moves {
        let evaluation = -search(board.make_move_new(child), depth-1, -beta, -alpha);
        if evaluation >= beta {
            return beta;
        }
        if evaluation > alpha {
            alpha = evaluation;
        }
    }

    return alpha;
}

/// Finds the best move and the variation's evaluation
/// up to the given depth.
pub fn bestmove(
    board: chess::Board,
    depth: i32
    ) -> (chess::ChessMove, i32) {
    assert!(depth > 0, "Depth must be greater than 0");

    let mut mov = chess::ChessMove::default();
    let mut alpha = -KING;
    let beta = KING;

    // Check captures first
    let mut moves = chess::MoveGen::new_legal(&board);
    moves.set_iterator_mask(*board.color_combined(!board.side_to_move()));
    for child in &mut moves {
        let evaluation = -search(board.make_move_new(child), depth-1, -beta, -alpha);
        if evaluation >= beta {
            return (child, beta);
        }
        if evaluation > alpha {
            mov = child;
            alpha = evaluation;
        }
    }

    // Check non-captures
    moves.set_iterator_mask(!chess::EMPTY);
    for child in &mut moves {
        let evaluation = -search(board.make_move_new(child), depth-1, -beta, -alpha);
        if evaluation >= beta {
            return (child, beta);
        }
        if evaluation > alpha {
            mov = child;
            alpha = evaluation;
        }
    }

    return (mov, alpha);
}