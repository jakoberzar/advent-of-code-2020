#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-04.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-04.txt");

fn main() {
    // Star 1
    let valid = star1(INPUT);
    println!("{} entries have (almost) all the fields", valid);

    // Star 2
    let valid = star2(INPUT);
    println!(
        "{} entries have (almost) all the fields and all valid",
        valid
    );
}

fn star1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|passport| {
            passport
                .trim()
                .split(&['\n', ' '][..])
                .filter(|&x| x.split(':').next().unwrap() != "cid")
                .count()
        })
        .filter(|x| *x == 7)
        .count()
}

fn star2(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|&passport| validate_passport(passport))
        .count()
}

fn validate_passport(passport: &str) -> bool {
    passport
        .trim()
        .split(&['\n', ' '][..])
        .filter(|&x| {
            let mut s = x.split(':');
            validate_field(s.next().unwrap(), s.next().unwrap()).unwrap_or(false)
        })
        .count()
        == 7 // All 7 required fields are valid
}

fn validate_field(key: &str, value: &str) -> Result<bool, std::num::ParseIntError> {
    match key {
        "byr" => {
            let year: u32 = value.parse()?;
            Ok(1920 <= year && year <= 2002)
        }
        "iyr" => {
            let year: u32 = value.parse()?;
            Ok(2010 <= year && year <= 2020)
        }
        "eyr" => {
            let year: u32 = value.parse()?;
            Ok(2020 <= year && year <= 2030)
        }
        "hgt" => {
            if value.ends_with("cm") {
                let height: u32 = value[..value.len() - 2].parse()?;
                return Ok(150 <= height && height <= 193);
            } else if value.ends_with("in") {
                let height: u32 = value[..value.len() - 2].parse()?;
                return Ok(59 <= height && height <= 76);
            }
            return Ok(false);
        }
        "hcl" => {
            if !value.starts_with('#') {
                return Ok(false);
            }

            let hex_chars = value
                .chars()
                .skip(1)
                .filter(|x| x.is_ascii_hexdigit())
                .count();
            Ok(hex_chars == 6)
        }
        "ecl" => {
            let accepted = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            Ok(accepted.contains(&value))
        }
        "pid" => Ok(value.chars().filter(|x| x.is_ascii_digit()).count() == 9),
        "cid" => Ok(false),
        _ => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_star1() {
        assert_eq!(star1(INPUT), 230);
    }

    #[test]
    fn full_star2() {
        assert_eq!(star2(INPUT), 156);
    }
}
