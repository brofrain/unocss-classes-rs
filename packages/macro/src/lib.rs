use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, ExprLit, Lit, LitStr, Token,
};
use unocss_classes_variant_group_transformer::transform;

struct UnoClassExpr(Expr);

impl Parse for UnoClassExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.parse()? {
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit_str),
                attrs,
            }) => {
                let transformed_value = transform(&lit_str.value());
                let new_lit_str = LitStr::new(&transformed_value, lit_str.span());

                Ok(Self(Expr::Lit(ExprLit {
                    lit: Lit::Str(new_lit_str),
                    attrs,
                })))
            }
            expr => Ok(Self(expr)),
        }
    }
}

struct UnoClasses(Punctuated<UnoClassExpr, Token![,]>);

impl Parse for UnoClasses {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input
            .parse_terminated(UnoClassExpr::parse, Token![,])
            .map(Self)
    }
}

impl ToTokens for UnoClasses {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let max_i = self.0.len() - 1;

        let expr_and_comma = self.0.iter().enumerate().map(|(i, expr)| {
            let UnoClassExpr(expr) = expr;
            if i == max_i {
                quote! { #expr }
            } else {
                quote! { #expr, }
            }
        });

        tokens.extend(quote! { unocss_classes::exports::classes::classes!(#(#expr_and_comma)*) });
    }
}

#[proc_macro]
pub fn uno(input: TokenStream) -> TokenStream {
    let classes = parse_macro_input!(input as UnoClasses);
    TokenStream::from(classes.into_token_stream())
}
