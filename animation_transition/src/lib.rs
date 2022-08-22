mod traits;

mod prelude {
    pub use crate::traits::*;
}

pub use prelude::*;

// Re-export #[derive(AnimationTransitionMacro)].
//
// The reason re-exporting is not enabled by default is that disabling it would
// be annoying for crates that provide handwritten impls or data formats. They
// would need to disable default features and then explicitly re-enable std.
#[cfg(feature = "animation_transition_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate animation_transition_derive;
#[cfg(feature = "animation_transition_derive")]
#[doc(hidden)]
pub use animation_transition_derive::*;
