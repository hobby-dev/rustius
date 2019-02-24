//! # lib
//!
//! Main library for rustius project
#![deny(
    unused_imports,
    anonymous_parameters,
    box_pointers,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused,
    unsafe_code
)]
#![warn(variant_size_differences, missing_docs)]

/// Searches input string for a longest substring that consists of at most two characters
/// and returns that substring. Single pass.
pub fn search_longest_2char_substr(input: &str) -> &str {
    // we don't need any heap allocation
    // we don't need to visit each char more than once
    let mut longest_sequence_begin = 0;
    let mut longest_sequence_end = 0;
    let mut a: char = ' ';
    let mut b: char = ' ';
    let mut current_seq_begin = 0;
    let mut current_bbb_begin = 0;
    for (position, c) in input.char_indices() {
        if c == b {
            // b repeats
            continue;
        }
        if c == a {
            // a and b swaps
            use std::mem;
            mem::swap(&mut a, &mut b);
            current_bbb_begin = position;
            continue;
        }
        // Current a-b sequence ends here!
        // Is it longest sequence so far?
        if position - current_seq_begin > longest_sequence_end - longest_sequence_begin {
            // Yes, it is
            longest_sequence_begin = current_seq_begin;
            longest_sequence_end = position;
        }
        a = b;
        b = c;
        current_seq_begin = current_bbb_begin;
        current_bbb_begin = position;
    }
    // Check if last sequence was the longest
    let position = input.len();
    if position - current_seq_begin > longest_sequence_end - longest_sequence_begin {
        longest_sequence_begin = current_seq_begin;
        longest_sequence_end = position;
    }
    &input[longest_sequence_begin..longest_sequence_end]
}
