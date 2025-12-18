#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// 長方形の面積を計算する。
    #[must_use]
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /// 別の長方形が self の中に完全に収まるかを判定する。
    /// 幅と高さの両方が self より小さい場合に true を返す。
    #[must_use]
    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }

    /// 正方形を作成する関連関数（コンストラクタ）。
    /// 幅と高さが同じ Rectangle を返す。
    #[must_use]
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width = 30;
    let height = 50;

    let rect = (30, 50);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("長方形の面積は、{}平方ピクセルです", area_wh(width, height));
    println!("長方形の面積は、{}平方ピクセルです", area_rect_dim(rect));
    println!("長方形の面積は、{}平方ピクセルです", area_rect(&rect1));
    println!("長方形の面積は、{}平方ピクセルです", rect1.area());

    println!("rect は {:?} です", rect);
    println!("rect1 は {:?} です", rect1);

    println!("rect は {:#?} です", rect);
    println!("rect1 は {:#?} です", rect1);

    println!("rect1 は rect2 を収容できる？ {}", rect1.can_hold(&rect2));
    println!("rect1 は rect3 を収容できる？ {}", rect1.can_hold(&rect3));

    // 正方形の作成
    let sq = Rectangle::square(20);
    println!("正方形: {:?}", sq);
    println!("正方形の面積は、{}平方ピクセルです", sq.area());
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
