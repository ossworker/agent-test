use proc_macro::TokenStream;

#[proc_macro]
pub fn call_jvmti(input: TokenStream) -> TokenStream {
    for tt in input.into_iter() {
        println!("tt: {:#?}", tt)
    }
    TokenStream::new()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
