extern crate proc_macro;

use proc_macro::*;
use std::collections::{HashMap};
use std::fmt::Write as _;

#[proc_macro]
pub fn enum_parser(input: TokenStream) -> TokenStream {

    let src = input.to_string();

    let (name, ident_str) = get_name_and_ident_string_tuples(src);

    let flattened = CharTree::construct(ident_str);

    let mut out_src = String::new();


    out_src.parse().unwrap()

}

fn get_name_and_ident_string_tuples(src: String) -> (String, Vec<(String, String)>) {

    let mut in_string = false;
    let mut escaped = false;

    let mut tuples = Vec::new();

    let mut name = Vec::new();

    let mut ident = Vec::new();
    let mut raw_string = Vec::new();

    let mut contains_alpha = false;
    let mut is_first_ident = true;

    let mut prev_byte = 0u8;

    for c in src.bytes() {

        if in_string {
            if escaped && (c == b'"' || c == b'\\') {
                match c {
                    b'"' => ident.extend(b"Quote"),
                    b'\\' => ident.extend(b"BackSlash"),
                    _ => ()
                }
            } else {
                match c {
                    b'A'..= b'Z' | b'a'..= b'z' | b'_' => {
                        contains_alpha = true;
                        if prev_byte.is_ascii_digit() || prev_byte == b'_' {
                            ident.push(c.to_ascii_uppercase());
                        } else {
                            ident.push(c);
                        }
                    },
                    b'0'..=b'9' => {
                        if is_first_ident {
                            ident.push(b'_');
                        }
                        ident.push(c);
                    }
                    b'<' => ident.extend(b"Lt"),
                    b'>' => ident.extend(b"Gt"),
                    b'=' => {
                        match prev_byte {
                            b'-' => {
                                ident.pop();ident.pop();ident.pop();
                                ident.extend(b"Sub")
                            },
                            b'+' => {
                                ident.pop();ident.pop();ident.pop();
                                ident.extend(b"Add")
                            },
                            b'=' => ident.extend(b"uals"),
                            _ => ident.extend(b"Eq")
                        }
                    },
                    b'!' => ident.extend(b"Not"),
                    b'^' => ident.extend(b"Xor"),
                    b'-' => {
                        if prev_byte == 0 {
                            ident.extend(b"Neg")
                        } else if prev_byte == b'-' {
                            ident.pop();ident.pop();ident.pop();
                            ident.extend(b"Dec")
                        } else if !contains_alpha {
                            ident.extend(b"Sub")
                        }
                    },
                    b'+' => {
                        if prev_byte == 0 {
                            ident.extend(b"Pos");
                        } else if prev_byte == b'+' {
                            ident.pop();ident.pop();ident.pop();
                            ident.extend(b"Inc")
                        } else {
                            ident.extend(b"Add")
                        }
                    },
                    b'*' => ident.extend(b"Mul"),
                    b'/' => ident.extend(b"Div"),
                    b'%' => ident.extend(b"Mod"),
                    b'(' => ident.extend(b"OParen"),
                    b')' => ident.extend(b"CParen"),
                    b'[' => ident.extend(b"OBracket"),
                    b']' => ident.extend(b"CBracket"),
                    b'{' => ident.extend(b"OBrace"),
                    b'}' => ident.extend(b"CBrace"),
                    b'@' => ident.extend(b"At"),
                    b'#' => ident.extend(b"Hash"),
                    b'$' => ident.extend(b"Sigil"),
                    b'.' => ident.extend(b"Dot"),
                    b',' => ident.extend(b"Sep"),
                    b':' => ident.extend(b"Col"),
                    b';' => ident.extend(b"Semi"),
                    b'&' => {
                        if prev_byte != b'&' {
                            ident.extend(b"And")
                        }
                    },
                    b'|' => {
                        if prev_byte != b'|' {
                            ident.extend(b"Or")
                        }
                    },

                    b'"' => {
                        contains_alpha = false;
                        in_string = false;

                        ident[0] = ident[0].to_ascii_uppercase();

                        let ident_parsed = String::from_utf8(ident).unwrap();
                        ident = Vec::new();

                        let raw_string_parsed = String::from_utf8(raw_string).unwrap();
                        raw_string = Vec::new();

                        tuples.push((ident_parsed, raw_string_parsed));
                        continue
                    },
                    b'\\' => {
                        escaped = true;
                        prev_byte = c;
                        raw_string.push(c);
                        continue
                    }
                    _ => continue
                }
            }
            raw_string.push(c);
            is_first_ident = false;
        } else {
            match c {
                b'"' => {
                    is_first_ident = true;
                    in_string = true;
                    prev_byte = 0;
                    continue;
                },
                b'A' ..= b'Z' | b'a' ..= b'z' | b'0' ..= b'9' | b'_' => name.push(c),
                _ => (),
            }
        }

        prev_byte = c;
        escaped = false;
    }

    let name = String::from_utf8(name).unwrap();

    (name, tuples)
}

fn gen_enum(enum_name: String, ident_string_tuples: Vec<(String, String)>) -> String {
    let mut src_out = String::new();

    write!(src_out, "enum {} {{", enum_name).unwrap();

    for ident in ident_string_tuples {
        write!(src_out, "{},", ident.0).unwrap();
    }

    write!(src_out, "}}").unwrap();

    src_out
}

fn gen_parser(enum_name: String, ident_string_tuples: Vec<(String, String)>, char_tree: &CharTree) -> String {
    format!(r#"
        impl Parser for {} {{
            fn get_error(&self, src: &mut Source) -> ParseError {{
                {}
            }}

            fn get_suggestions(&self, partial: &[u8]) -> Vec<Suggestion> {{
                {}
            }}

            fn parse<'a>(&self, src: &mut Source, context: &'a mut LayoutContext) {{
                {}
            }}
        }}
        "#,
            enum_name,
            gen_get_error(&enum_name, &ident_string_tuples),
            gen_get_suggestions(&enum_name, &ident_string_tuples),
            gen_parse(&enum_name, char_tree)
    )
}

fn gen_get_error(enum_name: &String, ident_string_tuples: &Vec<(String, String)>) -> String {
    let mut error = String::new();

    write!(error, "expected one of");

    error
}

fn gen_get_suggestions(enum_name: &String, ident_string_tuples: &Vec<(String, String)>) -> String {
    todo!()
}

fn gen_parse(enum_name: &String, char_tree: &CharTree) -> String {
    todo!()
}

struct FlattenedChar {
    complete: Option<String>,
    next: HashMap<u8, usize>,
    prev: Option<usize>,
    cumulative: String,
}

struct CharTree {
    contents: Vec<FlattenedChar>,
}
impl CharTree {
    fn construct(strings: Vec<(String, String)>) -> CharTree {
        let mut char_tree = CharTree {
            contents: vec![FlattenedChar {
                complete: None,
                next: HashMap::new(),
                prev: None,
                cumulative: "".to_string(),
            }],
        };

        let mut current = 0usize;

        for (ident, raw_str) in strings {
            for (i, byte) in raw_str.bytes().enumerate() {
                let index = char_tree.contents.len();
                let curr = &mut char_tree.contents[current];

                if !curr.next.contains_key(&byte) {
                    curr.next.insert(byte, index);
                    char_tree.contents.push(FlattenedChar {
                        complete: None,
                        next: HashMap::new(),
                        prev: Some(current),
                        cumulative: raw_str[0..i].to_string()
                    });
                    current = index;
                } else {
                    current = curr.next[&byte];
                }
            }
            char_tree.contents[current].complete = Some(ident.clone());
        }

        char_tree
    }
}

fn gen_match(enum_name: &String, char_tree: &CharTree, start_index: usize) -> String {

    let mut src_out = String::new();
    let curr = &char_tree.contents[start_index];

    match &curr.complete {
        Some(ident) => {
            write!(src_out, "match src.peek() {{").unwrap();
            for char in curr.next.keys() {
                write!(src_out, r#"b'{char}' => {{
                    src.skip(1)
                    {};
                }},"#, gen_match(enum_name, char_tree, curr.next[char])).unwrap();
            }
            write!(src_out, "_ => Instr::{enum_name}({enum_name}::{ident}), ").unwrap();
        },
        None => {
            write!(src_out, "match src.next() {{").unwrap();
            for char in curr.next.keys() {
                write!(src_out, "b'{char}' => {},", gen_match(enum_name, char_tree, curr.next[char])).unwrap();
            }
            write!(src_out, "_ => Instr::ParseError{{ error: &'a self.get_error(src), suggestion: &'a self.get_suggestions(b\"{}\")}},", curr.cumulative).unwrap();
        },
    }

    write!(src_out, "}}").unwrap();

    src_out
}

