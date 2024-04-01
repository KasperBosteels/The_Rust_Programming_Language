struct Point<X1,Y1>{
    x:X1,
    y:Y1,
}

impl<X1,Y1> Point<X1,Y1> {
    fn mixup<X2,Y2>(
        self,
        other:Point<X2,Y2>,
    ) -> Point<X1,Y2> {
        Point {
            x:self.x,
            y: other.y
        }
    }
}
 
f&n main() {
    let p1 = Point {x:2, y:12.44};
    let p2 = Point { x:"BYE", y:'u'};

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}",p3.x,p3.y);

} 
