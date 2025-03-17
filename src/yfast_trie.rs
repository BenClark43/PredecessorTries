use std::hash::Hash;

#[derive(Debug)]
struct YFastTrie<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    trie: XFastTrie<K, V>,  //Top half of the diagram
    trees: Vec<BTreeMap<K, V>>,
    table_size: usize,
}

