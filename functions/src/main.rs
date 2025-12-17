fn main() {
    println!("再帰版 vs ビネの公式版:");
    for n in 0..20 {
        let recursive = fib(n);
        let binet = fib_binet(n);
        let status = if recursive == binet { "✓" } else { "✗" };
        println!("fib({n:2}) = {recursive:5} | fib_binet({n:2}) = {binet:5} {status}");
    }
}

/// フィボナッチ数列の n 番目の値を計算する（再帰版）
fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

/// フィボナッチ数列の n 番目の値を計算する（ビネの公式版）
///
/// ビネの公式: F(n) = (φ^n - ψ^n) / √5
/// - φ (黄金比) = (1 + √5) / 2 ≈ 1.618
/// - ψ (共役黄金比) = (1 - √5) / 2 ≈ -0.618
///
/// 注意: 浮動小数点の精度により、大きな n では誤差が生じる可能性あり
#[allow(
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::manual_midpoint
)]
fn fib_binet(n: u32) -> u32 {
    let sqrt5 = 5.0_f64.sqrt();
    // 黄金比の計算: (1 + √5) / 2 は midpoint ではなく黄金比の定義式
    let phi = (1.0 + sqrt5) / 2.0;
    let psi = (1.0 - sqrt5) / 2.0; // 共役黄金比

    // n は u32 なので i32 への変換は n <= i32::MAX の範囲で安全
    // 結果は常に正の整数なので u32 への変換も安全
    let result = (phi.powi(n as i32) - psi.powi(n as i32)) / sqrt5;
    result.round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    // fib() のテスト
    #[test]
    fn test_fib_base_cases() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
    }

    #[test]
    fn test_fib_known_values() {
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(10), 55);
        assert_eq!(fib(20), 6765);
    }

    // fib_binet() のテスト
    #[test]
    fn test_fib_binet_base_cases() {
        assert_eq!(fib_binet(0), 0);
        assert_eq!(fib_binet(1), 1);
    }

    #[test]
    fn test_fib_binet_known_values() {
        assert_eq!(fib_binet(2), 1);
        assert_eq!(fib_binet(3), 2);
        assert_eq!(fib_binet(4), 3);
        assert_eq!(fib_binet(5), 5);
        assert_eq!(fib_binet(10), 55);
        assert_eq!(fib_binet(20), 6765);
    }

    // 両関数の一致テスト
    #[test]
    fn test_fib_and_fib_binet_match() {
        for n in 0..25 {
            assert_eq!(
                fib(n),
                fib_binet(n),
                "fib({n}) と fib_binet({n}) が一致しない"
            );
        }
    }

    // 計算速度の比較テスト
    #[test]
    fn test_performance_comparison() {
        use std::time::Instant;

        let n = 30; // 再帰版が遅くなる値

        // 再帰版の計測
        let start = Instant::now();
        let result_recursive = fib(n);
        let duration_recursive = start.elapsed();

        // ビネの公式版の計測
        let start = Instant::now();
        let result_binet = fib_binet(n);
        let duration_binet = start.elapsed();

        println!("\n=== 計算速度比較 (n = {n}) ===");
        println!(
            "fib({n})       = {result_recursive:10} | 時間: {:?}",
            duration_recursive
        );
        println!(
            "fib_binet({n}) = {result_binet:10} | 時間: {:?}",
            duration_binet
        );
        println!(
            "ビネの公式は再帰版の約 {:.0} 倍高速",
            duration_recursive.as_nanos() as f64 / duration_binet.as_nanos().max(1) as f64
        );

        // 結果が一致することを確認
        assert_eq!(result_recursive, result_binet);

        // ビネの公式が再帰版より高速であることを確認
        assert!(
            duration_binet < duration_recursive,
            "ビネの公式版が再帰版より遅い（予想外）"
        );
    }

    // 大きな n での速度テスト（ビネの公式のみ）
    #[test]
    fn test_binet_large_n_performance() {
        use std::time::Instant;

        println!("\n=== ビネの公式の大きな n での性能 ===");
        for n in [50, 100, 500, 1000] {
            let start = Instant::now();
            let result = fib_binet(n);
            let duration = start.elapsed();
            println!("fib_binet({n:4}) = {:20} | 時間: {:?}", result, duration);
        }
    }
}
