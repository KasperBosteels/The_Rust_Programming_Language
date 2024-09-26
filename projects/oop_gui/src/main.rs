use oop_gui::{ Draw, Screen };

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to draw
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![String::from("yes"), String::from("maybe"), String::from("no")],
            }),
        ],
    };
    screen.run();
}
