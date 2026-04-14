use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player) -> (i32, usize, usize) {
        if board.game_over() {
            return (board.score(), 0, 0);
        }

        let moves = board.moves();
        let mut best_move = moves[0];
        let mut best_score = match player {
            Player::X => i32::MIN,
            Player::O => i32::MAX,
        };

        for m in moves {
            todo!()
        }

        (best_score, best_move.0, best_move.1)
    }
}
