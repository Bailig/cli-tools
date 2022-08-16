use std::cmp;

// https://leetcode.com/problems/container-with-most-water/
pub fn max_water(heights: Vec<u32>) -> u32 {
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
pub fn backspace_compare(s: String, t: String) -> bool {
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
