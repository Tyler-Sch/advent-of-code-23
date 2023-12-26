use crate::read_data::read_data;

fn get_diff(s: &Vec<i32>) -> Option<Box<Vec<i32>>> {
    let mut r = vec![];
    let mut all_zero = false;
    for i in 0..s.len() {
        if i != s.len() - 1 {
            r.push(s[i + 1] - s[i]);
        }
    }
    if r.iter().filter(|x| **x == 0).collect::<Vec<&i32>>().len() == s.len() - 1 {
        None
    } else {
        Some(Box::new(r))
    }
}

fn get_last_sequence(s: Vec<i32>) -> Option<i32> {
    let mut res = Box::new(s);
    let mut stack = vec![res.clone()];
    while let Some(i) = get_diff(&res) {
        stack.push(i.to_owned());
        res = i;
    }
    // println!("stack is: {:?}", stack);

    let mut previous_last = 0;
    for i in stack.into_iter().rev() {
        let last_val = i.last().unwrap();
        previous_last = *last_val + previous_last;
    }
    Some(previous_last)
}

fn get_first_sequence(s: Vec<i32>) -> Option<i32> {
    let mut res = Box::new(s);
    let mut stack = vec![res.clone()];
    while let Some(i) = get_diff(&res) {
        stack.push(i.to_owned());
        res = i;
    }
    // println!("stack is: {:?}", stack);

    let mut previous_last = 0;
    for i in stack.into_iter().rev() {
        let last_val = i.first().unwrap();
        previous_last = *last_val - previous_last;
    }
    Some(previous_last)
}

fn parse_data(s: &str) -> Vec<i32> {
    s.split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[test]
fn test_last_sequence() {
    let s = vec![10, 13, 16, 21, 30, 45];
    let r = get_last_sequence(s);
    println!("r is {:?}", r);
}

#[test]
fn test_get_diff() {
    let got = get_diff(&vec![1, 2, 3, 4, 5]);
    let got2 = get_diff(&got.unwrap());
    // println!("got2 is: {:?}", got2);
    assert!(got2 == None);
}

#[test]
fn test_run_day_nine() {
    //     let data = "0 3 6 9 12 15
    // 1 3 6 10 15 21
    // 10 13 16 21 30 45";
    let data = read_data("input-9.txt");

    let parsed = data.lines().map(|x| parse_data(x));
    let r = parsed
        .map(|x| get_last_sequence(x))
        .fold(0, |x, y| x + y.unwrap());
    println!("day nine result is {}", r);
}

#[test]
fn test_run_day_nine_part2() {
//         let data = "0 3 6 9 12 15
// 1 3 6 10 15 21
// 10 13 16 21 30 45";
    let data = read_data("input-9.txt");

    let parsed = data.lines().map(|x| parse_data(x));
    let r = parsed
        .map(|x| get_first_sequence(x))
        .fold(0, |x, y| x + y.unwrap());
    println!("day nine result is {}", r);
}