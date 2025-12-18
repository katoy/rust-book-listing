# Enums - 列挙型プログラム

Rust の列挙型（enum）と `Display` トレイトを使ったサンプルプログラムです。

## 目次

- [概要](#概要)
- [実行方法](#実行方法)
- [プログラム構成](#プログラム構成)
- [参考](#参考)

---

## 概要

このプログラムは Rust の列挙型の様々な使い方を示しています：

| enum | 説明 |
|------|------|
| `IpAddr` | IPv4/IPv6 アドレスを表現 |
| `Message` | 4種類のバリアント（ユニット、構造体、タプル） |
| `Coin` | 硬貨の種類と価値を表現 |

---

## 実行方法

```bash
cargo run
```

### 出力結果

```text
home: 127.0.0.1
loopback: 0:0:0:0:0:0:0:1
quit: Quit
move_msg: Move(10, 20)
write: Write(Hello)
color: ChangeColor(255, 0, 0)
Penny は 1 セントです。
Nickel は 5 セントです。
Dime は 10 セントです。
Quarter は 25 セントです。
```

---

## プログラム構成

### IpAddr enum と Display トレイト

```rust
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl fmt::Display for IpAddr { ... }
```

### Message enum（4種類のバリアント）

```rust
enum Message {
    Quit,                       // ユニットバリアント
    Move { x: i32, y: i32 },    // 構造体バリアント
    Write(String),              // タプルバリアント
    ChangeColor(i32, i32, i32), // タプルバリアント
}
```

### Coin enum とメソッド

```rust
enum Coin { Penny, Nickel, Dime, Quarter }

impl Coin {
    #[must_use]
    fn value_in_cents(&self) -> u32 { ... }
}

impl fmt::Display for Coin { ... }
```

---

## 学習ポイント

### `Display` トレイト

`{}` でフォーマット出力するためのトレイト。`#[derive(Debug)]` の `{:?}` とは異なり、ユーザー向けの表示を自分で定義できます。

```rust
impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Coin::Penny => write!(f, "Penny"),
            // ...
        }
    }
}
```

### `#[must_use]` 属性

戻り値を使わないと警告が出る属性。副作用のないメソッドに付けることで、戻り値の使い忘れを防ぎます。

```rust
// ❌ 警告: 戻り値を無視
coin.value_in_cents();

// ✅ OK: 戻り値を使用
let cents = coin.value_in_cents();
```

---

## 参考

- [The Rust Programming Language 日本語版 - Enumを定義する](https://doc.rust-jp.rs/book-ja/ch06-01-defining-an-enum.html)
- [The Rust Programming Language 日本語版 - matchフロー制御演算子](https://doc.rust-jp.rs/book-ja/ch06-02-match.html)
