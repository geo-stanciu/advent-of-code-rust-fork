use aoc::year2022::day06::*;

const EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

#[test]
fn part1_example() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 7);
}

#[test]
fn part2_example() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 19);
}
