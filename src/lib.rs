mod tests {

}

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
    }
}