fn main() {
    let input = include_str!("../../inputs/day4.txt");
    println!("Part 1: {}", &part1(input));
    println!("Part 2: {}", &part2(input));

    divan::main();
}

fn find_words(chars: &[Vec<char>], word: &str) -> u32 {
    let mut count = 0;
    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            let mut north = true;
            let mut north_east = true;
            let mut east = true;
            let mut south_east = true;
            let mut south = true;
            let mut south_west = true;
            let mut west = true;
            let mut north_west = true;
            if chars[i][j] == word.chars().next().unwrap() {
                for (k, chr) in word[1..].chars().enumerate() {
                    let k = k + 1;
                    if north {
                        if i >= k {
                            if chars[i - k][j] != chr {
                                north = false;
                            }
                        } else {
                            north = false;
                        }
                    }
                    if north_east {
                        if i >= k && j + k < chars[i].len() {
                            if chars[i - k][j + k] != chr {
                                north_east = false;
                            }
                        } else {
                            north_east = false;
                        }
                    }
                    if east {
                        if j + k < chars[i].len() {
                            if chars[i][j + k] != chr {
                                east = false;
                            }
                        } else {
                            east = false;
                        }
                    }
                    if south_east {
                        if i + k < chars.len() && j + k < chars[i].len() {
                            if chars[i + k][j + k] != chr {
                                south_east = false;
                            }
                        } else {
                            south_east = false;
                        }
                    }
                    if south {
                        if i + k < chars.len() {
                            if chars[i + k][j] != chr {
                                south = false;
                            }
                        } else {
                            south = false;
                        }
                    }
                    if south_west {
                        if i + k < chars.len() && j >= k {
                            if chars[i + k][j - k] != chr {
                                south_west = false;
                            }
                        } else {
                            south_west = false;
                        }
                    }
                    if west {
                        if j >= k {
                            if chars[i][j - k] != chr {
                                west = false;
                            }
                        } else {
                            west = false;
                        }
                    }
                    if north_west {
                        if i >= k && j >= k {
                            if chars[i - k][j - k] != chr {
                                north_west = false;
                            }
                        } else {
                            north_west = false;
                        }
                    }
                }
                count += [north, north_east, east, south_east, south, south_west, west, north_west].iter().filter(|&x| *x).count() as u32;
            }
        }
    }
    
    count
}

fn find_x(chars: &[Vec<char>], word: &str) -> u32 {
    assert_eq!(word.len(), 3);
    let mut word_chr = word.chars();

    let first = word_chr.next().unwrap();
    let mid = word_chr.next().unwrap();
    let last = word_chr.next().unwrap();

    let mut count = 0;
    for i in 1..(chars.len() - 1) {
        for j in 1..(chars[i].len() - 1) {
            if chars[i][j] == mid {
                let top_left = chars[i - 1][j - 1];
                let top_right = chars[i - 1][j + 1];
                let bottom_left = chars[i + 1][j - 1];
                let bottom_right = chars[i + 1][j + 1];

                let left_to_right = (top_left == first && bottom_right == last) || (bottom_right == first && top_left == last);
                let right_to_left = (top_right == first && bottom_left == last) || (bottom_left == first && top_right == last);

                if left_to_right && right_to_left {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn part1(input: &str) -> u32 {
    find_words(&input.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>(), "XMAS")
}

pub fn part2(input: &str) -> u32 {
    find_x(&input.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>(), "MAS")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#.to_string();
        let result = part1(&test);
        assert_eq!(result, 18)
    }

    #[test]
    fn test_part2() {
        let test = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#.to_string();
        let result = part2(&test);
        assert_eq!(result, 9)
    }

    #[divan::bench]
    fn bench_part1() {
       part1(divan::black_box(include_str!(
            "../../inputs/day4.txt",
        )));
    }

    #[divan::bench]
    fn bench_part2() {
        part2(divan::black_box(include_str!(
            "../../inputs/day4.txt",
        )));
    }
}