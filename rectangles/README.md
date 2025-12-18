# Rectangles - 長方形プログラム

Rust の構造体（struct）とメソッドを使った長方形の面積計算サンプルプログラムです。

## 目次

- [Rectangles - 長方形プログラム](#rectangles---長方形プログラム)
  - [目次](#目次)
  - [概要](#概要)
  - [実行方法](#実行方法)
    - [出力結果](#出力結果)
  - [プログラム構成](#プログラム構成)
    - [構造体とメソッド](#構造体とメソッド)
    - [面積計算関数](#面積計算関数)
    - [`#[must_use]` 属性](#must_use-属性)
    - [`Self` と `&Self`](#self-と-self)
  - [参考](#参考)

---

## 概要

このプログラムは、長方形の面積を計算する複数のアプローチを比較しています：

| アプローチ | 関数/メソッド | 特徴 |
|-----------|--------------|------|
| 個別の引数 | `area_wh` | シンプルだが引数の関連性が不明確 |
| タプル | `area_rect_dim` | グループ化されるが意味が不明確 |
| 構造体（関数） | `area_rect` | フィールド名で意味が明確 |
| 構造体（メソッド） | `rect.area()` | オブジェクト指向的で最も推奨 ✓ |

また、`can_hold` メソッドで別の長方形が収容できるかを判定します。

---

## 実行方法

```bash
cargo run
```

### 出力結果

```text
長方形の面積は、1500平方ピクセルです
長方形の面積は、1500平方ピクセルです
長方形の面積は、1500平方ピクセルです
長方形の面積は、1500平方ピクセルです
rect は (30, 50) です
rect1 は Rectangle { width: 30, height: 50 } です
rect は (
    30,
    50,
) です
rect1 は Rectangle {
    width: 30,
    height: 50,
} です
rect1 は rect2 を収容できる？ true
rect1 は rect3 を収容できる？ false
正方形: Rectangle { width: 20, height: 20 }
正方形の面積は、400平方ピクセルです
```

---

## プログラム構成

### 構造体とメソッド

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// 長方形の面積を計算する。
    #[must_use]
    fn area(&self) -> u32

    /// 別の長方形が self の中に完全に収まるかを判定する。
    #[must_use]
    fn can_hold(&self, other: &Self) -> bool

    /// 正方形を作成する関連関数（コンストラクタ）。
    #[must_use]
    fn square(size: u32) -> Self
}
```

### 面積計算関数

```rust
fn area_wh(width: u32, height: u32) -> u32       // 個別の引数
fn area_rect_dim(dimensions: (u32, u32)) -> u32  // タプル
fn area_rect(rectangle: &Rectangle) -> u32       // 構造体参照
```

### `#[must_use]` 属性

戻り値を使わないと警告が出る属性です。副作用のないメソッドに付けることで、戻り値の使い忘れを防ぎます。

```rust
// ❌ 警告: 戻り値を無視
rect1.area();

// ✅ OK: 戻り値を使用
let a = rect1.area();
println!("{}", rect1.area());
```

### `Self` と `&Self`

`impl` ブロック内で使える特別なキーワードです。

| 記法 | 意味 | 等価 |
|-----|------|-----|
| `Self` | 実装対象の型そのもの | `Rectangle` |
| `&Self` | 実装対象の型への参照 | `&Rectangle` |

```rust
impl Rectangle {
    // &Rectangle と同じ意味
    fn can_hold(&self, other: &Self) -> bool

    // Rectangle と同じ意味
    fn square(size: u32) -> Self
}
```

型名が変わっても `Self` を使っていれば修正不要です。

---

## 参考

- [The Rust Programming Language 日本語版 - 構造体を使ったプログラム例](https://doc.rust-jp.rs/book-ja/ch05-02-example-structs.html)
- [The Rust Programming Language 日本語版 - メソッド記法](https://doc.rust-jp.rs/book-ja/ch05-03-method-syntax.html)
