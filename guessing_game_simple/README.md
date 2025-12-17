# Guessing Game Simple

数当てゲームの簡易版です。

## 目次

- [概要](#概要)
- [プロジェクト構造](#プロジェクト構造)
- [使い方](#使い方)
- [開発](#開発)
- [API](#api)
- [テスト項目・結果](#テスト項目結果)
- [ライセンス](#ライセンス)

## 概要

数当てゲームの簡易版です。1〜100の範囲でランダムに選ばれた数字を当てます。

### 機能

- 1〜100の範囲で数当てゲームを実行
- 入力値のバリデーション（範囲外・数値以外はエラーメッセージを表示）
- 大小のヒント表示（大阪弁）
- Ctrl-C による中断時のメッセージ表示・正常終了

### オリジナル版との違い

| 項目 | guessing_game | guessing_game_simple |
| --- | --- | --- |
| API | `run_game()` + 内部ジェネリック関数 | `run_game()` のみ |
| テスト容易性 | 高い（内部関数でモック可能） | 低い（純粋関数のみテスト可能） |
| シンプルさ | やや複雑 | シンプル |

このバージョンでは、`run_game()` 関数内で標準入出力を直接使用するため、ゲームループ自体のテストは困難です。代わりに、ロジックを純粋関数（`parse_guess`, `get_hint`, `format_hint_message`）に分離してテスト可能にしています。

## プロジェクト構造

```text
guessing_game_simple/
├── Cargo.toml       # プロジェクト設定
├── README.md        # このファイル
└── src/
    ├── main.rs      # エントリーポイント
    └── lib.rs       # ゲームロジック
```

## 使い方

### ビルド

```bash
cargo build
```

### 実行

```bash
cargo run
```

## 開発

### コードチェック (Clippy)

```bash
cargo clippy -- -W clippy::pedantic -D warnings
```

### テスト

```bash
cargo test
```

## API

### `run_game`

```rust
pub fn run_game() -> io::Result<()>
```

ゲームのメインロジック。標準入出力を直接使用。

### `parse_guess`

```rust
#[must_use]
pub fn parse_guess(input: &str) -> Option<u32>
```

入力文字列を1〜100の範囲の数値に変換。

### `get_hint`

```rust
#[must_use]
pub fn get_hint(guess: u32, secret: u32) -> Ordering
```

予想と秘密の数字を比較。

### `format_hint_message`

```rust
#[must_use]
pub fn format_hint_message(ordering: Ordering) -> &'static str
```

ヒントメッセージを返す。

### `config` モジュール

```rust
pub const MIN_NUMBER: u32 = 1;   // 最小値
pub const MAX_NUMBER: u32 = 100; // 最大値
```

## テスト項目・結果

### テストケース一覧（10テスト）

#### ユニットテスト（6テスト）

| テスト名 | 説明 | 結果 |
| --- | --- | --- |
| `test_parse_guess_valid` | 有効な数値・空白処理 | ✅ OK |
| `test_parse_guess_boundary` | 境界値（0, 1, 100, 101） | ✅ OK |
| `test_parse_guess_invalid` | 無効な入力（負数、文字列、空白のみ、小数、科学的表記） | ✅ OK |
| `test_get_hint` | 比較ロジック（Less/Greater/Equal）+ 境界値 | ✅ OK |
| `test_format_hint_message` | メッセージ生成 | ✅ OK |
| `test_config_values` | 定数値の確認 | ✅ OK |

#### Docテスト（4テスト）

| 関数 | 結果 |
| --- | --- |
| モジュール例 | ✅ OK |
| `parse_guess` | ✅ OK |
| `get_hint` | ✅ OK |
| `format_hint_message` | ✅ OK |

**テスト実行結果:** 10 passed, 0 failed

### Clippy 結果

**結果:** ✅ 警告なし（pedantic レベル）

## ライセンス

[MIT License](../LICENSE)
