pub fn annotate(garden: &[&str]) -> Vec<String> {
    // todo!("\nAnnotate each square of the given garden with the number of flowers that surround said square (blank if there are no surrounding flowers):\n{garden:#?}\n");
    if garden.is_empty() {
        return vec![];
    }

    let rows = garden.len();
    let cols = garden[0].len();

    let mut result = vec![vec!['0'; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            if garden[i].as_bytes()[j] == b'*' {
                result[i][j] = '*';
                continue;
            }

            let mut count = 0;
            for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let ni = i as isize + di;
                    let nj = j as isize + dj;
                    if ni >= 0
                        && ni < rows as isize
                        && nj >= 0
                        && nj < cols as isize
                        && garden[ni as usize].as_bytes()[nj as usize] == b'*'
                    {
                        count += 1;
                    }
                }
            }
            if count > 0 {
                result[i][j] = (b'0' + count) as char;
            } else {
                result[i][j] = ' ';
            }
        }
    }
    
    result.into_iter().map(|row| row.into_iter().collect()).collect()
}
