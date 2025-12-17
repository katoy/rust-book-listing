# フィボナッチ数列 (Functions)

フィボナッチ数列を計算する2つの異なるアルゴリズムの実装と比較です。

## 目次

- [機能](#機能)
- [プロジェクト構造](#プロジェクト構造)
- [使い方](#使い方)
- [開発](#開発)
- [API](#api)
- [テスト項目・結果](#テスト項目結果)
- [ライセンス](#ライセンス)

## 機能

- **再帰版** (`fib`): シンプルな再帰による実装
- **ビネの公式版** (`fib_binet`): 黄金比を使用した O(1) 計算
- 両アルゴリズムの計算速度比較

## プロジェクト構造

```text
functions/
├── Cargo.toml       # プロジェクト設定
├── README.md        # このファイル
└── src/
    └── main.rs      # メインコードとテスト
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

出力例:

```text
再帰版 vs ビネの公式版:
fib( 0) =     0 | fib_binet( 0) =     0 ✓
fib( 1) =     1 | fib_binet( 1) =     1 ✓
...
fib(19) =  4181 | fib_binet(19) =  4181 ✓
```

## 開発

### コードチェック (Clippy)

```bash
cargo clippy
```

### テスト

```bash
cargo test
```

テスト中の `println!` 出力を表示するには:

```bash
cargo test -- --nocapture
```

出力例:

```text
running 7 tests

=== ビネの公式の大きな n での性能 ===
fib_binet(  50) =           4294967295 | 時間: 166ns
fib_binet( 100) =           4294967295 | 時間: 83ns
fib_binet( 500) =           4294967295 | 時間: 83ns
fib_binet(1000) =           4294967295 | 時間: 83ns
test tests::test_binet_large_n_performance ... ok
test tests::test_fib_base_cases ... ok
test tests::test_fib_binet_base_cases ... ok
test tests::test_fib_binet_known_values ... ok
test tests::test_fib_known_values ... ok
test tests::test_fib_and_fib_binet_match ... ok

=== 計算速度比較 (n = 30) ===
fib(30)       =     832040 | 時間: 6.732125ms
fib_binet(30) =     832040 | 時間: 83ns
ビネの公式は再帰版の約 81110 倍高速
test tests::test_performance_comparison ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

> **Note:** デフォルトでは、テスト成功時に stdout 出力はキャプチャされ表示されません。
> `--nocapture` または `--show-output` オプションで出力を確認できます。

## API

### `fib(n: u32) -> u32`

再帰によるフィボナッチ数列計算。

- **時間計算量**: O(2^n)
- **用途**: 教育目的、小さな n 向け

### `fib_binet(n: u32) -> u32`

ビネの公式によるフィボナッチ数列計算。

```text
F(n) = (φ^n - ψ^n) / √5
φ (黄金比) = (1 + √5) / 2 ≈ 1.618
ψ (共役黄金比) = (1 - √5) / 2 ≈ -0.618
```

- **時間計算量**: O(1)
- **注意**: 浮動小数点精度により大きな n では誤差あり

## テスト項目・結果

### テストケース一覧 (7テスト)

| テスト名 | 説明 | 結果 |
| --- | --- | --- |
| `test_fib_base_cases` | `fib(0)=0`, `fib(1)=1` の確認 | ✅ OK |
| `test_fib_known_values` | `fib(2,3,4,5,10,20)` の確認 | ✅ OK |
| `test_fib_binet_base_cases` | `fib_binet(0)=0`, `fib_binet(1)=1` の確認 | ✅ OK |
| `test_fib_binet_known_values` | `fib_binet(2,3,4,5,10,20)` の確認 | ✅ OK |
| `test_fib_and_fib_binet_match` | 0〜24 で両関数が一致することを確認 | ✅ OK |
| `test_performance_comparison` | 計算速度比較（n=30） | ✅ OK |
| `test_binet_large_n_performance` | 大きな n での性能確認 | ✅ OK |

**テスト実行結果:** 7 passed, 0 failed

### 計算速度比較 (n = 30)

| メソッド | 結果 | 実行時間 |
| --- | --- | --- |
| `fib(30)` | 832,040 | 約 6.5ms |
| `fib_binet(30)` | 832,040 | 約 42ns |

> ビネの公式は再帰版の約 **150,000 倍高速**

### Clippy 結果

```text
$ cargo clippy
    Finished `dev` profile [unoptimized + debuginfo] target(s)
```

**結果:** ✅ 警告なし

## ライセンス

[MIT License](../LICENSE)
