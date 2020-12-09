#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-05.txt");

fn main() {
    let input = make_input_binary(INPUT);
    let mut ids: Vec<u32> = input.lines().map(parse_id).collect();

    // Star 1
    let max = star1(&ids);
    println!("Max id is {}.", max);

    // Star 2
    let my_seat = star2(&mut ids);
    println!("Id of empty seat is {}.", my_seat);
}

fn star1(ids: &Vec<u32>) -> &u32 {
    ids.iter().max().expect("No ids found!")
}

fn star2(ids: &mut Vec<u32>) -> u32 {
    ids.sort();
    ids.iter()
        .zip(ids.iter().skip(1))
        .find(|(&left, &right)| left + 1 < right)
        .map(|(left, _)| left + 1)
        .expect("No empty seat found!")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_star1() {
        let input = make_input_binary(INPUT);
        let ids: Vec<u32> = input.lines().map(parse_id).collect();
        let max = star1(&ids);
        assert_eq!(*max, 832);
    }

    #[test]
    fn full_star2() {
        let input = make_input_binary(INPUT);
        let mut ids: Vec<u32> = input.lines().map(parse_id).collect();
        let my_seat = star2(&mut ids);
        assert_eq!(my_seat, 517);
    }
}
