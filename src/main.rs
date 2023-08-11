const ALLOWED_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn generate_string(num: i32) -> String {
    let mut result = String::new(); // Assuming you're generating passwords of length 4
    let mut num = num;

    while num >= 0 {
        let index = (num % ALLOWED_CHARS.len() as i32) as usize;
        result.push(ALLOWED_CHARS.chars().nth(index).unwrap());
        num = (num / ALLOWED_CHARS.len() as i32) - 1;
    }

    result
}


fn brute_force(s: &str) -> bool {

    for i in 0..1_000_000_000 {
        if generate_string(i) == s {
            return true;
        }
    }
    false
}

fn main() {
    const MAX_LENGTH: usize = 4;
    let password = "aaaa";
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
