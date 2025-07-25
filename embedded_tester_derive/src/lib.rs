extern crate proc_macro;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{Data, DeriveInput, LitInt, LitStr, parse_macro_input, spanned::Spanned};

#[derive(darling::FromDeriveInput, Debug)]
#[darling(attributes(test_runner))]
struct MetaData {
    error_message_size: LitInt,
}

#[proc_macro_derive(TestRunner, attributes(test_runner))]
pub fn test_runner(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let meta_data = MetaData::from_derive_input(&input)
        .expect("Expected `#[test_runner(error_message_size = ...)]` attribute");
    let error_message_size = meta_data.error_message_size;
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (test_indexes, (test_names, test_fields)): (Vec<_>, (Vec<_>, Vec<_>)) = match &input.data {
        Data::Struct(data) => data
            .fields
            .iter()
            .enumerate()
            .map(|(index, f)| {
                let f = f.ident.as_ref().expect("Expected field to be named");
                (
                    LitInt::new(&index.to_string(), input.span()),
                    LitStr::new(&f.to_string(), input.span()),
                    f,
                )
            })
            .collect::<Vec<_>>()
            .into_iter()
            .unzip(),
        Data::Enum(_) => panic!("Expected a named struct, but found an enum"),
        Data::Union(_) => panic!("Expected a named struct, but found an union"),
    };
    let tests_count = LitInt::new(&test_fields.len().to_string(), input.span());
    quote::quote! {
        impl #impl_generics ::embedded_tester::TestRunner for #name #ty_generics #where_clause {
            fn execute(self) {
                struct Iter<'a, const ERROR_DESCRIPTION_SIZE: usize> {
                    index: usize,
                }
                impl<'a, const ERROR_DESCRIPTION_SIZE: usize> ::core::iter::Iterator
                    for Iter<'a, ERROR_DESCRIPTION_SIZE>
                {
                    type Item = fn(
                        ::embedded_tester::Assertion<'a, ERROR_DESCRIPTION_SIZE>,
                    ) -> TestResult<'a, ERROR_DESCRIPTION_SIZE>;
                    fn next(&mut self) -> ::core::option::Option<Self::Item> {
                        let r = match self.index {
                            #(#test_indexes => self.#test_fields)*
                            _ => None,
                        };
                        self.index += 1;
                        r
                    }
                }
                let mut test_error = ::embedded_tester::error::TestError::new("", ::embedded_tester::heapless::String::<#error_message_size>::new());
                #(
                    ::embedded_tester::TestContext::new(test_names, self.#test_fields.into()).run(test_error);
                )*
            }
        }
        impl #impl_generics #name #ty_generics #where_clause {
            fn run(self, test_error: ::embedded_tester::error::TestError) -> ::embedded_tester::TestResults<'a, #tests_count>> {
                let total_tests = self.tests.len();
                let mut errors: Vec<_, #tests_count> = self
                    .tests
                    .into_iter()
                    .filter_map(|test| {
                        if let Err(e) = test.run() {
                            Some(e)
                        } else {
                            None
                        }
                    })
                    .collect();
                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(TestErrors {
                        total_tests,
                        errors,
                    })
                }
            }
        }
    }
    .into()
}
