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

// WORK IN PROGRESS - BOUNDARY ISSUES trying to convert Pythonic solution
// pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
//     if matrix.is_empty() || matrix[0].is_empty() {
//         return false;
//     }

//     let row_len = matrix.len();
//     let col_len = matrix[0].len();

//     // 'first' (rows) b-search:
//     let mut top_row = 0;
//     let mut bot_row = row_len - 1;
//     // let mut mid_row = 0;

//     while top_row <= bot_row {
//         let mid_row = top_row + (top_row + bot_row) / 2;
//         // target is greater than greatest value of 'mid' row (last val), incr. `top_row` "inward"
//         if target > matrix[mid_row][col_len - 1] {
//             top_row = mid_row + 1;
//             // target is less than than smallest value of 'mid' row (first val), decr. `bot_row` "inward"
//         } else if target < matrix[mid_row][0] {
//             bot_row = mid_row - 1;
//             // the target value is within the current row
//         } else {
//             break;
//         }
//     }

//     // after iter: `target` does not exist within provided `matrix`
//     if top_row > bot_row {
//         return false;
//     }

//     // second 'cols' b-search
//     let mid_row = top_row + (top_row + bot_row) / 2;
//     let mut left = 0;
//     let mut right = col_len - 1;

//     while left <= right {
//         let mid_col = left + (left + right) / 2;

//         // target is greater than current 'mid' val, incr. `left` to new range
//         if target > matrix[mid_row][mid_col] {
//             left = mid_col + 1;
//             // target is less than current 'mid' val, decr. `right` to new range
//         } else if target < matrix[mid_row][mid_col] {
//             right = mid_col - 1;
//             // target is found
//         } else {
//             return true;
//         }
//     }

//     // no target found within `mid_row`
//     false
// }
