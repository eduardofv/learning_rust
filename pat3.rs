enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(1,2,2));

    match msg {
        Message::ChangeColor(Color::Rgb(
}
