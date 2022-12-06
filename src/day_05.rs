use std::collections::VecDeque;

/** On Day 5, the fun part is the parsing. */
pub fn parse_input_data(input: &str) -> (Vec<VecDeque<char>>, Vec<(usize, usize, usize)>) {
    // Split the input string in two strings, where the empty line is
    let mut split = input.split("\n\n");
    let stacks_str = split.next().expect("Unable to get first part of input");
    let instructions_str = split.next().expect("Unable to get second part of input");

    //println!("Stacks: {}", stacks_str);
    //println!("Instructions: {}", instructions_str);

    let stacks = parse_stacks(stacks_str);
    let instructions = parse_instructions(instructions_str);

    return (stacks, instructions);
}

pub fn parse_stacks(input: &str) -> Vec<VecDeque<char>> {
    let lines = input.lines().collect::<Vec<&str>>();

    let lines_length = lines.get(0).expect("Unable to get first line").len();
    // Check that all lines have the same length
    for line in lines.iter().skip(1) {
        if line.len() != lines_length {
            panic!(
                "Line {} has a different length than the first line. \
                    Did your editor remove trailing whitespaces without telling you?",
                line
            );
        }
    }

    let nb_stack = lines_length / 4 + 1;
    //println!("Number of stacks: {}", nb_stack);
    let mut stacks = Vec::with_capacity(nb_stack);
    // Allocate the stacks
    for _ in 0..nb_stack {
        stacks.push(VecDeque::new());
    }

    // For every line except the last one
    for line in lines.iter().take(lines.len() - 1) {
        let chars = line.chars().collect::<Vec<char>>();
        // For every fourth character in the line, starting at 2
        for i in (1..lines_length).step_by(4) {
            let chararacter = chars.get(i).expect("Unable to get character");
            // If Character is not a space but between A and Z
            if *chararacter != ' ' && *chararacter >= 'A' && *chararacter <= 'Z' {
                // Add it to the stack
                let stack_index = (i - 1) / 4;
                stacks
                    .get_mut(stack_index)
                    .expect("Unable to get stack")
                    .push_front(*chararacter);
            }
        }
    }

    //println!("Stacks: {:?}", stacks);
    return stacks;
}

pub fn parse_instructions(input: &str) -> Vec<(usize, usize, usize)> {
    return input
        .lines()
        .map(|line| {
            //println!("Line: {}", line);
            let (moves, from, to) = sscanf::sscanf!(line, "move {usize} from {usize} to {usize}")
                .expect("Unable to parse line");
            return (moves, from, to);
        })
        .collect();
}

pub fn day_5_part_1(data: &str) -> String {
    let (stacks, instructions) = parse_input_data(data);
    let mut stacks = stacks;

    for (moves, from, to) in instructions {
        for _ in 0..moves {
            //println!("Move {} from {} to {}", moves, from, to);
            //println!("Stacks: {:?}", stacks);
            let character = stacks
                .get_mut(from - 1)
                .expect("Unable to get from stack")
                .pop_back()
                .expect("Unable to get character from stack");

            stacks
                .get_mut(to - 1)
                .expect("Unable to get to stack")
                .push_back(character);
        }
    }

    //println!("Final Stacks: {:?}", stacks);

    // Concattane the first string of each stack
    let string = stacks
        .iter()
        .map(|stack| {
            stack
                .back()
                .expect("Unable to get first character")
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("");

    return string;
}

pub fn day_5_part_2(data: &str) -> String {
    let ranges = parse_input_data(data);

    return "98".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "    [D]    \n\
[N] [C]    \n\
[Z] [M] [P]
 1   2   3 \n\n\
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_day_5_part_1() {
        assert_eq!(day_5_part_1(EXAMPLE), "CMZ");
    }

    #[test]
    fn test_day_5_part_2() {
        assert_eq!(day_5_part_2(EXAMPLE), "CMZ");
    }
}
