use crate::read_data::read_data;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Eq)]
struct Hand<'a> {
    hand: &'a str,
    bet: i32,
}
impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Some(self.hand.cmp(other.hand))
        let order_map = HashMap::from([
            ('A', 1),
            ('K', 2),
            ('Q', 3),
            // ('J', 4), // comment out for part 2
            ('T', 5),
            ('9', 6),
            ('8', 7),
            ('7', 8),
            ('6', 9),
            ('5', 10),
            ('4', 11),
            ('3', 12),
            ('2', 13),
            ('J', 14), // modify for part 2
        ]);
        let p = self
            .hand
            .chars()
            .zip(other.hand.chars())
            .map(|(i, j)| order_map.get(&i).unwrap().cmp(order_map.get(&j).unwrap()))
            .collect::<Vec<_>>();
        for i in p {
            if i != Ordering::Equal {
                return Some(i);
            }
        }
        Some(Ordering::Equal)
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a> Hand<'a> {
    fn new(hand: &'a str, bet: i32) -> Self {
        Hand { hand, bet }
    }

    fn parse(&self) -> HashMap<char, i32> {
        let mut hm = HashMap::new();
        self.hand.chars().for_each(|x| {
            hm.entry(x).and_modify(|e| *e += 1).or_insert(1);
        });
        // alter for part2
        let num_js = &hm.remove(&'J');
        // let _ = match num_js  {
            if let Some(i)  = num_js {
                if *i == 5 {
                    hm.insert('J', 5);
                } else {
                   let max_val = &hm.iter().fold(('a', 0), |(x1,y1), (x2, y2)| {
                    if y1 > *y2 { (x1, y1) } else {(*x2, *y2)}
                   });
                   hm.entry(max_val.0).and_modify(|e| *e += i);
                }
            };
        // end alter
        hm
    }
}

#[derive(PartialEq, Debug, Eq, PartialOrd, Ord)]
enum HandType<'b> {
    Fiver(Hand<'b>),
    Four(Hand<'b>),
    FullHouse(Hand<'b>),
    Three(Hand<'b>),
    TwoPair(Hand<'b>),
    Pair(Hand<'b>),
    HighCard(Hand<'b>),
    Missing,
}

impl<'b> HandType<'b> {
    fn new(s: Hand<'b>) -> Self {
        let parsed = s.parse();
        let mut vectorized = parsed.into_iter().map(|(x, y)| y).collect::<Vec<i32>>();
        vectorized.sort();
        // println!("vectorized: {:?}", vectorized);
        match vectorized[..] {
            [5] => HandType::Fiver(s),
            [1, 4] => HandType::Four(s),
            [2, 3] => HandType::FullHouse(s),
            [1, 1, 3] => HandType::Three(s),
            [1, 2, 2] => HandType::TwoPair(s),
            [1, 1, 1, 2] => HandType::Pair(s),
            [1, 1, 1, 1, 1] => HandType::HighCard(s),
            _ => HandType::Missing,
        }
    }

    fn get_bet(&self) -> i32 {
        match self {
            HandType::Fiver(i) => i.bet,
            HandType::Four(i) => i.bet,
            HandType::FullHouse(i) => i.bet,
            HandType::HighCard(i) => i.bet,
            HandType::Pair(i) => i.bet,
            HandType::Three(i) => i.bet,
            HandType::TwoPair(i) => i.bet,
            _ => panic!("could not find match to extract bet"),
        }
    }
}

fn parse_input(s: &str) -> Hand {
    let mut it = s.split_whitespace();
    let h = it.next();
    let b = it.next().map(|x| x.parse::<i32>().unwrap());
    Hand::new(h.unwrap(), b.unwrap())
}

#[test]
fn test_example() {
    let data = read_data("input-7.txt");
    //     let data = "32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483";
    let mut r = data.lines().collect::<Vec<&str>>();
    let mut results = vec![];
    for i in r {
        if i != "" {
            let parsed = parse_input(i);
            println!("{:?}", parsed);
            let ht = HandType::new(parsed);
            results.push(ht);
        }
    }

    results.sort();
    let p = results
        .into_iter()
        .rev()
        .enumerate()
        .inspect(|(x, y)| println!("{},{:?}", x, y))
        .map(|(x, y)| (x as i32 + 1) * y.get_bet())
        .reduce(|x, y| x + y);
    println!("{:?}", p);
}

// #[test]
// fn test_ordering_hand() {
//     let h1 = Hand::new("AAAAA", 1);
//     let h2 = Hand::new("33333", 1);
//     assert!(h1 < h2);
//     // println!("{:?}", r);
//     let h1 = Hand::new("AAAAA", 1);
//     let h2 = Hand::new("AAAAA", 1);
//     assert!(h1 == h2);
//     let h1 = Hand::new("A2345", 1);
//     let h2 = Hand::new("AQ345", 15);
//     assert!(h1 > h2);
//     let mut v = vec![&h1, &h2];
//     v.sort();
//     // println!("{:?}", v);
//     assert!(v[..] == [&h2, &h1]);
// }

// #[test]
// fn test_sort_enum() {
//     let hand = Hand::new("AAAAA", 1);
//     let hand2 = Hand::new("KAAAA", 1);
//     let hand3 = Hand::new("AAAAA", 14);
//     let mut p = vec![HandType::HighCard(&hand), HandType::Pair(&hand2), HandType::Three(&hand3)];
//     p.sort();
//     println!("{:?}", p);

//     let mut i = vec![HandType::Three(&hand2), HandType::Three(&hand3)];
//     i.sort();
//     println!("{:?}", i);

// }

// #[test]
// fn test_HandTypeNew() {
//     let hand = Hand::new("AAAAA", 1);
//     assert!(HandType::new(&hand) == HandType::Fiver(&hand));
//     let hand = Hand::new("AAAA3", 1);
//     assert!(HandType::new(&hand) == HandType::Four(&hand));
//     let hand = Hand::new("AA4AA", 1);
//     assert!(HandType::new(&hand) == HandType::Four(&hand));
//     let hand = Hand::new("A4A4A", 1);
//     assert!(HandType::new(&hand) == HandType::FullHouse(&hand));
//     let hand = Hand::new("A4A3A", 1);
//     assert!(HandType::new(&hand) == HandType::Three(&hand));
//     let hand = Hand::new("AKA3K", 1);
//     assert!(HandType::new(&hand) == HandType::TwoPair(&hand));
//     let hand = Hand::new("A454K", 1);
//     assert!(HandType::new(&hand) == HandType::Pair(&hand));
//     let hand = Hand::new("A453K", 1);
//     assert!(HandType::new(&hand) == HandType::HighCard(&hand));
// }

// #[test]
// fn test_hand_parse() {
//     let hand = Hand::new("AAAAA", 1);
//     let hm = hand.parse();
//     let expected = HashMap::from([('A', 5)]);
//     assert!(hm == expected);
//     let hand = Hand::new("AA122", 1);
//     let hm = hand.parse();
//     let expected = HashMap::from([('A', 2), ('1', 1), ('2', 2)]);
//     assert!(hm == expected);
// }
