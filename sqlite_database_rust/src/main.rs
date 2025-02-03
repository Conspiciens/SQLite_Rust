mod parser; 
mod tokenizer;

use std::io; 

fn accept_text(string: &str) {
    tokenizer::break_into_words(string);
}

fn main() {
    let stdin = io::stdin; 
    let input = &mut String::new(); 

    loop {
        input.clear(); 
        stdin().read_line(input); 

        if input == "exit" {
            break; 
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser; 
   
    #[test]
    fn test_parser() {
        let tree = parser::Simple_ast_tree::new();
        let tree = tree.parse_into_tree("lol");
        let tree = tree.parse_into_tree("hello");

        assert_eq!(tree.head(), Some("lol"));
    }
}