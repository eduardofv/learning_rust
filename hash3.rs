use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("blue", 10);

    scores.entry("blue").or_insert(15);
    scores.entry("yellow").or_insert(15);

    println!("{:?}", scores);


    let text = "hello world wonderful world";

    let mut wc = HashMap::new();

    for word in text.split_whitespace() {
        let count = wc.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", wc);
}
