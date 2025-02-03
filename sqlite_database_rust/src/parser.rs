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

        while tmp.is_none() == true {

            if tmp.as_ref().map(|tmp_node| tmp_node.borrow_mut().right.is_some()) == Some(false) {
                tmp.as_ref().map(|tmp_node| tmp_node.borrow_mut().right = node);

                return Simple_ast_tree {
                    head: self.head.clone(),
                }
            }

            if tmp.as_ref().map(|tmp_node| tmp_node.borrow_mut().left.is_some()) == Some(false) {
                tmp.as_ref().map(|tmp_node| tmp_node.borrow_mut().left = node);

                return Simple_ast_tree {
                    head: self.head.clone(), 
                }
            }

            tmp = tmp.as_ref().map(|tmp_node| tmp_node.borrow_mut().left.clone()).expect("Issue");
        }

        return Simple_ast_tree {
            head: self.head.clone(),
        }
    }
}
