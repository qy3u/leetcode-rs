// question 79

// 1. question

// Given a 2D board and a word, find if the word exists in the grid.

// The word can be constructed from letters of sequentially adjacent cell, where "adjacent" cells are those horizontally or vertically neighboring. The same letter cell may not be used more than once.

// Example:

// board =
// [
//   ['A','B','C','E'],
//   ['S','F','C','S'],
//   ['A','D','E','E']
// ]
//
// Given word = "ABCCED", return true.
// Given word = "SEE", return true.
// Given word = "ABCB", return false.
//
// Constraints:

//     board and word consists only of lowercase and uppercase English letters.
//     1 <= board.length <= 200
//     1 <= board[i].length <= 200
//     1 <= word.length <= 10^3

// 2. my solution

// pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
//     let m = board.len();
//     let n = board[0].len();
//     let word_chars: Vec<char> = word.chars().collect();
//     let mut searched = vec![vec![false; n]; m];

//     for i in 0..m {
//         for j in 0..n {
//             if search(&board, &word_chars[..], (i, j), &mut searched) {
//                 return true;
//             }
//             reset_matrix(&mut searched);
//         }
//     }
//     false
// }

// fn search(
//     board: &Vec<Vec<char>>,
//     word: &[char],
//     start: (usize, usize),
//     searched: &mut Vec<Vec<bool>>,
// ) -> bool {
//     if word.len() == 0 {
//         return true;
//     }

//     if board[start.0][start.1] != word[0] {
//         return false;
//     }

//     searched[start.0][start.1] = true;
//     if word.len() == 1 {
//         return true;
//     }

//     let unsearched = get_next_unsearched(&board, start, &searched);
//     for coor in unsearched {
//         if search(board, &word[1..], coor, searched) {
//             return true;
//         }
//         searched[coor.0][coor.1] = false;
//     }
//     false
// }

// fn get_next_unsearched(
//     board: &Vec<Vec<char>>,
//     start: (usize, usize),
//     searched: &Vec<Vec<bool>>,
// ) -> Vec<(usize, usize)> {
//     let mut ans = vec![];
//     let x = start.0;
//     let y = start.1;

//     if x > 0 && !searched[x - 1][y] {
//         ans.push((x - 1, y));
//     }

//     if x + 1 < board.len() && !searched[x + 1][y] {
//         ans.push((x + 1, y));
//     }

//     if y > 0 && !searched[x][y - 1] {
//         ans.push((x, y - 1));
//     }

//     if y + 1 < board[0].len() && !searched[x][y + 1] {
//         ans.push((x, y + 1));
//     }

//     ans
// }

// fn reset_matrix(m: &mut Vec<Vec<bool>>) {
//     for i in 0..m.len() {
//         for j in 0..m[0].len() {
//             m[i][j] = false;
//         }
//     }
// }

// 3. other's solution

// 简短
// public boolean exist(char[][] board, String word) {
//     char[] w = word.toCharArray();
//     for (int y=0; y<board.length; y++) {
//     	for (int x=0; x<board[y].length; x++) {
//     		if (exist(board, y, x, w, 0)) return true;
//     	}
//     }
//     return false;
// }

// private boolean exist(char[][] board, int y, int x, char[] word, int i) {
//     if (i == word.length) return true;
//     if (y<0 || x<0 || y == board.length || x == board[y].length) return false;
//     if (board[y][x] != word[i]) return false;
//     board[y][x] ^= 256;
//     boolean exist = exist(board, y, x+1, word, i+1)
//         || exist(board, y, x-1, word, i+1)
//         || exist(board, y+1, x, word, i+1)
//         || exist(board, y-1, x, word, i+1);
//     board[y][x] ^= 256;
//     return exist;
// }

// 4. What can be improved
//     1) 用套路, 而不是脑袋硬想来结题
//

// 5. improve again
//
// pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
//     if word.len() == 0 {
//         return true;
//     }

//     if board.len() == 0 {
//         return false;
//     }

//     if board[0].len() == 0 {
//         return false;
//     }

//     let mut board = board;
//     let words: Vec<char> = word.chars().collect();
//     for row in 0..board.len() {
//         for col in 0..board[0].len() {
//             if exists_helper(&mut board, &words, row as isize, col as isize) {
//                 return true;
//             }
//         }
//     }
//     false
// }

// fn exists_helper(board: &mut Vec<Vec<char>>, words: &[char], row: isize, col: isize) -> bool {
//     if words.len() == 0 {
//         return true;
//     }

//     if row as usize >= board.len()
//         || col as usize >= board[0].len()
//         || row < 0
//         || col < 0
//         || words[0] != board[row as usize][col as usize]
//     {
//         return false;
//     }

//     board[row as usize][col as usize] = '*';
//     let result = exists_helper(board, &words[1..], row + 1, col)
//         || exists_helper(board, &words[1..], row - 1, col)
//         || exists_helper(board, &words[1..], row, col + 1)
//         || exists_helper(board, &words[1..], row, col - 1);

//     board[row as usize][col as usize] = words[0];
//     result
// }

// 6. thinking
//     总结常见套路, 分类练习
