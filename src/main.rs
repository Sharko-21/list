use std::rc::Rc;
use std::rc::Weak;
use std::cell::{RefCell, Ref};
use std::fmt;
use std::fmt::{Formatter, Error};

fn main() {
    let mut list = List::new();
    list.insert_to_end(Node::new(String::from("first insert"), 3));
    list.insert_to_begin(Node::new(String::from("second insert"), 2));
    list.insert_to_begin(Node::new(String::from("second insert"), 123));
    list.insert_to_end(Node::new(String::from("third insert"), 4));
    list.insert_to_end(Node::new(String::from("third insert"), 5));
    list.insert_to_end(Node::new(String::from("fff insert"), 5));
    dbg!(list.find_by_key(String::from("third insert")));
    println!("{}", list.to_string())
}

struct Node {
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
    key: String,
    val: u32
}

impl Node {
    fn new(key: String, val: u32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node{next:Option::None, prev:Option::None, key, val}))
    }
}

struct List {
    next: Option<Rc<RefCell<Node>>>,
    end: Option<Rc<RefCell<Node>>>
}

impl List {
    fn new() -> List {
        List{next:Option::None, end:Option::None}
    }

    fn iter(&self) -> Iter {
        Iter {
            next: Some(self.next.clone().unwrap())
        }
    }

    fn insert_to_begin(&mut self, mut node: Rc<RefCell<Node>>) {
        match &self.next {
            Some( node_ref) => {
                match &self.end {
                    Some(_) => {
                        let node_ref_clone = node_ref.clone();
                        node_ref_clone.borrow_mut().prev = Option::from(Rc::downgrade(&node));
                        node.borrow_mut().next = Option::from(node_ref_clone);
                        self.next = Option::from(node);
                    }
                    None => {
                        self.next = Option::from(node.clone());
                        self.end = Option::from(node);
                    }
                    _ => {}
                }
            }
            None => {
                self.next = Option::from(node.clone());
                self.end = Option::from(node);
            }
            _ => {}
        }
    }

    fn insert_to_end(&mut self, mut node: Rc<RefCell<Node>>) {
        match &self.end {
            Some( node_ref) => {
                node_ref.borrow_mut().next = Option::from(node.clone());
                self.end = Option::from(node);
            }
            None => {
                self.next = Option::from(node.clone());
                self.end = Option::from(node);
            }
            _ => {}
        }
    }

    fn find_by_key(&self, key_to_search: String) -> Vec<u32> {
        let mut iter = &self.next;
        let mut done = false;
        let mut values = vec![];
        for (key, value) in self.iter() {
            println!("key {}", key);
            if key == key_to_search {
                values.push(value);
            }
        }
        return values;
    }
}

struct Iter {
    next: Option<Rc<RefCell<Node>>>
}
// currentNode
impl Iterator for Iter {
    type Item = (String, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_outer = None;
        let mut result = None;
        match &self.next {
            Some(node) => {
                let node  = node.borrow();
                match &node.next {
                    Some(next) => {
                        next_outer = Some(next.clone());
                    }
                    None => {
                        next_outer = None;
                    }
                }
                result = Some((node.key.clone(), node.val));
         }
            _ => {
                return None;
            }
        }
        self.next = next_outer;
        return result;
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut print_body: String = String::new();

        match &self.next {
            Some(node_ref) => {
                let mut go_on = true;
                let mut node = node_ref.clone();
                print_body += format!("{} ->", node.borrow().key).as_ref();
                while go_on {
                    match &node.clone().borrow().next {
                        Some(next_ref) => {
                            node = next_ref.clone();
                        }
                        None => {
                            go_on = false;
                        }
                        _ => {
                            go_on = false;
                        }
                    }
                    if go_on {
                        print_body += format!(" {} ->", node.borrow().key).as_ref();
                    }
                }
                print_body += format!(" NULL").as_ref();
            }
            None => {
                print_body =  "Empty list...".to_string();
            }
            _ => {}
        }

        write!(f, "{}", print_body)
    }
}