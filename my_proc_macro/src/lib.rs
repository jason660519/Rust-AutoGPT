extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

/// 使用外部的 proc_macro crate，它允许我们定义过程宏
///
/// proc_macro 是 Rust 的一个内置库，用于编写宏。
/// TokenStream 是 proc_macro 库中的一个类型，它表示 Rust 编译器解析的代码块。
/// quote 和 ToTokens 是 quote crate 中的模块，用于生成 Rust 代码。
/// parse_macro_input 和 ItemFn 是 syn crate 中的模块，用于解析 Rust 语法结构。
///

#[proc_macro_attribute]
/// 定义一个过程宏函数 `function_to_string`，用于将函数转换为字符串表示
/// @param _attr: TokenStream 未使用的宏属性
/// @param item: TokenStream 输入的函数
/// @return: TokenStream 输出新的函数
pub fn function_to_string(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析输入的函数，将 TokenStream 转换为结构化的 `ItemFn`
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
   
    // 创建函数的字符串表示
    // 使用 `ToTokens` trait 将 `input_fn` 转换为 TokenStream，再转换为字符串
    let function_str: String = format!("{}", input_fn.to_token_stream());
   
    // 定义一个具有与输入函数相同签名的新函数
    let fn_ident: proc_macro2::Ident = input_fn.sig.ident; // 获取函数的名字
    let fn_inputs: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma> = input_fn.sig.inputs; // 获取函数参数
    let fn_generics: syn::Generics = input_fn.sig.generics; // 获取泛型参数
   
    // 生成输出函数
    // 使用 `quote!` 宏生成一个新的函数，该函数返回输入函数的字符串表示
    let output: proc_macro2::TokenStream = quote! {
        pub fn #fn_ident #fn_generics(#fn_inputs) -> &'static str {
            #function_str
        }
    };
    // 将生成的 TokenStream 转换回 proc_macro 的 TokenStream 类型
    output.into()
}