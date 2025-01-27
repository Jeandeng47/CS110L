
struct Node {
    value : u32,
    next: Option<Box<Node>>,  // Box or Null
    // next: &Node -> Error: who to borrow from?
}

struct LinkedList {
    head: Option<Box<Node>>, // Head or Null head
    size: usize,
}


impl Node {
    // Constructor of node
    pub fn new(value: u32, next: Option<Box<Node>>) -> Node {
        Node { value : value, next : next}
    }
}

impl LinkedList {
    // Constructor of linked list
    pub fn new() -> LinkedList {
        LinkedList  {head: None, size: 0}
    }

    // Return size of linked list
    pub fn get_size(&self) -> usize {
        self.size
    }

    // Check if the linked list is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // Push element to the list
    pub fn push(&mut self, value : u32) {
        // take(): take ownership of head
        let new_node : Box<Node> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node); // head is Option<>, use some() to wrap
        self.size += 1;
    }

    // Pop element from the list
    pub fn pop(&mut self) -> Option<u32> {
        // Could not do this: need to unwrap optional head
        // self.head = self.head.unwrap().next;

        let node = self.head.take()?; // ? : unwrap if it's Node otherwise return None
        self.head = node.next;
        self.size -= 1;
        Some(node.value) // Return the value (no semi-colon!)
    }

    // Iterator over the list
    pub fn display(&self) { // shared reference: not modifying
        let mut current : &Option<Box<Node>> = &self.head;
        let mut result = String::new();

        loop {
            match current {
                //  Some(node): "destructures" the Option, node contains Box<Node>
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                }
                None => break,
            }
        }
        println!("{}", result);
    }    
}


fn main() {
    // Box: pointer to a chunk of memory
    let x : Box<u32> = Box::new(10);
    println!("{}", x); // 10

    // Auto - derefence
    let x : Box<u32> = Box::new(10);
    println!("{}", *x); // 10

    // Create a lsit
    let mut list : LinkedList = LinkedList::new();
    println!("{}", list.is_empty());
    assert!(list.is_empty());

    // Take(): give ownership of data in Option and leave None
    let mut x : Option<u32> = Some(5);
    let x_ref : &mut Option<u32> = &mut x;
    println!("Result of take: {:?}", x_ref.take()); // Result of take: Some(5)
    println!("Left at x: {:?}", x); // Left at x: None

    // Push elements into list and iterate
    for i in 1..10 {
        list.push(i);
    }
    list.display(); // 9 8 7 6 5 4 3 2 1

    // Pop element from list
    list.pop();
    list.display(); //  8 7 6 5 4 3 2 1

    

    
}
