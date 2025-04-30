use crate::{
    Board, Player,
    mcts::{algo::state::State, utils::reward::Reward},
};

use super::{mvs::path::ActionList, utils::AppError};

impl State<ActionList, Player, AppError> for Board {
    fn is_terminal(&self) -> bool {
        self.get_reward() != Reward::Continue
    }

    fn get_reward(&self) -> Reward<Player> {
        if self.north == 0 {
            return Reward::WonBy(Player::South);
        }

        if self.south == 0 {
            return Reward::WonBy(Player::North);
        }

        let (n, s) = self.qmvs;
        if n == 20 || s == 20 {
            return Reward::Draw;
        }

        let possible_mvs = self.get_actions();

        if possible_mvs.len() == 0 {
            return Reward::WonBy(!self.turn);
        }

        Reward::Continue
    }

    fn apply_action(&self, action: &ActionList) -> Result<(Self, Player), AppError> {
        // let mut board = self.clone();
        let Some(state) = self.play(action) else {
            return Err(AppError::IllegalMove);
        };

        Ok((state, state.turn))
    }

    fn get_current_player(&self) -> &Player {
        &self.turn
    }

    fn view(&self) -> String {
        self.to_string()
    }

    fn get_actions(&self) -> Vec<ActionList> {
        self.options(self.turn)
    }

    // fn get_actions(&self) -> Vec<A>;
}
