fn main() {
    let books = [
        Book {
            title: String::from("The Great Gatsby"),
            author: String::from("F. Scott Fitzgerald"),
            pages: 180,
        },
        Book {
            title: String::from("Heralds of the King"),
            author: String::from("Dave Hunt"),
            pages: 250,
        },
    ];
    for book in books {
        println!("{}", book.get_summary());
    }
}

struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    fn get_summary(&self) -> String {
        format!("{} by {} - {} pages", self.title, self.author, self.pages)
    }
}
