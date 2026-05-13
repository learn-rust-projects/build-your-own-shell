# 架构设计

## 整体架构

本 Shell 采用**流水线架构**，数据依次流经词法分析 → 语法解析 → 命令执行三个阶段，由 `main.rs` 中的 `parse_and_handle_line()` 函数编排。

### 数据流

```text
┌────────────────────────────────────────────────────────────────────────┐
│  main.rs (编排层)                                                      │
│  REPL 循环 → parse_and_handle_line()                                   │
└────────┬───────────────────────────────────────────────────────────────┘
         │ 用户输入行
         ▼
┌────────────────────────────────────────────────────────────────────────┐
│  阶段 1: 词法分析 (lexer.rs)                                           │
│  输入: &str                                                           │
│  输出: Vec<RawToken>                                                  │
│  算法: 5 状态有限状态机                                                │
│  功能: 分词、处理引号/转义、识别管道|、后台&、重定向操作符              │
└────────┬───────────────────────────────────────────────────────────────┘
         │ Vec<RawToken>
         ▼
┌────────────────────────────────────────────────────────────────────────┐
│  阶段 2: 语法解析 (parse.rs)                                           │
│  输入: &[RawToken]                                                    │
│  输出: Vec<CommandGroup>                                              │
│  功能: 解析简单命令、管道分组、重定向解析、$VAR 变量展开                │
└────────┬───────────────────────────────────────────────────────────────┘
         │ Vec<CommandGroup>
         ▼
┌────────────────────────────────────────────────────────────────────────┐
│  阶段 3: 命令执行 (executor/ + parse.rs)                               │
│  策略模式: CommandHandlerFactory → BuiltinCommandHandler               │
│                                → ExternalCommandHandler               │
│  管道执行: libc::pipe() 连接多个子进程                                  │
│  重定向: ExecutionContext 文件描述符替换                                 │
└────────────────────────────────────────────────────────────────────────┘
```

## 全局状态管理

项目使用 `std::sync::LazyLock`（Rust 1.80+ 稳定化）管理全局静态变量：

```rust
// src/main.rs
GLOBAL_VEC: LazyLock<Vec<PathBuf>>               // 缓存 PATH 目录
HOME_DIR: LazyLock<String>                       // 用户 home 目录
GLOBAL_EDITOR: LazyLock<Mutex<Editor<...>>>      // rustyline 编辑器（互斥访问）
GLOBAL_JOB: LazyLock<Mutex<JobList>>             // 后台作业列表
GLOBAL_COMPLETION_MANAGER: LazyLock<Mutex<...>>  // 自定义补全注册表
GLOBAL_COMPLETION_DECLARE: LazyLock<Mutex<...>>  // declare 变量存储
```

设计考量：

- `Editor` 需要 `Mutex` 包裹，因为 rustyline 不是 `Send` 的，需要在 REPL 循环中跨闭包使用
- `GLOBAL_VEC` 是只读的，不需要互斥
- 作业列表和补全管理器需要在多个内置命令之间共享状态

## 模块职责矩阵

| 模块                     | 职责                                           | 关键类型                                                     |
| ------------------------ | ---------------------------------------------- | ------------------------------------------------------------ |
| `main.rs`                | REPL 循环、全局状态、编排                      | -                                                            |
| `lexer.rs`               | 词法分析                                       | `RawToken`, `RedirectOp`, `LexerState`                       |
| `parse.rs`               | 解析、执行上下文、管道、重定向、变量展开        | `Command`, `CommandGroup`, `Redirection`, `ExecutionContext` |
| `executor/mod.rs`        | 命令执行策略、CommandHandler trait、工厂      | `CommandHandler` trait, `CommandResult`, `CommandHandlerFactory` |
| `executor/pipe_handler.rs` | 管道执行、单命令执行                          | `execute_pipeline()`, `excuete_single_command()`             |
| `executor/builtin_command_handler.rs` | 内置命令执行器                    | `BuiltinCommandHandler`                                      |
| `executor/external_command_handler.rs` | 外部命令执行器                    | `ExternalCommandHandler`                                     |
| `executor/prelude.rs`    | 执行器模块公共导入                            | `Write`, `Stdio`, `Context`, `ExecutionContext`              |
| `builtin_commands/`      | 9 个内置命令实现                               | `Builtin` trait, `BuiltinCommand` enum, `BuiltinFactory`     |
| `auto_completion.rs`     | Tab 补全                                       | `MyCompleter` (rustyline `Completer`)                        |
| `history.rs`             | 历史管理                                       | `handle_history_options()`, `read_history_file()`             |
| `utils.rs`               | PATH 查找                                      | `find_executable_file_in_paths()`, `find_all_executable_file_in_paths()` |

## 关键技术选型

| 选型                          | 理由                                                                        |
| ----------------------------- | --------------------------------------------------------------------------- |
| `libc::pipe()` 而非 `os_pipe` | 更接近 POSIX 语义，与 `from_raw_fd` 配合                                    |
| `LazyLock` 而非 `once_cell`   | Rust 1.80+ 标准库原生支持                                                   |
| `strum` 枚举派生              | 零成本抽象，`BuiltinCommand` 枚举自动派生 `Display`/`EnumString`/`EnumIter` |
| `radix_trie`                  | 前缀树查询 O(n) 复杂度，适合 Tab 补全场景                                   |
| `regex`                       | `$VAR` / `${VAR}` 展开使用正则替换                                          |
