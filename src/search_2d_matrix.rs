// # 74 Search a 2D Matrix
// note: "must" write on O(log(m * n)) time
// - Each row's columns are sorted, all rows are sorted

// BINARY SEARCH
// - O(m log n) would perform b-search on each row to find target
// - O(log m) + O(log n) = determine via row's greatest val whether to search row (binary search rows -> binary search within row)

// "Flattened" B-Search
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut start = 0;
    let mut end = rows * cols - 1; // enables 'flattened' 2-D array b-search logic

    while start <= end {
        let mid = start + (end - start) / 2;
        // [mid / cols] -> yields # of 'complete' rows before row containing `mid` (mid_row)
        // [mid % cols] -> yields the idx (col) of `mid`  (mid_col)
        // account for 'complete' rows, remaining values 'after' are modulo'd for remainder ( the idx of `mid_val`)
        let mid_val = matrix[mid / cols][mid % cols];

        if mid_val == target {
            return true;
        } else if mid_val < target {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }

    false
}

//
//
//

use std::cmp::Ordering::{Equal, Greater, Less};

// B-Search Rows, B-Search Cols
pub fn search_matrix_ordering(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }

    let (mut top, mut bot) = (0, matrix.len());
    let mut mid_row = 0;

    while top < bot {
        mid_row = top + (bot - top) / 2;
        let first_val = matrix[mid_row][0];
        let last_val = *matrix[mid_row].last().unwrap();

        if target == first_val || target == last_val {
            return true;
        } else if target < first_val {
            bot = mid_row;
        } else if target > last_val {
            top = mid_row + 1
        } else {
            break;
        }
    }

    if top > bot {
        return false;
    }

    let (mut left, mut right) = (0, matrix[mid_row].len());

    while left < right {
        let mid_col = left + (right - left) / 2;

        match target.cmp(&matrix[mid_row][mid_col]) {
            Equal => return true,
            Less => right = mid_col,
            Greater => left = mid_col + 1,
        }
    }

    false
}
