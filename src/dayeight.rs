use crate::read_data::read_data;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Node<'a> {
    current: &'a str,
    left: &'a str,
    right: &'a str,
}

fn parse_input(s: &str) -> Node {
    let mut splitsy = s.split(" = ");
    let current = splitsy.next().unwrap();
    let mut tup = splitsy.next().unwrap().split(", ");
    let left = &tup.next().unwrap()[1..];
    let right = &tup.next().unwrap()[..3];
    Node {
        current,
        left,
        right,
    }
}

fn build_graph(v: Vec<Node>) -> HashMap<&str, Node> {
    let mut hm = HashMap::new();
    v.into_iter().for_each(|node| {
        hm.insert(node.current, node);
    });
    hm
}

fn traverse_graph(instructions: &str, graph: &HashMap<&str, Node>) -> i32 {
    let mut counter = 0;
    let mut current_key = "AAA";
    let mut instruct = instructions.chars();
    while current_key != "ZZZ" {
        let i = instruct.next();
        let current_instruct = if let Some(c) = i {
            c
        } else {
            instruct = instructions.chars();
            instruct.next().unwrap()
        };
        let current_node = graph.get(current_key).unwrap();

        if current_instruct == 'R' {
            current_key = current_node.right;
        } else {
            current_key = current_node.left
        }
        counter += 1;
    }
    counter
}

fn check_all_z(vecs: &Vec<&str>) -> bool {
    vecs.into_iter()
        .filter(|x| x.ends_with('Z'))
        .collect::<Vec<_>>()
        .len()
        == vecs.len()
}

#[test]
fn test_all_z() {
    let a = vec!["AAA", "BBB", "CCC"];
    let r = check_all_z(&a);
    assert!(!r);
    let a = vec!["AAZ", "ZZZ", "ZBZ"];
    let r = check_all_z(&a);
    assert!(r);
}

fn traverse_graph_part2(instructions: &str, graph: &HashMap<&str, Node>) -> i32 {
    let mut counter = 0;
    let mut current_key = graph
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| *x)
        .collect::<Vec<&str>>();
    let mut instruct = instructions.chars();
    while !check_all_z(&current_key) {
        let i = instruct.next();
        let current_instruct = if let Some(c) = i {
            c
        } else {
            instruct = instructions.chars();
            instruct.next().unwrap()
        };
        let current_node = current_key.iter().map(|x| graph.get(x).unwrap());

        if current_instruct == 'R' {
            // current_key = current_node.right;
            current_key = current_node.map(|x| x.right).collect::<Vec<&str>>();
        } else {
            current_key = current_node.map(|x| x.left).collect::<Vec<&str>>();
        }
        counter += 1;
        if &current_key.iter().filter(|x| x.ends_with('Z')).collect::<Vec<_>>().len() > &(0 as usize) {
            println!("counter is {}; current_key is {:?}", counter, current_key);
        } 

        // if counter % 1000000 == 0 {
        // println!("counter: {}", counter);
        // }
    }
    counter
}

pub fn dayeight_part2() {
    //     let data = "LR

    // 11A = (11B, XXX)
    // 11B = (XXX, 11Z)
    // 11Z = (11B, XXX)
    // 22A = (22B, XXX)
    // 22B = (22C, 22C)
    // 22C = (22Z, 22Z)
    // 22Z = (22B, 22B)
    // XXX = (XXX, XXX)";

    let data = read_data("input-8.txt");

    let mut split = data.lines();
    let instructions = split
        .next()
        .unwrap_or_else(|| panic!("didnt find instructions"));
    split.next();

    let g = split
        .map(|x| parse_input(x))
        .inspect(|x| println!("{:?}", x))
        .collect::<Vec<_>>();
    let graph = build_graph(g);
    let result = traverse_graph_part2(instructions, &graph);
    println!("result is {:?}", result);
}

#[test]
fn test_run() {
    //     let data = "LLR

    // AAA = (BBB, BBB)
    // BBB = (AAA, ZZZ)
    // ZZZ = (ZZZ, ZZZ)";

    let data = read_data("input-8.txt");

    let mut split = data.lines();
    let instructions = split
        .next()
        .unwrap_or_else(|| panic!("didnt find instructions"));
    split.next();

    let g = split
        .map(|x| parse_input(x))
        .inspect(|x| println!("{:?}", x))
        .collect::<Vec<_>>();
    let graph = build_graph(g);
    let result = traverse_graph(instructions, &graph);
    println!("result is {:?}", result);
    // println!("{:?}", graph);
}
