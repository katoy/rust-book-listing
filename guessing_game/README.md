# 数当てゲーム (Guessing Game)

Rust Book のチュートリアルに基づいた数当てゲームの実装です。

## 目次

- [数当てゲーム (Guessing Game)](#数当てゲーム-guessing-game)
  - [目次](#目次)
  - [機能](#機能)
  - [プロジェクト構造](#プロジェクト構造)
  - [使い方](#使い方)
    - [ビルド](#ビルド)
    - [実行](#実行)
  - [開発](#開発)
    - [コードチェック (Clippy)](#コードチェック-clippy)
    - [テスト](#テスト)
    - [カバレッジ計測](#カバレッジ計測)
  - [API](#api)
  - [テスト項目・結果](#テスト項目結果)
    - [テストケース一覧](#テストケース一覧)
    - [Clippy 結果](#clippy-結果)
    - [カバレッジ結果](#カバレッジ結果)
  - [ライセンス](#ライセンス)

## 機能

- 1〜100の範囲で数当てゲームを実行
- ユーザーからの数値入力を受け付け
- 入力値のバリデーション（範囲外・数値以外はエラーメッセージを表示）
- 大小のヒント表示（「もっと大きいで！」「もっと小さいで！」）
- I/O エラーハンドリング（`Result` 型による適切なエラー処理）
- 大阪弁による親しみやすいメッセージ

## プロジェクト構造

```text
guessing_game/
├── Cargo.toml       # プロジェクト設定
├── LICENSE          # MIT ライセンス
├── README.md        # このファイル
├── src/
│   ├── main.rs      # エントリーポイント
│   └── lib.rs       # ゲームロジック
└── tests/
    └── test_lib.rs  # 統合テスト
```

## 使い方

### ビルド

```bash
cargo build
```

リリースビルド:

```bash
cargo build --release
```

### 実行

```bash
cargo run
```

## 開発

### コードチェック (Clippy)

Clippy を使用してコードの品質をチェックします:

```bash
cargo clippy -- -W clippy::all
```

すべての警告をエラーとして扱う場合:

```bash
cargo clippy -- -D warnings
```

### テスト

統合テストを実行:

```bash
cargo test
```

詳細な出力付きで実行:

```bash
cargo test -- --nocapture
```

### カバレッジ計測

[cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) を使用してコードカバレッジを計測します。

**インストール:**

```bash
cargo install cargo-llvm-cov
```

**サマリーのみ表示:**

```bash
cargo llvm-cov --summary-only
```

**詳細表示（未実行行を確認）:**

```bash
cargo llvm-cov --text
```

**main.rs を除外して計測（推奨）:**

`main.rs` はエントリーポイントでテストから呼び出せないため、除外して計測することを推奨します:

```bash
cargo llvm-cov --summary-only --ignore-filename-regex 'main\.rs'
```

**HTML レポート生成:**

```bash
cargo llvm-cov --html
open target/llvm-cov/html/index.html
```

## API

### `run_game`

```rust
pub fn run_game<R: BufRead, W: Write>(reader: &mut R, writer: &mut W) -> io::Result<()>
```

ゲームのメインロジック。1〜100のランダムな秘密の数字を生成し、ユーザーに当てさせる。

### `run_game_with_secret`

```rust
pub fn run_game_with_secret<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    secret_number: u32,
) -> io::Result<()>
```

秘密の数字を指定してゲームを実行する。テスト用に公開されている。

### `parse_guess`

```rust
pub fn parse_guess(input: &str) -> Option<u32>
```

入力文字列を数値に変換する。1〜100の範囲内の有効な数値の場合は `Some(u32)` を返し、それ以外は `None` を返す。

## テスト項目・結果

### テストケース一覧

#### parse_guess テスト

| テスト名 | 説明 | 結果 |
| --- | --- | --- |
| `test_parse_guess_valid_number` | 有効な数値（42, 1, 100）のパース | ✅ OK |
| `test_parse_guess_with_whitespace` | 空白・改行を含む入力のトリム処理 | ✅ OK |
| `test_parse_guess_invalid_input` | 無効な入力（文字列、空、小数）の処理 | ✅ OK |
| `test_parse_guess_negative_number` | 負の数（u32 なので None）の処理 | ✅ OK |

#### run_game テスト

| テスト名 | 説明 | 結果 |
| --- | --- | --- |
| `test_run_game_valid_input` | 正常な数値入力時のゲームフロー | ✅ OK |
| `test_run_game_invalid_input` | 無効な入力時のエラーメッセージ表示 | ✅ OK |

#### run_game_with_secret テスト（全パスカバー）

| テスト名 | 説明 | 結果 |
| --- | --- | --- |
| `test_run_game_with_secret_guess_too_small` | 予想が小さい場合 → 「もっと大きいで！」 | ✅ OK |
| `test_run_game_with_secret_guess_too_big` | 予想が大きい場合 → 「もっと小さいで！」 | ✅ OK |
| `test_run_game_with_secret_correct_guess` | 正解時 → 「正解や！やったな！」 | ✅ OK |

#### I/O エラーハンドリングテスト

| テスト名 | 説明 | 結果 |
| --- | --- | --- |
| `test_run_game_write_error_first_writeln` | 最初の writeln でのエラー処理 | ✅ OK |
| `test_run_game_write_error_second_writeln` | 2番目の writeln でのエラー処理 | ✅ OK |
| `test_run_game_read_error` | read_line でのエラー処理 | ✅ OK |
| `test_run_game_write_error_after_read` | 結果表示（有効入力）でのエラー処理 | ✅ OK |
| `test_run_game_write_error_on_invalid_input_message` | 結果表示（無効入力）でのエラー処理 | ✅ OK |
| `test_run_game_with_secret_write_error_on_less` | Less時のエラー処理 | ✅ OK |
| `test_run_game_with_secret_write_error_on_greater` | Greater時のエラー処理 | ✅ OK |
| `test_run_game_with_secret_write_error_on_equal` | Equal時のエラー処理 | ✅ OK |

**テスト実行結果:** 17 passed, 0 failed

### Clippy 結果

```text
$ cargo clippy -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s)
```

**結果:** ✅ 警告なし

### カバレッジ結果

```text
$ cargo llvm-cov --summary-only

Filename   Regions  Cover    Functions  Executed  Lines   Cover
----------------------------------------------------------------
lib.rs          69  88.41%           4   100.00%     37  100.00%
main.rs         14   0.00%           1     0.00%      8    0.00%
----------------------------------------------------------------
TOTAL           83  73.49%           5    80.00%     45   82.22%
```

> **Note:** `main.rs` が 0% なのは、`cargo test` ではバイナリの `main()` 関数が実行されないためです
> （一般的な動作）。

**main.rs を除外した場合:**

```text
$ cargo llvm-cov --summary-only --ignore-filename-regex 'main\.rs'

Filename   Regions  Cover    Functions  Executed  Lines   Cover
----------------------------------------------------------------
lib.rs          69  88.41%           4   100.00%     37  100.00%
----------------------------------------------------------------
TOTAL           69  88.41%           4   100.00%     37  100.00%
```

> **Note:** Lines カバレッジは **100%** です。Regions が 100% でないのは、`writeln!` マクロの
> 内部実装詳細（llvm-cov のカウント方法）に起因し、実用上は問題ありません。

## ライセンス

[MIT License](../LICENSE)
