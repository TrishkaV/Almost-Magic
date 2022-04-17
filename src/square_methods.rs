use rand::Rng;
use std::cmp;
use std::collections::HashMap;

static ACCEPTED_SQUARES: [u32; 3] = [1, 2, 3];

pub enum SquaresErrors {
    SquareNotInMagixBox,
    SquareNotValid,
}

// squares 2, 3, 4.
// the constraints are with the previous square, except square 4 wich has the last two constraints with the first
fn get_square_contraints() -> [HashMap<u32, u32>; 3] {
    return [
        HashMap::from([(0, 5), (3, 8)]),
        HashMap::from([(1, 6), (2, 7)]),
        HashMap::from([(5, 0), (8, 3), (1, 6), (2, 7)]),
    ];
}

pub fn populate_first_square() -> [u32; 9] {
    let mut return_square: [u32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut rng = rand::thread_rng();

    while !is_square_magic(return_square) || contains_duplicates(return_square) {
        let mut h1: u32 = 0;
        for i in 0..return_square.len() {
            if i == 4 {
                h1 = return_square[0] + return_square[1] + return_square[2];
            }
            match i {
                4 => {
                    if return_square[3] + 1 >= h1 {
                        break;
                    }
                    let h1_range = h1 - return_square[3] - 1;
                    let range_to_test = cmp::min(255, h1_range);
                    let mut is_unsolved = false;
                    let mut tries = 0;
                    loop {
                        tries += 1;
                        return_square[i] = rng.gen_range(1..=range_to_test);
                        if tries > range_to_test {
                            is_unsolved = true;
                            break;
                        } else if !check_duplicate(return_square, i) {
                            break;
                        }
                    }
                    if is_unsolved {
                        break;
                    }
                }
                5 => {
                    if return_square[3] + return_square[4] >= h1 {
                        break;
                    }
                    return_square[i] = h1 - (return_square[3] + return_square[4]);
                }
                6 => {
                    if return_square[0] + return_square[3] >= h1 {
                        break;
                    }
                    return_square[i] = h1 - (return_square[0] + return_square[3]);
                }
                7 => {
                    if return_square[1] + return_square[4] >= h1 {
                        break;
                    }
                    return_square[i] = h1 - (return_square[1] + return_square[4]);
                }
                8 => {
                    if return_square[6] + return_square[7] >= h1 {
                        break;
                    }
                    return_square[i] = h1 - (return_square[6] + return_square[7]);
                }
                _ => loop {
                    return_square[i] = rng.gen_range(1..255);
                    if !check_duplicate(return_square, i) {
                        break;
                    }
                },
            }
        }
    }
    #[cfg(debug_assertions)]
    println!("Square n.{} --> {:?}", 1, return_square);
    return_square
}

pub fn populate_second_square(square: [u32; 9], square_n: u32) -> Result<[u32; 9], SquaresErrors> {
    if !ACCEPTED_SQUARES.contains(&square_n) {
        return Err(SquaresErrors::SquareNotInMagixBox);
    }
    let square_n_index = square_n as usize;
    let square_contraints = get_square_contraints();

    let mut n_tryes = 0;
    let mut return_square: [u32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut rng = rand::thread_rng();
    while !is_square_magic(return_square) || contains_duplicates(return_square) {
        n_tryes += 1;
        let mut h1: u32 = 0;
        for i in 0..return_square.len() {
            if square_contraints[square_n_index - 1].contains_key(&(i as u32)) {
                let value_for_key = square_contraints[square_n_index - 1][&(i as u32)];
                return_square[i] = square[value_for_key as usize];
                continue;
            }

            if i == 4 {
                h1 = return_square[0] + return_square[1] + return_square[2];
            }
            match i {
                4 => {
                    if return_square[3] + 1 >= h1 {
                        break;
                    }
                    let h1_range = h1 - return_square[3] - 1;
                    let range_to_test = cmp::min(255, h1_range);
                    let mut is_unsolved = false;
                    let mut tries = 0;
                    loop {
                        tries += 1;
                        return_square[i] = rng.gen_range(1..=range_to_test);
                        if tries > range_to_test {
                            is_unsolved = true;
                            break;
                        } else if !check_duplicate(return_square, i) {
                            break;
                        }
                    }
                    if is_unsolved {
                        break;
                    }
                }
                5 => {
                    if return_square[3] + return_square[4] >= h1 {
                        break;
                    }
                    return_square[i] = h1 - (return_square[3] + return_square[4]);
                }
                6 => {
                    if return_square[0] + return_square[3] >= h1 {
                        break;
                    }
                    return_square[i] = h1 - (return_square[0] + return_square[3]);
                }
                7 => {
                    if return_square[1] + return_square[4] >= h1 {
                        break;
                    }
                    return_square[i] = h1 - (return_square[1] + return_square[4]);
                }
                8 => {
                    if return_square[6] + return_square[7] >= h1 {
                        break;
                    }
                    return_square[i] = h1 - (return_square[6] + return_square[7]);
                }
                _ => loop {
                    return_square[i] = rng.gen_range(1..255);
                    if !check_duplicate(return_square, i) {
                        break;
                    }
                },
            }
        }
        if n_tryes > 3500000 {
            return Err(SquaresErrors::SquareNotValid);
        }
    }
    #[cfg(debug_assertions)]
    println!("Square n.{} --> {:?}", square_n + 1, return_square);
    Ok(return_square)
}

pub fn populate_third_square(
    square: [u32; 9],
    square_n: u32,
    first_sq: [u32; 9],
) -> Result<[u32; 9], SquaresErrors> {
    if !ACCEPTED_SQUARES.contains(&square_n) {
        return Err(SquaresErrors::SquareNotInMagixBox);
    }
    let square_n_index = square_n as usize;
    let square_contraints = get_square_contraints();

    let mut n_tryes = 0;
    let mut return_square: [u32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut rng = rand::thread_rng();
    while !is_square_magic(return_square)
        || contains_duplicates(return_square)
        || odd_fourth_cases(return_square, first_sq)
    {
        n_tryes += 1;
        let mut h1: u32 = 0;
        for i in 0..return_square.len() {
            if square_contraints[square_n_index - 1].contains_key(&(i as u32)) {
                let value_for_key = square_contraints[square_n_index - 1][&(i as u32)];
                return_square[i] = square[value_for_key as usize];
                continue;
            }

            if i == 4 {
                h1 = return_square[0] + return_square[1] + return_square[2];
            }
            match i {
                0 => loop {
                    return_square[i] = rng.gen_range(1..255);
                    if return_square[i] != first_sq[6] && return_square[i] != first_sq[7] {
                        break;
                    }
                },
                3 => loop {
                    return_square[i] = rng.gen_range(1..255);
                    if !check_duplicate(return_square, i)
                        && return_square[i] != first_sq[6]
                        && return_square[i] != first_sq[7]
                    {
                        break;
                    }
                },
                4 => {
                    if return_square[3] + 1 >= h1 {
                        break;
                    }
                    let h1_range = h1 - return_square[3] - 1;
                    let range_to_test = cmp::min(255, h1_range);
                    let mut is_unsolved = false;
                    let mut tries = 0;
                    loop {
                        tries += 1;
                        return_square[i] = rng.gen_range(1..=range_to_test);
                        if tries > range_to_test {
                            is_unsolved = true;
                            break;
                        } else if !check_duplicate(return_square, i) {
                            break;
                        }
                    }
                    if is_unsolved {
                        break;
                    }
                }
                5 => {
                    if return_square[3] + return_square[4] >= h1 {
                        break;
                    }
                    return_square[i] = h1 - (return_square[3] + return_square[4]);
                }
                6 => {
                    if return_square[0] + return_square[3] >= h1 {
                        break;
                    }
                    return_square[i] = h1 - (return_square[0] + return_square[3]);
                }
                7 => {
                    if return_square[1] + return_square[4] >= h1 {
                        break;
                    }
                    return_square[i] = h1 - (return_square[1] + return_square[4]);
                }
                8 => {
                    if return_square[6] + return_square[7] >= h1 {
                        break;
                    }
                    return_square[i] = h1 - (return_square[6] + return_square[7]);
                }
                _ => loop {
                    return_square[i] = rng.gen_range(1..255);
                    if !check_duplicate(return_square, i) {
                        break;
                    }
                },
            }
        }

        if n_tryes > 3500000 {
            return Err(SquaresErrors::SquareNotValid);
        }
    }
    #[cfg(debug_assertions)]
    println!("Square n.{} --> {:?}", square_n + 1, return_square);
    Ok(return_square)
}

pub fn populate_fourth_square(
    square: [u32; 9],
    first_sq: [u32; 9],
) -> Result<[u32; 9], SquaresErrors> {
    let square_n_index = 3;
    let square_contraints = get_square_contraints();

    let mut n_tryes = 0;
    let mut return_square: [u32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut rng = rand::thread_rng();
    let h1: u32;
    for i in 0..return_square.len() {
        if square_contraints[square_n_index - 1].contains_key(&(i as u32)) {
            let value_for_key = square_contraints[square_n_index - 1][&(i as u32)];
            return_square[i] = if i <= 2 {
                first_sq[value_for_key as usize]
            } else {
                square[value_for_key as usize]
            };
        }
    }
    h1 = return_square[2] + return_square[5] + return_square[8];
    while !is_square_magic(return_square) {
        n_tryes += 1;
        for i in 0..return_square.len() {
            match i {
                1 | 2 | 5 | 8 => continue,
                0 => {
                    return_square[i] = h1 - return_square[1] - return_square[2];
                }
                3 => {
                    let range_to_test = h1 - return_square[5] - 1;
                    let mut is_unsolved = false;
                    let mut tries = 0;
                    loop {
                        tries += 1;
                        return_square[i] = rng.gen_range(1..=range_to_test);
                        if tries > range_to_test {
                            is_unsolved = true;
                            break;
                        } else if !check_duplicate(return_square, i) {
                            break;
                        }
                    }
                    if is_unsolved {
                        break;
                    }
                }
                4 => {
                    return_square[i] = h1 - return_square[5] - return_square[3];
                }
                6 => {
                    if return_square[0] + return_square[3] >= h1 {
                        break;
                    }
                    return_square[i] = h1 - (return_square[0] + return_square[3]);
                }
                7 => {
                    if return_square[1] + return_square[4] >= h1 {
                        break;
                    }
                    return_square[i] = h1 - (return_square[1] + return_square[4]);
                }
                _ => {
                    panic!("TODO")
                }
            }
        }
        if n_tryes > 3500000 {
            return Err(SquaresErrors::SquareNotValid);
        }
    }
    #[cfg(debug_assertions)]
    println!("Square n.4 --> {:?}", return_square);
    Ok(return_square)
}

fn is_square_magic(square: [u32; 9]) -> bool {
    let h1: u32 = square[0] + square[1] + square[2];
    let h2: u32 = square[3] + square[4] + square[5];
    let h3: u32 = square[6] + square[7] + square[8];
    let v1: u32 = square[0] + square[3] + square[6];
    let v2: u32 = square[1] + square[4] + square[7];
    let v3: u32 = square[2] + square[5] + square[8];
    let d1: u32 = square[0] + square[4] + square[8];
    let d2: u32 = square[2] + square[4] + square[6];

    if (h2 > h1 + 1 || h2 < h1 - 1)
        || (h3 > h1 + 1 || h3 < h1 - 1)
        || (v1 > h1 + 1 || v1 < h1 - 1)
        || (v2 > h1 + 1 || v2 < h1 - 1)
        || (v3 > h1 + 1 || v3 < h1 - 1)
        || (d1 > h1 + 1 || d1 < h1 - 1)
        || (d2 > h1 + 1 || d2 < h1 - 1)
        // this serves to speed up operations
        || h1 > 200
    {
        return false;
    }
    return true;
}

// odd cases where in fourth_square's entries will be forced into not allowed values
fn odd_fourth_cases(return_square: [u32; 9], first_sq: [u32; 9]) -> bool {
    let fourth_h1 = return_square[0] + return_square[3] + first_sq[7];
    if first_sq[6] + first_sq[7] >= fourth_h1
        || fourth_h1 - first_sq[6] - first_sq[7] == first_sq[6]
        || fourth_h1 - first_sq[6] - first_sq[7] == first_sq[7]
        || fourth_h1 - first_sq[6] - first_sq[7] == return_square[0]
        || fourth_h1 - first_sq[6] - first_sq[7] == return_square[3]
    {
        return true;
    }
    return false;
}

fn contains_duplicates(square: [u32; 9]) -> bool {
    let is_duplicate = (1..square.len()).any(|i| square[i..].contains(&square[i - 1]));
    return is_duplicate;
}

fn check_duplicate(square: [u32; 9], pos: usize) -> bool {
    for i in 0..pos {
        if square[i] == square[pos] {
            return true;
        }
    }
    return false;
}
