use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{self, Expr, ExprAssign, Token};

struct BtreeExps(Vec<(Expr, Expr)>);

/// Parses strings like [1 = 2, 3 = 4]
impl Parse for BtreeExps {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut v: Vec<(Expr, Expr)> = Vec::new();

        while !input.is_empty() {
            let exp: ExprAssign = input.parse()?;
            v.push((*exp.left, *exp.right));
            let _ = input.parse::<Token![,]>();
        }

        Ok(BtreeExps(v))
    }
}

#[proc_macro]
pub fn btreemap(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    init_btreemap(&ast)
}

fn init_btreemap(ast: &BtreeExps) -> TokenStream {
    let exps = {
        let qs = ast.0.iter().map(|(left, right)| {
            quote! {
                #left, #right
            }
        });
        qs
    };
    let gen = quote! {{
        use std::collections::BTreeMap;
        let mut v = BTreeMap::new();
        #( v.insert(#exps); )*
        v
    }};
    gen.into()
}
