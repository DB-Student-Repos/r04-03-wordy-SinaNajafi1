pub fn answer(command: &str) -> Option<i32> {
    let command = command.trim_end_matches('?');
    let mut splits = command.split_whitespace();
    if splits.next() != Some("What") || splits.next() != Some("is") {
        return None;
    }

    let mut result = splits.next()?.parse::<i32>().ok()?;
    while let Some(part) = splits.next() {
        match part {
            "plus" => result += splits.next()?.parse::<i32>().ok()?,
            "minus" => result -= splits.next()?.parse::<i32>().ok()?,
            "multiplied" => {
                if splits.next() != Some("by") {
                    return None;
                }
                result *= splits.next()?.parse::<i32>().ok()?;
            }, 
            "divided" => {
                if splits.next() != Some("by") {
                    return None;
                }
                result /= splits.next()?.parse::<i32>().ok()?;
            },
            _ => return None,
        }
    }

    Some(result)
}
