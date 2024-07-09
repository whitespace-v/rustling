#![warn(clippy::all, clippy::pedantic)]

use std::cmp::Ordering;

fn main() {
    let arr = [-1, 2, 3, 45, 6, 7, 8, 834, 5, 345_345];
    let result = bin_search(&arr, 6);
    match result {
        Some((found_value, found_index)) => {
            println!("Found value: {found_value}, index: {found_index}");
        }
        None => println!("Not Found"),
    }
}

fn bin_search(arr: &[i32], desired_value: i32) -> Option<(i32, usize)> {
    let low_bound: usize = 0;
    let mut up_bound: usize = arr.len() - 1;
    let mut i: usize = 0;

    while low_bound <= up_bound {
        i += 1;
        let mid_index = (up_bound + low_bound) / 2;
        let mid_value = arr[mid_index];

        match mid_value.cmp(&desired_value) {
            Ordering::Equal => return Some((mid_value, mid_index)),
            Ordering::Less => up_bound = mid_index - 1,
            Ordering::Greater => up_bound = mid_index.checked_sub(1)?,
        }
        println!("Step {i}");
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARR: [i32; 10] = [-1, 2, 34, 45, 6, 7, 8, 12, 2, 4];

    #[test]
    fn element_found() {
        assert_eq!((-1, 0), bin_search(&ARR, -1).unwrap())
    }

    #[test]
    fn element_not_found() {
        let result = bin_search(&ARR, 1234);
        assert!(result.is_none());
    }
}
