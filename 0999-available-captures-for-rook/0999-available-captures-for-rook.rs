impl Solution {
pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
    let mut rook_pos = (0, 0);
    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] == 'R' {
                rook_pos = (i, j);
                break;
            }
        }
    }

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut count = 0;

    for (dx, dy) in directions.iter() {
        let mut x = rook_pos.0 as i32 + dx;
        let mut y = rook_pos.1 as i32 + dy;

        while x >= 0 && x < 8 && y >= 0 && y < 8 {
            match board[x as usize][y as usize] {
                '.' => {
                    x += dx;
                    y += dy;
                },
                'B' => break,
                'p' => {
                    count += 1;
                    break;
                },
                _ => break,
            }
        }
    }

    count
}


}