use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // We want a HashMap of HashMaps. It will be a HashMap of anagrams, which will be a HashMap of characters.

        // let charMaps = strs
        //     .into_iter()
        //     .map(|s| {
        //         let hash = HashMap::<char, i32>::from_iter(s.chars().map(|c| (c, 1)));
        //         (s, hash)
        //     })
        //     .collect::<HashMap<String, HashMap<char, i32>>>();
        let results: Vec<Vec<String>> = vec![];
        let anagrams = HashSet::<HashMap<char, i32>>::new();

        for s in strs {
            let hash = HashMap::<char, i32>::from_iter(s.chars().map(|c| (c, 1)));
        }

        println!("{:?}", anagrams);

        return vec![];
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anagram_hash_of_hash() {
        // Is the hashset of a hashmap of an anagram the same as the hashset of a hashmap of the same anagram?
        let str1 = "eat".to_string();
        let str2 = "tea".to_string();

        let hash1 = HashMap::<char, i32>::from_iter(str1.chars().map(|c| (c, 1)));
        let hash2 = HashMap::<char, i32>::from_iter(str2.chars().map(|c| (c, 1)));

        println!("{:?}", hash1);
        println!("{:?}", hash2);

        // Apparently, yes.
        println!("{:?}", hash1 == hash2);
    }

    #[test]
    fn test_simple() {
        assert_eq!(
            Solution::group_anagrams(vec!["eat".to_string(), "tea".to_string()]),
            vec![vec!["eat".to_string(), "tea".to_string()]]
        );
    }

    #[test]
    fn test_49() {
        assert_eq!(
            Solution::group_anagrams(vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string()
            ]),
            vec![
                vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
                vec!["nat".to_string(), "tan".to_string()],
                vec!["bat".to_string()]
            ]
        );
    }
}
