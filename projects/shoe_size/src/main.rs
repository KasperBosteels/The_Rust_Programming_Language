#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 20,
                style: String::from("snakes"),
            },
            Shoe {
                size: 11,
                style: String::from("human skin"),
            },
            Shoe {
                size: 20,
                style: String::from("shoe"),
            },
        ];

        let in_my_size = shoe_in_size(shoes, 20);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 20,
                    style: String::from("snakes"),
                },
                Shoe {
                    size: 20,
                    style: String::from("shoe"),
                },
            ]
        );
    }
}

fn main() {}
