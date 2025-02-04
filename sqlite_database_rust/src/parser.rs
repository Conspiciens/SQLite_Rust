use std::mem; 
use std::rc::Rc; 
use std::cell::RefCell;

enum type_keyword {
    terminal_keyword, 
    nonterminal_keyword, 
}

/* Should be a AST Tree */
pub struct Simple_ast_tree<'a> {
    head: Option<Rc<RefCell<Node<'a>>>> 
}

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
        return Simple_ast_tree { 
            head: None
        }
    }

    pub fn head(&self) -> Option<&str> { 
        return self.head.as_ref().map(|node| node.borrow_mut().val)?;
    }

    pub fn parse_into_tree(&self, string: &'a str) -> Simple_ast_tree<'a> {
        let mut tmp: Option<Rc<RefCell<Node<'a>>>> = self.head.clone();

        let node = Some(Rc::new(RefCell::new(Node {
            left: None, 
            right: None, 
            val: Some(string)
        }))); 

        if tmp.is_none() == true {
            return Simple_ast_tree {
                head: node,
            }
        }

        while tmp.is_none() == false {
            if tmp.as_ref().map(|tmp_node| tmp_node.borrow_mut().left.is_some()) == Some(false) {
                tmp.as_ref().map(|tmp_node| tmp_node.borrow_mut().left = node);

                return Simple_ast_tree {
                    head: self.head.clone(), 
                }
            }

            if tmp.as_ref().map(|tmp_node| tmp_node.borrow_mut().right.is_some()) == Some(false) {
                tmp.as_ref().map(|tmp_node| tmp_node.borrow_mut().right = node);

                return Simple_ast_tree {
                    head: self.head.clone(),
                }
            }

            tmp = tmp.as_ref().map(|tmp_node| tmp_node.borrow_mut().right.clone()).expect("Issue");
        }

        return Simple_ast_tree {
            head: self.head.clone(),
        }
    }

    pub fn iter(&self) -> Iter {
        let mut tmp: Option<Rc<RefCell<Node>>> = self.head.clone(); 
        let mut stack: Vec<&'a str> = Vec::new(); 

        while tmp.is_none() == false {
            if tmp.as_ref().map(|node| node.borrow_mut().right.is_some()) == Some(true) {
                let right_node = tmp.as_ref().map(|node| node.borrow_mut().right.clone()).expect("None"); 
                stack.push(right_node.as_ref().map(|node| node.borrow_mut().val.expect("None")).expect("None"));
                continue; 
            }

            if tmp.as_ref().map(|node| node.borrow_mut().left.is_some()) == Some(true) {
                let left_node = tmp.as_ref().map(|node| node.borrow_mut().left.clone()).expect("None");
                stack.push(left_node.as_ref().map(|node| node.borrow_mut().val.expect("None")).expect("None"));
                continue; 
            }

            tmp = tmp.as_ref().map(|node| node.borrow_mut().right.clone()).expect("Can't move to the right");
        }

        Iter { values: stack }
    }
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
        let tree = parser::Simple_ast_tree::new();
        let tree = tree.parse_into_tree("SELECT");

        assert_eq!(tree.head(), Some("SELECT"));
    }

    #[test]
    fn test_parser_left() {
        let tree = parser::Simple_ast_tree::new();
        let tree = tree.parse_into_tree("SELECT");
        let tree = tree.parse_into_tree("*");

        let tmp: Option<Rc<RefCell<Node>>> = tree.head.clone();
        let tmp = tmp.as_ref().map(|node| node.borrow_mut().left.clone()).expect("None"); 

        assert_eq!(tmp.as_ref().map(|node| node.borrow_mut().val.expect("None")), Some("*"));
    }
}
