fn main() {
    println!("Hello, world!");
}

/*
Given an integer num, return the number of steps to reduce it to zero.

In one step, if the current number is even, you have to divide it by 2,
otherwise, you have to subtract 1 from it.

*/

struct Solution {}

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut steps = 0;
        let mut curr = num;
        while curr != 0 {
            match curr % 2 {
                0 => curr /= 2,
                _ => curr -= 1,
            }
            steps += 1
        }
        steps
    }
}
