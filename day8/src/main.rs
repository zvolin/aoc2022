use std::iter;

static INPUT: &str = include_str!("../input");

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().filter_map(|ch| ch.to_digit(10)).collect())
        .collect()
}

fn visibility<I: Iterator<Item = u32>>(input: I) -> Vec<bool> {
    let mut highest = -1isize;
    input
        .map(|e| e as isize)
        .map(move |e| {
            if e > highest {
                highest = e;
                true
            } else {
                false
            }
        })
        .collect()
}

fn transpose<T: Copy>(input: &[Vec<T>]) -> Vec<Vec<T>> {
    (0..input[0].len())
        .map(|n| input.iter().map(|row| row[n]).collect())
        .collect()
}

fn visibility_map(input: &[Vec<u32>]) -> Vec<Vec<bool>> {
    // idea to create 4 bitmasks, one from each side, and OR them at the end sound funny at
    // first...
    // trust me
    let from_left: Vec<Vec<_>> = input
        .iter()
        .map(|row| visibility(row.iter().copied()))
        .collect();
    let from_right: Vec<Vec<_>> = input
        .iter()
        .map(|row| {
            visibility(row.iter().copied().rev())
                .into_iter()
                .rev()
                .collect()
        })
        .collect();
    let from_top: Vec<Vec<_>> = transpose(input)
        .into_iter()
        .map(|col| visibility(col.into_iter()))
        .collect();
    let from_top = transpose(&from_top);
    let from_bottom: Vec<Vec<_>> = transpose(input)
        .into_iter()
        .map(|col| {
            visibility(col.into_iter().rev())
                .into_iter()
                .rev()
                .collect()
        })
        .collect();
    let from_bottom = transpose(&from_bottom);

    (0..input.len())
        .map(|y| {
            (0..input[0].len())
                .map(|x| from_left[y][x] || from_right[y][x] || from_top[y][x] || from_bottom[y][x])
                .collect()
        })
        .collect()
}

fn view_map(input: &[Vec<u32>]) -> Vec<usize> {
    let row_len = input[0].len();
    let col_len = input.len();
    let carthesian_product = (0..row_len).flat_map(|x| iter::repeat(x).zip(0..col_len));
    carthesian_product
        .map(|(x, y)| {
            let height = input[y][x];

            let is_lower = |x: usize, y: usize| input[y][x] < height;

            // yeah please just don't comment, a bad day xd
            let above: Vec<_> = (0..y).rev().take_while(|&y| is_lower(x, y)).collect();
            let above =
                above.len() + above.last().map(|&y| y != 0).unwrap_or_else(|| y != 0) as usize;
            let below: Vec<_> = (y + 1..col_len).take_while(|&y| is_lower(x, y)).collect();
            let below = below.len()
                + below
                    .last()
                    .map(|&y| y != col_len - 1)
                    .unwrap_or_else(|| y != col_len - 1) as usize;
            let on_left: Vec<_> = (0..x).rev().take_while(|&x| is_lower(x, y)).collect();
            let on_left =
                on_left.len() + on_left.last().map(|&x| x != 0).unwrap_or_else(|| x != 0) as usize;
            let on_right: Vec<_> = (x + 1..row_len).take_while(|&x| is_lower(x, y)).collect();
            let on_right = on_right.len()
                + on_right
                    .last()
                    .map(|&x| x != row_len - 1)
                    .unwrap_or_else(|| x != row_len - 1) as usize;

            above * below * on_left * on_right
        })
        .collect()
}

fn main() {
    let _input = "30373
25512
65332
33549
35390";
    let forest = parse_input(INPUT);
    let visibility_map = visibility_map(&forest);

    let visible = visibility_map
        .iter()
        .map(|row| row.iter().filter(|&&e| e).count())
        .sum::<usize>();
    println!("{visible}");

    let view_map = view_map(&forest);
    println!("{}", view_map.iter().max().unwrap());
}
