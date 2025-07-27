extern crate proc_macro;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use heck::ToUpperCamelCase;
use syn::{
    Data, DeriveInput, Field, GenericArgument, Ident, Lifetime, LitInt, LitStr, Path,
    PathArguments, ReturnType, Type, TypePath, parse_macro_input, spanned::Spanned,
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
    quote::quote! {
        #[derive(Debug)]
        enum #test_types_ident {
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
        impl #impl_generics ::embedded_tester::TestRunner for #name #ty_generics #where_clause {
            fn execute(self) -> impl ::core::iter::Iterator<Item = ::embedded_tester::TestResult<'static, impl ::core::error::Error>> {
                struct InnerIter #impl_generics {
                    index: usize,
                    test_runner: #name #ty_generics,
                }
                impl #impl_generics ::core::iter::Iterator
                    for InnerIter #ty_generics
                {
                    type Item = ::embedded_tester::TestResult<'static, #test_types_ident>;
                    fn next(&mut self) -> ::core::option::Option<Self::Item> {
                        let r = match self.index {
                            #(#test_indexes => {
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
                        };
                        self.index += 1;
                        r
                    }
                }
                InnerIter { index: 0, test_runner: self }.into_iter()
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
