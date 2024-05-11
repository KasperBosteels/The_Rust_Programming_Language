use std::fmt::Display;

fn longest_with_an_announcement<'a,T>(
    x:&'a str,
    y:&'a str,
    ann:T,
) -> &'a str
where T:Display
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    }else{
        y
    }
}

fn main() {
    let x = String::from("this is X");
    let y = String::from("this longer Y");
    let ann = <dyn Display>
    longest_with_an_announcement(&x,&y,ann)
}