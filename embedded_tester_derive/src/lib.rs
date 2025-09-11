extern crate proc_macro;
use heck::ToUpperCamelCase;
use proc_macro::TokenStream;
use syn::{
    Data, DeriveInput, Field, GenericArgument, Ident, LitInt, LitStr, PathArguments, ReturnType,
    Type, TypePath, parse_macro_input, spanned::Spanned,
};

#[proc_macro_derive(TestRunner)]
pub fn test_runner(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (test_indexes, (test_names, (test_fields, (test_error_name, test_error)))): (
        Vec<_>,
        (Vec<_>, (Vec<_>, (Vec<_>, Vec<_>))),
    ) = match &input.data {
        Data::Struct(data) => data
            .fields
            .iter()
            .enumerate()
            .map(|(index, f)| {
                let error_type = extract_error_type(f)
                    .expect("Expected `... -> Result<(), E>` where `E` is an error type");
                let f = f.ident.as_ref().expect("Expected field to be named");
                let error_name = Ident::new(&f.to_string().to_upper_camel_case(), input.span());
                (
                    LitInt::new(&index.to_string(), input.span()),
                    (
                        LitStr::new(&f.to_string(), input.span()),
                        (f, (error_name, error_type)),
                    ),
                )
            })
            .collect::<Vec<_>>()
            .into_iter()
            .unzip(),
        Data::Enum(_) => panic!("Expected a named struct, but found an enum"),
        Data::Union(_) => panic!("Expected a named struct, but found an union"),
    };
    let test_types_ident = Ident::new(&format!("{name}Error"), input.span());
    let iter_ident = Ident::new(&format!("{name}Iter"), input.span());
    quote::quote! {
        #[derive(Debug)]
        pub enum #test_types_ident {
            #(#test_error_name(#test_error),)*
        }
        impl ::core::fmt::Display for #test_types_ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    #(Self::#test_error_name(e) => write!(f, "{e}"),)*
                }
            }
        }
        impl ::core::error::Error for #test_types_ident {}
        pub struct #iter_ident #impl_generics {
            index: usize,
            test_runner: #name #ty_generics,
        }
        impl #impl_generics ::core::iter::Iterator
            for #iter_ident #ty_generics
        {
            type Item = ::embedded_tester::TestResult<'static, #test_types_ident>;
            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                match self.index {
                    #(#test_indexes => {
                        self.index += 1;
                        let assertion = ::embedded_tester::assertion::Assertion::new(#test_names);
                        let f = self.test_runner.#test_fields;
                        let r = match ::embedded_tester::TestContext::new(assertion, f).run() {
                            Ok(s) => Ok(s),
                            Err(e) => Err(e.map_error(|e| {
                                #test_types_ident::#test_error_name(e)
                            })),
                        };
                        Some(r)
                    })*
                    _ => None,
                }
            }
        }
        impl #impl_generics ::embedded_tester::TestRunner for #name #ty_generics #where_clause {
            type Error = #test_types_ident;
            type Iterator = #iter_ident;
            fn execute(self) -> Self::Iterator {
                Self::Iterator { index: 0, test_runner: self }
            }
        }
    }
    .into()
}

fn extract_error_type(field: &Field) -> Option<&Type> {
    if let Type::BareFn(fn_type) = &field.ty
        && let ReturnType::Type(_, ret_type) = &fn_type.output
        && let Type::Path(TypePath { path, .. }) = &**ret_type
        && let Some(seg) = path.segments.last()
        && seg.ident == "Result"
        && let PathArguments::AngleBracketed(ref args) = seg.arguments
        && args.args.len() == 2
        && let GenericArgument::Type(err_ty) = &args.args[1]
    {
        Some(err_ty)
    } else {
        None
    }
}

/// `TestScheduler` requires every inner `TestRunner` to implement the `Default` trait.
#[proc_macro_derive(TestScheduler)]
pub fn test_scheduler(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (test_names, (field_index, (fields, (field_types, (generic_variant, error_names))))): (
        Vec<_>,
        (Vec<_>, (Vec<_>, (Vec<_>, (Vec<_>, Vec<_>)))),
    ) = match &input.data {
        Data::Struct(data) => data
            .fields
            .iter()
            .enumerate()
            .map(|(i, f)| {
                if let Type::Path(ref path) = f.ty {
                    let type_name = path.path.get_ident().expect("Expected name").to_string();
                    let field_ident = f.ident.as_ref().expect("Expected named field").to_string();
                    let generic_name = Ident::new(&field_ident.to_upper_camel_case(), input.span());
                    let error = Ident::new(&format!("{type_name}Error"), input.span());
                    (
                        LitStr::new(&format!("{field_ident}:"), input.span()),
                        (
                            LitInt::new(&i.to_string(), input.span()),
                            (
                                Ident::new(&field_ident, input.span()),
                                (&f.ty, (generic_name, error)),
                            ),
                        ),
                    )
                } else {
                    panic!("Expected type path")
                }
            })
            .unzip(),
        Data::Enum(_) => panic!("Test suite must be a struct, but found an enum"),
        Data::Union(_) => panic!("Test suite must be a struct, but found an union"),
    };
    let test_types_ident = Ident::new(&format!("{name}Error"), input.span());
    let iter_ident = Ident::new(&format!("{name}Iter"), input.span());
    let iter_enum_ident = Ident::new(&format!("{name}IterType"), input.span());
    quote::quote! {
        #[derive(Debug)]
        pub enum #test_types_ident {
            #(#error_names(#error_names),)*
        }
        impl ::core::fmt::Display for #test_types_ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    #(Self::#error_names(e) => e.fmt(f),)*
                }
            }
        }
        impl ::core::error::Error for #test_types_ident {}
        #(
            impl From<#error_names> for #test_types_ident {
                fn from(e: #error_names) -> Self {
                    Self::#error_names(e)
                }
            }
        )*
        impl #impl_generics ::core::default::Default for #name #ty_generics #where_clause {
            fn default() -> Self {
                Self {
                    #(#fields: #field_types::default(),)*
                }
            }
        }
        impl #impl_generics ::embedded_tester::TestScheduler for #name #ty_generics #where_clause {
            fn execute_suites(self, logger: impl ::embedded_tester::TestLogger) {
                #(
                    logger.log_test_suite_name(#test_names);
                    for test_runner in ::embedded_tester::TestRunner::execute(self.#fields) {
                        logger.log_test_result(test_runner);
                    }
                )*
            }
        }
        pub struct #iter_ident {
            index: usize,
            iter_type: ::core::option::Option<#iter_enum_ident>,
            test_runners: (#(::core::option::Option<#field_types>,)*),
        }
        impl ::core::iter::Iterator for #iter_ident {
            type Item = ::embedded_tester::TestResult<'static, #test_types_ident>;
            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                if let Some(ref mut iter) = self.iter_type && let Some(value) = iter.next() {
                    Some(value)
                } else {
                    loop {
                        let mut iter_type = match self.index {
                            #(#field_index => {
                                self.index += 1;
                                #iter_enum_ident::#generic_variant(::embedded_tester::TestRunner::execute(self.test_runners.#field_index.take().unwrap()).into_iter().map(|test_result| {
                                    test_result
                                        .map_err(|e| e.map_error(#test_types_ident::#error_names))
                                }))
                            })*
                            _ => { break None; }
                        };
                        if let Some(value) = iter_type.next() {
                            self.iter_type = Some(iter_type);
                            break Some(value);
                        }
                    }
                }
            }
        }
        pub enum #iter_enum_ident {
            #(#generic_variant(::core::iter::Map<<#field_types as ::embedded_tester::TestRunner>::Iterator, fn(::embedded_tester::TestResult<'static, <#field_types as ::embedded_tester::TestRunner>::Error>) -> ::embedded_tester::TestResult<'static, #test_types_ident>>),)*
        }
        impl ::core::iter::Iterator for #iter_enum_ident {
            type Item = ::embedded_tester::TestResult<'static, #test_types_ident>;
            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    #(Self::#generic_variant(iter) => if let Some(value) = iter.next() {
                        Some(value.into())
                    } else {
                        None
                    },)*
                }
            }
        }
        impl #impl_generics ::embedded_tester::TestRunner for #name #ty_generics #where_clause {
            type Error = #test_types_ident;
            type Iterator = #iter_ident;
            fn execute(self) -> Self::Iterator {
                Self::Iterator {
                    index: 0,
                    iter_type: None,
                    test_runners: (#(Some(self.#fields),)*),
                }
            }
        }
    }
    .into()
}
