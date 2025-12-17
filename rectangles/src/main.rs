#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 30;
    let height = 50;

    let rect = (30, 50);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("長方形の面積は、{}平方ピクセルです", area_wh(width, height));
    println!("長方形の面積は、{}平方ピクセルです", area_rect_dim(rect));
    println!("長方形の面積は、{}平方ピクセルです", area_rect(&rect1));

    println!("rect は {:?} です", rect);
    println!("rect1 は {:?} です", rect1);

    println!("rect は {:#?} です", rect);
    println!("rect1 は {:#?} です", rect1);
}

fn area_wh(width: u32, height: u32) -> u32 {
    width * height
}

fn area_rect_dim(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_rect(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
