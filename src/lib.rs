//! This module is for finding prefixes between binary sequences.
//! The intented use is for making range queries on key-value stores which only accept prefix queries. (e.g. Redis and S3)
//!
//! # Parameters and Return Types
//! The base type that all the functions operate on is a slice of booleans.
//! The examples pass array references, but vectors are also compatible.
//! Each element in the slice represents a binary zero or one.
//! Prefixes are returned as slices of the original inputs.

/// Utility function to pad an input value with leading zeros.
///
/// # Example
///
/// ```
/// use binary_prefix::pad;
///
/// pad(3, &[true, false]);
/// // [false, true, false]
/// ```
pub fn pad(size: usize, input: &[bool]) -> Vec<bool> {
    let input_len = input.len();
    let mut out: Vec<bool> = Vec::new();

    // add padding
    for _ in input_len..size {
        out.push(false);
    }

    // add contents of old vector
    for item in input.iter() {
        out.push(item.clone());
    }

    out
}

fn find_seq(initial: bool, collection: &[bool]) -> usize {
    let mut final_count = 0;
    let mut expected_value = initial;

    for entry in collection {
        // println!("entry: {}, expected: {}", entry, expected_value);

        if *entry == expected_value {
            final_count += 1;
        } else {
            break;
        }

        if expected_value == initial {
            expected_value = !initial;
        }
    }

    final_count
}

/// Finds the longest possible shared prefix between two binary vectors.
///
/// # Example
///
/// ```
/// use binary_prefix::shared_prefix;
///
/// let a = vec![true, false, true, false];
/// let b = vec![true, false, true, true];
///
/// shared_prefix(&a, &b);
/// // [true, false, true]
/// ```
pub fn shared_prefix<'a>(start: &'a [bool], end: &[bool]) -> &'a [bool] {
    let pairs = start.iter().zip(end);
    let mut slice_end = 0;
    for pair in pairs {
        if pair.0 == pair.1 {
            slice_end += 1;
        } else {
            break;
        }
    }
    &start[0..slice_end]
}

/// Finds the two longest prefixes that cover a binary range.
///
/// # Example
///
/// ```
/// use binary_prefix::range_prefix;
///
/// let a = vec![true, false, true, false, true, false, false, false, true, true];
/// let b = vec![true, false, true, false, true, false, false, true, true, true];
///
/// range_prefix(&a, &b);
/// // (
/// //     [true, false, true, false, true, false, false, false, true, true],
/// //     [true, false, true, false, true, false, false, true]
/// // )
/// ```
pub fn range_prefix<'a, 'b>(start: &'a [bool], end: &'b [bool]) -> (&'a [bool], &'b [bool]) {

    let segment_len = end.len();
    // let start = pad_vec(false, segment_len, start);

    let base = shared_prefix(&start, end);
    let base_len = base.len();

    let start_special = find_seq(false, &start[base_len..segment_len]);
    let end_special = find_seq(true, &end[base_len..segment_len]);

    let start_prefix_len = base_len + start_special;
    let end_prefix_len   = base_len + end_special;

    (&start[0..start_prefix_len], &end[0..end_prefix_len])
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::cmp::PartialEq;
    use super::*;

    fn assert_vec_equal<T>(a: &[T], b: &[T]) where T: Debug, T: PartialEq {
        let pairs = a.iter().zip(b);
        for entry in pairs {
            assert_eq!(entry.0, entry.1);
        }
    }
    #[test]
    fn finds_shared_prefix() {
        let start = vec![true, false, true, false, true, false, false, false, true, true];
        let end   = vec![true, false, true, false, true, false, false, true, true, true];
        let expected = &start[0..7];

        let result = shared_prefix(&start, &end);

        assert_vec_equal(result, expected);
    }
    #[test]
    fn gets_agressive_prefix() {
        let start = vec![true, false, true, false, true, false, false, false, true, true];
        let end   = vec![true, false, true, false, true, false, false, true, true, true];

        let start_prefix = vec![true, false, true, false, true, false, false, false, true, true];
        let end_prefix = vec![true, false, true, false, true, false, false, true];

        let result = range_prefix(&start, &end);

        assert_vec_equal(&start_prefix, result.0);
        assert_vec_equal(&end_prefix, result.1);
    }
    #[test]
    fn do_pad_vec() {
        let expected = [false, true, false];
        let result = pad(3, &vec![true, false]);

        assert_vec_equal(&result, &expected);
    }
    #[test]
    fn pad_empty() {
        let expected = [false, false, false];
        let input: Vec<bool> = Vec::new();
        let result = pad(3, &input);

        assert_vec_equal(&result, &expected);
    }
    #[test]
    fn no_pad() {
        let expected = [false, true, false];
        let result = pad(3, &vec![false, true, false]);

        assert_vec_equal(&result, &expected);
    }
}
