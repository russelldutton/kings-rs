use rand::{thread_rng, Rng};

pub fn generate_random_code() -> String {
    let chars: Vec<char> = (48..58) // ASCII values for numbers 0-9
        .chain(65..91) // ASCII values for uppercase letters A-Z
        .map(|i| i as u8 as char)
        .collect();

    let mut rng = thread_rng();
    let code: String = (0..4)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect();

    code
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::generate_random_code;

    #[test]
    fn ensure_unique_codes() {
        let count = 1000;
        let codes: HashSet<String> = (0..count).map(|_i| generate_random_code()).collect();

        assert!(codes.len() > 900);
    }

    #[test]
    fn ensure_right_length() {
        let code = generate_random_code();

        assert_eq!(code.len(), 4);
    }
}
