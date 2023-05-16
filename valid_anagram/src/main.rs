struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // Check if s and t are the same length. If not, then they are not anagrams.
        if s.len() != t.len() {
            return false;
        }

        // Just sort the two strings and compare them.
        let mut s_chars: Vec<char> = s.chars().collect();
        let mut t_chars: Vec<char> = t.chars().collect();
        s_chars.sort();
        t_chars.sort();

        s_chars == t_chars
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_242() {
        assert_eq!(
            Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
        assert_eq!(
            Solution::is_anagram("rat".to_string(), "car".to_string()),
            false
        );
        assert_eq!(
            Solution::is_anagram("aacc".to_string(), "ccac".to_string()),
            false
        )
    }
}
