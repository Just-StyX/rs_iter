use itertools::Itertools;

fn main() {
    let items = 1..=3;
    let _pairs: Vec<_> = items.into_iter().combinations(2).collect();
    // println!("Pairs: {:#?}", _pairs);

    let words = vec!["Rust", "is", "Awsome"];
    let _sentence = words.into_iter().join(" -> ");
    // println!("{}", _sentence);

    let timer: U32Range = (14, 7).into();
    let sum: u32 = timer.sum();
    println!("Total sum: {}", sum);
}

struct U32Range {
    start: u32,
    end: u32,
}

impl From<(u32, u32)> for U32Range {
    fn from(value: (u32, u32)) -> Self {
        let value = swap(value.0, value.1);
        println!("{:?}", value);
        Self { start: value.0, end: value.1 }
    }
}

impl Iterator for U32Range {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}

fn swap(x: u32, y: u32) -> (u32, u32) {
    if x > y { (y, x) } else { (x, y) }
}