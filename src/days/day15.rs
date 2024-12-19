use super::Day;

pub struct Day15;

impl Day for Day15 {
    type Input = (Vec<Vec<char>>, Vec<char>);
    fn parse_input(input: &str) -> Self::Input {
        let mut lines = input.split("\n\n");
        let grid = lines.next().unwrap();
        let grid = grid.lines().map(|s| s.chars().collect()).collect();

        let moves = lines
            .last()
            .unwrap()
            .chars()
            .filter(|c| c != &'\n')
            .collect();
        (grid, moves)
    }

    type OP1 = u64;
    fn part_1((mut grid, moves): Self::Input) -> Self::OP1 {
        let mut ja = 0;
        let mut ia = grid
            .iter()
            .position(|l| match l.into_iter().position(|c| c == &'@') {
                Some(j) => {
                    ja = j;
                    true
                }
                None => false,
            })
            .unwrap();

        for mv in moves.into_iter() {
            match mv {
                '>' => {
                    if grid[ia][ja + 1] == '#' {
                        continue;
                    }

                    if grid[ia][ja + 1] == 'O' {
                        let mut j = ja + 1;
                        while j < grid[ia].len() && grid[ia][j] == 'O' {
                            j += 1;
                        }
                        j -= 1;

                        if grid[ia][j + 1] == '#' {
                            continue;
                        }

                        while j != ja {
                            grid[ia][j + 1] = grid[ia][j];
                            grid[ia][j] = '.';
                            j -= 1;
                        }
                    }

                    grid[ia][ja + 1] = grid[ia][ja];
                    grid[ia][ja] = '.';
                    ja += 1;
                }
                'v' => {
                    if grid[ia + 1][ja] == '#' {
                        continue;
                    }

                    if grid[ia + 1][ja] == 'O' {
                        let mut i = ia + 1;
                        while i < grid.len() && grid[i][ja] == 'O' {
                            i += 1;
                        }
                        i -= 1;

                        if grid[i + 1][ja] == '#' {
                            continue;
                        }

                        while i != ia {
                            grid[i + 1][ja] = grid[i][ja];
                            grid[i][ja] = '.';
                            i -= 1;
                        }
                    }

                    grid[ia + 1][ja] = grid[ia][ja];
                    grid[ia][ja] = '.';
                    ia += 1;
                }
                '<' => {
                    if grid[ia][ja - 1] == '#' {
                        continue;
                    }

                    if grid[ia][ja - 1] == 'O' {
                        let mut j = ja - 1;
                        while j > 0 && grid[ia][j] == 'O' {
                            j -= 1;
                        }
                        j += 1;

                        if grid[ia][j - 1] == '#' {
                            continue;
                        }

                        while j != ja {
                            grid[ia][j - 1] = grid[ia][j];
                            grid[ia][j] = '.';
                            j += 1;
                        }
                    }

                    grid[ia][ja - 1] = grid[ia][ja];
                    grid[ia][ja] = '.';
                    ja -= 1;
                }
                '^' => {
                    if grid[ia - 1][ja] == '#' {
                        continue;
                    }

                    if grid[ia - 1][ja] == 'O' {
                        let mut i = ia - 1;
                        while i > 0 && grid[i][ja] == 'O' {
                            i -= 1;
                        }
                        i += 1;

                        if grid[i - 1][ja] == '#' {
                            continue;
                        }

                        while i != ia {
                            grid[i - 1][ja] = grid[i][ja];
                            grid[i][ja] = '.';
                            i += 1;
                        }
                    }

                    grid[ia - 1][ja] = grid[ia][ja];
                    grid[ia][ja] = '.';
                    ia -= 1;
                }
                _ => (),
            };
        }

        grid.into_iter().enumerate().fold(0, |acc, (i, r)| {
            acc + r.into_iter().enumerate().fold(0, |rc, (j, c)| {
                if c == 'O' {
                    return rc + (100 * i + j) as u64;
                }
                rc
            })
        })
    }

    type OP2 = u64;
    fn part_2((grid, moves): Self::Input) -> Self::OP2 {
        let mut grid: Vec<_> = grid
            .into_iter()
            .map(|r| {
                let mut row = Vec::new();

                for c in r.into_iter() {
                    match c {
                        '#' => {
                            row.push('#');
                            row.push('#');
                        }
                        'O' => {
                            row.push('[');
                            row.push(']');
                        }
                        '.' => {
                            row.push('.');
                            row.push('.');
                        }
                        '@' => {
                            row.push('@');
                            row.push('.');
                        }
                        _ => unreachable!(),
                    };
                }

                row
            })
            .collect();

        let mut ja = 0;
        let mut ia = grid
            .iter()
            .position(|l| match l.into_iter().position(|c| c == &'@') {
                Some(j) => {
                    ja = j;
                    true
                }
                None => false,
            })
            .unwrap();

        let next_blocks = |i: usize, grid: &Vec<Vec<char>>, prev_blocks: Vec<(usize, usize)>| {
            let mut blocks = Vec::new();
            let mut j = 2;
            while j < grid[i].len() {
                if grid[i][j] == '[' {
                    if prev_blocks.is_empty() {
                        blocks.push((j, j + 1));
                    } else {
                        if prev_blocks
                            .iter()
                            .any(|(s, e)| *s == j || *e == j + 1 || *s == j + 1 || *e == j)
                        {
                            blocks.push((j, j + 1));
                        }
                    }
                    j += 2;
                    continue;
                }
                if grid[i][j] == ']' {
                    if prev_blocks.is_empty() {
                        blocks.push((j - 1, j));
                    } else {
                        if prev_blocks
                            .iter()
                            .any(|(s, e)| *s == j || *e == j - 1 || *s == j - 1 || *e == j)
                        {
                            blocks.push((j - 1, j));
                        }
                    }
                    j += 2;
                    continue;
                }
                j += 1;
            }
            blocks.sort();
            blocks
        };

        for mv in moves.into_iter() {
            match mv {
                '>' => {
                    if grid[ia][ja + 1] == '#' {
                        continue;
                    }

                    if grid[ia][ja + 1] == '[' {
                        let mut j = ja + 1;
                        while j < grid[ia].len() && grid[ia][j] == '[' {
                            j += 2;
                        }
                        j -= 1;

                        if grid[ia][j + 1] == '#' {
                            continue;
                        }

                        while j > ja {
                            grid[ia][j + 1] = grid[ia][j];
                            grid[ia][j] = grid[ia][j - 1];
                            grid[ia][j - 1] = '.';
                            j -= 2;
                        }
                    }

                    grid[ia][ja + 1] = grid[ia][ja];
                    grid[ia][ja] = '.';
                    ja += 1;
                }
                'v' => {
                    if grid[ia + 1][ja] == '#' {
                        continue;
                    }

                    if grid[ia + 1][ja] == ']' || grid[ia + 1][ja] == '[' {
                        let (s, e) = if grid[ia + 1][ja] == '[' {
                            (ja, ja + 1)
                        } else {
                            (ja - 1, ja)
                        };

                        let mut mv_blocks = vec![];
                        let mut blocks = vec![(s, e)];
                        mv_blocks.push((ia + 1, blocks.clone()));

                        let mut i = ia + 1;
                        loop {
                            if i == grid.len() {
                                break;
                            }
                            i += 1;

                            if !blocks.is_empty() {
                                if blocks
                                    .iter()
                                    .all(|(s, e)| grid[i][*s] == '.' && grid[i][*e] == '.')
                                {
                                    break;
                                }

                                if blocks
                                    .iter()
                                    .any(|(s, e)| grid[i][*s] == '#' || grid[i][*e] == '#')
                                {
                                    mv_blocks.clear();
                                    break;
                                }
                            }

                            blocks = next_blocks(i, &grid, blocks);
                            if blocks.is_empty() {
                                break;
                            }
                            mv_blocks.push((i, blocks.clone()));
                        }
                        mv_blocks.sort_by(|(a, _), (b, _)| a.cmp(b).reverse());

                        let allowed = mv_blocks.len() > 0;
                        for (i, blocks) in mv_blocks {
                            for (s, e) in blocks {
                                grid[i + 1][s] = grid[i][s];
                                grid[i + 1][e] = grid[i][e];

                                grid[i][s] = '.';
                                grid[i][e] = '.';
                            }
                        }

                        if allowed {
                            grid[ia + 1][ja] = grid[ia][ja];
                            grid[ia][ja] = '.';
                            ia += 1;
                        }
                    } else {
                        grid[ia + 1][ja] = grid[ia][ja];
                        grid[ia][ja] = '.';
                        ia += 1;
                    }
                }
                '<' => {
                    if grid[ia][ja - 1] == '#' {
                        continue;
                    }

                    if grid[ia][ja - 1] == ']' {
                        let mut j = ja - 1;
                        while j > 0 && grid[ia][j] == ']' {
                            j -= 2;
                        }
                        j += 1;

                        if grid[ia][j - 1] == '#' {
                            continue;
                        }

                        while j < ja {
                            grid[ia][j - 1] = grid[ia][j];
                            grid[ia][j] = grid[ia][j + 1];
                            grid[ia][j + 1] = '.';
                            j += 2;
                        }
                    }

                    grid[ia][ja - 1] = grid[ia][ja];
                    grid[ia][ja] = '.';
                    ja -= 1;
                }
                '^' => {
                    if grid[ia - 1][ja] == '#' {
                        continue;
                    }

                    if grid[ia - 1][ja] == ']' || grid[ia - 1][ja] == '[' {
                        let (s, e) = if grid[ia - 1][ja] == '[' {
                            (ja, ja + 1)
                        } else {
                            (ja - 1, ja)
                        };

                        let mut mv_blocks = vec![];
                        let mut blocks = vec![(s, e)];
                        mv_blocks.push((ia - 1, blocks.clone()));

                        let mut i = ia - 1;
                        loop {
                            if i == 0 {
                                break;
                            }
                            i -= 1;

                            if !blocks.is_empty() {
                                if blocks
                                    .iter()
                                    .all(|(s, e)| grid[i][*s] == '.' && grid[i][*e] == '.')
                                {
                                    break;
                                }

                                if blocks
                                    .iter()
                                    .any(|(s, e)| grid[i][*s] == '#' || grid[i][*e] == '#')
                                {
                                    mv_blocks.clear();
                                    break;
                                }
                            }

                            blocks = next_blocks(i, &grid, blocks);
                            if blocks.is_empty() {
                                break;
                            }
                            mv_blocks.push((i, blocks.clone()));
                        }
                        mv_blocks.sort_by(|(a, _), (b, _)| a.cmp(b));

                        let allowed = mv_blocks.len() > 0;
                        for (i, blocks) in mv_blocks {
                            for (s, e) in blocks {
                                grid[i - 1][s] = grid[i][s];
                                grid[i - 1][e] = grid[i][e];

                                grid[i][s] = '.';
                                grid[i][e] = '.';
                            }
                        }

                        if allowed {
                            grid[ia - 1][ja] = grid[ia][ja];
                            grid[ia][ja] = '.';
                            ia -= 1;
                        }
                    } else {
                        grid[ia - 1][ja] = grid[ia][ja];
                        grid[ia][ja] = '.';
                        ia -= 1;
                    }
                }
                _ => (),
            };
        }

        grid.into_iter().enumerate().fold(0, |acc, (i, r)| {
            acc + r.into_iter().enumerate().fold(0, |rc, (j, c)| {
                if c == '[' {
                    return rc + (100 * i + j) as u64;
                }
                rc
            })
        })
    }
}
