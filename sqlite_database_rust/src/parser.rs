use std::mem; 
use std::rc::Rc; 
use std::cell::RefCell;

enum type_keyword {
    terminal_keyword, 
    nonterminal_keyword, 
}

/* Should be a AST Tree */
#[derive(Debug)]
pub struct Simple_ast_tree<'a> {
    head: Option<Rc<RefCell<Node<'a>>>> 
}

#[derive(Debug)]
struct Node<'a>{
    left: Option<Rc<RefCell<Node<'a>>>>,
    right: Option<Rc<RefCell<Node<'a>>>>,
    val: Option<&'a str> 
}

/* 
    SELECT * FROM table 

          SELECT 
          /   \ 
        *     FROM 
             / 
            table 

    INSERT INTO Student (ROLL_NO, NAME, PHONE, AGE) VALUES 
        (1, Muny, 94324232, 20)

           

*/

impl<'a> Simple_ast_tree<'a> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn head(&self) -> Option<&str> { 
        return self.head.as_ref()
            .map(|node| node.borrow_mut().val)?;
    }

    pub fn insert(&mut self, string: &'a str) {

        if self.head.is_none() == true {
            self.head = Some(Rc::new(
                RefCell::new(Node {val: Some(string), left: None, right: None })));
            return;
        }

        let node = Rc::clone(self.head.as_ref().unwrap());

        loop {
            if node.borrow().left.is_none() == true {
                node.borrow_mut().left = Some(Rc::new(
                    RefCell::new(Node {val: Some(string), left: None, right: None}))); 
                return;
            }

            if node.borrow().right.is_none() == true {
                node.borrow_mut().right = Some(Rc::new(
                    RefCell::new(Node {val: Some(string), left: None, right: None}))); 
                return; 
            }

            let node = node.borrow().right.as_deref().expect("Error with going to the right node"); 
        }

    }


    /* pub fn iter(&self) -> Iter {
        if self.head.is_none() == true { 
            return; 
        }
        
        let node = Rc::clone(self.head.as_mut().unwrap());

    } */
}

pub struct Iter<'a> {
    values: Vec<&'a str>
}

/* impl<'a> Iterator for Iter<'a> {
    type Item = &'a str; 

    fn next(&mut self) -> Option<Self::Item> {
    }
} */

#[cfg(test)]
mod test {
    use crate::parser; 
    use crate::parser::Node;
    use core::cell::RefCell; 
    use std::rc::Rc;
   
    #[test]
    fn test_parser() {
        let mut tree = parser::Simple_ast_tree::new();
        tree.insert("SELECT");

        assert_eq!(tree.head(), Some("SELECT"));
    }

    #[test]
    fn test_parser_left() {
        let mut tree = parser::Simple_ast_tree::new();
        tree.insert("SELECT");
        tree.insert("*");

        let node = Rc::clone(&tree.head.unwrap());
        let left_node = node.borrow().left.as_deref().expect("Issue left").borrow().val;

        assert_eq!(left_node, Some("*"));
    }
}
