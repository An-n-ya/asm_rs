use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Expr, ExprArray, Ident, Lit};

#[proc_macro]
pub fn register_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ExprArray);
    let mut arr = vec![];
    for elem in &input.elems {
        match elem {
            Expr::Lit(expr_lit) => match &expr_lit.lit {
                Lit::Str(s) => {
                    arr.push(s.value());
                }
                _ => {}
            },
            _ => {}
        }
    }

    let mut tags = quote!();
    for elem in &arr {
        tags.extend(quote! {
            nom::bytes::complete::tag(#elem),
        });
    }
    let mut branches = quote! {};
    assert!(arr.len() >= 3, "size of arguments at least 3");
    branches.extend(quote! {
        stringify!(arr[0]) => crate::register::Register::BX(crate::register::Len::Full),
    });
    branches.extend(quote! {
        stringify!(arr[1]) => crate::register::Register::BX(crate::register::Len::Low32),
    });
    branches.extend(quote! {
        stringify!(arr[2]) => crate::register::Register::BX(crate::register::Len::Low16),
    });
    if arr.len() > 3 {
        branches.extend(quote! {
            stringify!(arr[3]) => crate::register::Register::BX(crate::register::Len::High8),
        });
        branches.extend(quote! {
            stringify!(arr[4]) => crate::register::Register::BX(crate::register::Len::Low8),
        });
    }
    branches.extend(quote! {
        _ => unreachable!(),
    });
    let func_ident = Ident::new(&arr[0], Span::call_site());

    quote! {
        fn #func_ident(input: &str) -> nom::IResult<&str, crate::register::Register> {
            let (res, input) = nom::branch::alt((#tags))(input)?;
            let res = match res {
                #branches
            };
            Ok((input, res))
        }
    }
    .into()
}
