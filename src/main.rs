mod xfast_trie;

// Declare the module from the `person.rs` file
use xfast_trie::XFastTrie; // Bring the `Person` struct into scope

fn main() {

    let mut test: XFastTrie<u32> = XFastTrie::new();
    let _ = test.insert(15, 15);
    _ = test.insert(16, 16);
    _ = test.insert(27, 27);

    test.hashmap_contents();
/*
    let mut level = 1;
    let max_levels = 9;
    while level < max_levels {
        let prefix = 179 >> (max_levels - level);
        println!("prefix: {:8b}", prefix);
        level += 1;
    }
*/

}
