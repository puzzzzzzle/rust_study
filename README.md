# Rust 学习实验

这里按主题保存可独立运行的练习。根目录的 Cargo workspace 只包含
Rust 日常练习；C/C++、Python 互操作示例放在独立的 `interop/` workspace 中，
需要时单独打开或运行，并安装对应的构建环境。本仓库的 VS Code 配置
会让 rust-analyzer 默认忽略 `interop/`；单独打开 `interop/` 即可分析这些示例。
工具链版本见 `rust-toolchain.toml`。

| 目录 / 包 | 内容 | 运行方式 |
| --- | --- | --- |
| `rust_language` | 基础语法、所有权、集合、错误处理、异步等 | `cargo test -p rust_language --all-targets` |
| `module_test/guess_game`、`module_test/hello_mod` | crate、模块与跨包依赖 | `cargo test -p guess_game -p hello_mod` |
| `declarative_macro` | 声明宏 | `cargo test -p declarative_macro --all-targets` |
| `proc_macros`、`proc_macros_trait` | 过程宏及配套 trait | `cargo test -p proc_macros --all-targets` |
| `common` | 日志初始化和 benchmark | `cargo test -p common`；`cargo bench -p common` |
| `interop/c/*` | C、C++ FFI；`bindgen`、`autocxx` 需要 Clang / libclang，C++ 示例需要 C++ 编译器 | `cargo test --manifest-path interop/Cargo.toml -p rust_call_c_auto` 等 |
| `interop/python/py_astar` | PyO3 + 寻路，另有 Python 包和测试 | `cargo test --manifest-path interop/Cargo.toml -p py_astar` |

常用命令：

```sh
cargo test                       # 主 workspace：不含 FFI
cargo check --workspace --all-targets
cargo test -p rust_language --example basic_types
cargo run -p rust_language --example async_await_test
```

`rust_language/examples/control.rs` 目前仅是占位入口。
需要检查互操作实验时，指定另一个 workspace：

```sh
cargo test --manifest-path interop/Cargo.toml --workspace --all-targets
cargo run --manifest-path interop/Cargo.toml -p use_cxx_lib
```

Python 扩展除了 Rust 测试外，还可在 `interop/python/py_astar` 中运行
`uv sync`、`uv run maturin develop` 和 `uv run python -m pytest tests/test.py`；
此示例要求 Python 3.11+ 和 `uv` / `maturin`。

## 新增实验

- 单个 API 或语法练习：放进相关包的 `examples/`，用 `cargo run -p <包> --example <名称>` 运行；测试可写在示例的 `#[test]` 中。
- 需要自己的依赖、配置或测试集：新建独立包，并在对应 workspace 的 `Cargo.toml` 中登记。
- 依赖其他语言或其系统库的实验放在 `interop/c/` 或 `interop/python/`，不要加入根 workspace。

两个 workspace 各自共享一个 `Cargo.lock`。
测试尽量使用临时目录和显式传参，避免修改进程当前目录或重复初始化全局 logger。
