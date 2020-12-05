#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-05.txt");

fn main() {
    let input = make_input_binary(INPUT);
    let mut ids: Vec<u32> = input.lines().map(parse_id).collect();
    star1(&ids);
    star2(&mut ids);
}

fn star1(ids: &Vec<u32>) {
    let max = ids.iter().max().expect("No ids found!");
    println!("Max id is {}.", max);
}

fn star2(ids: &mut Vec<u32>) {
    ids.sort();
    let my_seat = ids
        .iter()
        .zip(ids.iter().skip(1))
        .filter(|(&left, &right)| left + 1 < right)
        .map(|(&left, _)| left + 1)
        .next()
        .expect("No empty seat found!");

    println!("Id of empty seat is {}.", my_seat);
}

fn make_input_binary(input: &str) -> String {
    input
        .trim()
        .replace(&['F', 'L'][..], "0")
        .replace(&['B', 'R'][..], "1")
}

fn parse_id(line: &str) -> u32 {
    u32::from_str_radix(line, 2).expect(&format!("Invalid binary in line: {}", line))
}
