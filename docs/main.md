# Rust bindgen
C/C++ の Rust ラッパーを作成できるライブラリ。   

## Prerequisites

## How to use
1. 対象とするコードを作成する。
1. `Cargo.toml` の `[build-dependencies]` に `bindgen` の使用宣言を追加する。
    * `*.c/cpp` ファイルをインクルードする場合は `cc` の使用宣言も必要か。
1. `build.rs` を作成する。
1. `cargo build` を実行する。


## 参考
* https://docs.rs/bindgen/0.53.2/bindgen/
    * Docs.rs
* https://github.com/rust-lang/rust-bindgen
    * GitHub
* https://docs.rs/cc/1.0.53/cc/
    * cc の Docs.rs
* https://qiita.com/moriai/items/e8e8b9c6a12f5a529d85
    * bindgen を使用した Qiita の記事
* https://doc.rust-lang.org/cargo/reference/build-scripts.html
    * `build.rs` に関する情報が掲載されている。