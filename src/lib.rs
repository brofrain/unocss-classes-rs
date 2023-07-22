#![warn(clippy::pedantic)]

use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use regex::{Captures, Regex};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Expr, ExprLit, Lit, LitStr, Token,
};

static CLASS_GROUP_REG: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?m)((?:[!@<~\w+:_/-]|\[&?>?:?\S*\])+?)(-|:)\(((?:[~!<>\w\s:/\\,%#.$?-]|\[.*?\])+?)\)",
    )
    .unwrap()
});
static WHITESPACE_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s").unwrap());
static WHITESPACES_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());
static IMPORTANCE_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(!?)(.*)").unwrap());

const SEPARATORS: [&str; 2] = ["-", ":"];
const DEPTH: u8 = 10;

fn shallow_transform(str: &str) -> String {
    let str = WHITESPACES_REG.replace_all(str, " ");

    CLASS_GROUP_REG
        .replace_all(str.trim(), |caps: &Captures| {
            if !SEPARATORS.contains(&&caps[2]) {
                return caps[0].to_string();
            }

            WHITESPACE_REG
                .split(&caps[3])
                .filter(|item| !item.is_empty())
                .map(|item| {
                    if item == "~" {
                        caps[1].to_string()
                    } else {
                        IMPORTANCE_REG
                            .replace(item, format!("${{1}}{}{}${{2}}", &caps[1], &caps[2]))
                            .to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join(" ")
        })
        .into_owned()
}

// based on https://github.com/unocss/unocss/blob/main/packages/core/src/utils/variantGroup.ts
fn transform(str: &str) -> String {
    let mut depth = DEPTH;
    let mut previous = String::from(str);

    loop {
        let transformed = shallow_transform(&previous);
        depth -= 1;

        if transformed == previous || depth == 0 {
            break previous;
        }

        previous = transformed;
    }
}

struct UnoClassExpr(LitStr);

const ERROR_MSG: &str = "Only string literals are allowed";

impl Parse for UnoClassExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.parse()? {
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit_str),
                ..
            }) => {
                let transformed_value = transform(&lit_str.value());
                let new_lit_str = LitStr::new(&transformed_value, lit_str.span());

                Ok(Self(new_lit_str))
            }
            expr => Err(syn::Error::new(expr.span(), ERROR_MSG)),
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

        let class_and_space = self.0.iter().enumerate().map(|(i, class_expr)| {
            let UnoClassExpr(class) = class_expr;
            if i == max_i {
                quote! { #class }
            } else {
                quote! { #class, ' ', }
            }
        });

        tokens.extend(quote! { concat!(#(#class_and_space)*) });
    }
}

/// TODO: docs
///
/// # Example
///
/// ```
/// use unocss_variant_group_transformer::uno;
///
/// assert_eq!(uno!("text-red"), "text-red");
///
/// assert_eq!(uno!("text-(red sm)"), "text-red text-sm");
///
/// assert_eq!(
///     uno!("text-(blue lg)", "placeholder:(italic text-(red sm))"),
///     "text-blue text-lg placeholder:italic placeholder:text-red placeholder:text-sm"
/// );
/// ```
#[proc_macro]
pub fn uno(input: TokenStream) -> TokenStream {
    let classes = parse_macro_input!(input as UnoClasses);
    TokenStream::from(classes.into_token_stream())
}
