#[derive(Debug, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    items: Vec<ShirtColor>,
}

impl Inventory {

    fn giveaway(&self, preference: Option<ShirtColor>) -> ShirtColor {
        preference.unwrap_or_else(|| self.most_stocked())
//        preference.unwrap_or(self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_count = 0;
        let mut blue_count = 0;

        for color in &self.items {
            match color {
                ShirtColor::Red => red_count += 1,
                ShirtColor::Blue => blue_count += 1,
            }
        }

        if red_count > blue_count {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory { 
        items: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue], 
    };

    let user_pref_1 = Some(ShirtColor::Red);
    let user_gets_1 = store.giveaway(user_pref_1);
    println!("User 1 wanted {:?} and got {:?}", user_pref_1, user_gets_1);


    let user_pref_2 = None;
    let user_gets_2 = store.giveaway(user_pref_2);
    println!("User 2 wanted {:?} and got {:?}", user_pref_2, user_gets_2);
}

