pub use crate::prelude::*;

pub fn impl_animation_transition_macro(ast: syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    let generics = &ast.generics;

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!();
    };

    let variant_type = fields.iter().find_map(|f| {
        let field_with_variant = variant_of(f);
        let ty = &f.ty;

        match field_with_variant {
            Some(_) => Some(ty.clone()),
            None => None,
        }
    });

    let gen = match variant_type {
        Some(ty) => {
            quote! {
                impl #impl_generics AnimationTransition<#ty> for #struct_name #type_generics #where_clause {
                    fn wrapping_next_idx(&mut self) -> usize {
                        let current_idx = self.idx;
                        let (offset, size) = self.variant.page();

                        self.idx = offset + (current_idx + 1) % size;

                        self.idx
                    }

                    fn transition_variant(&mut self, to: #ty) {
                        let (offset, _) = to.page();

                        if self.variant != to {
                            self.variant = to;
                            self.idx = offset;
                        }
                    }
                }
            }
        }
        None => {
            quote! {
                compile_error!("No field marked with `variant` attribute found.")
            }
        }
    };

    gen.into()
}

fn variant_of(f: &syn::Field) -> Option<&syn::Attribute> {
    for attr in &f.attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "variant" {
            return Some(attr);
        }
    }
    None
}
