struct Text {
    content: String,
}

fn print_text(text: &Text) {
    println!("{}", text.content);
}

fn append_text(text: &mut Text, extra: &str) {
    text.content.push_str(extra);
}

fn get_length(text: &Text) -> usize {
    text.content.len()
}

fn clear_text(text: &mut Text) {
    text.content.clear();
}

fn main() {
    let mut text = Text {
        content: String::from("Hello"),
    };

    print_text(&text);
    append_text(&mut text, ", world!");
    print_text(&text);
    let length = get_length(&text);

    println!("Length of text: {}", length);

    clear_text(&mut text);

    // empty text
    print_text(&text);
}