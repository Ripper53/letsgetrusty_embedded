extern crate proc_macro;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Lifetime, LitInt, LitStr, parse_macro_input, spanned::Spanned};

#[derive(darling::FromDeriveInput, Debug)]
#[darling(attributes(test_runner_config))]
struct MetaData {
    error_message_size: LitInt,
}

#[proc_macro_derive(TestRunner, attributes(test_runner_config))]
pub fn test_runner(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let meta_data = MetaData::from_derive_input(&input)
        .expect("Expected `#[test_runner_config(error_message_size = ...)]` attribute");
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
                    (LitStr::new(&f.to_string(), input.span()), f),
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
        impl #impl_generics ::embedded_tester::TestRunner<#error_message_size> for #name #ty_generics #where_clause {
            fn execute<'zzzz>(self) -> impl ::core::iter::Iterator<Item = ::embedded_tester::TestResult<'zzzz, #error_message_size>> + 'zzzz {
                struct InnerIter #impl_generics {
                    index: usize,
                    test_runner: #name #ty_generics,
                }
                impl #impl_generics ::core::iter::Iterator
                    for InnerIter #ty_generics
                {
                    type Item = (&'static str, for<'a> fn(::embedded_tester::assertion::Assertion<'a, #error_message_size>) -> ::embedded_tester::TestResult<'a, #error_message_size>);
                    fn next(&mut self) -> ::core::option::Option<Self::Item> {
                        let r = match self.index {
                            #(#test_indexes => Some((#test_names, self.test_runner.#test_fields)),)*
                            _ => None,
                        };
                        self.index += 1;
                        r
                    }
                }
                InnerIter { index: 0, test_runner: self }.into_iter()
                    .map(|(test_name, f)| {
                        let test_error = ::embedded_tester::error::TestError::<'zzzz, #error_message_size>::new(test_name, ::embedded_tester::heapless::String::<#error_message_size>::new());
                        let assertion = ::embedded_tester::assertion::Assertion::<'zzzz, #error_message_size>::new(test_error);
                        ::embedded_tester::TestContext::<'zzzz, #error_message_size, _>::new(assertion, f).run()
                    })
            }
        }
    }
    .into()
}
