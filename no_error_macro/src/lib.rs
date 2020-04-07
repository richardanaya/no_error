extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro::TokenTree;
use proc_macro_hack::proc_macro_hack;
use std::str::FromStr;

#[proc_macro_hack]
pub fn error(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter().peekable();
    let mut description;
    let mut source;
    if let Some(TokenTree::Literal(t)) = tokens.next() {
        description = format!("{}",t);
    } else {
        panic!("first param of error! should be a string")
    }

    if description.chars().last().unwrap() != '"' {
        panic!("first param of error! should be a string")
    }
    description.pop();
    println!("{}",file!());
    if let None = tokens.peek() {
        #[cfg(build = "release")]
        return TokenStream::from_str(&format!("Err(({}\\0\",\"\"))", description)).unwrap();
    }

    if let Some(TokenTree::Punct(p)) = tokens.next() {
        if p.as_char() != ',' {
            panic!("expected , in error!")
        }
    }

    if let Some(TokenTree::Literal(t)) = tokens.next() {
        source = format!("{}",t);
    } else {
        panic!("second param of error! should be a string")
    }


    if source.chars().last().unwrap() != '"' {
        panic!("second param of error! should be a string")
    }
    source.pop();

    return TokenStream::from_str(&format!("Err(({}\\0\",{}\\0\"))", description, source)).unwrap();
}
