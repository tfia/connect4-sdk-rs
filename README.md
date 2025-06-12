# Connect 4 SDK for Rust

清华大学计算机系《人工智能导论》四子棋大作业的 **非官方** Rust 语言 SDK，能够同时支持 Saiblo 平台在线评测和祖传本地测试框架。

# 如何使用

你需要实现 `strategy` 模块中的 `get_point` 函数。该函数的签名和意义与官方 Rust SDK 完全相同，返回你的下一步落子点。

若想使用本地测试，则运行

``` bash
cargo build --lib --release
```

这会在 `target/release` 下生成 `libstrategy.so`。该动态库导出了 `getPoint` 和 `clearPoint` 两个函数，签名和意义与官方 C++ SDK 完全相同且兼容 C ABI，因此可以像对 C++ 编写的 AI 进行本地测试那样，使用课程组祖传的本地测试框架进行测试。

Saiblo 上的默认编译命令为

``` bash
cargo build --offline -j1 --release
```

这会同时编译动态库目标 `strategy` 和二进制目标 `main`，后者是可以被平台正常评测的可执行文件。Saiblo 会忽略除了 `main` 以外的编译产物。