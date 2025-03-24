mod xfast_trie;

// Declare the module from the `person.rs` file
use xfast_trie::XFastTrie; // Bring the `Person` struct into scope

fn main() {

    let mut test: XFastTrie<u32> = XFastTrie::new();
    let _ = test.insert(15, 1000);
    _ = test.insert(16, 2000);
    _ = test.insert(27, 3000);
    _ = test.insert(33, 5000);
   test.hashmap_contents();
   
    println!("15 = {}, 16 = {}, 27 = {}", test.get(&15).unwrap(), test.get(&16).unwrap(), test.get(&27).unwrap());
    println!("{}",test.predecessor(&18).unwrap());
    println!("{}",test.predecessor(&19).unwrap());
    println!("{}",test.predecessor(&27).unwrap());
    println!("{}",test.predecessor(&16).unwrap());
    println!("{}",test.predecessor(&26).unwrap());
    println!("{}",test.predecessor(&35).unwrap());

    
/*,
    let mut level = 1;
    let max_levels = 9;
    while level < max_levels {
        let prefix = 179 >> (max_levels - level);
        println!("prefix: {:8b}", prefix);
        level += 1;
    }
*/

}
