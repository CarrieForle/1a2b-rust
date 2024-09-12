use rand;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ab_good() {
        assert_eq!(Ok(AB(1, 1)), read_ab("1a1b"));
        assert_eq!(Ok(AB(2, 2)), read_ab("2A2B"));
        assert_eq!(Ok(AB(0, 4)), read_ab("4B"));
        assert_eq!(Ok(AB(3, 0)), read_ab("3a"));
        assert_eq!(Ok(AB(0, 0)), read_ab("0"));
        assert_eq!(Ok(AB(0, 0)), read_ab("0a0b"));
    }
    
    #[test]
    fn test_ab_fail() {
        assert!(read_ab("-1a5b").is_err());
        assert!(read_ab("").is_err());
        assert!(read_ab("4a2b").is_err());
        assert!(read_ab("3a1b").is_err());
        assert!(read_ab("what the fuck").is_err());
    }

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
}

#[derive(Debug)]
#[derive(PartialEq)]
struct AB(u32, u32);

impl AB {
    fn is_valid(input: &str) -> bool {
        if input.len() != 4 {
            return false;
        }

        if input.parse::<u32>().is_err() {
            return false;
        }

        let v: &[u8] = input.as_bytes();

        if v[0] == '-' as u8 {
            return false;
        }

        for i in 0..4 {
            for j in i+1..4 {
                if v[i] == v[j] {
                    return false;
                }
            }
        }

        true
    }

    fn new(guess: &str, answer: &str) -> Result<AB, &'static str> {
        if !AB::is_valid(guess) {
            return Err("Invalid guess");
        }
        
        if !AB::is_valid(answer) {
            return Err("Invalid answer");
        }

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
}

impl From<AB> for String {
    fn from(ab: AB) -> String {
        format!("{}a{}b", ab.0, ab.1)
    }
}

pub fn start_game() {
    
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