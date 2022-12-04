use std::fs;

fn part1(file_name: &str) -> u32 {
    let input = fs::read_to_string(file_name).unwrap();
    let mut score = 0;
    for line in input.lines() {
        let moves = line.split(" ").collect::<Vec<_>>();
        let elf_move = moves[0];
        let my_move = moves[1];

        if elf_move == "A" {
            if my_move == "X" {
                // Draw
                score += 4;
            } else if my_move == "Y" {
                // Win
                score += 8;
            } else {
                // Lose
                score += 3;
            }
        } else if elf_move == "B" {
            if my_move == "X" {
                // Lose
                score += 1;
            } else if my_move == "Y" {
                // Draw
                score += 5;
            } else {
                // Win
                score += 9;
            }
        } else {
            if my_move == "X" {
                // Win
                score += 7;
            } else if my_move == "Y" {
                // Lose
                score += 2;
            } else {
                // Draw
                score += 6;
            }
        }
    }
    return score;
}

fn part2(file_name: &str) -> u32 {
    let input = fs::read_to_string(file_name).unwrap();
    let mut score = 0;
    for line in input.lines() {
        let moves = line.split(" ").collect::<Vec<_>>();
        let elf_move = moves[0];
        let outcome = moves[1];

        if elf_move == "A" {
            if outcome == "X" {
                // Lose
                score += 3;
            } else if outcome == "Y" {
                // Draw
                score += 4;
            } else {
                // Win
                score += 8;
            }
        } else if elf_move == "B" {
            if outcome == "X" {
                // Lose
                score += 1;
            } else if outcome == "Y" {
                // Draw
                score += 5;
            } else {
                // Win
                score += 9;
            }
        } else {
            if outcome == "X" {
                // Lose
                score += 2;
            } else if outcome == "Y" {
                // Draw
                score += 6;
            } else {
                // Win
                score += 7;
            }
        }
    }
    return score;
}

fn main() {
    let input = "test_input.txt";
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}
