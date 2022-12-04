use std::fs;

fn main() {
    let input = fs::read_to_string("test_input.txt").unwrap();
    let mut calorie_counts = Vec::new();
    let mut curr = 0;
    for line in input.lines() {
        if line.len() == 0 {
            calorie_counts.push(curr);
            curr = 0
        } else {
            curr += line.parse::<i32>().unwrap();
        }
    }
    
    // The loop ends without adding the final elf to the vector
    if curr != 0 {
        calorie_counts.push(curr);
    }

    calorie_counts.sort();
    
    let (highest, second, third) = match &calorie_counts[..] {
        [.., a, b, c] => (c, b, a),
        _ => panic! ("Array has fewer than 3 elements"),
    };

    // Part 1 answer
    // println!("{:?}", calorie_counts.last().unwrap());
    println!("{:?}", highest);

    // Part 2 answer
    println!("{:?}", highest+second+third);
}
