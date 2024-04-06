use std::fmt;



pub trait Summary {
    fn summarize(&self) ->  String {
        format!("(Read more from {}...)",self.summarize_author())
    }

    fn summarize_author(&self) ->String;
}


pub struct NewsArticle {
    pub headline:String,
    pub location:String,
    pub author:String,
    pub content:String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) ->String {
        format!("(Read more from {}...", self.author)
    }

    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.author,
            self.location
        )
    }
}


pub struct Tweet {
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool,
}


impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.content)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) ->String {
        format!("@{}", self.username)
    }
    
    fn summarize(&self) ->  String {
        format!("(Read more from {}...)",self.summarize_author())
    }
    
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username:String::from("horse_eboosk"),
        content: String::from("of course, as you probably already know, people",
    ),
    reply:false,
    retweet:false,
    }
}

pub fn notify(item:&impl Summary) {
    println!("Breaking News! {}", item.summarize())
}
/*
pub fn notify<item: T, item2: U>(t:&T, u:&U) -> i32 
Where 
T: Display + Clone,
U: Display + Debug,
{
    //code
}

*/

fn main() {

let tweet = Tweet {
    username:String::from("Horese_ebooks"),
    content: String::from("of course, as you already know people"),
    reply:false,
    retweet:false,
};
    println!("1 new tweet: {}", tweet.summarize());


    let article = NewsArticle {
        headline: String::from("Penguins win the stanley cup championship!"),
        location: String::from("Pittsburg, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The pittsburg Penguins once again are the best hockey team in the NHL"),
    };

    println!("news artictle available: {}", article.summarize());
}
