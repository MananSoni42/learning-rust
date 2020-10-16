#[derive(Debug)] // opt-in for debugging for this struct
struct Rect { // A rectangle type
    width: u32,
    height: u32,
}

impl Rect { // add methods
    fn area(&self) -> u32 { // calc area
        self.width * self.height
    }

    fn holds(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn calc_area(r: &Rect) -> u32 { // equivalent function to calc rectangle area
    r.width * r.height
}

fn main() {
    let r1 = Rect { width: 30, height: 40 };
    let r2 = Rect { width: 20, height: 30 };
    let r3 = Rect { width: 50, height: 20 };

    println!("rect 1: {:?}", r1);
    println!("rect 2: {:#?}", r2);
    println!("rect 3: {:#?}", r3);
    println!("Area using function: {}",calc_area(&r1));
    println!("Area using method: {}",r1.area());
    println!("r1 holds r2? {}",r1.holds(&r2));
    println!("r1 holds r3? {}",r1.holds(&r3));
}
