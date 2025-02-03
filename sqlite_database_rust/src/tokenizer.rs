use crate::parser::Simple_ast_tree;

/* Splits the string into words */
pub fn break_into_words(string: &str) {
    let tree = Simple_ast_tree::new();
    for word in string.split_whitespace() {
        tree.parse_into_tree(word);
    }
}