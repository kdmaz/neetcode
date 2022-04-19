use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = vec![HashSet::new(); 9];
    let mut cols = vec![HashSet::new(); 9];
    let mut boxes = vec![HashSet::new(); 9];

    for r in 0..9 {
        for c in 0..9 {
            let cell = board[r][c];
            if cell == '.' {
                continue;
            }

            // since c and r are both usize, dividing truncates them effectively rounding down to floor
            let box_col = c / 3;
            let box_row = r / 3;
            // boxes by index:
            // [
            //    0, 1, 2
            //    3, 4, 5
            //    6, 7, 8
            // ]
            //
            // if you mulitply by the row by 3 and add the column, can tell what index the box belongs to
            // ex. r = 1, c = 2
            // 2 + (1 * 3) = 5
            let b = box_col + (box_row * 3);
            if rows[r].contains(&cell) || cols[c].contains(&cell) || boxes[b].contains(&cell) {
                return false;
            }

            rows[r].insert(cell);
            cols[c].insert(cell);
            boxes[b].insert(cell);
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_valid_sudoku;

    #[test]
    fn example_1() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sudoku(board), true);
    }

    #[test]
    fn example_2() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sudoku(board), false);
    }
}
