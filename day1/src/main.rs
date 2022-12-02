static INPUT: &str = include_str!("../input");

fn main() {
    let mut elves = INPUT.lines().fold(vec![0usize], |mut elves, line| {
        if line.is_empty() {
            elves.push(0);
        } else {
            *elves.last_mut().unwrap() += line.parse::<usize>().unwrap();
        }
        elves
    });
    // part 1 answer
    println!("{}", elves.iter().max().unwrap());

    elves.sort_unstable();
    let (_, top3) = elves.split_at(elves.len() - 3);
    println!("{}", top3.iter().sum::<usize>());
}
