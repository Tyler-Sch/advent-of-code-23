pub mod read_data;
pub mod daytwo;
pub mod daythree;
pub mod dayfour;
pub mod dayfive;
pub mod daysix;
pub mod dayseven;
pub mod dayeight;
pub mod daynine;
pub mod dayten;
pub mod dayeleven;

use regex::Regex;
use read_data::read_data;
use dayfour::run_part2;
use dayfive::run_day_5;
use dayeight::dayeight_part2;

fn main() {
    // let input = read_data("input-1.txt");
    // let split = input.split("\n");
    // let mut ar = vec![];
    // for i in split {
    //     if i == "" {
    //         break
    //     }
    //     let intted = parse_input(i);
    //     ar.push(intted);
    // }
    // let s = ar.into_iter().reduce(|x, y| x + y).unwrap();
    // println!("{}", s);

    // let r = run_part2();
    // println!("the result for dayfour part 2 are {}", r);
    // let r = run_day_5();
    // let r = dayeight_part2();


}



fn parse_input(s: &str) -> i32 {
    let re = Regex::new(r".*?([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re2 = Regex::new(r".*([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let nums = re.captures(s).unwrap();
    let second_nums = re2.captures(s).unwrap();
    let first = nums.get(1).unwrap().as_str();
    let second = second_nums.get(1).unwrap().as_str();
    let first_translate = translate_number(first);
    let second_translate = translate_number(second);
    let r = format!("{}{}", first_translate, second_translate);
    r.parse::<i32>().unwrap()
}

fn translate_number(word: &str) -> char {
    match word {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => word.chars().next().unwrap()
    }
}


#[test]
fn test_translate_number() {
    let d = "one";
    let r = translate_number(d);
    println!("{}", r);

    let d = "1";
    let r = translate_number(d);
    println!("{}", r);
}

#[test]
fn test_this_thing() {
    let i = "2qlljdqcbeight";
    let r = parse_input(i);
    println!("{}",r);
    let i = "a1b2c3d4e5f";
    let r = parse_input(i);
    println!("{}", r);
    let i = "8hl5eight";
    let r = parse_input(i);
    println!("{}", r);

}
