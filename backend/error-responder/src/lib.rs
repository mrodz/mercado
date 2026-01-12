use darling::{FromDeriveInput, FromVariant};
use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, parse_macro_input};

#[derive(Debug, FromDeriveInput, FromVariant, Default)]
#[darling(default, attributes(respond))]
struct Opts {
    #[darling(default)]
    status: Option<syn::Path>,
}

#[proc_macro_derive(ErrorResponder, attributes(respond))]
pub fn error_responder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, data, .. } = input;

    let response_status = match opts.status {
        Some(attr) => attr.to_token_stream(),
        None => quote! { ::rocket::http::Status::InternalServerError },
    };

    let enum_data = if let syn::Data::Enum(x) = data {
        x
    } else {
        panic!("Can only be used in an enum");
    };

    let variant_response_status = enum_data.variants.iter().map(|v| {
        let v_ident = &v.ident;

        // pattern must match `&self`, so it needs a leading `&`
        let pat = match &v.fields {
            syn::Fields::Unit => quote! { &Self::#v_ident },
            syn::Fields::Unnamed(_) => quote! { &Self::#v_ident ( .. ) },
            syn::Fields::Named(_) => quote! { &Self::#v_ident { .. } },
        };

        if let Ok(x) = Opts::from_variant(v) {
            if let Some(y) = x.status {
                let z = y.into_token_stream();
                quote! { #pat => #z, }
            } else {
                quote! {}
            }
        } else {
            quote! {}
        }
    });

    let output = quote! {
            impl<'r, 'o: 'r> ::rocket::response::Responder<'r, 'o> for #ident {
                fn respond_to(self, request: &'r ::rocket::Request<'_>) -> ::rocket::response::Result<'o> {
					let status = match &self {
						#(#variant_response_status)*
						_ => #response_status,
					};

					let mut res = ::rocket::serde::json::Json(Inner { error: self }).respond_to(request)?;
					res.set_status(status);
					Ok(res)
				}
            }

			#[derive(::rocket::serde::Serialize)]
            #[serde(crate = "::rocket::serde")]
            struct Inner { error: #ident }
        };

    output.into()
}
