use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

fn get_vec_index_from_row_col(row: usize, col: usize, cols: usize) -> usize {
    row * cols + col
}

/*
    determines whether a cell is island and mark the neighboring cells 
*/
fn check_is_land(
    islands: &Vec<bool>,
    visited: &mut HashSet<usize>,
    row: usize,
    col: usize,
    cols: usize,
    rows: usize,
) -> bool {
    let vec_index = get_vec_index_from_row_col(row, col, cols);

    let is_island = islands[vec_index];

    if is_island && !visited.contains(&vec_index) {
        visited.insert(vec_index);
        if col > 0 {
            check_is_land(islands, visited, row, col - 1, cols, rows);
        }
        if col < cols - 1 {
            check_is_land(islands, visited, row, col + 1, cols, rows);
        }
        if row > 0 {
            check_is_land(islands, visited, row - 1, col, cols, rows);
        }
        if row < rows - 1 {
            check_is_land(islands, visited, row + 1, col, cols, rows);
        }
        return true;
    }
    false
}
fn number_of_islands(islands: Vec<bool>, cols: usize, rows: usize) -> usize {
    let mut visited = HashSet::<usize>::new();
    let mut count = 0;
    for row in 0..rows {
        for col in 0..cols {
            if check_is_land(&islands, &mut visited, row, col, cols, rows) {
                println!("row {}, col {} is island", row, col);
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::number_of_islands;

    #[test]
    fn test_1() {
        let islands = vec![true, true, true, false, false, false, false, true, true];

        assert_eq!(number_of_islands(islands, 3, 3), 2);
    }
}
