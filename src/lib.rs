mod tests {

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

pub fn start_game() {

}

fn read_ab(ab_str: &str) -> Result<AB, &'static str> {
    if ab_str == "0" {
        return Ok(AB(0, 0));
    }

    let ab_array: Vec<char> = ab_str.bytes().map( |byte| (byte as char).to_ascii_uppercase()).collect();

    if ab_str.len() == 2 {
        let n = ab_array[0].to_digit(10).ok_or("Invalid AB string")?;

        if n > 4 {
            return Err("Invalid AB string");
        }

        if ab_array[1] == 'A' {
            return Ok(AB(n, 0));
        } else if ab_array[1] == 'B' {
            return Ok(AB(0, n));
        } else {
            return Err("Invalid AB string");
        }
    } else if ab_str.len() == 4 {
        if ab_array[1] != 'A' || ab_array[3] != 'B' {
            return Err("Invalid AB string");
        }

        let a = ab_array[0].to_digit(10).ok_or("Invalid AB string")?;
        let b = ab_array[2].to_digit(10).ok_or("Invalid AB string")?;

        if a + b > 4 || (a + b == 4 && b == 1) {
            return Err("Invalid AB string");
        }

        return Ok(AB(a, b));
    }
    
    Err("Invalid AB string")
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