extern crate grid_2d;
extern crate proc_macro;
extern crate quote;
extern crate syn;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, ExprLit, ExprPath, Ident, Lit, Path, Token};

fn expect_u32(expr: &Expr) -> u32 {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(int), ..
        }) => int.base10_parse().expect("failed to parse int"),
        _ => panic!("unexpected expression type"),
    }
}

fn expect_path(expr: &Expr) -> Path {
    match expr {
        Expr::Path(ExprPath { path, .. }) => path.clone(),
        _ => panic!("unexpected expression type"),
    }
}

fn expect_ident(expr: &Expr) -> Ident {
    expect_path(expr)
        .get_ident()
        .expect("expected identifier")
        .clone()
}

#[derive(Debug)]
struct Args {
    radius: u32,
    array_ident: Ident,
    num_ident: Ident,
    coord_type: Path,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args = Punctuated::<Expr, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect::<Vec<_>>();
        let radius = expect_u32(&args[0]);
        let array_ident = expect_ident(&args[1]);
        let num_ident = expect_ident(&args[2]);
        let coord_type = expect_path(&args[3]);
        Ok(Self {
            radius,
            array_ident,
            num_ident,
            coord_type,
        })
    }
}

#[proc_macro]
pub fn make_circle(tokens: TokenStream) -> TokenStream {
    let Args {
        radius,
        array_ident,
        num_ident,
        coord_type,
    } = parse_macro_input!(tokens as Args);
    let expanded = quote! {
        const X: [#coord_type; 2] = [#coord_type::new(0, 0), #coord_type::new(1, 1)];
    };
    TokenStream::from(expanded)
}
