pub use crate::prelude::*;

pub trait AnimationLoop {
    fn page(&self) -> (usize, usize);
}

pub trait AnimationTransition<T: AnimationLoop> {
    fn wrapping_next_idx(&mut self) -> usize;
    fn transition_variant(&mut self, to: T);
}
