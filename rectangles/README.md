# Rectangles - 長方形プログラム

Rust の構造体（struct）を使った長方形の面積計算サンプルプログラムです。

## 目次

- [Rectangles - 長方形プログラム](#rectangles---長方形プログラム)
  - [目次](#目次)
  - [概要](#概要)
  - [実行方法](#実行方法)
    - [出力結果](#出力結果)
  - [プログラム構成](#プログラム構成)
    - [構造体定義](#構造体定義)
    - [面積計算関数](#面積計算関数)
  - [参考](#参考)

---

## 概要

このプログラムは、長方形の面積を計算する3つのアプローチを比較しています：

| アプローチ | 関数名 | 特徴 |
|-----------|--------|------|
| 個別の引数 | `area_wh` | シンプルだが引数の関連性が不明確 |
| タプル | `area_rect_dim` | グループ化されるが意味が不明確 |
| 構造体 | `area_rect` | フィールド名で意味が明確 ✓ |

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
```

---

## プログラム構成

### 構造体定義

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

`#[derive(Debug)]` により `{:?}` や `{:#?}` でのデバッグ出力が可能。

### 面積計算関数

```rust
// 1. 個別の引数
fn area_wh(width: u32, height: u32) -> u32

// 2. タプル
fn area_rect_dim(dimensions: (u32, u32)) -> u32

// 3. 構造体（推奨）
fn area_rect(rectangle: &Rectangle) -> u32
```

---

## 参考

- [The Rust Programming Language 日本語版 - 構造体を使ったプログラム例](https://doc.rust-jp.rs/book-ja/ch05-02-example-structs.html)
