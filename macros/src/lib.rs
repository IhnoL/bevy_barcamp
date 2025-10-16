use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parse,
    parse_macro_input,
    parse_quote,
    spanned::Spanned,
    DeriveInput,
    Expr,
    Ident,
    Path,
    Result,
    Token,
};

#[derive(Default)]
struct TestStepArgs {
    event_path: Option<Path>,
}

impl Parse for TestStepArgs {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        if ident != "event" {
            return Err(syn::Error::new(
                ident.span(),
                "unsupported attribute key; expected `event`",
            ));
        }
        input.parse::<Token![=]>()?;
        let path: Path = input.parse()?;
        Ok(Self {
            event_path: Some(path),
        })
    }
}

#[proc_macro_derive(TestStep, attributes(step_dispatch))]
pub fn derive_step_dispatch(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_step_dispatch(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand_step_dispatch(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    let ident = input.ident;
    let generics = input.generics;

    let mut event_path: Option<Path> = None;
    for attr in input.attrs.iter().filter(|attr| attr.path().is_ident("step_dispatch")) {
        let args: TestStepArgs = attr.parse_args()?;
        if event_path.is_some() {
            return Err(syn::Error::new(
                attr.span(),
                "duplicate `step_dispatch` attribute",
            ));
        }
        event_path = args.event_path;
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let mut where_clause = where_clause.cloned();

    let body = if let Some(path) = event_path {
        quote! {
            fn send(&self, world: &mut ::bevy::prelude::World) {
                world.trigger(#path);
            }
        }
    } else {
        let where_clause = where_clause.get_or_insert_with(|| syn::WhereClause {
            where_token: Default::default(),
            predicates: syn::punctuated::Punctuated::new(),
        });
        where_clause
            .predicates
            .push(parse_quote!(#ident #ty_generics: Clone));

        quote! {
            fn send(&self, world: &mut ::bevy::prelude::World) {
                world.trigger(self.clone());
            }
        }
    };

    let output = quote! {
        impl #impl_generics crate::includes::TestStep for #ident #ty_generics #where_clause {
            #body
        }

        impl #impl_generics ::std::fmt::Display for #ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", ::std::any::type_name::<#ident>())
            }
        }
    };

    Ok(output)
}

#[proc_macro]
pub fn step(input: TokenStream) -> TokenStream {
    let expression = parse_macro_input!(input as Expr);
    quote!(Box::new(#expression) as Box<dyn TestStep>).into()
}
