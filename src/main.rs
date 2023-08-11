const ALLOWED_CHARS: [char; 62] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

fn generate_string(mut num: i32) -> String {
    let mut result = String::with_capacity(4);

    while num >= 0 {
        let index = (num % ALLOWED_CHARS.len() as i32) as usize;
        result.push(ALLOWED_CHARS[index]);
        num = (num / ALLOWED_CHARS.len() as i32) - 1;
    }

    result
}

fn brute_force(s: &str) -> bool {
    for i in 0..1_000_000 {
        if generate_string(i) == s {
            return true;
        }
    }
    false
}

fn main() {
    const MAX_LENGTH: usize = 4;
    let password = "9999";

    if password.len() > MAX_LENGTH {
        println!("Password should be less than {} symbols", MAX_LENGTH);
    } else {
        let start = std::time::Instant::now();
        let found = brute_force(password);
        let end = std::time::Instant::now();
        let duration = end - start;

        if found {
            println!("Password Found! During: {:?} milliseconds", duration.as_millis());
        } else {
            println!("Failed in {:?} milliseconds", duration.as_millis());
        }
    }
}
