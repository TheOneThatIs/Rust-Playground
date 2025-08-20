struct Point {
    x: i32,
    y: i32,
}

fn main() {
    one(1);

}

fn one(x: i32){
    let mut a_point = Point { x, y: x+2 };
    println!("({},{})", a_point.x, a_point.y);
    a_point.x = 3;
    println!("({},{})", a_point.x, a_point.y);
}