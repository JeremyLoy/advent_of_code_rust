use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::convert::identity;

/// Counts the number of increasing pairs in windowed sums of given data.
///
/// # Arguments
///
/// * `data` - A vector of integers.
/// * `window_size` - The size of the window used to calculate sums.
///
/// # Examples
///
/// ```
/// # use advent_of_code_rust::_2021::count_of_increasing_pairs_in_windowed_sums;
///
/// let data = vec![1, 2, 3, 4, 5];
/// let window_size = 3;
/// let count = count_of_increasing_pairs_in_windowed_sums(&data, window_size);
/// // 1 + 2 + 3 = 6
/// // 2 + 3 + 4 = 9
/// // 3 + 4 + 5 = 12
///
/// // 12 > 9
/// // 9 > 6
///
/// assert_eq!(count, 2);
/// ```
pub fn count_of_increasing_pairs_in_windowed_sums(data: &[i32], window_size: usize) -> i32 {
    let windowed_sums: Vec<i32> = data
        .windows(window_size)
        .map(|window| window.iter().sum::<i32>())
        .collect();

    let count_increasing: i32 = windowed_sums
        .windows(2)
        .filter(|window_pair| window_pair[0] < window_pair[1])
        .count() as i32;

    count_increasing
}

#[derive(Debug)]
pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    pub fn parse(line: &str) -> Option<Self> {
        let mut line = line.split_whitespace();
        let direction = line.next()?;
        let amount = line.next()?;
        let amount = amount.parse::<i32>().ok()?;
        match direction {
            "forward" => Some(Self::Forward(amount)),
            "down" => Some(Self::Down(amount)),
            "up" => Some(Self::Up(amount)),
            _ => None,
        }
    }

    pub fn parse_batch(lines: impl Iterator<Item = String>) -> Vec<Self> {
        lines
            .into_iter()
            .filter_map(|line| Self::parse(&line))
            .collect()
    }
}

/// Calculates the submarine's distance from origin based on a series of commands.
///
/// # Arguments
///
/// * `commands` - A vector of `Command` objects representing the actions to be performed.
///
/// # Returns
///
/// The horizontal position and depth multiplied together
///
/// # Example
///
/// ```
/// # use advent_of_code_rust::_2021::{Command, calculate_distance};
///
/// let commands = vec![
///     Command::Forward(10),
///     Command::Down(5),
///     Command::Up(3),
/// ];
///
/// let distance = calculate_distance(commands);
/// assert_eq!(distance, 20);
/// ```
pub fn calculate_distance(commands: Vec<Command>) -> i32 {
    let mut horizontal_position = 0;
    let mut vertical_depth = 0;
    for command in commands {
        match command {
            Command::Forward(amount) => horizontal_position += amount,
            Command::Down(amount) => vertical_depth += amount,
            Command::Up(amount) => vertical_depth -= amount,
        }
    }
    horizontal_position * vertical_depth
}

/// Calculates the aim and distance of the submarine based on the given commands.
///
/// # Arguments
///
/// * `commands` - A vector of `Command` representing the commands to be executed.
///
/// # Returns
///
/// The horizontal position and depth multiplied together
///
/// # Examples
///
/// ```
/// # use advent_of_code_rust::_2021::{Command, calculate_aim_and_distance};
///
/// let commands = vec![
///     Command::Down(5),
///     Command::Up(2),
///     Command::Forward(10),
/// ];
/// let result = calculate_aim_and_distance(commands);
/// assert_eq!(result, 300);
/// ```
pub fn calculate_aim_and_distance(commands: Vec<Command>) -> i32 {
    let mut horizontal_position = 0;
    let mut vertical_depth = 0;
    let mut aim = 0;
    for command in commands {
        match command {
            Command::Forward(amount) => {
                horizontal_position += amount;
                vertical_depth += aim * amount;
            }
            Command::Down(amount) => aim += amount,
            Command::Up(amount) => aim -= amount,
        }
    }
    horizontal_position * vertical_depth
}

pub fn find_all_most_common_bits(binary_report: &Vec<String>) -> String {
    let mut freq_of_ones = HashMap::new();

    for s in binary_report {
        for (i, c) in s.char_indices() {
            match c {
                '1' => {
                    let count = freq_of_ones.entry(i).or_insert(0);
                    *count += 1;
                }
                _ => (),
            }
        }
    }
    let mut ret = String::new();

    for i in 0..freq_of_ones.len() {
        match freq_of_ones.get(&i) {
            Some(i) => {
                if *i > (binary_report.len() / 2) {
                    ret.push('1')
                } else {
                    ret.push('0')
                }
            }
            _ => panic!("index {} wasn't found in freq map", i),
        }
    }

    ret
}

#[derive(Debug)]
pub enum BitCriteria {
    Oxygen,
    CO2,
}

pub fn find_component_rating(mut binary_report: Vec<String>, bit_criteria: BitCriteria) -> String {
    let mut freq0 = 0;
    let mut freq1 = 0;
    let mut position = 0;

    while binary_report.len() != 1 {
        for s in &binary_report {
            match s.chars().nth(position) {
                Some('0') => freq0 += 1,
                Some('1') => freq1 += 1,
                Some(e) => panic!("unhandled char {}", e),
                None => panic!("no char at pos {}", position),
            }
        }
        let bit_to_keep = match bit_criteria {
            BitCriteria::Oxygen => {
                if freq1 >= freq0 {
                    '1'
                } else {
                    '0'
                }
            }
            BitCriteria::CO2 => {
                if freq0 > freq1 {
                    '1'
                } else {
                    '0'
                }
            }
        };
        binary_report.retain(|s| s.chars().nth(position).eq(&Some(bit_to_keep)));
        position += 1;
        freq0 = 0;
        freq1 = 0;
    }

    binary_report.pop().unwrap()
}

pub fn flip_binary_str_bits(binary: &str) -> String {
    binary
        .chars()
        .map(|bit| match bit {
            '0' => '1',
            '1' => '0',
            _ => panic!("Invalid bit: {}", bit),
        })
        .collect()
}

pub fn binary_str_to_decimal(binary: &str) -> i32 {
    i32::from_str_radix(binary, 2).expect("Failed to convert binary string to decimal")
}

#[derive(Debug)]
pub struct BingoBoard([[BingoCell; 5]; 5]);

#[derive(Debug, Copy, Clone)]
pub enum BingoCell {
    Marked(i32),
    Unmarked(i32),
}
impl BingoBoard {
    // Extracting cell parsing logic to a separate function
    fn parse_cell(number_str: &str) -> Option<BingoCell> {
        let number = number_str.parse::<i32>().ok()?;
        Some(BingoCell::Unmarked(number))
    }
    pub fn parse(input: impl Iterator<Item = String>) -> Option<Self> {
        let mut board = [[BingoCell::Unmarked(0); 5]; 5];
        for (i, line) in input.enumerate() {
            for (j, number_str) in line.split_whitespace().enumerate() {
                board[i][j] = Self::parse_cell(number_str)?;
            }
        }
        Some(BingoBoard(board))
    }

    pub fn parse_batch(lines: impl Iterator<Item = String>) -> Vec<Self> {
        lines
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .chunks(5)
            .into_iter()
            .map(BingoBoard::parse)
            .filter_map(identity)
            .collect()
    }

    pub fn calculate_score(&self, last_call: i32) -> i32 {
        let mut score = 0;
        for row in self.0.iter() {
            for cell in row.iter() {
                if let BingoCell::Unmarked(value) = cell {
                    score += value;
                }
            }
        }
        score * last_call
    }

    pub fn mark(&mut self, number: i32) {
        for row in self.0.iter_mut() {
            for cell in row.iter_mut() {
                if let BingoCell::Unmarked(value) = cell {
                    if *value == number {
                        *cell = BingoCell::Marked(number);
                    }
                }
            }
        }
    }

    pub fn is_winner(&self) -> bool {
        for row in self.0.iter() {
            if row.iter().all(|&cell| matches!(cell, BingoCell::Marked(_))) {
                return true;
            }
        }
        for col in 0..5 {
            if self
                .0
                .iter()
                .all(|row| matches!(row[col], BingoCell::Marked(_)))
            {
                return true;
            }
        }
        false
    }
}

pub fn parse_calls_and_bingo_boards(
    mut lines: impl Iterator<Item = String>,
) -> (Vec<i32>, Vec<BingoBoard>) {
    let calls = lines.next().unwrap_or_default();
    let calls = calls
        .split(',')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    let boards = BingoBoard::parse_batch(lines);
    (calls, boards)
}

pub fn play_bingo(calls: Vec<i32>, mut boards: Vec<BingoBoard>) -> Vec<i32> {
    let mut winning_scores = Vec::new();
    let mut past_winners = HashSet::new();

    for call in calls {
        for (i, board) in boards.iter_mut().enumerate() {
            board.mark(call);
            if board.is_winner() {
                winning_scores.push(board.calculate_score(call));
                past_winners.insert(i);
            }
        }
        let mut i: usize = 0;
        boards.retain(|_| {
            let keep = !past_winners.contains(&i);
            i += 1;
            keep
        });
        past_winners.clear()
    }

    winning_scores
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn parse_line_to_point(point_str: &str) -> Option<Self> {
        let (x_str, y_str) = point_str.split_once(",")?;
        let x = x_str.trim().parse::<i32>().ok()?;
        let y = y_str.trim().parse::<i32>().ok()?;
        Some(Point { x, y })
    }

    pub fn parse_line_to_pair(line: &str) -> Option<(Self, Self)> {
        let (start_str, end_str) = line.split_once("->")?;
        let start_point = Self::parse_line_to_point(start_str)?;
        let end_point = Self::parse_line_to_point(end_str)?;
        Some((start_point, end_point))
    }

    pub fn parse_batch(lines: impl Iterator<Item = String>) -> impl Iterator<Item = (Self, Self)> {
        lines
            .into_iter()
            .filter_map(|line| Self::parse_line_to_pair(&line))
    }
}

pub enum Diagonals {
    Include,
    Exclude,
}

pub fn plot_points(
    points: impl Iterator<Item = (Point, Point)>,
    plot_diagonals: Diagonals,
) -> HashMap<Point, i32> {
    let mut grid = HashMap::new();
    for (mut start, end) in points {
        if matches!(plot_diagonals, Diagonals::Exclude) && start.x != end.x && start.y != end.y {
            continue;
        }
        while start.x != end.x || start.y != end.y {
            let count = grid.entry(start).or_insert(0);
            *count += 1;

            if start.x < end.x {
                start.x += 1;
            }
            if start.x > end.x {
                start.x -= 1;
            }

            if start.y < end.y {
                start.y += 1;
            }
            if start.y > end.y {
                start.y -= 1;
            }
        }
        let count = grid.entry(start).or_insert(0);
        *count += 1;
    }
    grid
}

pub fn count_overlapping_points(grid: HashMap<Point, i32>) -> i32 {
    grid.into_iter().fold(0, |mut count, (_point, value)| {
        if value > 1 {
            count += 1;
        }
        count
    })
}

pub fn parse_lantern_fish_histogram(input: Vec<usize>) -> Vec<u128> {
    input.iter().fold(vec![0; 9], |mut acc, &i| {
        acc[i] += 1;
        acc
    })
}

pub fn advance_lantern_fish_days(mut hist: Vec<u128>, days: i32) -> u128 {
    for _ in 0..days {
        hist.rotate_left(1);
        // Every 0 spawned exactly one fish. In other words, the number of new parents is equal to the
        // number of new children.
        // Parents should reset to 6 as opposed to new children being 8
        //
        // Therefore:
        // 6 = the old 7's + the new parents
        hist[6] += hist[8];
    }

    hist.iter().sum()
}

/// Calculates the Nth triangle number.
///
/// A triangle number is the sum of all positive integers up to and including N.
/// The formula used to calculate the Nth triangle number is (N * (N + 1)) / 2.
///
/// # Arguments
///
/// * `n` - The Nth number for which the triangle number needs to be calculated.
///
/// # Returns
///
/// The Nth triangle number.
///
/// # Example
///
/// ```rust
/// # use advent_of_code_rust::_2021::triangle_number;
/// assert_eq!(triangle_number(5), 5 + 4 + 3 + 2 + 1);
/// ```
pub fn triangle_number(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

pub fn find_cheapest_horizontal_position(crabs: Vec<i32>, fuel_calculator: fn(i32) -> i32) -> i32 {
    let max_crab_pos = *crabs.iter().max().unwrap();
    (0..max_crab_pos)
        .map(|horiz_pos| {
            crabs
                .iter()
                .map(|&crab_pos| fuel_calculator((horiz_pos - crab_pos).abs()))
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::_2021::test::Input::*;
    use crate::_2021::test::Separator::*;
    use std::fmt::Debug;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read};
    use std::str::FromStr;

    enum Input<'a> {
        Path(&'a str),
        Raw(&'a str),
    }

    enum Separator {
        Comma,
        Newline,
    }

    fn to_lines(input: Input) -> Box<dyn Iterator<Item = String> + '_> {
        match input {
            Path(path) => {
                let file = File::open(path).expect("Failed to open file");
                let reader = BufReader::new(file);
                Box::new(
                    reader
                        .lines()
                        .filter_map(Result::ok)
                        .map(|s| s.trim().to_owned())
                        .filter(|s| !s.is_empty()),
                )
            }
            Raw(s) => Box::new(
                s.lines()
                    .map(|s| s.trim().to_owned())
                    .filter(|s| !s.is_empty()),
            ),
        }
    }

    fn to_vec<T>(input: Input, delim: Separator) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let str = match input {
            Path(path) => {
                let mut file = File::open(path).unwrap();
                let mut str = String::new();
                file.read_to_string(&mut str).unwrap_or_default();
                str
            }
            Raw(s) => s.to_string(),
        };
        let string_parser = |s: &str| s.parse::<T>().ok();
        match delim {
            Newline => str
                .lines()
                .map(|s| s.trim())
                .filter_map(string_parser)
                .collect_vec(),
            Comma => str.split(",").filter_map(string_parser).collect_vec(),
        }
    }

    #[test]
    fn test_1_1_sample() {
        let input = "
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
        ";
        let numbers = to_vec(Raw(input), Newline);

        let count = count_of_increasing_pairs_in_windowed_sums(&numbers, 1);

        assert_eq!(count, 7);
    }

    #[test]
    fn test_1_1() {
        let numbers = to_vec(Path("input/2021/1.txt"), Newline);

        let count = count_of_increasing_pairs_in_windowed_sums(&numbers, 1);

        assert_eq!(count, 1583);
    }

    #[test]
    fn test_1_2_sample() {
        let input = "
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
        ";
        let numbers = to_vec(Raw(input), Newline);

        let count = count_of_increasing_pairs_in_windowed_sums(&numbers, 3);

        assert_eq!(count, 5);
    }

    #[test]
    fn test_1_2() {
        let numbers: Vec<i32> = to_vec(Path("input/2021/1.txt"), Newline);

        let count = count_of_increasing_pairs_in_windowed_sums(&numbers, 3);

        assert_eq!(count, 1627);
    }

    #[test]
    fn test_2_1_sample() {
        let input = to_lines(Raw("
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
        "));
        let commands = Command::parse_batch(input);

        let result = calculate_distance(commands);

        assert_eq!(result, 150);
    }

    #[test]
    fn test_2_1() {
        let input = to_lines(Path("input/2021/2.txt"));
        let commands = Command::parse_batch(input);

        let result = calculate_distance(commands);

        assert_eq!(result, 2_150_351);
    }

    #[test]
    fn test_2_2_sample() {
        let input = to_lines(Raw("
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
        "));
        let commands = Command::parse_batch(input);

        let result = calculate_aim_and_distance(commands);

        assert_eq!(result, 900);
    }

    #[test]
    fn test_2_2() {
        let input = to_lines(Path("input/2021/2.txt"));
        let commands = Command::parse_batch(input);

        let result = calculate_aim_and_distance(commands);

        assert_eq!(result, 1_842_742_223);
    }

    #[test]
    fn test_3_1_sample() {
        let input: Vec<String> = to_lines(Raw("
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
        "))
        .collect();

        let gamma_rate = find_all_most_common_bits(&input);
        let epsilon_rate = flip_binary_str_bits(&gamma_rate);

        let power_consumption =
            binary_str_to_decimal(&gamma_rate) * binary_str_to_decimal(&epsilon_rate);

        assert_eq!(power_consumption, 198)
    }

    #[test]
    fn test_3_1() {
        let input: Vec<String> = to_lines(Path("input/2021/3.txt")).collect();

        let gamma_rate = find_all_most_common_bits(&input);
        let epsilon_rate = flip_binary_str_bits(&gamma_rate);

        let power_consumption =
            binary_str_to_decimal(&gamma_rate) * binary_str_to_decimal(&epsilon_rate);

        assert_eq!(power_consumption, 3_633_500)
    }

    #[test]
    fn test_3_2_sample() {
        let input: Vec<String> = to_lines(Raw("
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
        "))
        .collect();

        let oxygen_generator_rating = find_component_rating(input.clone(), BitCriteria::Oxygen);
        let co2_scrubber_rating = find_component_rating(input, BitCriteria::CO2);
        let life_support_rating = binary_str_to_decimal(&oxygen_generator_rating)
            * binary_str_to_decimal(&co2_scrubber_rating);

        assert_eq!(life_support_rating, 230)
    }

    #[test]
    fn test_3_2() {
        let input: Vec<String> = to_lines(Path("input/2021/3.txt")).collect();

        let oxygen_generator_rating = find_component_rating(input.clone(), BitCriteria::Oxygen);
        let co2_scrubber_rating = find_component_rating(input, BitCriteria::CO2);
        let life_support_rating = binary_str_to_decimal(&oxygen_generator_rating)
            * binary_str_to_decimal(&co2_scrubber_rating);

        assert_eq!(life_support_rating, 4_550_283)
    }

    #[test]
    fn test_4_1_sample() {
        let input = to_lines(Raw("
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
        
         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
        "));

        let (calls, boards) = parse_calls_and_bingo_boards(input);

        let winning_scores = play_bingo(calls, boards);

        assert_eq!(*winning_scores.first().unwrap(), 4_512);
    }

    #[test]
    fn test_4_1() {
        let input = to_lines(Path("input/2021/4.txt"));

        let (calls, boards) = parse_calls_and_bingo_boards(input);

        let winning_scores = play_bingo(calls, boards);

        assert_eq!(*winning_scores.first().unwrap(), 8_136);
    }

    #[test]
    fn test_4_2_sample() {
        let input = to_lines(Raw("
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
        
         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
        "));

        let (calls, boards) = parse_calls_and_bingo_boards(input);

        let winning_scores = play_bingo(calls, boards);

        assert_eq!(*winning_scores.last().unwrap(), 1_924);
    }

    #[test]
    fn test_4_2() {
        let input = to_lines(Path("input/2021/4.txt"));

        let (calls, boards) = parse_calls_and_bingo_boards(input);

        let winning_scores = play_bingo(calls, boards);

        assert_eq!(*winning_scores.last().unwrap(), 12_738);
    }

    #[test]
    fn test_5_1_sample() {
        let input = to_lines(Raw("
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
        "));

        let grid = plot_points(Point::parse_batch(input), Diagonals::Exclude);

        assert_eq!(count_overlapping_points(grid), 5);
    }

    #[test]
    fn test_5_1() {
        let input = to_lines(Path("input/2021/5.txt"));

        let grid = plot_points(Point::parse_batch(input), Diagonals::Exclude);

        assert_eq!(count_overlapping_points(grid), 8_111);
    }

    #[test]
    fn test_5_2_sample() {
        let input = to_lines(Raw("
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
        "));

        let grid = plot_points(Point::parse_batch(input), Diagonals::Include);

        assert_eq!(count_overlapping_points(grid), 12);
    }

    #[test]
    fn test_5_2() {
        let input = to_lines(Path("input/2021/5.txt"));

        let grid = plot_points(Point::parse_batch(input), Diagonals::Include);

        assert_eq!(count_overlapping_points(grid), 22_088);
    }

    #[test]
    fn test_6_1_sample() {
        let input = to_vec(Raw("3,4,3,1,2"), Comma);

        let lantern_fish = parse_lantern_fish_histogram(input);

        let total = advance_lantern_fish_days(lantern_fish, 80);

        assert_eq!(total, 5_934);
    }

    #[test]
    fn test_6_1() {
        let input = to_vec(Path("input/2021/6.txt"), Comma);

        let lantern_fish = parse_lantern_fish_histogram(input);

        let total = advance_lantern_fish_days(lantern_fish, 80);

        assert_eq!(total, 363_101);
    }

    #[test]
    fn test_6_2_sample() {
        let input = to_vec(Raw("3,4,3,1,2"), Comma);

        let lantern_fish = parse_lantern_fish_histogram(input);

        let total = advance_lantern_fish_days(lantern_fish, 256);

        assert_eq!(total, 26_984_457_539);
    }

    #[test]
    fn test_6_2() {
        let input = to_vec(Path("input/2021/6.txt"), Comma);

        let lantern_fish = parse_lantern_fish_histogram(input);

        let total = advance_lantern_fish_days(lantern_fish, 256);

        assert_eq!(total, 1_644_286_074_024);
    }

    #[test]
    fn test_7_1_sample() {
        let crabs = to_vec(Raw("16,1,2,0,4,2,7,1,2,14"), Comma);

        assert_eq!(find_cheapest_horizontal_position(crabs, identity), 37);
    }

    #[test]
    fn test_7_1() {
        let crabs = to_vec(Path("input/2021/7.txt"), Comma);

        assert_eq!(find_cheapest_horizontal_position(crabs, identity), 348_996);
    }

    #[test]
    fn test_7_2_sample() {
        let crabs = to_vec(Raw("16,1,2,0,4,2,7,1,2,14"), Comma);

        assert_eq!(
            find_cheapest_horizontal_position(crabs, triangle_number),
            168
        );
    }

    #[test]
    fn test_7_2() {
        let crabs = to_vec(Path("input/2021/7.txt"), Comma);

        assert_eq!(
            find_cheapest_horizontal_position(crabs, triangle_number),
            98_231_647
        );
    }

    fn count_1478(input: Box<dyn Iterator<Item = String>>) -> i32 {
        input
            .map(|line| line.split_once("|").unwrap().1.trim().to_owned())
            .map(|output| {
                output.split_whitespace().fold(0, |mut count, digit| {
                    if [2, 3, 4, 7].contains(&(digit.len() as i32)) {
                        count += 1
                    }
                    count
                })
            })
            .sum()
    }

    const DIGIT_MASKS: [(char, u8); 7] = [
        ('a', 0b01000000),
        ('b', 0b00100000),
        ('c', 0b00010000),
        ('d', 0b00001000),
        ('e', 0b00000100),
        ('f', 0b00000010),
        ('g', 0b00000001),
    ];
    fn get_bit(c: char) -> u8 {
        for (ch, bit) in DIGIT_MASKS.iter() {
            if *ch == c {
                return *bit;
            }
        }
        0
    }
    fn signal_to_mask(s: &str) -> u8 {
        let mut mask = 0;
        for ch in s.chars() {
            mask |= get_bit(ch);
        }
        mask
    }

    fn overlaps(a: u8, b: u8) -> bool {
        a & b == b
    }

    fn determine_output(row: &str) -> i32 {
        let signals = row
            .split_whitespace()
            .map(|s| s.trim())
            .filter(|s| *s != "|")
            .map(signal_to_mask)
            .collect_vec();
        let (signals, output) = signals.split_at(10);

        let mut digit_to_mask = [0; 10];
        digit_to_mask[1] = *signals
            .iter()
            .find(|signal| signal.count_ones() == 2)
            .unwrap();
        digit_to_mask[4] = *signals
            .iter()
            .find(|signal| signal.count_ones() == 4)
            .unwrap();
        digit_to_mask[7] = *signals
            .iter()
            .find(|signal| signal.count_ones() == 3)
            .unwrap();
        digit_to_mask[8] = *signals
            .iter()
            .find(|signal| signal.count_ones() == 7)
            .unwrap();

        digit_to_mask[3] = *signals
            .iter()
            .filter(|signal| signal.count_ones() == 5)
            .find(|signal| overlaps(**signal, digit_to_mask[1]))
            .unwrap();

        digit_to_mask[9] = *signals
            .iter()
            .filter(|signal| signal.count_ones() == 6)
            .find(|signal| overlaps(**signal, digit_to_mask[3]))
            .unwrap();

        digit_to_mask[0] = *signals
            .iter()
            .filter(|signal| signal.count_ones() == 6)
            .filter(|signal| **signal != digit_to_mask[9])
            .filter(|signal| overlaps(**signal, digit_to_mask[7]))
            .find(|signal| overlaps(**signal, digit_to_mask[1]))
            .unwrap();

        digit_to_mask[6] = *signals
            .iter()
            .filter(|signal| signal.count_ones() == 6)
            .filter(|signal| **signal != digit_to_mask[9])
            .find(|signal| **signal != digit_to_mask[0])
            .unwrap();

        digit_to_mask[5] = *signals
            .iter()
            .filter(|signal| signal.count_ones() == 5)
            .find(|signal| overlaps(digit_to_mask[6], **signal))
            .unwrap();

        digit_to_mask[2] = *signals
            .iter()
            .filter(|signal| signal.count_ones() == 5)
            .filter(|signal| **signal != digit_to_mask[5])
            .find(|signal| **signal != digit_to_mask[3])
            .unwrap();

        output
            .iter()
            .filter_map(|o| digit_to_mask.iter().find_position(|s| **s == *o))
            .map(|i| i.0.to_string())
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
    }

    #[test]
    fn test_determine_output() {
        assert_eq!(determine_output("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"), 5353);
        assert_eq!(determine_output("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"), 8394);
        assert_eq!(determine_output("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"), 9781);
        assert_eq!(
            determine_output(
                "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"
            ),
            1197
        );
        assert_eq!(determine_output("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"), 9361);
        assert_eq!(determine_output("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"), 4873);
        assert_eq!(determine_output("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"), 8418);
        assert_eq!(determine_output("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"), 4548);
        assert_eq!(determine_output("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"), 1625);
        assert_eq!(
            determine_output(
                "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"
            ),
            8717
        );
        assert_eq!(
            determine_output(
                "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            ),
            4315
        );
    }

    #[test]
    fn test_8_1_sample() {
        let input = Raw("
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        ");

        let signal = to_lines(input);

        assert_eq!(count_1478(signal), 26);
    }

    #[test]
    fn test_8_1() {
        let input = to_lines(Path("input/2021/8.txt"));

        assert_eq!(count_1478(input), 530);
    }

    #[test]
    fn test_8_2_sample() {
        let input = to_lines(Raw("
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        "));

        assert_eq!(input.map(|l| determine_output(&l)).sum::<i32>(), 61_229);
    }

    #[test]
    fn test_8_2() {
        let input = to_lines(Path("input/2021/8.txt"));

        assert_eq!(input.map(|l| determine_output(&l)).sum::<i32>(), 1_051_087);
    }
}
