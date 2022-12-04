use std::ops::RangeInclusive;

static INPUT: &str = include_str!("../input");

fn parse_range(input: &str) -> Option<RangeInclusive<usize>> {
    let (start, end) = input.split_once('-')?;
    let start = start.parse().ok()?;
    let end = end.parse().ok()?;
    Some(start..=end)
}

trait Wraps {
    fn wraps(&self, other: &Self) -> bool;
}

impl<T: PartialOrd> Wraps for RangeInclusive<T> {
    fn wraps(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }
}

trait Overlaps {
    fn overlaps(&self, other: &Self) -> bool;
}

impl<T: PartialOrd> Overlaps for RangeInclusive<T> {
    fn overlaps(&self, other: &Self) -> bool {
        let has_other_start = self.start() <= other.start() && self.end() >= other.start();
        let has_other_end = self.start() <= other.end() && self.end() >= other.end();
        has_other_start || has_other_end || other.wraps(self)
    }
}

#[cfg(test)]
#[test]
fn test_overlap() {
    assert!((0..=5).overlaps(&(5..=8)));
    assert!((5..=8).overlaps(&(0..=5)));
    assert!(!(0..=4).overlaps(&(5..=8)));
    assert!((0..=7).overlaps(&(5..=8)));
    assert!((5..=8).overlaps(&(0..=7)));

    assert!((6..=6).overlaps(&(4..=6)));
    assert!((6..=6).overlaps(&(4..=8)));
    assert!((4..=8).overlaps(&(6..=6)));
}

fn main() {
    let res = INPUT
        .lines()
        .flat_map(|line| line.split_once(','))
        .flat_map(|(fst, snd)| Some((parse_range(fst)?, parse_range(snd)?)))
        .filter(|(fst, snd)| fst.wraps(snd) || snd.wraps(fst))
        .count();

    // part1
    println!("{res}");

    let res = INPUT
        .lines()
        .flat_map(|line| line.split_once(','))
        .flat_map(|(fst, snd)| Some((parse_range(fst)?, parse_range(snd)?)))
        .filter(|(fst, snd)| fst.overlaps(snd))
        .count();

    // part2
    println!("{res}");
}
