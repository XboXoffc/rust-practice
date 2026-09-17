

fn main(){

}

fn fizzbuzz(n: u32){
    for i in 1..=n {
        if (i % 3) + (i % 5) == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else { println!("{}", i); }
    }
}

fn count_vowels(s: &str) -> usize {
    let mut count:usize = 0;
    for i in s.chars() {
        if matches!(i.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u') {
            count += 1;
        }
    }
    count
}

use std::collections::HashMap;
fn char_freq(s: &str) -> HashMap<char, usize> {
    let mut CharsMap = HashMap::new();
    for i in s.chars() {
        *CharsMap.entry(i).or_insert(0) += 1;
    }
    CharsMap
}

use std::collections::HashSet;
use std::vec::Vec;
fn rm_dub(nums: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    let mut temp = HashSet::new();
    for num in nums {
        if temp.insert(num) {
            output.push(num);
        }
    }
    output
}

struct Stack<T> { items: Vec<T> }
impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            items: Vec::new(),
        }
    }
    fn push(&mut self, value: T) {
        self.items.push(value);
    }
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    fn len(&self) -> usize {
        self.items.len()
    }
}

fn is_anagram(s1: &str, s2: &str) -> bool {
    fn char_freq_low(s: &str) -> HashMap<char, usize> {
        let mut CharsMap = HashMap::new();
        for i in s.chars() {
            *CharsMap.entry(i.to_ascii_lowercase()).or_insert(0) += 1;
        }
        CharsMap
    }

    if char_freq_low(s1) == char_freq_low(s2) {
        true
    } else { false }
}

fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_sum:i32 = i32::MIN;
    for i in 0..nums.len() {
        for j in i..nums.len() {
            let mut sum:i32 = 0;
            for num in &nums[i..=j] {
                sum += num;
            }
            if sum > max_sum { max_sum = sum; }
        }
    }
    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_test() {
        let n:u32 = 3;
        fizzbuzz(n)
    }

    #[test]
    fn count_vowels_test() {
        let s:&str = "abcdefg";
        assert_eq!(count_vowels(s), 2);
    }

    #[test]
    fn char_freq_test() {
        let s:&str = "hello";
        let mut map = HashMap::new();
        map.insert('h', 1);
        map.insert('e', 1);
        map.insert('l', 2);
        map.insert('o', 1);
        assert_eq!(char_freq(s), map);
    }

    #[test]
    fn rm_dub_test() {
        let mut nums = vec![1, 2, 2, 3, 4, 5, 5, 5, 6, 6, 7];
        assert_eq!(rm_dub(nums), vec![1, 2, 3, 4, 5, 6, 7])

    }

    #[test]
    fn is_anagram_test() {
        assert!(is_anagram("LiSten", "sIleNt"))
    }
    
    #[test]
    fn max_subarray_sum_test() {
        assert_eq!(max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6)
    }
}
