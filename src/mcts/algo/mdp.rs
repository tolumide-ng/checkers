use crate::mcts::traits::{MCTSError, MctsAction, Player};

use super::state::State;

/// Makrov's Decision Process
pub trait MDP {
    /// Returns the reward for transitioning from state to nextState via action
    fn get_reward<S, A, P, E>(&self, action: A, next_state: S) -> Option<i64>
    where
        S: State<A, P, E>,
        A: MctsAction,
        P: Player,
        E: MCTSError;
}
