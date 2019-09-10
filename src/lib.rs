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

struct ZeroSized;

struct SouthSouthEastOctant {
    coords_anti_clockwise: Vec<Coord>,
    ends_on_diagonal: bool,
}

impl SouthSouthEastOctant {
    fn new(radius_squared: u32) -> Result<Self, ZeroSized> {
        if radius_squared == 0 {
            return Err(ZeroSized);
        }
        let mut radius = 0;
        while (radius + 1) * (radius + 1) <= radius_squared {
            radius += 1;
        }
        let mut current = Coord::new(0, radius as i32);
        let mut coords = Vec::new();
        let ends_on_diagonal = loop {
            coords.push(current);
            if current.x == current.y {
                break true;
            }
            current.x += 1;
            if current.magnitude2() > radius_squared {
                current.y -= 1;
            } else if current.x == current.y {
                // If the octant ends on a diagonal, and the last step was not diagonal, it will
                // create an awkward-looking bend at the point where this and the next octant meet.
                // This prevents emitting the final coord in such cases.
                break false;
            }
            if current.x > current.y {
                break false;
            }
        };
        Ok(Self {
            coords_anti_clockwise: coords,
            ends_on_diagonal,
        })
    }
    fn south_east_quadrant_anti_clockwise(&self) -> Vec<Coord> {
        let south_south_east = self.coords_anti_clockwise.iter();
        let mut east_south_east = self.coords_anti_clockwise[1..]
            .iter()
            .map(|&Coord { x, y }| Coord::new(y, x))
            .collect::<Vec<_>>();
        east_south_east.reverse();
        let start_offset = if self.ends_on_diagonal { 1 } else { 0 };
        south_south_east
            .chain(east_south_east[start_offset..].iter())
            .cloned()
            .collect()
    }
}

fn coord_rotate_anti_clockwise_90(Coord { x, y }: Coord) -> Coord {
    Coord::new(y, -x)
}

fn make_circle(radius_squared: u32) -> Vec<Coord> {
    let south_south_east = match SouthSouthEastOctant::new(radius_squared) {
        Err(ZeroSized) => return vec![Coord::new(0, 0)],
        Ok(octant) => octant,
    };
    let south_east_buffer = south_south_east.south_east_quadrant_anti_clockwise();
    let south_east = south_east_buffer.iter().cloned();
    let north_east = south_east.clone().map(coord_rotate_anti_clockwise_90);
    let north_west = north_east.clone().map(coord_rotate_anti_clockwise_90);
    let south_west = north_west.clone().map(coord_rotate_anti_clockwise_90);
    south_east
        .chain(north_east)
        .chain(north_west)
        .chain(south_west)
        .collect()
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
    let x_iter = circle.iter().map(|c| c.x);
    let y_iter = circle.iter().map(|c| c.y);
    let num = circle.len();
    let expanded = quote! {
        const #num_ident: usize = #num;
        const #array_ident: [#coord_type; #num_ident] = [ #( #coord_type::new(#x_iter, #y_iter), )* ];
    };
    TokenStream::from(expanded)
}
