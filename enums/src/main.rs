use std::fmt;

/// IPv4 アドレスを表す構造体
/// 4つの 8ビット整数（オクテット）で構成される
struct Ipv4Addr {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

/// IPv6 アドレスを表す構造体
/// 8つの 16ビット整数で構成される
struct Ipv6Addr {
    segments: [u16; 8],
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

/// IpAddr enum に Display トレイトを実装
impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IpAddr::V4(addr) => write!(f, "{}.{}.{}.{}", addr.a, addr.b, addr.c, addr.d),
            IpAddr::V6(addr) => {
                let segments: Vec<String> =
                    addr.segments.iter().map(|s| format!("{:x}", s)).collect();
                write!(f, "{}", segments.join(":"))
            }
        }
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/// Message enum に Display トレイトを実装
impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Message::Quit => write!(f, "Quit"),
            Message::Move { x, y } => write!(f, "Move({}, {})", x, y),
            Message::Write(s) => write!(f, "Write({})", s),
            Message::ChangeColor(r, g, b) => write!(f, "ChangeColor({}, {}, {})", r, g, b),
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    /// コインの価値をセントで返す
    #[must_use]
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

/// Coin enum に Display トレイトを実装
impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Coin::Penny => write!(f, "Penny"),
            Coin::Nickel => write!(f, "Nickel"),
            Coin::Dime => write!(f, "Dime"),
            Coin::Quarter => write!(f, "Quarter"),
        }
    }
}

fn main() {
    // IPv4: 127.0.0.1
    let home = IpAddr::V4(Ipv4Addr {
        a: 127,
        b: 0,
        c: 0,
        d: 1,
    });

    // IPv6: ::1
    let loopback = IpAddr::V6(Ipv6Addr {
        segments: [0, 0, 0, 0, 0, 0, 0, 1],
    });

    // Display トレイトで IpAddr を表示
    println!("home: {}", home);
    println!("loopback: {}", loopback);

    // Message enum の使用例
    let quit = Message::Quit;
    let move_msg = Message::Move { x: 10, y: 20 };
    let write = Message::Write(String::from("Hello"));
    let color = Message::ChangeColor(255, 0, 0);

    println!("quit: {}", quit);
    println!("move_msg: {}", move_msg);
    println!("write: {}", write);
    println!("color: {}", color);

    // Coin enum と for ループの使用例
    let coins = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];
    for coin in coins {
        println!("{} は {} セントです。", coin, coin.value_in_cents());
    }
}
