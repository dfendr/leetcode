use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}
struct Solution {}

impl Solution {
    // naive solution using hashmaps, not very efficient, accesses more than it
    // needs to
    pub fn can_construct_naive(ransom_note: String, magazine: String) -> bool {
        let mut letter_count: HashMap<char, i32> = HashMap::new();
        for c in magazine.chars() {
            letter_count.entry(c).and_modify(|i| *i += 1).or_insert(1);
        }
        for c in ransom_note.chars() {
            letter_count.entry(c).and_modify(|i| *i -= 1).or_insert(-1);
            if letter_count.get(&c).unwrap() < &0 {
                return false;
            }
        }
        true
    }
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // early exit condition
        if ransom_note.len() > magazine.len() {
            return false;
        }

        // use an array to track ascii char counts
        let mut counts = [0; 128];

        // increment counts for chars in magazine
        for c in magazine.bytes() {
            counts[c as usize] += 1;
        }

        // decrement counts for chars in use ransom note
        for c in ransom_note.bytes() {
            counts[c as usize] -= 1;
            if counts[c as usize] < 0 {
                return false;
            }
        }

        // reached if ransom_note can be created from magazine
        true
    }
}
