use crate::read_data::read_data;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Pos { row, col }
    }
    fn get_distance(&self, other: &Pos) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

fn transpose(d: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_matrix = vec![vec![]; d[0].len()];
    for i in d.into_iter() {
        for (pos, ch) in i.into_iter().enumerate() {
            new_matrix[pos].push(ch);
        }
    }
    new_matrix
}

fn get_dot_lines_pos(matrix: &Vec<Vec<char>>) -> Vec<usize> {
    matrix
        .iter()
        .enumerate()
        .filter(|(x, y)| !y.iter().any(|x| *x == '#'))
        .map(|(x, _)| x)
        .collect::<Vec<_>>()
}

fn add_rows(num_rows_to_add: usize, d: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expand_rows = vec![];
    d.into_iter().for_each(|line| {
        let has_galaxy = line.iter().any(|x| *x == '#');
        if !has_galaxy {
            for i in 0..num_rows_to_add {
                expand_rows.push(vec!['.'; line.len()])
            }
        }
        expand_rows.push(line)
    });
    expand_rows
}

fn expand(s: &str) -> Vec<Vec<char>> {
    let lines = s
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let added_rows = add_rows(1, lines);
    let added_columns = add_rows(1, transpose(added_rows));
    transpose(added_columns)
}

fn get_pos(m: &Vec<Vec<char>>) -> Vec<Pos> {
    let mut r = vec![];
    for (row, v) in m.into_iter().enumerate() {
        for (col, ch) in v.into_iter().enumerate() {
            if *ch == '#' {
                r.push(Pos::new(row, col));
            }
        }
    }
    r
}

fn get_differences(a: &Vec<Pos>) -> usize {
    let mut distance: usize = 0;
    let mut replica: Vec<&Pos> = a.iter().rev().collect();
    for p in a {
        replica.pop(); // remove current element
        for node2 in &replica {
            let d = p.get_distance(node2);
            distance += d;
        }
    }
    distance
}

#[test]
fn test_tranpose() {
    let i = vec![vec!['1', '2'], vec!['3', '4']];
    let t = transpose(i);
    assert!(t == vec![vec!['1', '3'], vec!['2', '4']]);
}

#[test]
fn test_expand() {
    let s = ".#.
...";
    let got = expand(s);
    got.into_iter().for_each(|x| println!("{:?}", x));
}

#[test]
fn test_run_day_eleven() {
    //     let data = "...#......
    // .......#..
    // #.........
    // ..........
    // ......#...
    // .#........
    // .........#
    // ..........
    // .......#..
    // #...#.....";
    let data = read_data("input-11.txt");

    let expanded = expand(&data);
    let positions = get_pos(&expanded);
    let distance = get_differences(&positions);
    println!("distance is {}", distance);
}

fn get_differences_part2(
    a: &Vec<Pos>,
    row_pos: &Vec<usize>,
    col_pos: &Vec<usize>,
    num_to_add: usize,
) -> usize {
    let mut distance: usize = 0;
    let mut replica: Vec<&Pos> = a.iter().rev().collect();
    for p in a {
        replica.pop(); // remove current element
        for node2 in &replica {
            let mut d = p.get_distance(node2);
            for r in row_pos.iter() {
                if (p.row > *r && *r > node2.row) || (node2.row > *r && *r > p.row) {
                    d += num_to_add;
                }
            }
            for c in col_pos.iter() {
                if (p.col > *c && *c > node2.col) || (node2.col > *c && *c > p.col) {
                    d += num_to_add;
                }
            }
            distance += d;
        }
    }
    distance
}

#[test]
fn test_part2() {
    //     let data = "...#......
    // .......#..
    // #.........
    // ..........
    // ......#...
    // .#........
    // .........#
    // ..........
    // .......#..
    // #...#.....";

    let data = read_data("input-11.txt");
    let matrix = data
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let row_pos = get_dot_lines_pos(&matrix);
    println!("row pos: {:?}", row_pos);
    let t = transpose(matrix);
    let col_pos = get_dot_lines_pos(&t);
    println!("col pos: {:?}", col_pos);
    let orig = transpose(t);
    let g_pos = get_pos(&orig);
    let r = get_differences_part2(&g_pos, &row_pos, &col_pos, 999999); // minus one for row/col already there
    println!("result part 2 is {}", r);
}
