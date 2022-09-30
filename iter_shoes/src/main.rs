#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, show_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == show_size).collect()
}

/*
fn shoes_in_size_2(shoes: &Vec<Shoe>, show_size: u32) -> Vec<Shoe> {
    (&shoes).into_iter().filter(|s| s.size == show_size).collect()
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn filters_by_size() {

        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        println!("{:?}", shoes);
        let in_my_size = shoes_in_size(shoes, 10);
        //let in_my_size = shoes_in_size_2(&shoes, 10);

        //println!("{:?}", shoes);
        println!("{:?}", in_my_size);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );

    }
}
