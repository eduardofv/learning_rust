#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Option<UsState>)
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(opt_state) => {

/*
            match opt_state {
                Some(state) => {
                    println!("Quarter from {:?}", state)
                }
                None => (),//println!("No state"),
            };
*/

            if let Some(state) = opt_state {
                println!("Quarter from {:?}", state)
            }
            25
        }
    }
}


fn main() {
    let my_coin = Coin::Quarter(Option::Some(UsState::Alaska));
    //let my_coin = Coin::Quarter(Option::None);

    let value = value_in_cents(my_coin);

    println!("The coin value is {value}");
}
