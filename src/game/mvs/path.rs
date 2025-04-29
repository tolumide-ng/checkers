use std::ptr::{null, null_mut};

use super::action::Action;

#[derive(Debug, Clone)]
pub struct MvPath {
    pub(crate) mv: Action,
    pub(crate) next: Option<Box<MvPath>>,
}

pub struct ActionList {
    pub(crate) head: Option<Box<MvPath>>,
    pub(crate) tail: *mut MvPath,
    pub(crate) len: usize,
    /// Used only internally for tracking the state of the iterator
    curr: *const MvPath,
}

impl ActionList {
    pub(crate) fn new() -> Self {
        Self {
            head: None,
            tail: null_mut(),
            len: 0,
            curr: null(),
        }
    }

    pub(crate) fn append(&mut self, mv: Action) {
        let mut node = Box::new(MvPath { mv, next: None });

        let start = self.tail.is_null();
        let new_tail: *mut MvPath = &mut *node;

        match start {
            true => {
                unsafe { (*self.tail).next = Some(node) };
                self.tail = new_tail;
            }
            false => {
                self.curr = &*node;
                self.head = Some(node);
                self.tail = new_tail;
            }
        };

        self.len += 1;
    }

    pub(crate) fn pop_front(&mut self) -> Option<Action> {
        let Some(curr) = self.head.take() else {
            return None;
        };

        match curr.next {
            Some(node) => {
                self.curr = &*node;
                self.head = Some(node);
                self.len -= 1;
            }
            None => self.tail = null_mut(),
        }

        return Some(curr.mv);
    }

    pub(crate) fn peek(&self) -> Option<Action> {
        self.head.as_ref().map(|a| a.mv)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl From<Vec<Action>> for ActionList {
    fn from(value: Vec<Action>) -> Self {
        let mut head = Self::new();

        value.into_iter().for_each(|a| head.append(a));

        head
    }
}

impl Iterator for ActionList {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.is_null() {
            return None;
        }

        let curr = unsafe { &(*self.curr).next };
        let MvPath { mv, .. } = unsafe { self.curr.read() };
        if let Some(new_curr) = curr {
            self.curr = &(*new_curr.as_ref());
        } else {
            self.curr = null()
        }

        return Some(mv);
    }
}

impl From<ActionList> for Vec<Action> {
    fn from(value: ActionList) -> Self {
        let mut list = Vec::with_capacity(value.len);

        for action in value {
            list.push(action);
        }

        list
    }
}
