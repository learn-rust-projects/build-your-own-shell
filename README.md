# Codecrafters Shell

基于 Rust 实现的 POSIX 兼容 Shell，是 [Codecrafters "Build Your Own Shell"](https://app.codecrafters.io/courses/shell) 挑战的完整实现。

## 功能特性

- **REPL** — 交互式读取-求值-打印循环，`$` 提示符
- **词法分析** — 5 状态有限状态机，支持单引号、双引号、转义、管道、重定向、后台标记
- **语法解析** — 解析为 `CommandGroup` / `Command` / `Redirection` 层级结构，支持管道链
- **命令执行** — 内置命令与外部命令统一执行，支持 I/O 上下文传递
- **9 个内置命令**：

  | 命令       | 功能                                           |
  | ---------- | ---------------------------------------------- |
  | `exit`     | 退出 shell，持久化历史到 `HISTFILE`            |
  | `echo`     | 输出文本                                       |
  | `pwd`      | 显示当前工作目录                               |
  | `cd`       | 切换目录（支持 `~`）                           |
  | `type`     | 显示命令类型（内置/外部及路径）                |
  | `history`  | 管理命令历史（`-r`/`-w`/`-a`，数量限制）       |
  | `jobs`     | 列出后台作业（`Running`/`Done`，`+`/`-` 标记） |
  | `complete` | 注册/查询/移除自定义补全脚本（`-p`/`-C`/`-r`） |
  | `declare`  | 设置/查询 shell 变量（`-p`、`key=value`）      |

- **I/O 重定向** — `>` / `>>` / `<` / `<<` / `>&` / `<&`
- **管道** — `|` 连接多命令，通过 `libc::pipe()` 系统调用实现
- **后台作业** — `&` 后台运行，支持作业状态追踪与 bash 兼容格式输出
- **自动补全** — Tab 补全（内置命令 + PATH 可执行文件 + 文件路径 + 自定义补全脚本）
- **命令历史** — 基于 rustyline 的持久化历史，支持 `HISTFILE`
- **变量展开** — `${VAR}` / `$VAR` 替换（基于 `declare` 设置的变量）
- **信号处理** — `Ctrl+C` (Interrupted)、`Ctrl+D` (Eof)

## 架构设计

```text
输入行
  │
  ▼
┌──────────────────────────────────────────────────────┐
│  main.rs (REPL 循环 / 全局状态管理)                   │
│  GLOBAL_VEC, HOME_DIR, GLOBAL_EDITOR, GLOBAL_JOB     │
│  GLOBAL_COMPLETION_MANAGER, GLOBAL_COMPLETION_DECLARE │
└──────────┬───────────────────────────────────────────┘
           │ parse_and_handle_line()
           ▼
┌──────────────────────┐     RawToken       ┌──────────────────────┐
│  lexer.rs            │ ──────────────────► │  parse.rs            │
│  5 状态有限状态机     │   tokenize_line()  │  解析为 Command       │
│  Normal/SingleQuote/  │                    │  + CommandGroup      │
│  DoubleQuote/Escaping/│                    │  + Redirection       │
│  DoubleQuoteEscaping  │                    │  expand() 变量展开   │
└──────────────────────┘                     └──────────┬───────────┘
                                                        │
              ┌─────────────────────────────────────────┘
              ▼
   ┌──────────────────────┐    策略模式      ┌──────────────────────┐
   │  executor/mod.rs     │ ──────────────►  │  CommandHandler      │
   │  CommandHandlerFactory│                  │  trait 的两个实现:    │
   │  (内置 vs 外部判断)   │                  │  BuiltinCommandHandler│
   └──────────────────────┘                  │  ExternalCommandHandler│
                                             └──────────────────────┘
                                                        │
                               ┌────────────────────────┤
                               ▼                        ▼
                    ┌────────────────────┐    ┌────────────────────┐
                    │ BuiltinCommandHandler│   │ExternalCommandHandler│
                    │ BuiltinFactory →    │    │ std::process::Command│
                    │ 9 个 Builtin 实现    │    │ PATH 查找 + spawn() │
                    └────────────────────┘    └────────────────────┘
```

## 文件结构

```text
src/
├── main.rs                   # 入口，REPL 循环，全局静态变量
├── lexer.rs                  # 词法分析器（5 状态 FSM）
├── parse.rs                  # 解析器 + 执行上下文 + 管道执行 + 重定向 + 变量展开
├── auto_completion.rs        # Tab 补全（radix_trie + rustyline）
├── history.rs                # 历史文件管理（读/写/追加）
├── utils.rs                  # PATH 可执行文件查找
├── builtin_commands/
│   ├── mod.rs                # Builtin trait + BuiltinFactory
│   ├── echo_command.rs       # echo
│   ├── cd_command.rs         # cd
│   ├── pwd_command.rs        # pwd
│   ├── type_command.rs       # type
│   ├── exit_command.rs       # exit
│   ├── history_command.rs    # history
│   ├── jobs_command.rs       # jobs + JobList
│   ├── complete_command.rs   # complete
│   └── declare_command.rs    # declare
└── executor/
    ├── mod.rs                # CommandHandler trait + 工厂
    ├── builtin_command_handler.rs  # 内置命令执行器
    └── external_command_handler.rs # 外部命令执行器
```

## 使用方法

```bash
# 编译
cargo build --release

# 运行
cargo run --release

# 使用示例
$ echo "Hello, World!"
Hello, World!

$ pwd
/home/user/projects

$ cd /tmp && pwd
/tmp

$ ls -la | grep src | wc -l

$ echo hello > output.txt && cat output.txt

$ sleep 10 &
[1]  Running                 sleep 10 &

$ type echo
echo is a shell builtin
$ type cargo
cargo is /usr/bin/cargo

$ declare MY_VAR=hello && echo $MY_VAR
hello

$ history
    1  echo hello
    2  pwd
    3  ls -la
```

## 设计文档

详见 [doc/](doc/) 目录：

| 文档                                             | 内容                                               |
| ------------------------------------------------ | -------------------------------------------------- |
| [doc/architecture.md](doc/architecture.md)       | 整体架构与数据流、模块职责、全局状态管理           |
| [doc/lexer.md](doc/lexer.md)                     | 5 状态有限状态机、引号/转义/重定向处理             |
| [doc/parser-executor.md](doc/parser-executor.md) | 解析策略、管道执行、重定向系统、变量展开、策略模式 |
| [doc/job-control.md](doc/job-control.md)         | 后台作业管理、JobList 设计、作业状态追踪           |
| [doc/completion.md](doc/completion.md)           | Tab 补全、radix_trie、文件路径补全、自定义补全脚本 |

## 许可证

MIT License
