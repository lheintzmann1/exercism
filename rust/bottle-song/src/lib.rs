
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut verses = Vec::new();

    for i in 0..take_down {
        let current = start_bottles - i;
        let next = current - 1;

        let verse = create_verse(current, next);
        verses.push(verse);
    }

    verses.join("\n\n")
}

fn create_verse(current: u32, next: u32) -> String {
    let current_word = num_to_word(current);
    let next_word = num_to_word(next);

    format!(
        "{} green {} hanging on the wall,\n{} green {} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {} green {} hanging on the wall.",
        capitalize(&current_word), bottle_word(current),
        capitalize(&current_word), bottle_word(current),
        next_word, bottle_word(next)
    )
}

fn num_to_word(n: u32) -> String {
    match n {
        0 => "no".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        _ => n.to_string(),
    }
}

fn bottle_word(n: u32) -> &'static str {
    if n == 1 {
        "bottle"
    } else {
        "bottles"
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
