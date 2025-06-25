```shell
安装链接器
brew install michaeleisel/zld/zld
安装cargo-watch
cargo install cargo-watch
监听源代码
cargo watch -x check
cargo watch -x check -x test -x run
代码覆盖率
cargo install cargo-tarpaulin
安装rust提示器clippy
rustup component add clippy
静态分析
cargo clippy
cargo clippy -- -D warning
安装rust格式器
rustup component add rustfmt
格式化所有代码
cargo fmt
cargo fmt -- --check
安装依赖安全检测
cargo install cargo-audit

cargo audit

cargo install cargo-deny

安装web框架
cargo add actix-web@4.0.0

安装宏调试
cargo install cargo-expand

rustup toolchain install nightly --allow-downgrade

cargo expand
或
cargo +nightly expand

curl -v http://127.0.0.1:8080/health_check
```