// question 79
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let m = board.len();
    let n = board[0].len();
    let word_chars: Vec<char> = word.chars().collect();
    let mut searched = vec![vec![false; n]; m];

    for i in 0..m {
        for j in 0..n {
            if search(&board, &word_chars[..], (i, j), &mut searched) {
                return true;
            }
            reset_matrix(&mut searched);
        }
    }
    false
}

fn search(
    board: &Vec<Vec<char>>,
    word: &[char],
    start: (usize, usize),
    searched: &mut Vec<Vec<bool>>,
) -> bool {
    if word.len() == 0 {
        return true;
    }

    if board[start.0][start.1] != word[0] {
        return false;
    }

    searched[start.0][start.1] = true;
    if word.len() == 1 {
        return true;
    }

    let unsearched = get_next_unsearched(&board, start, &searched);
    for coor in unsearched {
        if search(board, &word[1..], coor, searched) {
            return true;
        }
        searched[coor.0][coor.1] = false;
    }
    false
}

fn get_next_unsearched(
    board: &Vec<Vec<char>>,
    start: (usize, usize),
    searched: &Vec<Vec<bool>>,
) -> Vec<(usize, usize)> {
    let mut ans = vec![];
    let x = start.0;
    let y = start.1;

    if x > 0 && !searched[x - 1][y] {
        ans.push((x - 1, y));
    }

    if x + 1 < board.len() && !searched[x + 1][y] {
        ans.push((x + 1, y));
    }

    if y > 0 && !searched[x][y - 1] {
        ans.push((x, y - 1));
    }

    if y + 1 < board[0].len() && !searched[x][y + 1] {
        ans.push((x, y + 1));
    }

    ans
}

fn reset_matrix(m: &mut Vec<Vec<bool>>) {
    for i in 0..m.len() {
        for j in 0..m[0].len() {
            m[i][j] = false;
        }
    }
}
