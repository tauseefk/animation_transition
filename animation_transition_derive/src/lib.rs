mod animation_transition_macro;

mod prelude {
    pub use proc_macro::TokenStream;
    pub use quote::quote;
    pub use syn::{self, parse_macro_input, DeriveInput};

    pub use crate::animation_transition_macro::*;
}

use prelude::*;

#[proc_macro_derive(AnimationTransitionMacro, attributes(variant))]
pub fn animation_transition_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    impl_animation_transition_macro(ast)
}
