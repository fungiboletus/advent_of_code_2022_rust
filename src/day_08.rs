use ndarray::Array2;

pub fn parse_input_data(input: &str) -> Array2<u8> {
    let numbers = input
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();

    // compute sqrt of the number of elements
    let sqrt = (numbers.len() as f64).sqrt() as usize;

    return Array2::from_shape_vec((sqrt, sqrt), numbers).expect("Failed to parse input data");
}

pub fn day_8_part_1(data: &str) -> i64 {
    let trees_heights = parse_input_data(data);
    let dim = trees_heights.dim();
    let mut visible_trees: Array2<u8> = Array2::zeros(dim);

    // for every tree, check if it is visible
    for ((i, j), height) in trees_heights.indexed_iter() {
        // if it's a tree on the perimeter of the forest, it's visible
        if i == 0 || i == dim.0 - 1 || j == 0 || j == dim.1 - 1 {
            visible_trees[[i, j]] = 1;
            continue;
        }

        // look at the trees north of it
        visible_trees[[i, j]] = 1;

        for k in 0..i {
            if trees_heights[[k, j]] >= *height {
                visible_trees[[i, j]] = 0;
                break;
            }
        }

        if visible_trees[[i, j]] == 1 {
            continue;
        }
        visible_trees[[i, j]] = 1;
        // look at the trees south of it
        for k in i + 1..dim.0 {
            if trees_heights[[k, j]] >= *height {
                visible_trees[[i, j]] = 0;
                break;
            }
        }

        if visible_trees[[i, j]] == 1 {
            continue;
        }
        visible_trees[[i, j]] = 1;
        // look at the trees west of it
        for k in 0..j {
            if trees_heights[[i, k]] >= *height {
                visible_trees[[i, j]] = 0;
                break;
            }
        }

        if visible_trees[[i, j]] == 1 {
            continue;
        }
        visible_trees[[i, j]] = 1;
        // look at the trees east of it
        for k in j + 1..dim.1 {
            if trees_heights[[i, k]] >= *height {
                visible_trees[[i, j]] = 0;
                break;
            }
        }
    }

    // println!("{:?}", visible_trees);

    let mut sum: i64 = 0;
    for visible in visible_trees.iter() {
        sum += *visible as i64;
    }
    return sum;
}

pub fn day_8_part_2(data: &str) -> i64 {
    let data = parse_input_data(data);

    return data.len() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_day_8_part_1() {
        assert_eq!(day_8_part_1(EXAMPLE), 21);
    }

    #[test]
    fn test_day_8_part_2() {
        assert_eq!(day_8_part_2(EXAMPLE), 45000);
    }
}
