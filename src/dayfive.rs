use std::{fmt::Error, string::ParseError, collections::btree_map::Range};

use crate::read_data::read_data;

#[derive(Debug)]
struct SeedRange {
    source_num: i64,
    dest_num: i64,
    range: i64,
}

impl SeedRange {
    // fn map_range(&self, s: Seed_range) -> Option<Seed_range> {
    //     // range is totally less than possible range
    //     let source_high = self.source_num + self.range;

    //     if s.high < self.source_num {
    //         None
    //     } else if s.low <= self.source_num && s.high > self.source_num && s.high <= source_high {
    //         // range starts lower but ends in range
    //         let range_diff_high = s.high - self.source_num;
    //         Some(Seed_range::new(self.dest_num, self.dest_num + range_diff_high))
    //     } else if s.low <= self.source_num && s.high > source_high {
    //         // range starts lower but ends higher
    //         Some(Seed_range::new(self.dest_num, self.dest_num + self.range))
    //     } else if s.low >= self.source_num && s.high <= source_high {
    //         // range starts higher ends lower
    //         let range_diff_low = s.low - self.source_num;
    //         let range_diff_high = s.high - self.source_num;
    //         Some(Seed_range::new(self.dest_num + range_diff_low, self.dest_num + range_diff_high))
    //     } else if s.low >= self.source_num && s.low < source_high && s.high > source_high {
    //         // range starts higher ends higher
    //         let range_diff_low = s.low - self.source_num; 
    //         Some(seed_range::new(self.dest_num + range_diff_low, self.dest_num + self.range))
    //     } else if s.low > source_high {
    //         // range starts higher then end
    //         None
    //     } 
    //     else {
    //         panic!("Did not find any map range matches")
    //     }
    // }
}

#[derive(Debug)]
struct SeedMap<'a> {
    name: &'a str,
    ranges: Vec<SeedRange>,
}

struct Seed_range {
    low: i64,
    high: i64
}

impl Seed_range {
    fn new(l: i64, h: i64) -> Self {
        Seed_range { low: l, high: h}
    }
}


impl SeedMap<'_> {
    fn map_item(&self, seed_num: i64) -> i64 {
        for ranges in &self.ranges {
            let source = ranges.source_num;
            let source_upper = ranges.source_num + ranges.range;
            if (seed_num >= source && seed_num < source_upper) {
                return ranges.dest_num + (seed_num - source)
            } 
        }
        seed_num
    }
    
    // fn map_range(&self, mut sr: Seed_range) -> Vec<Seed_range> {
    //     // situations:
    //     let mut result = vec![];
    //     for i in seed_range {
    //     // range is totally less than possible range
    //     // range starts lower but ends in mid of range
    //     // range starts lower but ends higher
    //     // range starts higher ends lower
    //     // range starts higher ends higher
    //     // range starts higher then end
    //     }

    // }
}

impl SeedRange {
    fn new(d: i64, s: i64, r: i64) -> Self {
        SeedRange {
            source_num: s,
            dest_num: d,
            range: r
        }
    }
}
fn parse_seeds(s: &str) -> Vec<i64> {
    s.split(": ")
        .last()
        .unwrap_or_else(|| panic!("error parsing seed line"))
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn parse_map(s: &str) -> Result<SeedMap, Error> {
    let mut iters = s.split("\n");
    let name = iters.next().unwrap().split_whitespace().next().unwrap();
    // println!("{}", name);
    let mut range = vec![];
    for line in iters {
        let split_line = line
            .split_whitespace()
            .map(|x| {
                x.parse::<i64>()
                    .unwrap_or_else(|z| panic!("error parsing int in parse_map"))
            })
            .collect::<Vec<i64>>();
        let sr = SeedRange {
            dest_num: split_line[0],
            source_num: split_line[1],
            range: split_line[2],
        };
        range.push(sr);
    }

    Ok(SeedMap {
        name: name,
        ranges: range,
    })
}

fn parse_day_5_input(s: &str) -> (Vec<i64>, Vec<SeedMap>) {
    let mut input_iter = s.split("\n\n");
    let seeds = input_iter
        .next()
        .unwrap_or_else(|| panic!("could not get seed line"));
    let parsed_seeds = parse_seeds(seeds);
    // println!("{:?}", parsed_seeds);
    let mut seedmaps = vec![];
    for i in input_iter {
        let r = parse_map(i).unwrap();
        seedmaps.push(r);
    }
    (parsed_seeds, seedmaps)
}

fn apply_map(s: i64, maps: &Vec<SeedMap>) -> i64 {
        // println!("applying {:?}\n", maps);
        let p = &maps.into_iter().fold(s, |x, y| y.map_item(x));
        // println!("result is {}", p);
        p.to_owned()
}

fn parse_seeds_part_2(v: Vec<i64>, maps: &Vec<SeedMap>) -> Vec<i64> {
    let mut m = vec![];
    let mut iters = v.into_iter();
    for _ in 0..(iters.len() / 2) {
        let first_num = iters.next().unwrap();
        let range = iters.next().unwrap();
        println!("first_num: {:?}, range: {:?}", first_num, range);
        // for i in first_num..(first_num + range) {
        //     m.push(i);
        // }
        let z = first_num..(first_num + range);
        let r = z.into_iter().map(|x| apply_map(x, maps)).reduce(|x, y| {
            if x > y {x} else {y}
        });
        m.push(r.unwrap())
    }
    m
}

#[test]
fn test_range() {
    let mut a = 1..4;
    let mut b = 5i64..1_000_000_000_000;
    // let c = a.nth(1);
    println!("{:?}", b.last());
    // println!("{:?}", a.next());
}

// #[test]
pub fn run_day_5() {
    let data = read_data("input-5.txt");
//     let data = "seeds: 79 14 55 13

// seed-to-soil map:
// 50 98 2
// 52 50 48

// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15

// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4

// water-to-light map:
// 88 18 7
// 18 25 70

// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13

// temperature-to-humidity map:
// 0 69 1
// 1 0 69

// humidity-to-location map:
// 60 56 37
// 56 93 4";
    // let (seeds, maps) = parse_day_5_input(&data);
    // let mut r = vec![];
    // for s in seeds {
    //     // let p = &maps.into_iter().fold(s, |x, y| y.map_item(x));
    //     let p = apply_map(s, &maps);
    //     // println!("{:?}", p);
    //     r.push(p)
    // }
    // // r.sort();
    // println!("{:?}", r);
    // let z = r.into_iter().reduce(|x, y| {
    //     if x < y {
    //         x
    //     } else { y}
    // });
    // println!("{:?}", z);

    let (seeds, maps) = parse_day_5_input(&data);
    let new_seeds = parse_seeds_part_2(seeds, &maps);
    // println!("{:?}", new_seeds);
    // let mut r = vec![];
    // for s in new_seeds {
    //     // let p = &maps.into_iter().fold(s, |x, y| y.map_item(x));
    //     let p = apply_map(s, &maps);
    //     // println!("{:?}", p);
    //     r.push(p)
    // }
    
    // r.sort();
    // println!("{:?}", r);
    let z = new_seeds.into_iter().reduce(|x, y| {
        if x < y {
            x
        } else { y}
    });
    println!("{:?}", z);


}


#[test]
fn test_seed_ranges() {
    let s1  = SeedRange::new(50, 98, 2);
    let s2 = SeedRange::new(52, 50, 48);
    let sm = SeedMap {
        name: "seed-to-soil",
        ranges: vec![s1, s2]
    };

    let seed1 = sm.map_item(10);
    assert!(seed1 == 10);
    let seed2 = sm.map_item(98);
    assert!(seed2 == 50);
    let seed3 = sm.map_item(99);
    assert!(seed3 == 51);
    let seed4 = sm.map_item(53);
    assert!(seed4 == 55);
}