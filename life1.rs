fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let str1 = String::from("abcd");
    let res;

    {
        let str2 = "xyzzz";
        //let str2 = String::from("xyzzz");

        res = longest(str1.as_str(), str2);
    }

    //println!("str2 is {}", str2);
    println!("Longest is {}", res);
}
