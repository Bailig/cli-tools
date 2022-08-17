use std::{cmp, collections::HashMap};

// https://leetcode.com/problems/container-with-most-water/
fn max_water(heights: Vec<u32>) -> u32 {
    let mut start = 0;
    let mut end = heights.len() - 1;
    let mut max: u32 = 0;

    while start < end {
        let water = ((end - start) as u32) * cmp::min(heights[start], heights[end]);

        max = cmp::max(max, water);

        if heights[start] < heights[end] {
            start += 1;
        } else {
            end -= 1;
        }
    }
    max
}

// https://leetcode.com/problems/backspace-string-compare/
fn backspace_compare(s: String, t: String) -> bool {
    let mut i: i32 = (s.len() as i32) - 1;
    let mut j: i32 = (t.len() as i32) - 1;

    let mut s_hash_count = 0;
    let mut t_hash_count = 0;

    while i >= 0 || j >= 0 {
        let s_char = if i >= 0 {
            &s[(i as usize)..(i as usize) + 1]
        } else {
            ""
        };
        let t_char = if j >= 0 {
            &t[(j as usize)..(j as usize) + 1]
        } else {
            ""
        };

        if s_char == "#" {
            s_hash_count += 1;
            i -= 1;
        } else if s_hash_count > 0 {
            s_hash_count -= 1;
            i -= 1;
        } else if t_char == "#" {
            t_hash_count += 1;
            j -= 1;
        } else if t_hash_count > 0 {
            t_hash_count -= 1;
            j -= 1;
        } else if s_char == t_char {
            i -= 1;
            j -= 1;
        } else {
            return false;
        }
    }
    i < 0 && j < 0
}

// https://leetcode.com/problems/longest-substring-without-repeating-characters/
fn length_of_longest_substring(s: String) -> i32 {
    if s.len() < 2 {
        return s.len() as i32;
    }

    let mut char_index_map: HashMap<&str, i32> = HashMap::new();
    char_index_map.insert(&s[0..1], 0);
    let mut i = 0;
    let mut j = 1;
    let mut max = 0;

    while j < s.len() {
        let right = &s[j..j + 1];

        if char_index_map.get(right).is_some() && *char_index_map.get(right).unwrap() >= i as i32 {
            max = cmp::max(max, j - i);
            let index = char_index_map.get(right).unwrap();
            i = *index as usize + 1;
        }
        char_index_map.insert(right, j as i32);
        j += 1;
    }
    max = cmp::max(max, j - i);
    max as i32
}

pub fn run() {
    println!("{}", max_water(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    println!(
        "{}",
        backspace_compare("ab###acd".to_string(), "ad#cddd###d".to_string())
    );
    println!("{}", length_of_longest_substring("abba".to_string()))
}
