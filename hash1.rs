use std::collections::HashMap;

fn default_value() -> &'static i32 {
    println!("default");
    &11
}

fn main() {
    let mut scores = HashMap::new();

    scores.insert("blue", 10);
    
/*
    let t1 = (1, 2);

    scores.insert(t1, 10);
*/

    if let Some(s) = scores.get("blue"){
        println!("{s}");
    }

    let s2 = scores.get("blue").unwrap_or(&default_value());
    println!("{s2}");

    let s4 = scores.get("blue").unwrap_or_default();
    println!("{s4}");

    //let s3 = scores.get("blue").unwrap_or_else(default_value);
    //println("{s3}");
}

