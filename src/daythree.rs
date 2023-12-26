use std::cmp::{max, min};
use std::collections::HashSet;
use crate::read_data::read_data;

#[derive(PartialEq, Debug)]
enum MVals {
    Num(char),
    Symb,
    Dot,
}

impl MVals {
    fn new(c: char) -> MVals {
        match c {
            '0'..='9' => MVals::Num(c),
            '.' => MVals::Dot,
            '!'..='-' => MVals::Symb,
            '/'..='@' => MVals::Symb,
            _ => panic!("suprise char"),
        }
    }
}

fn create_matrix(s: &str) -> Vec<Vec<MVals>> {
    s.split("\n")
        .map(|x| {
            x.chars()
                .map(|character| MVals::new(character))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn parse_matrix(m: Vec<Vec<MVals>>) -> Vec<i32> {
    let mut results = vec![];
    for i in 0..m.len() {
        let mut search_list = HashSet::new();
        let mut potential_num = vec![];
        for j in 0..m[0].len() {
            let maybe = &m[i][j];
            match maybe {
                MVals::Dot => {
                    // check if potential_num register has values
                    if potential_num.len() > 0 {
                        let check = search(&search_list, &m, &potential_num);
                        if let Some(num) = check {
                            results.push(num);
                        }
                        search_list = HashSet::new();
                        potential_num = vec![];
                    }
                }
                MVals::Num(n) => {
                    // add char to potential_num
                    potential_num.push(*n);
                    // add search coordinates
                    // up one row
                    if i > 1 {
                        if j > 0 {
                            search_list.insert((i - 1, j - 1));
                        }
                        search_list.insert((i - 1, j));
                        if j < m[0].len() - 1 {
                            search_list.insert((i - 1, j + 1));
                        }
                    }
                    // left
                    if j > 0 {
                        search_list.insert((i, j - 1));
                    }
                    // right
                    if j < m[0].len() - 1 {
                        search_list.insert((i, j + 1));
                    }
                    // down one row
                    if i < m.len() - 1 {
                        if j > 0 {
                            search_list.insert((i + 1, j - 1));
                        }
                        search_list.insert((i + 1, j));
                        if j < m[0].len() - 1 {
                            search_list.insert((i + 1, j + 1));
                        }
                    }
                }
                MVals::Symb => {
                    // act same as Dot, though this could be abbreviated
                    if potential_num.len() > 0 {
                        let check = search(&search_list, &m, &potential_num);
                        if let Some(num) = check {
                            results.push(num);
                        }
                        search_list = HashSet::new();
                        potential_num = vec![];
                    }
                }
            }
        }
        // check potential_num in case num happens at end
        if potential_num.len() > 0 {
            let check = search(&search_list, &m, &potential_num);
            if let Some(num) = check {
                results.push(num);
            }
            search_list = HashSet::new();
            potential_num = vec![];
        }
    }
    results
}

fn search(
    search_coords: &HashSet<(usize, usize)>,
    grid: &Vec<Vec<MVals>>,
    num: &Vec<char>,
) -> Option<i32> {
    let mut valid = false;
    for i in search_coords {
        let a = i.0;
        let b = i.1;
        let v: &MVals = &grid[a][b];
        if let MVals::Symb = v {
            valid = true;
        }
    }
    if valid {
        let a = num.into_iter().collect::<String>();
        Some(a.parse::<i32>().unwrap())
    } else {
        None
    }
}

#[test]
fn test_run_day_three_part1() {
    let data = read_data("input-3.txt");
    let matrix = create_matrix(&data[..]);
    let r = parse_matrix(matrix);
    let result = r.into_iter().reduce(|x, y| x + y).unwrap();
    println!("{:?}", result);

}

#[test]
fn test_search() {
    let s = vec![
        vec![MVals::Num('1'), MVals::Num('2'), MVals::Dot],
        vec![MVals::Dot, MVals::Dot, MVals::Symb],
    ];
    let mut search_map = HashSet::new();
    search_map.insert((0 as usize, 0 as usize));
    search_map.insert((0 as usize, 1 as usize));
    search_map.insert((0 as usize, 2 as usize));
    search_map.insert((1 as usize, 0 as usize));
    search_map.insert((1 as usize, 1 as usize));
    search_map.insert((1 as usize, 2 as usize));
    let a = search(&search_map, &s, &vec!['1', '2']);
    println!("{:?}", a);
}

#[test]
fn test_parse() {
    let s = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let m = create_matrix(s);
    // parse_matrix(m);
    let r = parse_matrix(m);
    println!("{:?}", r);
}

#[test]
fn test_create_matrix() {
    let s = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let m = create_matrix(s);
    println!("{:?}", m);
}

#[test]
fn test_Mval() {
    let i = MVals::new('2');
    assert!(i == MVals::Num('2'));
    let i = MVals::new('0');
    assert!(i == MVals::Num('0'));
    let i = MVals::new('9');
    assert!(i == MVals::Num('9'));

    assert!(MVals::new('!') == MVals::Symb);
    assert!(MVals::new('@') == MVals::Symb);
    assert!(MVals::new('&') == MVals::Symb);
    assert!(MVals::new('.') == MVals::Dot);
}
