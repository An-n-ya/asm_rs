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
    let reg = arr[0].clone();
    let ident = match reg.as_str() {
        "rax" => "AX",
        "rbx" => "BX",
        "rcx" => "CX",
        "rdx" => "DX",
        "rsp" => "SP",
        "rbp" => "BP",
        "rsi" => "SI",
        "rdi" => "DI",
        _ => panic!("cannot handle this register {}", reg),
    };
    let reg_ident = Ident::new(ident, Span::call_site());
    branches.extend(quote! {
        #reg => crate::register::Register::#reg_ident(crate::register::Len::Full),
    });
    let reg = &arr[1];
    branches.extend(quote! {
        #reg => crate::register::Register::#reg_ident(crate::register::Len::Low32),
    });
    let reg = &arr[2];
    branches.extend(quote! {
        #reg => crate::register::Register::#reg_ident(crate::register::Len::Low16),
    });
    if arr.len() > 3 {
        let reg = &arr[3];
        branches.extend(quote! {
            #reg => crate::register::Register::#reg_ident(crate::register::Len::High8),
        });
        let reg = &arr[4];
        branches.extend(quote! {
            #reg => crate::register::Register::#reg_ident(crate::register::Len::Low8),
        });
    }
    branches.extend(quote! {
        _ => {
            return Err(nom::Err::Error(nom::error::Error::new("cannot parse register", nom::error::ErrorKind::Tag)));
        },
    });
    // branches.extend(quote! {
    //     _ => unreachable!("cannot recognized the register {}", res),
    // });
    let func_ident = Ident::new(&arr[0], Span::call_site());

    quote! {
        fn #func_ident(input: &str) -> nom::IResult<&str, crate::register::Register> {
            let (input, res) = nom::branch::alt((#tags))(input)?;
            let res = match res {
                #branches
            };
            Ok((input, res))
        }
    }
    .into()
}
