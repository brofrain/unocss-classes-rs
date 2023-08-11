use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, ExprLit, Lit, LitStr, Token,
};
use unocss_classes_utils::transform_variant_groups;

struct UnoClasses(Punctuated<Punctuated<Expr, Token![=>]>, Token![,]>);

fn parse_uno_classes_expr(input: ParseStream) -> syn::Result<Expr> {
    match input.parse()? {
        Expr::Lit(ExprLit {
            lit: Lit::Str(lit_str),
            attrs,
        }) => {
            let transformed_value = transform_variant_groups(lit_str.value());
            let new_lit_str = LitStr::new(&transformed_value, lit_str.span());

            Ok(Expr::Lit(ExprLit {
                lit: Lit::Str(new_lit_str),
                attrs,
            }))
        }
        expr => Ok(expr),
    }
}

fn parse_uno_classes_punctuated_expr(
    input: ParseStream,
) -> syn::Result<Punctuated<Expr, Token![=>]>> {
    let mut punctuated_expr = Punctuated::new();

    punctuated_expr.push_value(parse_uno_classes_expr(input)?);

    if input.is_empty() || !input.peek(Token![=>]) {
        return Ok(punctuated_expr);
    }

    let punct = input.parse()?;
    punctuated_expr.push_punct(punct);

    punctuated_expr.push_value(parse_uno_classes_expr(input)?);

    Ok(punctuated_expr)
}

impl Parse for UnoClasses {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input
            .parse_terminated(parse_uno_classes_punctuated_expr, Token![,])
            .map(Self)
    }
}

impl ToTokens for UnoClasses {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let expr_and_comma = self.0.iter().enumerate().map(|(i, punctuated)| {
            let expr_and_arrow = punctuated
                .iter()
                .enumerate()
                .map(|(j, expr)| {
                    if j == 0 {
                        quote! { #expr }
                    } else {
                        quote! { => #expr }
                    }
                })
                .collect::<TokenStream2>();

            if i == 0 {
                quote! { #expr_and_arrow }
            } else {
                quote! { , #expr_and_arrow }
            }
        });

        tokens.extend(quote! { unocss_classes::exports::classes::classes!(#(#expr_and_comma)*) });
    }
}

#[proc_macro]
pub fn uno_classes(input: TokenStream) -> TokenStream {
    let classes = parse_macro_input!(input as UnoClasses);
    TokenStream::from(classes.into_token_stream())
}
