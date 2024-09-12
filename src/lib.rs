use rand;
use std::io::{Error, Write, stdin, stdout};
use std::fmt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick() {
        for _ in 0..10000 {
            let picked = pick();

            assert_eq!(4, picked.len());
            let picked_array: &[u8] = picked.as_bytes();

            let mut is_not_duplicated = true;

            'outer: for i in 0..4 {
                for j in i+1..4 {
                    if picked_array[i] == picked_array[j] {
                        is_not_duplicated = false;
                        break 'outer;
                    }
                }
            }

            assert!(is_not_duplicated, "{picked} is duplicated");
        }
    }

    #[test]
    fn ab_validation_work() {
        assert!(AB::is_valid("1234").is_ok());
        assert!(AB::is_valid("0194").is_ok());
        assert!(AB::is_valid("-194").is_err());
        assert!(AB::is_valid("fjkl").is_err());
        assert!(AB::is_valid("31256").is_err());
        assert!(AB::is_valid("1123").is_err());
    }

    #[test]
    fn get_ab_work() {
        assert_eq!(Ok(AB(0, 0)), AB::new("1234", "5678"));
        assert_eq!(Ok(AB(2, 1)), AB::new("1235", "1243"));
        assert_eq!(Ok(AB(4, 0)), AB::new("1234", "1234"));
        assert_eq!(Ok(AB(0, 4)), AB::new("1423", "2341"));
        assert!(AB::new("1211", "1243").is_err());
    }

    #[test]
    fn string_from_ab_work() {
        assert_eq!("1a2b", format!("{}", AB(1, 2)));
        assert_eq!("3a6b", format!("{}", AB(3, 6)));
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
struct AB(u32, u32);

impl AB {
    fn is_valid(input: &str) -> Result<(), &'static str> {
        if input.len() != 4 {
            return Err("invalid");
        }

        if input.parse::<u32>().is_err() {
            return Err("not a number")
        }

        let v: &[u8] = input.as_bytes();

        if v[0] == '-' as u8 {
            return Err("not positive");
        }

        for i in 0..4 {
            for j in i+1..4 {
                if v[i] == v[j] {
                    return Err("found duplicate digit")
                }
            }
        }

        Ok(())
    }

    fn new(guess: &str, answer: &str) -> Result<AB, &'static str> {
        AB::is_valid(guess)?;
        AB::is_valid(answer)?;

        let guess = guess.bytes();
        let answer = answer.bytes();

        let mut a = 0;
        let mut b = 0;

        for (i, guess_byte) in guess.enumerate() {
            for (j, answer_byte) in answer.clone().enumerate() {
                if guess_byte == answer_byte {
                    if i == j {
                        a += 1;
                    } else {
                        b += 1;
                    }
                }
            }
        }

        Ok(AB(a, b))
    }

    fn is_won(&self) -> bool {
        *self == AB(4, 0)
    }
}

impl fmt::Display for AB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}a{}b", self.0, self.1)
    }
}

pub fn run_game() -> Result<(), Error> {
    let answer = pick();
    let mut tries = 0;
    let mut guess = String::new();

    let mut res = AB(0, 0);

    while !res.is_won() {
        tries += 1;

        print!("Make your #{} guess = ", tries);
        stdout().flush()?;

        loop {
            guess.clear();

            if let Err(e) = stdin().read_line(&mut guess) {
                print!("Failed to read your guess: {e} Please try again = ");

                continue;
            }

            let guess = guess.trim_end();

            if guess.is_empty() {
                continue;
            }

            match AB::new(guess, &answer) {
                Ok(ab) => res = ab,
                Err(e) => {
                    println!("Error: {e}. Please try again!");

                    continue;
                },
            }

            break;
        }

        print!("{res} ");
    }

    println!("The answer is {answer}! You guessed it in {tries} {}!", if tries == 1 { "try" } else { "tries" });

    Ok(())
}

fn pick() -> String {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut res = 0;

    for _ in 0..4 {
        let index = rand::random::<usize>() % nums.len();
        res = res * 10 + nums[index];
        nums.remove(index);
    }

    res.to_string()
}