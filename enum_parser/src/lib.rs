extern crate proc_macro;

use proc_macro::*;
use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;

pub fn log_entry_and_exit(input: TokenStream) -> TokenStream {

    let mut tokens = input.into_iter();

    let ident = tokens.next().unwrap().to_string();

    let x = format!(r#"
        fn dummy() {{
            println!("entering");
            println!("args tokens: {{}}", {args});
            println!("input tokens: {{}}", {input});
            println!("exiting");
        }}
    "#,
                    args = args.into_iter().count(),
                    input = input.into_iter().count(),
    );

    let x = 0u8;

    match x {
        b'a' => {
            // ...
        }
    }

    x.parse().expect("Generated invalid tokens")
}

fn gen_parse(enum_name: String, strings: Vec<Vec<u8>>) -> String {
    format!(r#"
        impl Parser for {} {{
        fn get_error(&self, src: &mut Source) -> ParseError {{
            todo!()
        }}

        fn get_suggestions(&self, partial: &[u8]) -> Vec<Suggestion> {{
            todo!()
        }}

        fn parse(&self, src: &mut Source, context: &mut LayoutContext) {{
            todo!()
        }}
    }}
    "#, enum_name)
}

struct FlattenedChar {
    complete: Option<String>,
    next: HashMap<u8, usize>,
    prev: Option<usize>,
}

struct Flattened {
    contents: Vec<FlattenedChar>,
}
impl Flattened {
    fn construct(strings: Vec<String>) {
        let mut flattened = Flattened {
            contents: vec![FlattenedChar {
                complete: None,
                next: HashMap::new(),
                prev: None,
            }],
        };

        let mut current = 0usize;

        let max_len = strings.iter().map(|s| s.len()).max().unwrap();
        for i in 0..max_len {
            for string in strings {
                if string.len() <= i {
                    continue;
                }

                let curr = &flattened.contents[current];
                let next_byte = &string.as_bytes()[i];

                if curr.next.contains_key(next_byte) {

                } else {

                }

            }
        }

    }
}

fn gen_match(strings: Vec<Vec<u8>>) -> String {

    let mut ret = String::new();
    let mut used_chars = [false; 128];
    for mut string in strings {
        match string.pop() {
            Some(x) => {
                if !used_chars[0] {
                    used_chars[0] = true;
                    write!(ret, "b'{}' => {{  }}", x).unwrap()
                }
            },
            None => {}
        };

    };
    ret
}

