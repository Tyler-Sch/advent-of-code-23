use core::num;
use std::collections::HashMap;

use crate::read_data;
use regex::Regex;

fn get_data() -> String {
    read_data("input-2.txt")
}

#[derive(Debug, PartialEq)]
struct GameMarbles {
    game_num: i32,
    red: Option<i32>,
    blue: Option<i32>,
    green: Option<i32>,
}

impl GameMarbles {
    fn new(game_num: i32, r: Option<i32>, b: Option<i32>, g: Option<i32>) -> Self {
        GameMarbles {
            game_num: game_num,
            red: r,
            blue: b,
            green: g,
        }
    }
    fn new_from_map(game_num: i32, m: HashMap<&str, i32>) -> Self {
        let r: Option<i32> = if m.contains_key("red") {
            let i = m.get("red").unwrap().to_owned();
            Some(i)
        } else {
            None
        };

        let b: Option<i32> = if m.contains_key("blue") {
            let i = m.get("blue").unwrap().to_owned();
            Some(i)
        } else {
            None
        };

        let g: Option<i32> = if m.contains_key("green") {
            let i = m.get("green").unwrap().to_owned();
            Some(i)
        } else {
            None
        };
        GameMarbles {
            game_num: game_num,
            red: r,
            blue: b,
            green: g,
        }
    }

    fn is_valid(&self) -> bool {
        if let Some(i) = self.red {
            if i > 12 {
                return false;
            }
        }

        if let Some(j) = self.blue {
            if j > 14 {
                return false;
            }
        }

        if let Some(k) = self.green {
            if k > 13 {
                return false;
            }
        }
        true
    }

    fn get_most(&self, other: GameMarbles) -> Self {
        let r = if self.red >= other.red {
            self.red
        } else {
            other.red
        };

        let b = if self.blue >= other.blue {
            self.blue
        } else {
            other.blue
        };

        let g = if self.green >= other.green {
            self.green
        } else {
            other.green
        };

        GameMarbles::new(self.game_num, r, b, g)
    }

    fn get_total(&self) -> i32 {
        self.red.unwrap() * self.blue.unwrap() * self.green.unwrap()
    }
}

fn parse_pull(gn: i32, s: &str) -> GameMarbles {
    let num_color_re = Regex::new(r" ?([0-9]*) ([a-z]*)").unwrap();
    let split = s.split(",");
    let mut color_map = HashMap::new();
    for c in split {
        let m = num_color_re.captures(c).unwrap();
        let num = m.get(1).unwrap().as_str();
        let numm = num.parse::<i32>().unwrap();
        let color = m.get(2).unwrap().as_str();
        color_map.insert(color, numm);
    }
    GameMarbles::new_from_map(gn, color_map)
}

fn parse_game_number(inp: &str) -> i32 {
    let re = Regex::new(r"^Game ([0-9]*):").unwrap();
    let game_num = re.captures(inp).unwrap();
    let gn = game_num.get(1).unwrap().as_str();
    gn.parse::<i32>().unwrap()
}

fn parse_line(inp: &str) -> Option<i32> {
    if inp == "" {
        return None;
    }
    let gn = parse_game_number(inp);
    let rest = inp.split(": ").last().unwrap();
    for i in rest.split(";") {
        let r = parse_pull(gn, i);
        if !r.is_valid() {
            return None;
        }
    }
    Some(gn)
}

fn parse_line_part_two(inp: &str) -> Option<i32> {
    if inp == "" {
        return Some(0);
    }
    let gn = parse_game_number(inp);
    let rest = inp.split(": ").last().unwrap();
    let z = rest
        .split(";")
        .into_iter()
        .map(|x| parse_pull(gn, x))
        .reduce(|x, y| x.get_most(y));
    Some(z.unwrap().get_total())
}

pub fn day_two_part_one() {
    let d = get_data();
    let lines = d.split("\n");
    let p = lines
        .into_iter()
        .map(|x| parse_line(x))
        .filter(|x| x.is_some())
        .reduce(|x, y| Some(x.unwrap() + y.unwrap()));
    println!("{:?}", p);

    let part_two = d
        .split("\n")
        .into_iter()
        .map(|x| parse_line_part_two(x))
        .reduce(|x, y| Some(x.unwrap() + y.unwrap()));
        // .collect::<Vec<Option<i32>>>();
    println!("part 2 answer is {:?}", part_two);
}

#[test]
fn test_parse_line_two() {
    let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let expected = Some(48);
    assert!(expected == parse_line_part_two(s));
    let s = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
    let expected = Some(1560);
    assert!(expected == parse_line_part_two(s));
}

#[test]
fn test_day_two() {
    day_two_part_one();
}

#[test]
fn get_more() {
    let a = GameMarbles::new(1, Some(2), None, Some(3));
    let b = GameMarbles::new(1, Some(3), Some(2), Some(1));
    let r = a.get_most(b);
    println!("{:?}", r);
}

#[test]
fn test_parse_whole_line() {
    let s = "Game 20: 2 red, 1 blue, 5 green; 11 blue, 1 red, 4 green; 6 blue, 2 red, 2 green; 13 blue, 2 red, 10 green; 7 green, 13 blue";
    let result = parse_line(s);
    assert!(result == Some(20));
    let s = "Game 19: 9 red, 3 green, 14 blue; 15 red, 2 blue, 1 green; 2 green, 15 red, 5 blue; 3 red, 3 blue, 1 green";
    let result = parse_line(s);
    assert!(result == None);
}

#[test]
fn test_parse_pull() {
    let s = "2 green, 15 red, 5 blue";
    let got = parse_pull(1, s);
    let expected = GameMarbles::new(1, Some(15), Some(5), Some(2));
    assert!(got == expected);

    let s = "2 green";
    let got = parse_pull(1, s);
    let expected = GameMarbles::new(1, None, None, Some(2));
    assert!(got == expected);
}

#[test]
fn test_parse_game_number() {
    let s = "Game 72: 3 blue, 1 green, 11 red; 5 green, 11 blue, 4 red; 7 blue, 13 red; 14 blue, 12 red, 5 green";
    let r = parse_game_number(s);
    assert!(r == 72);
}

#[test]
fn test_get_data() {
    let d = get_data();
    println!("{}", d);
}

#[test]
fn gameMarbles() {
    let valid = GameMarbles::new(1, Some(1), Some(2), None);
    assert!(valid.is_valid());
    let not_valid = GameMarbles::new(1, Some(15), Some(5), Some(15));
    assert!(!not_valid.is_valid());
    let not_valid = GameMarbles::new(1, Some(1), Some(15), Some(4));
    assert!(!not_valid.is_valid());
    let not_valid = GameMarbles::new(1, Some(1), Some(3), Some(14));
    assert!(!not_valid.is_valid());
}
