use crate::read_data::read_data;
use std::collections::HashSet;

#[derive(Debug, Default, PartialEq)]
struct Card<'a> {
    gm: &'a str,
    win_num: HashSet<&'a str>,
    pull_num: HashSet<&'a str>,
}

impl Card<'_> {
    fn from_str(s: &str) -> Card {
        if s == "" {
            return Default::default();
        }
        let mut win_num = HashSet::new();
        let mut pull_num = HashSet::new();
        let mut s_iter = s.split(":");
        let first = s_iter
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();
        let gm = first
            .get(1)
            .unwrap_or_else(|| panic!("missing game nubmer"))
            .to_owned();
        let _second = s_iter.next().map(|x| {
            let mut inner_iter = x.split("| ");
            // let a = inner_iter.next().unwrap().split_whitespace().for_each(|x| println!("{}", x));
            // println!("{:?}", a);
            let _ans = inner_iter.next().map(|n| {
                n.split_whitespace().for_each(|ni| {
                    win_num.insert(ni);
                })
            });
            let _pull = inner_iter.next().map(|n| {
                n.split_whitespace().for_each(|ni| {
                    pull_num.insert(ni);
                })
            });
        });

        Card {
            gm,
            win_num,
            pull_num,
        }
    }

    fn get_same_nums(&self) -> Vec<&str> {
        let mut win_nums = vec![];
        for i in &self.pull_num {
            if self.win_num.contains(i) {
                win_nums.push(*i)
            }
        }
        win_nums
    }

    fn do_math(&self) -> usize {
        let nums = self.get_same_nums();
        let z = nums
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        // println!("{:?}", z);
        if z.len() == 1 {
            1
        } else if z.len() > 1 {
            2_i32
                .pow((z.len() - 1).try_into().unwrap())
                .try_into()
                .unwrap()
        } else {
            0
        }
    }

    fn num_matches(&self) -> usize {
        self.get_same_nums().len()
    }
    fn get_card_num(&self) -> usize {
        self.gm.parse::<usize>().unwrap()
    }
}

fn traverse_cards(master_list: &Vec<Card>) -> usize {
    let mut stack = vec![];
    for i in master_list.iter().rev() {
        if i != &Card::default() {
        stack.push(i);
        }
    }
    let mut total_count = 0;

    while stack.len() > 0 {
        let item = stack.pop().unwrap_or_else(|| panic!("Didnt find a card"));
        total_count += 1;
        let num_matches = item.num_matches();
        let current_card_num = item.get_card_num();
        let slice = &master_list[current_card_num .. current_card_num + num_matches];

        for i in slice {
            stack.push(i);
        }




    } 
    total_count

}

#[test]
fn test_run() {
    let d = read_data("input-4.txt");
    // let lines = d.split("\n").next().unwrap();
    // let r = Card::from_str(lines);
    // println!("{:?}", r);
    // let z = r.get_same_nums();
    // println!("{:?}", z);
    // let zz = r.do_math();
    // println!("{}", zz);
    let lines = &d
        .split("\n")
        .map(|x| Card::from_str(x))
        .map(|x| x.do_math())
        .reduce(|x, y| x + y);
    println!("{:?}", lines);
}

pub fn run_part2() -> usize {
    let d = read_data("input-4.txt");
//     let test_data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    // let card = test_data.split("\n").map(|x| Card::from_str(x)).collect::<Vec<Card>>();
    // // println!("{:?}", card);
    // let r = traverse_cards(&card);
    // println!("{}", r);

    // TODO: optimize with memoization or look up an algo
    let day2_dat = d.split("\n").map(|x| Card::from_str(x)).collect::<Vec<Card>>();
    let r = traverse_cards(&day2_dat);
    r

}
