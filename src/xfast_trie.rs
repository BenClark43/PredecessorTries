use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const MAX_KEY_SIZE: usize = 31;
const MAX_LEVELS: i32 = 31;

// We use Rc<RefCell<>> because nodes are mutable and stored both in the tree and in the hashmaps

#[derive(Debug)]
struct XFastNode<V>
where
    V: Clone,
{
    key: Option<u32>,
    value: Option<V>,
    left: Option<Rc<RefCell<XFastNode<V>>>>,
    right: Option<Rc<RefCell<XFastNode<V>>>>,
}

impl<V> XFastNode<V>
where
    V: Clone,
{
    pub fn new() -> Self {
        XFastNode {
            key: None,
            value: None,
            left: None,
            right: None,
        }
    }

    pub fn new_leaf(key: u32, value: V) -> Self {
        XFastNode {
            key: Some(key),
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

    pub fn get_left(&self) -> Option<Rc<RefCell<XFastNode<V>>>> {
        self.left.as_ref().map(|node| Rc::clone(node))
    }

    pub fn get_right(&self) -> Option<Rc<RefCell<XFastNode<V>>>> {
        self.right.as_ref().map(|node| Rc::clone(node))
    }

    pub fn is_leaf(&self) -> bool {
        self.value.is_some() && self.key.is_some()
    }
}

#[derive(Debug)]
pub struct XFastTrie<V>
where
    V: Clone,
{
    max_leaf: Option<Rc<RefCell<XFastNode<V>>>>,
    min_leaf: Option<Rc<RefCell<XFastNode<V>>>>,
    root: XFastNode<V>,
    hashmaps: Vec<HashMap<u32, Rc<RefCell<XFastNode<V>>>>>,
}

impl<V> XFastTrie<V>
where
    V: Clone + std::fmt::Display,
{
    pub fn new() -> Self {
        let root = XFastNode::new();
        let max_leaf = None;
        let min_leaf = None;
        let hashmaps: Vec<HashMap<u32, Rc<RefCell<XFastNode<V>>>>> =
            vec![HashMap::new(); MAX_KEY_SIZE];

        XFastTrie {
            max_leaf,
            min_leaf,
            root,
            hashmaps,
        }
    }

    pub fn insert(&mut self, key: u32, value: V) -> bool {
        if self.contains(&key) {
            return false;
        }

        let leaf_pointer: Rc<RefCell<XFastNode<V>>> =
            Rc::new(RefCell::new(XFastNode::new_leaf(key, value)));

        match self.predecessor(&key) {
            Some(pred) => {
                leaf_pointer.borrow_mut().set_left(Rc::clone(
                    self.hashmaps[MAX_KEY_SIZE - 1].get(&pred).unwrap(),
                ));
            }
            None => {}
        };

        //Fix internal nodes pointers
        let mut curr_size: usize = self.longest_prefix_search(&key);
        for prefix_length in (1..curr_size) {
            let temp = (key >> (MAX_KEY_SIZE - prefix_length));
            let node_ref = self.hashmaps[prefix_length].get(&(key >> (MAX_KEY_SIZE - prefix_length))).unwrap().clone();
            let mut node = node_ref.borrow_mut();
            let left_ref = node.get_left().unwrap();
            let right_ref = node.get_right().unwrap();


            {
                let left = Rc::clone(left_ref);
                if left.is_leaf() && key < left.key.as_ref().unwrap().clone() {
                    drop(left);
                    node.set_left(Rc::clone(&leaf_pointer));
                }
            }
            /*
            {
                let left = left_ref.borrow();
                if left.is_leaf() && key < left.key.as_ref().unwrap().clone() {
                    drop(left);
                    node.set_left(Rc::clone(&leaf_pointer));
                }
            }
            {
                let right = right_ref.borrow();
                if right.is_leaf() && key > right.key.as_ref().unwrap().clone() {
                    drop(right);
                    node.set_right(Rc::clone(&leaf_pointer));
                }
            }
            drop(node);

             */
        }

        // Find the first node
        let mut curr_node: Rc<RefCell<XFastNode<V>>> = match curr_size {
            0 => {
                let first_node: Rc<RefCell<XFastNode<V>>> = Rc::new(RefCell::new(XFastNode::new()));
                self.hashmaps[0].insert((key >> (MAX_KEY_SIZE - 1)) & 1, Rc::clone(&first_node));
                if key & 1 > 0 {
                    self.root.set_right(Rc::clone(&first_node));
                } else {
                    self.root.set_left(Rc::clone(&first_node));
                }
                if (self.max_leaf.is_none()) {
                    self.max_leaf = Some(leaf_pointer.clone());
                }
                if self.min_leaf.is_none() {
                    self.min_leaf = Some(leaf_pointer.clone());
                }

                curr_size = 1;
                first_node
            }
            _ => {
                let bmax_leaf = self.max_leaf.as_ref();
                let max_leaf_key = bmax_leaf.unwrap().borrow().key;
                let bmin_leaf = self.min_leaf.as_ref();
                let min_leaf_key = bmin_leaf.unwrap().borrow().key;
                if (max_leaf_key.unwrap() < key) {
                    self.max_leaf = Some(Rc::clone(&leaf_pointer));
                }
                if (min_leaf_key.unwrap() > key) {
                    self.min_leaf = Some(Rc::clone(&leaf_pointer));
                }

                Rc::clone(
                    self.hashmaps[curr_size - 1]
                        .get(&(key >> (MAX_KEY_SIZE - curr_size)))
                        .unwrap(),
                )
            }
        };

        // Add the internal Nodes
        while curr_size < MAX_KEY_SIZE - 1 {
            curr_size += 1;
            let mut next_node: Rc<RefCell<XFastNode<V>>> = Rc::new(RefCell::new(XFastNode::new()));

            let mut curr_node_ref = curr_node.borrow_mut();
            if curr_size == MAX_KEY_SIZE - 1 {
                next_node = leaf_pointer.clone();
            }
            if (key & (1 << (MAX_KEY_SIZE - curr_size))) != 0 {
                if curr_node_ref.get_left().is_some() {
                    curr_node_ref.set_right(Rc::clone(&next_node));
                } else {
                    curr_node_ref.set_right(Rc::clone(&next_node));
                    curr_node_ref.set_left(Rc::clone(&leaf_pointer));
                }
            } else {
                if curr_node_ref.get_right().is_some() {
                    curr_node_ref.set_left(Rc::clone(&next_node));
                } else {
                    curr_node_ref.set_right(Rc::clone(&leaf_pointer));
                    curr_node_ref.set_left(Rc::clone(&next_node));
                }
            }
            drop(curr_node_ref);
            self.hashmaps[curr_size - 1]
                .insert(key >> (MAX_KEY_SIZE - curr_size), Rc::clone(&next_node));
            curr_node = next_node;
        }

        let mut curr_node_ref = curr_node.borrow_mut();
        if curr_node_ref.get_right().is_some() {
            curr_node_ref.set_left(Rc::clone(&leaf_pointer));
        } else {
            curr_node_ref.set_right(Rc::clone(&leaf_pointer));
            curr_node_ref.set_left(Rc::clone(&leaf_pointer));
        }

        self.hashmaps[curr_size].insert(key, Rc::clone(&leaf_pointer));
        true
    }

    pub fn get(&self, key: &u32) -> Option<V> {
        self.hashmaps[MAX_KEY_SIZE - 1]
            .get(key)
            .and_then(|node| node.borrow().value.clone())
    }

    pub fn contains(&self, key: &u32) -> bool {
        self.hashmaps[MAX_KEY_SIZE - 1].contains_key(key)
    }

    // WIP
    pub fn predecessor(&mut self, key: &u32) -> Option<u32> {
        let mut largest_prefix: usize = self.longest_prefix_search(&key);
        let curr_node: Rc<RefCell<XFastNode<V>>>;
        if (largest_prefix == 0) {
            if (self.root.get_left().is_none() && self.root.get_right().is_none()) {
                return None;
            } else if (self.root.get_left().is_some()) {
                return self.max_leaf.as_ref().unwrap().borrow().key;
            } else {
                return self.min_leaf.as_ref().unwrap().borrow().key;
            }
        } else {
            curr_node = self.hashmaps[largest_prefix - 1]
                .get(&(key >> (MAX_KEY_SIZE - largest_prefix)))
                .unwrap()
                .clone();
        }
        if largest_prefix == MAX_KEY_SIZE {
            return curr_node.borrow().key;
        } else {
            let pred_node: Rc<RefCell<XFastNode<V>>>;
            //check which child node we have
            match self.hashmaps[largest_prefix]
                .get(&((key >> (MAX_KEY_SIZE - largest_prefix)) << 1))
            {
                //node has left child
                Some(_) => {
                    pred_node = curr_node.borrow().get_right().unwrap();
                }
                //node has right child
                None => {
                    pred_node = curr_node.borrow().get_left().unwrap();
                }
            };
            if pred_node.borrow().key.as_ref().unwrap() < key {
                return pred_node.borrow().key;
            } else {
                let next_node = pred_node.borrow().get_left().unwrap();
                return next_node.borrow().key;
            }
        }

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

    // This needs to be modified to use leftmost bits instead of rightmost. DONE
    fn longest_prefix_search(&mut self, key: &u32) -> usize {
        let mut low = 0;
        let mut high = MAX_KEY_SIZE;
        let mut largest_prefix: usize = 0;

        while low <= high && high > 0 {
            let prefix_size = low + (high - low) / 2;

            if self.hashmaps[prefix_size - 1].contains_key(&(key >> (MAX_KEY_SIZE - prefix_size))) {
                largest_prefix = prefix_size;
                low = prefix_size + 1;
            } else {
                high = prefix_size - 1;
            }
        }

        largest_prefix
    }
}
