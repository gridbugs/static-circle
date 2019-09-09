extern crate grid_2d;
extern crate proc_macro;
extern crate quote;
extern crate syn;
use grid_2d::Coord;
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
    radius_squared: u32,
    array_ident: Ident,
    num_ident: Ident,
    coord_type: Path,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args = Punctuated::<Expr, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect::<Vec<_>>();
        let radius_squared = expect_u32(&args[0]);
        let array_ident = expect_ident(&args[1]);
        let num_ident = expect_ident(&args[2]);
        let coord_type = expect_path(&args[3]);
        Ok(Self {
            radius_squared,
            array_ident,
            num_ident,
            coord_type,
        })
    }
}

fn make_south_east_octant(radius_squared: u32) -> Vec<Coord> {
    let mut radius = 0;
    while (radius + 1) * (radius + 1) <= radius_squared {
        radius += 1;
    }
    let mut current = Coord::new(0, radius as i32);
    let mut octant = Vec::new();
    loop {
        octant.push(current);
        if current.x == current.y {
            break;
        }
        current.x += 1;
        if current.magnitude2() > radius_squared {
            current.y -= 1;
        }
        if current.x > current.y {
            break;
        }
    }
    octant
}

fn make_circle(radius_squared: u32) -> Vec<Coord> {
    let make_south_east_octant = make_south_east_octant(radius_squared);
    let mut circle = Vec::new();
    circle
}

#[proc_macro]
pub fn circle_with_squared_radius(tokens: TokenStream) -> TokenStream {
    let Args {
        radius_squared,
        array_ident,
        num_ident,
        coord_type,
    } = parse_macro_input!(tokens as Args);
    let circle = make_circle(radius_squared);
    eprintln!("{:#?}", circle);
    let expanded = quote! {
        const X: [#coord_type; 2] = [#coord_type::new(0, 0), #coord_type::new(1, 1)];
    };
    TokenStream::from(expanded)
}
