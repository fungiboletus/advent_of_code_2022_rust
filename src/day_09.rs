use std::collections::HashSet;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Instruction {
    direction: Direction,
    steps: i64,
}

pub fn parse_input_data(input: &str) -> Vec<Instruction> {
    return input
        .lines()
        .map(|instruction| {
            // Each line contains a letter and a number separated by a space
            let mut chars = instruction.chars();
            let direction = match chars.next().expect("Failed to parse direction") {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Failed to parse direction"),
            };
            // Skip the space
            chars.next();
            // Parse the number
            let steps = chars
                .collect::<String>()
                .parse::<i64>()
                .expect("Failed to parse steps");
            return Instruction { direction, steps };
        })
        .collect();
}

pub fn head_next_to_tail(head_position: (i64, i64), tail_position: (i64, i64)) -> bool {
    let (head_x, head_y) = head_position;
    let (tail_x, tail_y) = tail_position;

    // if the head is on the same position as the tail,
    // or if it's on an adjacent cell (north, south, east, west, and diagonals)
    return head_x == tail_x && head_y == tail_y
        || head_x == tail_x && head_y == tail_y + 1
        || head_x == tail_x && head_y == tail_y - 1
        || head_x == tail_x + 1 && head_y == tail_y
        || head_x == tail_x - 1 && head_y == tail_y
        || head_x == tail_x + 1 && head_y == tail_y + 1
        || head_x == tail_x + 1 && head_y == tail_y - 1
        || head_x == tail_x - 1 && head_y == tail_y + 1
        || head_x == tail_x - 1 && head_y == tail_y - 1;
}

pub fn move_tail_towards_head_if_needed(
    head_position: (i64, i64),
    tail_position: (i64, i64),
) -> (i64, i64) {
    let (head_x, head_y) = head_position;
    let (tail_x, tail_y) = tail_position;

    if head_next_to_tail(head_position, tail_position) {
        return (tail_x, tail_y);
    }

    // If the head and the tail are on the same column
    if head_x == tail_x {
        // If the head and the tail are also on the same row
        if head_y == tail_y {
            unreachable!("The head and the tail should have been detected as next to each other");
        }

        // If the head is above the tail, move down
        if head_y > tail_y {
            return (tail_x, tail_y + 1);
        }

        // Otherwise, if the head is below the tail, move up
        return (tail_x, tail_y - 1);
    }

    // Move the tail towards the head
    // If the head is on the same row as the tail, move left or right
    if head_y == tail_y {
        // If the head is on the right, move right
        if head_x > tail_x {
            return (tail_x + 1, tail_y);
        }
        // Otherwise, if the head is on the left, move left
        return (tail_x - 1, tail_y);
    }

    // If the head is on a different row, move diagonally
    if head_x > tail_x {
        if head_y > tail_y {
            return (tail_x + 1, tail_y + 1);
        }
        return (tail_x + 1, tail_y - 1);
    }
    if head_y > tail_y {
        return (tail_x - 1, tail_y + 1);
    }
    return (tail_x - 1, tail_y - 1);
}

pub fn day_9_part_1(data: &str) -> i64 {
    let data = parse_input_data(data);

    let mut head_position: (i64, i64) = (0, 0);
    let mut tail_position: (i64, i64) = (0, 0);

    let mut visited_positions: HashSet<(i64, i64)> = HashSet::new();

    for instruction in data {
        for _ in 0..instruction.steps {
            match instruction.direction {
                Direction::Up => head_position.1 += 1,
                Direction::Down => head_position.1 -= 1,
                Direction::Left => head_position.0 -= 1,
                Direction::Right => head_position.0 += 1,
            }
            tail_position = move_tail_towards_head_if_needed(head_position, tail_position);
            visited_positions.insert(tail_position);
        }
    }

    return visited_positions.len() as i64;
}

pub fn day_9_part_2(data: &str) -> i64 {
    let data = parse_input_data(data);

    let mut visited_positions: HashSet<(i64, i64)> = HashSet::new();

    let mut head_position: (i64, i64) = (0, 0);
    let mut knot_1_position: (i64, i64) = (0, 0);
    let mut knot_2_position: (i64, i64) = (0, 0);
    let mut knot_3_position: (i64, i64) = (0, 0);
    let mut knot_4_position: (i64, i64) = (0, 0);
    let mut knot_5_position: (i64, i64) = (0, 0);
    let mut knot_6_position: (i64, i64) = (0, 0);
    let mut knot_7_position: (i64, i64) = (0, 0);
    let mut knot_8_position: (i64, i64) = (0, 0);
    let mut tail_position: (i64, i64) = (0, 0);

    for instruction in data {
        for _ in 0..instruction.steps {
            match instruction.direction {
                Direction::Up => head_position.1 += 1,
                Direction::Down => head_position.1 -= 1,
                Direction::Left => head_position.0 -= 1,
                Direction::Right => head_position.0 += 1,
            }
            knot_1_position = move_tail_towards_head_if_needed(head_position, knot_1_position);
            knot_2_position = move_tail_towards_head_if_needed(knot_1_position, knot_2_position);
            knot_3_position = move_tail_towards_head_if_needed(knot_2_position, knot_3_position);
            knot_4_position = move_tail_towards_head_if_needed(knot_3_position, knot_4_position);
            knot_5_position = move_tail_towards_head_if_needed(knot_4_position, knot_5_position);
            knot_6_position = move_tail_towards_head_if_needed(knot_5_position, knot_6_position);
            knot_7_position = move_tail_towards_head_if_needed(knot_6_position, knot_7_position);
            knot_8_position = move_tail_towards_head_if_needed(knot_7_position, knot_8_position);
            tail_position = move_tail_towards_head_if_needed(knot_8_position, tail_position);
            visited_positions.insert(tail_position);
        }
    }

    return visited_positions.len() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const LARGER_EXAMPLE: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_head_next_to_tail() {
        assert_eq!(head_next_to_tail((2, 1), (1, 1)), true);
        assert_eq!(head_next_to_tail((3, 1), (1, 1)), false);
        assert_eq!(head_next_to_tail((3, 1), (2, 1)), true);

        assert_eq!(head_next_to_tail((1, 2), (1, 1)), true);
        assert_eq!(head_next_to_tail((1, 3), (1, 1)), false);
        assert_eq!(head_next_to_tail((1, 3), (1, 2)), true);

        assert_eq!(head_next_to_tail((2, 2), (1, 3)), true);
        assert_eq!(head_next_to_tail((2, 1), (1, 3)), false);
        assert_eq!(head_next_to_tail((2, 1), (2, 2)), true);

        assert_eq!(head_next_to_tail((3, 2), (1, 3)), false);
        assert_eq!(head_next_to_tail((3, 2), (2, 2)), true);
    }

    #[test]
    fn test_move_tail_towards_head_if_needed() {
        assert_eq!(move_tail_towards_head_if_needed((2, 1), (1, 1)), (1, 1));
        assert_eq!(move_tail_towards_head_if_needed((3, 1), (1, 1)), (2, 1));
        assert_eq!(move_tail_towards_head_if_needed((3, 1), (2, 1)), (2, 1));

        assert_eq!(move_tail_towards_head_if_needed((1, 2), (1, 1)), (1, 1));
        assert_eq!(move_tail_towards_head_if_needed((1, 3), (1, 1)), (1, 2));
        assert_eq!(move_tail_towards_head_if_needed((1, 3), (1, 2)), (1, 2));

        assert_eq!(move_tail_towards_head_if_needed((2, 2), (1, 3)), (1, 3));
        assert_eq!(move_tail_towards_head_if_needed((2, 1), (1, 3)), (2, 2));
        assert_eq!(move_tail_towards_head_if_needed((2, 1), (2, 2)), (2, 2));

        assert_eq!(move_tail_towards_head_if_needed((3, 2), (1, 3)), (2, 2));
        assert_eq!(move_tail_towards_head_if_needed((3, 2), (2, 2)), (2, 2));
    }

    #[test]
    fn test_day_9_part_1() {
        assert_eq!(day_9_part_1(EXAMPLE), 13);
    }

    #[test]
    fn test_day_9_part_2() {
        assert_eq!(day_9_part_2(EXAMPLE), 1);
        assert_eq!(day_9_part_2(LARGER_EXAMPLE), 36);
    }
}
