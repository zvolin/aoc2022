use itertools::Itertools;
use std::collections::HashSet;

static INPUT: &str = include_str!("../input");

type Item = char;

trait Priority {
    fn priority(self) -> usize;
}

impl Priority for Item {
    fn priority(self) -> usize {
        if self.is_ascii_lowercase() {
            (self as u32 - 96u32) as usize
        } else if self.is_ascii_uppercase() {
            (self as u32 - 38u32) as usize
        } else {
            0
        }
    }
}

fn main() {
    let res = INPUT
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(fst, snd)| {
            (
                fst.chars().collect::<HashSet<_>>(),
                snd.chars().collect::<HashSet<_>>(),
            )
        })
        .map(|(fst, snd)| *fst.intersection(&snd).next().unwrap())
        .map(Item::priority)
        .sum::<usize>();
    // part1
    println!("{res:?}");

    let res = INPUT
        .lines()
        .tuples::<(_, _, _)>()
        .map(|group| {
            (
                group.0.chars().collect::<HashSet<_>>(),
                group.1.chars().collect::<HashSet<_>>(),
                group.2.chars().collect::<HashSet<_>>(),
            )
        })
        .map(|group| &(&group.0 & &group.1) & &group.2)
        .map(|common| common.iter().next().unwrap().priority())
        .sum::<usize>();
    // part2
    println!("{res:?}");
}

#[cfg(test)]
#[test]
fn test_priority() {
    for (item, priority) in ('a'..='z').chain('A'..='Z').zip(1..53) {
        assert_eq!(item.priority(), priority);
    }
}
