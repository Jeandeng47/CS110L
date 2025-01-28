use std::fmt;
use std::option::Option;

pub struct LinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

struct Node {
    value: u32,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: u32, next: Option<Box<Node>>) -> Node {
        Node {value: value, next: next}
    }
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {head: None, size: 0}
    }
    
    pub fn get_size(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    
    pub fn push_front(&mut self, value: u32) {
        let new_node: Box<Node> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<u32> {
        let node: Box<Node> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}


// Display trait: define how linked list should be when
// using the {} placehodler
impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node>> = &self.head;
        let mut result = String::new(); // Accmuluate the string
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

// Drop trait: define custom cleanup logic when an object 
// goes out of scope (default Drop: recursive call)
impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            // take(): take ownership of the node, leaving none
            current = node.next.take();
        }
    }
}



