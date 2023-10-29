pub fn main() {
    let s1 = "first";
    let s2 = "apple";
    let s3 = "";

    let pig_latin_s1 = convert_to_pig_latin(s1);
    let pig_latin_s2 = convert_to_pig_latin(s2);
    let pig_latin_s3 = convert_to_pig_latin(s3);

    println!("s1 {}", pig_latin_s1);
    println!("s2 {}", pig_latin_s2);
    println!("s3 {}", pig_latin_s3);
}

fn convert_to_pig_latin(input: &str) -> String {
    if input.is_empty() {
        return "".to_string();
    }

    fn check_is_consonant(character: &char) -> bool {
        const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

        return !VOWELS.contains(character);
    }

    let first_char_option = input.chars().next();
    let mut result = String::new();

    if let Some(first_char) = first_char_option {
        if check_is_consonant(&first_char) {
            result.push_str(&input[1..]);
            result.push('-');
            result.push(first_char);
            result.push_str("ay");
        } else {
            result.push_str(&input);
            result.push_str("-hay");
        }
    }

    return result;
}
