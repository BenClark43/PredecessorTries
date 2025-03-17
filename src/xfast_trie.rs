use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const MAX_KEY_SIZE: usize = 6;
const MAX_LEVELS: i32 = 6;

// We use Rc<RefCell<>> because nodes are mutable and stored both in the tree and in the hashmaps

#[derive(Debug)]
struct XFastNode<V>
where
    V: Clone
{
    value: Option<V>,
    left: Option<Rc<RefCell<XFastNode<V>>>>,
    right: Option<Rc<RefCell<XFastNode<V>>>>,
}

impl<V> XFastNode<V>
where
    V: Clone
{
    pub fn new() -> Self {
        XFastNode {
            value: None,
            left: None,
            right: None,
        }
    }

    pub fn new_leaf(value: V) -> Self {
        XFastNode {
            value: Some(value),
            left: None,
            right: None,
        }
    }

    pub fn set_left(&mut self, node: Rc<RefCell<XFastNode<V>>>) {
        self.left = Some(node);
    }

    pub fn set_right(&mut self, node: Rc<RefCell<XFastNode<V>>>) {
        self.right = Some(node);
    }

    /*
    pub fn get_left(&self) -> Option<Rc<RefCell<XFastNode<V>>>> {
        self.left.as_ref().map(|node| Rc::clone(node))
    }

    pub fn get_right(&self) -> Option<Rc<RefCell<XFastNode<V>>>> {
        self.right = Some(node);
    }
     */

}

#[derive(Debug)]
pub struct XFastTrie<V>
where
    V: Clone
{
    root: XFastNode<V>,
    hashmaps: Vec<HashMap<u32, Rc<RefCell<XFastNode<V>>>>>,
}

impl<V> XFastTrie<V>
where
    V: Clone + std::fmt::Display
{
    pub fn new() -> Self {

        let root = XFastNode::new();
        let hashmaps: Vec<HashMap<u32, Rc<RefCell<XFastNode<V>>>>> =
            vec![HashMap::new(); MAX_KEY_SIZE];

        XFastTrie { root, hashmaps }
    }

    pub fn insert(&mut self, key: u32, value: V) -> bool {
        if self.contains(&key) {
            return false;
        }

        let leaf_pointer: Rc<RefCell<XFastNode<V>>> =
            Rc::new(RefCell::new(XFastNode::new_leaf(value)));

        // Once these operations are working we can link the leaf here
            // pred = self.predecessor(key)
            // succ = self.successor(key)
            // leaf_pointer.set_right(Rc::clone(&pred));
            // leaf_pointer.set_right(Rc::clone(&succ));

        let mut curr_size: usize = self.longest_prefix_search(&key);

        // Find the first node
        let curr_node: Rc<RefCell<XFastNode<V>>> = match curr_size {
            0 => {
                let first_node: Rc<RefCell<XFastNode<V>>> = Rc::new(RefCell::new(XFastNode::new()));
                self.hashmaps[0].insert(key & 1, Rc::clone(&first_node));
                if key & 1 > 0 {
                    self.root.set_right(Rc::clone(&first_node));
                } else {
                    self.root.set_left(Rc::clone(&first_node));
                }
                curr_size = 1;
                first_node
            }
            _ => {
                Rc::clone(
                    self.hashmaps[curr_size]
                        .get(&(key & ((1 << curr_size) - 1)))
                        .unwrap()
                )
            },
        };

        // Add the internal Nodes
        while curr_size < MAX_KEY_SIZE - 1 {
            curr_size += 1;
            let next_node: Rc<RefCell<XFastNode<V>>> =
                Rc::new(RefCell::new(XFastNode::new()));

            let mut curr_node_ref = curr_node.borrow_mut();
            if (key & (1 << curr_size)) != 0 {
                curr_node_ref.set_right(Rc::clone(&next_node));
                curr_node_ref.set_left(Rc::clone(&leaf_pointer));
            } else {
                curr_node_ref.set_right(Rc::clone(&leaf_pointer));
                curr_node_ref.set_left(Rc::clone(&next_node));
            }

            self.hashmaps[curr_size].insert(key & ((1 << curr_size) - 1), Rc::clone(&next_node));
        }

        // Link to the leaf node
        let mut curr_node_ref = curr_node.borrow_mut();
        curr_node_ref.set_right(Rc::clone(&leaf_pointer));
        curr_node_ref.set_left(Rc::clone(&leaf_pointer));
        self.hashmaps[curr_size].insert(key, Rc::clone(&leaf_pointer));
        true
    }

    pub fn get(&self, key: &u32) -> Option<V> {
        self.hashmaps[MAX_KEY_SIZE-1]
            .get(key)
            .and_then(|node| node.borrow().value.clone())
    }

    pub fn contains(&self, key: &u32) -> bool {
        self.hashmaps[MAX_KEY_SIZE-1]
            .contains_key(key)
    }

    // WIP
    pub fn predecessor(&mut self, key: &u32) -> Option<V> {
        let largest_prefix: usize = self.longest_prefix_search(&key);
        if largest_prefix == 0 {
            return None;
        }
        if largest_prefix == MAX_KEY_SIZE-1 {
            println!("LEAf node found");
        }
        let curr_node = self.hashmaps[largest_prefix]
            .get(&(key & ((1 << largest_prefix) - 1)))
            .unwrap();

        println!("prefix key: {}", curr_node.borrow().value.clone().unwrap());

       // let left_node: Rc<RefCell<XFastNode<V>>> =
       //     Rc::new(RefCell::new(XFastNode::new(key & ((1 << curr_size) - 1))));

      //  println!("Left key: {}", curr_node.borrow().left.clone().unwrap().value);
      //println!("Right key: {}", curr_node.borrow().right.clone().unwrap());

        None
    }

    // For Debugging
    pub fn hashmap_contents(&self) {
        for (i, map) in self.hashmaps.iter().enumerate() {
            println!("HashMap {} has {} items", i + 1, map.len());
            for (key, _value) in map.iter() {
                println!("  Key: {:0width$b}", key, width = i + 1);
            }
        }
    }

    // This needs to be modified to use leftmost bits instead of rightmost.
    fn longest_prefix_search(&mut self, key: &u32) -> usize {
        let mut low = 0;
        let mut high = MAX_KEY_SIZE;
        let mut largest_prefix: usize = 0;

        while low < high {
            let prefix_size = low + (high - low) / 2;

            if self.hashmaps[prefix_size].contains_key(&(key & ((1 << prefix_size) - 1))) {
                largest_prefix = prefix_size;
                low = prefix_size + 1;
            } else {
                high = prefix_size;
            }
        }

        largest_prefix
    }

}
