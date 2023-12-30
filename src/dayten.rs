use crate::read_data::read_data;
use std::collections::HashSet;

fn create_matrix(s: &str) -> Vec<Vec<char>> {
    s.lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Pos {
    row: usize,
    col: usize
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Pos { row, col}
    }
    fn traverse(&self, dir: Direction) -> Pos {
        let mut newCoords = (0, 0);
        let x = match dir {
           Direction::Up => newCoords = (self.row - 1,self.col),
            Direction::Down => newCoords = (self.row + 1, self.col),
            Direction::Left => newCoords = (self.row, self.col - 1),
            Direction::Right => newCoords = (self.row, self.col + 1),
            _ => panic!("Error in traverse")
        };
        Pos::new(newCoords.0, newCoords.1)

    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Copy, Clone, Hash)]
struct Node {
    character: char,
    enter: Pos,
    loc: Pos,
}
impl Node {
    fn new(ch: char, enter: Pos, loc: Pos) -> Self {
        Node {
            character: ch, enter, loc
        }
    }
    fn from_previous(previous: &Node, matrix:&Vec<Vec<char>>) -> Self {
        let link = matrix[previous.loc.row][previous.loc.col];
        let new_coords = match link {
            '-' => {
                if previous.enter.col > previous.loc.col {
                     previous.loc.traverse(Direction::Left)

                } else {
                    previous.loc.traverse(Direction::Right)
                }

            },
            '|' => {
                if previous.enter.row > previous.loc.row {
                   previous.loc.traverse(Direction::Up)
                } else {
                    previous.loc.traverse(Direction::Down)
                }
            },
            'L' => {
                if previous.enter.row < previous.loc.row {
                    previous.loc.traverse(Direction::Right)
                } else {previous.loc.traverse(Direction::Up)}
            },
            'J' => {
                if previous.enter.row < previous.loc.row {
                    previous.loc.traverse(Direction::Left)
                } else {previous.loc.traverse(Direction::Up)}
                },
            '7' => {
                if previous.enter.row > previous.loc.row {
                    previous.loc.traverse(Direction::Left)
                } else {previous.loc.traverse(Direction::Down)}
            },
            'F' => {
                if previous.enter.row > previous.loc.row {
                    previous.loc.traverse(Direction::Right)
                } else {previous.loc.traverse(Direction::Down)}
            }
            'S' => {
                unimplemented!("got to S")
            }
            _ => unimplemented!("not created")
        };

        Node::new(matrix[new_coords.row][new_coords.col], previous.loc, new_coords)
    }
}


fn find_start_coord(matrix: &Vec<Vec<char>>) -> Option<Node> {
    for (i, row) in matrix.iter().enumerate() {
       for (j, col) in row.iter().enumerate() {
           if matrix[i][j] == 'S' {
               return Some(Node::new('S', Pos::new(i, j), Pos::new(i, j)))
           }
       }
    }
    None
}

fn traverse_until_start(n: Node, m: &Vec<Vec<char>>) -> Vec<Node> {
    let mut current = n;
    let mut results = vec![];
    while current.character != 'S' {
        let next_node = Node::from_previous(&current, m);
       results.push(current);
        current = next_node
    }
    results
}

fn check_is_in(n: &Node, border: &HashSet<Pos>, m: &Vec<Vec<char>>) -> bool {
   let mut crosses = 0;
    let w = m[0].len() -1;
    let h = m.len()-1;
    let mut current_pos = n.loc;
    if border.contains(&current_pos) {return false}
    while (current_pos.row < h && current_pos.col < w) {
       current_pos = current_pos.traverse(Direction::Right).traverse(Direction::Down);
        let current_char = m[current_pos.row][current_pos.col];
        // dbg!(current_pos);
       if (border.contains(&current_pos) && current_char != '7' && current_char != 'L') {
            crosses += 1;
       }
    }
    if crosses % 2 == 1 {
        true
    } else {false}
}

fn count_pixel_in(start: Node, m: &Vec<Vec<char>>) -> i32 {
    let mut begin = traverse_until_start(start.clone(), m);
    let snode = find_start_coord(m).unwrap();
    begin.push(snode);
   let border = begin.into_iter().map(|x| x.loc);
    let border_hash = HashSet::from_iter(border);
    let mut counter = 0;
    for row in 0..m.len() {
        for col in 0..m[0].len() {
            let node_in_question = Node::new(m[row][col], Pos::new(0,0), Pos::new(row, col));
            if check_is_in(&node_in_question, &border_hash, m) {
                // dbg!("node at {:?} is in", &node_in_question.loc);
                counter += 1;
            }
        }
    }
    counter
}

fn traverse_path(first_start:Node, second_start:Node, matrix: &Vec<Vec<char>>) -> usize {
    // cheating a bit because I see my input:
    let first_res = traverse_until_start(first_start, matrix);
    let second_res = traverse_until_start(second_start, matrix);
    println!("first result is {}; second is {}", first_res.len(), second_res.len());
    first_res.len()
}

#[test]
fn test_new_node_from_previous() {
    let data = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    let matrix = create_matrix(data);
    let test_node = Node::new('J', Pos::new(2, 0), Pos::new(2, 1));
    let next_node = Node::from_previous(&test_node, &matrix);
    println!("{:?}", next_node);
    // assert!(next_node.character == 'F');
    let after_node = Node::from_previous(&next_node, &matrix);
    println!("{:?}", after_node);
    // assert!(after_node.character == 'J');
    let after_node = Node::from_previous(&after_node, &matrix);
    println!("{:?}", after_node);
    // assert!(after_node.character == 'F');
    let after_node = Node::from_previous(&after_node, &matrix);
    assert!(after_node.character == '7');
    println!("{:?}", after_node);
}

#[test]
fn test_run_dayten() {
//     let data = "..........
// .S------7.
// .|F----7|.
// .||OOOO||.
// .||OOOO||.
// .|L-7F-J|.
// .|II||II|.
// .L--JL--J.
// ..........";
    let data = read_data("input-10.txt");
    let matrix = create_matrix(&data[..]);
    // println!("{:?}", matrix);
    let start = find_start_coord(&matrix);
    // let first = Node::new('-', start.unwrap().loc, Pos::new(1, 2));
    // let second = Node::new('|', start.unwrap().loc, Pos::new(2, 1));
    let first = Node::new('L', start.unwrap().loc, start.unwrap().loc.traverse(Direction::Left));
    let second = Node::new('-', start.unwrap().loc, start.unwrap().loc.traverse(Direction::Right));
    let r = traverse_path(first, second, &matrix);
    println!("result is {}", r);

    // part 2
    // let first = Node::new('-', start.unwrap().loc, Pos::new(1, 2));
    let first = Node::new('L', start.unwrap().loc, start.unwrap().loc.traverse(Direction::Left));
    let r = count_pixel_in(first, &matrix);
    // let test_node = Node::new(')
    println!("part 2 result is {}", r);
}

