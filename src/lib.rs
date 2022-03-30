// strtol converts String to i64
pub fn strtol(s: &String) -> (Option<i64>, String) {
    if s.is_empty() {
        return (None, s.clone());
    }

    let mut pos = 0;
    let mut remaining = s.clone();
    let len = s.len();

    while pos < len {
        if !s.chars().nth(pos).unwrap().is_ascii_digit() {
            break;
        }
        pos += 1;
    }

    if pos == len {
        return (Some(remaining.parse::<i64>().unwrap()), "".into());
    }

    let t: String = remaining.drain(..pos).collect();
    (Some(t.parse::<i64>().unwrap()), remaining)
}