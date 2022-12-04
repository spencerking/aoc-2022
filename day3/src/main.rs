use std::fs;

fn get_priority(c: char) -> u32 {
    (match c {
	'a' ..= 'z' => c as u8 - b'a' + 1,
	'A' ..= 'Z' => c as u8 - b'A' + 27,
	_ => unreachable!()
    }) as u32
}

fn part1(file_name: &str) -> u32 {
    let input = fs::read_to_string(file_name).unwrap();
    let mut sum = 0;
    for line in input.lines() {
	let (left, right) = line.split_at(line.len() / 2);
	sum += get_priority(left.chars().find(|&c| right.contains(c)).unwrap());
    }
    return sum;
}

fn part2(file_name: &str) -> u32 {
    let input = fs::read_to_string(file_name).unwrap();
    let mut lines = input.lines();
    let mut sum = 0;
    while let (Some(first), Some(second), Some(third)) = (lines.next(), lines.next(), lines.next()) {
	sum += get_priority(first.chars().find(|&c| second.contains(c) && third.contains(c)).unwrap());
    }
    return sum;
}

fn main() {
    let input = "test_input.txt";
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
}
