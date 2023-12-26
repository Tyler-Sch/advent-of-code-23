use crate::read_data::read_data;
use std::iter::zip;

fn parse_input(s: &str) -> Vec<i64> {
    let p = s
        .split(":")
        .last()
        .map(|x| {
            x.split_whitespace()
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .unwrap();
    p
}

fn does_complete(speed: i64, time: i64, d: i64) -> Option<i64> {
    let new_time = time - speed;
    if speed * new_time > d {
        Some((speed * new_time) - d)
    } else {
        None
    }
}

fn compute(t: i64, d: i64) -> i64 {
    let r = (0..=t)
        .map(|i| does_complete(i, t, d))
        .filter(|fil| fil.is_some())
        .fold(0, |x, y| x + 1);
    r
}

#[test]
fn test_compute() {
    let s1 = compute(7, 9);
    assert!(s1 == 4);
    let s2 = compute(15, 40);
    assert!(s2 == 8);
    let s3 = compute(30, 200);
    // println!("{}", s3);
    assert!(s3 == 9);

    let s4 = compute(71530, 940200);
    println!("{:?}", s4);
}

#[test]
fn test_run() {
    let data = read_data("input-6.txt");
    //     let data = "Time:      7  15   30
    // Distance:  9  40  200";

    let mut lines = data.lines();
    let t = parse_input(lines.next().unwrap());
    let d = parse_input(lines.next().unwrap());
    let r = zip(t, d)
        .inspect(|x| println!("{:?}", x))
        .map(|(x, y)| compute(x, y))
        .reduce(|x, y| x * y);
    println!("{:?}", r);
}

#[test]
fn test_run_part2() {
    let data = "Time:      57726992
Distance:  291117211762026";

    let mut lines = data.lines();
    let t = parse_input(lines.next().unwrap());
    let d = parse_input(lines.next().unwrap());
    let r = zip(t, d)
        .inspect(|x| println!("{:?}", x))
        .map(|(x, y)| compute(x, y))
        .reduce(|x, y| x * y);
    println!("{:?}", r);
}

#[test]
fn test_does_complete() {
    let s1 = does_complete(0, 7, 9);
    assert!(s1 == None);
    let s2 = does_complete(2, 7, 9);
    assert!(s2 == Some(1));
    let s2 = does_complete(1, 7, 9);
    assert!(s2 == None);
    let s2 = does_complete(4, 7, 9);
    assert!(s2 == Some(3));
}
