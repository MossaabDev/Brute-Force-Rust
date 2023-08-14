const allowed_chars: [char; 62] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

fn generate_string(mut num: i32) -> String {
    let mut result = String::new();
    let mut num = num;
    while num >= 0 {
        result.push(allowed_chars[(num%allowed_chars.len()as i32)as usize]);
        num = (num / allowed_chars.len() as i32) -1;
    }

    result
}

fn brute_force(s: &str) -> bool {
    for i in 0..1_000_000_00 {
        if generate_string(i) == s {
            println!("found : {}", generate_string(i));
            return true;
        }
    }
    false
}

fn main() {
    const MAX_LENGTH: usize = 5;
    let password = "aaaaa";

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
