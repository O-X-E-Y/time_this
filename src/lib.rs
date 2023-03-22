#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, Expr, ItemFn};

/// this macro can be used to time any function you want using `std::time::Instant`. It may not work
/// correctly with `async fn` and it definitely doesn't work with `const fn` (even if called in a non-const
/// context. You can write a small wrapping fn if you need to time a `const fn`).
/// It will print:
/// * the time in ns if the function took less than 1μs.
/// * the time in μs if the function took less than 1ms.
/// * the time in ms if the function took longer than 1ms, but less than 1s.
/// * the time in s if the function took more than a second, with two decimal digits.
/// 
/// # Examples
/// 
/// ```
/// # use crate::time_this::time_this;
/// 
/// #[time_this]
/// fn add(a: u32, b: u32) -> u32 {
///     a + b
/// }
/// ```
/// 
/// Though it doesn't work on `const fn`:
/// 
/// ```compile_fail
/// # use crate::time_this::time_this;
/// 
/// #[time_this]
/// const fn const_fn(items: &[usize]) -> usize {
///     0
/// }
/// ```
#[proc_macro_attribute]
pub fn time_this(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(input as ItemFn);

    if sig.constness.is_some() {
        panic!("This macro does not work with `const fn`.")
    }

    let fn_name = sig.ident.to_string();

    quote! {
        #(#attrs)*
        #vis #sig {
            let __fn_name = #fn_name;
            
            #sig {
                #block
            }
            
            let __start_time = ::std::time::Instant::now();
            let res = { #block };

            let __elapsed_time = __start_time.elapsed();

            let __elapsed_msg = if __elapsed_time.checked_sub(::std::time::Duration::from_micros(1)).is_none() {
                format!("function '{}()' took {}ns", __fn_name, __elapsed_time.as_nanos())
    
            } else if __elapsed_time.checked_sub(::std::time::Duration::from_millis(1)).is_none() {
                format!("function '{}()' took {}μs", __fn_name, __elapsed_time.as_micros())
    
            } else if __elapsed_time.checked_sub(::std::time::Duration::from_millis(1000)).is_none() {
                format!("function '{}()' took {}ms", __fn_name, __elapsed_time.as_millis())
    
            } else {
                format!("function '{}()' took {:.2}s", __fn_name, __elapsed_time.as_secs_f64())
            };

            println!("{}", __elapsed_msg);

            res
        }
    }.into()
}

/// this macro can be used to time any expression you want using `std::time::Instant`. It returns the
/// result of the expression, similar to `dbg!()`. It may not work correctly with `async fn`.
/// It will print:
/// * the time in ns if the function took less than 1μs.
/// * the time in μs if the function took less than 1ms.
/// * the time in ms if the function took longer than 1ms, but less than 1s.
/// * the time in s if the function took longer than a second, with two decimal digits.
/// 
/// # Examples
/// 
/// ```
/// # use crate::time_this::time;
/// 
/// fn add(a: u32, b: u32) -> u32 {
///     a + b
/// }
/// 
/// fn main() {
///     let result = time!(add(3, 5));
/// }
/// ```
#[proc_macro]
pub fn time(input: TokenStream) -> TokenStream {
    let input_clone = input.clone();
    let expr = parse_macro_input!(input_clone as Expr);

    quote! {{
        let __str_expr = format!("[{}:{}]", file!(), line!());

        let __start_time = ::std::time::Instant::now();
        let res = #expr;

        let __elapsed_time = __start_time.elapsed();

        let __elapsed_msg = if __elapsed_time.checked_sub(::std::time::Duration::from_micros(1)).is_none() {
            format!("expression at {} took {}ns", __str_expr, __elapsed_time.as_nanos())

        } else if __elapsed_time.checked_sub(::std::time::Duration::from_millis(1)).is_none() {
            format!("expression at {} took {}μs", __str_expr, __elapsed_time.as_micros())

        } else if __elapsed_time.checked_sub(::std::time::Duration::from_millis(1000)).is_none() {
            format!("expression at {} took {}ms", __str_expr, __elapsed_time.as_millis())

        } else {
            format!("expression at {} took {:.2}s", __str_expr, __elapsed_time.as_secs_f64())
        };

        println!("{}", __elapsed_msg);

        res
    }}.into()
}