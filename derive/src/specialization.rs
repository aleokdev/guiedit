use quote::quote;

/// A structure for building [generalized autoref-based specialization] implementations.
///
/// Useful for making the behavior of a type change depending on its trait implementations.
/// In this crate, it is used to provide a default implementation of TreeNode and Inspectable for
/// types that do not implement it.
///
/// [generalized autoref-based specialization]: http://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html#using-autoderef-for--two-specialization-levels
pub struct Specialization {
    stream: proc_macro2::TokenStream,
}
pub struct TypeSpecialization<'s> {
    specialization: &'s mut Specialization,
    r#trait: syn::TypePath,
    wrap: syn::TypePath,
}

impl TypeSpecialization<'_> {
    pub fn add_case_for_bounds(
        self,
        bounds: syn::TraitBound,
        contents: proc_macro2::TokenStream,
    ) -> Self {
        let r#trait = &self.r#trait;
        let wrap = &self.wrap;
        let stream = &self.specialization.stream;
        self.specialization.stream = quote! {
            #stream

            impl<T: #bounds> #r#trait for #wrap {
                #contents
            }
        };

        TypeSpecialization {
            wrap: syn::parse_quote! { Wrap<#wrap> },
            ..self
        }
    }
}

impl Specialization {
    pub fn new() -> Self {
        Self {
            stream: quote! {
                struct Wrap<T>(T);

                impl<T> std::ops::Deref for Wrap<T> {
                    type Target = T;

                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }

                impl<T> std::ops::DerefMut for Wrap<T> {
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        &mut self.0
                    }
                }
            },
        }
    }

    pub fn default_case<'s>(
        &'s mut self,
        r#trait: syn::TypePath,
        contents: proc_macro2::TokenStream,
    ) -> TypeSpecialization<'s> {
        let stream = &self.stream;
        let wrap = quote! { Wrap<&mut T> };
        self.stream = quote! {
            #stream
            impl<T> #r#trait for #wrap {
                #contents
            }
        };

        TypeSpecialization {
            specialization: self,
            r#trait,
            wrap: syn::parse_quote! { Wrap<#wrap> },
        }
    }

    pub fn build(self) -> proc_macro2::TokenStream {
        self.stream
    }
}
