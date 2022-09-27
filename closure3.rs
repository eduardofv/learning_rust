#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rect { width: 10, height: 1 }, 
        Rect { width: 3, height: 5 }, 
        Rect { width: 7, height: 12 }, 
    ];

//    let mut counter = vec![];
//    let value = String::from("1");

    let mut num_counter = 0;

    list.sort_by_key(|r| {
        //counter.push(value);
        num_counter += 1;
        r.width
    });

    println!("{} steps to sort {:#?}", num_counter, list);
}
