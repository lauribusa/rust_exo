pub fn print_string() {
    let vec_utf8 = vec![104, 101, 108, 108, 111];

    let str = String::from_utf8(vec_utf8).expect("An error was blown out");
    let mut nounours_intro = String::from("T'es un forceur Nounours");

    nounours_intro.push('!');
    nounours_intro.push_str("!...");

    // This will return a SLICE of characters
    nounours_intro.as_str();

    // This will return the length of the string
    nounours_intro.len();

    // Replace a serie of characters by another, and returns a new String
    nounours_intro.replace("Nounours", "Nicolas");

    // To iterate through a string
    for c in str.chars() {
        println!("{}", c);
    }

    // If a string contains a specific string of characters
    str.contains("hello");

    // simple string manipulation lowercase/uppercase. This doesn't mute the original string and returns a new one
    let _ = str.to_lowercase();
    let _ = str.to_uppercase();
    println!("{}", str);
    println!("{}", nounours_intro);
}

pub fn exo_string() {
    loop {
        let mut input = read_input();
        input = input.to_lowercase();
        if input.len() > 0 {
            let mut vowels_count: u8 = 0;
            for c in input.chars() {
                if c == 'a' {
                    add_count(&mut vowels_count);
                }
                if c == 'e' {
                    add_count(&mut vowels_count);
                }
                if c == 'i' {
                    add_count(&mut vowels_count);
                }
                if c == 'o' {
                    add_count(&mut vowels_count);
                }
                if c == 'u' {
                    add_count(&mut vowels_count);
                }
                if c == 'y' {
                    add_count(&mut vowels_count);
                }
            }
            println!("Number of vowels for string [ {} ]", input);
            println!("{}", vowels_count);
            break;
        }
    }
}

pub fn exo_string_2(hidden_word: &str, hint: &str){
    let mut display_word = "".to_string();
    for _c in hidden_word.chars() {
        display_word.push('.');
    }

    println!("Hint: {}", hint);
    println!("{}", display_word);
    loop {
        let input = read_input().to_lowercase();
        let first_letter = input.chars().nth(0).unwrap();
        println!("Hint: {}", hint);
        if hidden_word.contains(first_letter) {
            let mut i: usize = 0;
            for c in hidden_word.chars() {
                if first_letter == c {
                    display_word.replace_range(i..i+1,c.to_string().as_str());
                }
                i = i + 1 as usize;
            }
            println!("{}", display_word);
        }
        if hidden_word.contains(&display_word) {
            println!("Found the hidden word: {}", display_word);
            break;
        }
    }
}

fn add_count(vowels_count: &mut u8) {
    *vowels_count += 1 as u8;
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read");
    input.trim().to_string()
}
