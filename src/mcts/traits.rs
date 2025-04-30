use std::{error::Error, fmt::Debug, hash::Hash};

/// Basic errors from this algo
pub trait MCTSError: Error {}

/// The game action (e.g move to)
pub trait MctsAction: Debug + Eq + PartialEq + Clone {}
/// The player(s) who would be playing this game
pub trait Player: Debug + PartialEq + Eq + Hash + Copy + Clone {}
