mod tests {

}

struct AB(u32, u32);

pub fn start_game() {

}

fn read_ab(ab_str: &str) -> Result<AB> {
    if ab_str == "0" {
        return AB(0, 0);
    }

    let ab_array = ab_str.as_bytes();

    if ab_str.len() == 2 {
        let n = ab_array[0].to_digit(10)?;

        if ab_array[1] == 'a' {
            return Ok(AB(n, 0));
        } else if ab_array[1] == 'b' {
            return Ok(AB(0, n));
        } else {
            return Err()
        }
    }
}