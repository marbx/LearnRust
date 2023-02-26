// https://gist.github.com/anonymous/5812ecd153ddcd5303422626c899a727
// https://www.reddit.com/r/rust/comments/5mnj3y/which_has_better_performance_a_hashmap_or_a/


#![feature(test)]

#[macro_use]
extern crate lazy_static;
extern crate test;

fn conv_map(digit: char) -> Option<u32> {
    match digit {
        'I' => Some(1),
        'V' => Some(5),
        'X' => Some(10),
        'L' => Some(50),
        'C' => Some(100),
        'D' => Some(500),
        'M' => Some(1000),
        _   => None,
    }
}

fn main() {
    println!("Hello, world");
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    lazy_static! {
        static ref CONV_MAP: HashMap<char, u32> = {
            let mut map = HashMap::new();
            map.insert('I', 1);
            map.insert('V', 5);
            map.insert('X', 10);
            map.insert('L', 50);
            map.insert('C', 100);
            map.insert('D', 500);
            map.insert('M', 1000);
            map
        };
    }
    use super::*;
    use test::Bencher;

    #[test]
    fn conv_map_test() {
        assert_eq!(conv_map('I'), Some(1));
    }

    #[test]
    fn conv_lookup_test() {
        assert_eq!(CONV_MAP.get(&'I'), Some(&1));
    }


    #[bench]
    fn conv_map_bench(b: &mut Bencher) {
        b.iter(|| {
            let letter = test::black_box('I'); conv_map(letter);
            let letter = test::black_box('I'); conv_map(letter);
        });
    }


    #[bench]
    fn conv_lookup_bench(b: &mut Bencher) {
        CONV_MAP.get(&'I'); // don't want to benchmark intialisation
        b.iter(|| CONV_MAP.get(&'I'));
    }
}