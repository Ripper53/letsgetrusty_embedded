extern crate proc_macro;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{
    Data, DeriveInput, Ident, Lifetime, LitInt, LitStr, Type, TypePath, parse_macro_input,
    spanned::Spanned,
};

#[derive(darling::FromDeriveInput, Debug)]
#[darling(attributes(test_runner_config))]
struct MetaData {
    error_message_size: LitInt,
    error_message_type: Option<Type>,
}

#[proc_macro_derive(TestRunner, attributes(test_runner_config))]
pub fn test_runner(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let meta_data = MetaData::from_derive_input(&input)
        .expect("Expected `#[test_runner_config(error_message_size = ...)]` attribute");
    let error_message_size = meta_data.error_message_size;
    /*let error_message_type = if let Some(e) = meta_data.error_message_type {
        e
    } else {
        todo!()
        //quote::quote!(::embedded:assertion::AssertionFailure)
        //Type::Path(TypePath::)
    };*/
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
        impl #impl_generics ::embedded_tester::TestRunner for #name #ty_generics #where_clause {
            type Error = ::embedded_tester::error::TestError<'static, #error_message_size>;
            fn execute<'zzzz>(self) -> impl ::core::iter::Iterator<Item = ::embedded_tester::TestResult<'zzzz, Self::Error>> + 'zzzz {
                struct InnerIter #impl_generics {
                    index: usize,
                    test_runner: #name #ty_generics,
                }
                impl #impl_generics ::core::iter::Iterator
                    for InnerIter #ty_generics
                {
                    type Item = (&'static str, for<'a> fn(::embedded_tester::assertion::Assertion<'a>) -> ::core::result::Result<(), ::embedded_tester::assertion::AssertionFailure<#error_message_size>>);
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
                        //let test_error = ::embedded_tester::error::TestError::<'zzzz, #error_message_size>::new(test_name, ::embedded_tester::heapless::String::<#error_message_size>::new());
                        let assertion = ::embedded_tester::assertion::Assertion::new(test_name);
                        match ::embedded_tester::TestContext::new(assertion, f).run() {
                            Ok(s) => Ok(s),
                            Err(e) => {
                                Err(::embedded_tester::error::TestError::new(test_name, e.take_error_description()))
                            }
                        }
                    })
            }
        }
    }
    .into()
}
