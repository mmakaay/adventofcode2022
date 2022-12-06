use std::collections::HashSet;

pub fn find_preamble_pos(of_size: usize, within: &str) -> Option<usize> {
    let haystack_length = within.len();
    if haystack_length < of_size {
        return None;
    }
    (0..(haystack_length - of_size)).filter(|&pos| {
        let marker = &within[pos..pos + of_size];
        let unique = marker.chars().collect::<HashSet<char>>();
        unique.len() == of_size
    }).next()
}

pub fn find_start_pos_after_preamble(of_size: usize, within: &str) -> Option<usize> {
    find_preamble_pos(of_size, within).map(|pos| pos + of_size)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_too_short_input() {
        assert_eq!(None, find_preamble_pos(20, "shorter than 20"));
    }

    #[test]
    fn test_no_preamble() {
        assert_eq!(None, find_preamble_pos(10, "there_isn't_a_preamble"));
    }

    #[test]
    fn test_preamble_at_start() {
        assert_eq!(Some(0), find_preamble_pos(4, "abcdxxxxxxxxxxxx"));
    }

    #[test]
    fn test_premble_in_the_middle() {
        assert_eq!(Some(4), find_preamble_pos(3, "aaaaabcaaaaa"));
    }

    #[test]
    fn test_preamble_at_the_end() {
        assert_eq!(Some(8), find_preamble_pos(5, "aaaaaaaaabcdef"));
    }

    #[test]
    fn test_find_start_pos_after_preamble() {
        assert_eq!(None, find_start_pos_after_preamble(20, "shorter than 20"));
        assert_eq!(None, find_start_pos_after_preamble(10, "there_isn't_a_preamble"));
        assert_eq!(Some(4), find_start_pos_after_preamble(4, "abcdxxxxxxxxxxxx"));
        assert_eq!(Some(7), find_start_pos_after_preamble(3, "aaaaabcaaaaa"));
        assert_eq!(Some(13), find_start_pos_after_preamble(5, "aaaaaaaaabcdef"));
    }
}