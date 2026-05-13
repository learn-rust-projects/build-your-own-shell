# 解析器与执行引擎设计

## 概述

解析器将词法分析产生的 `RawToken` 流转换为结构化的命令表示，执行引擎则负责调度内置/外部命令、管理管道和重定向。

## 两层解析结构

### 解析层级

```text
Vec<CommandGroup>                    // 顶层：一组命令组（由 & 分隔）
  ├── CommandGroup
  │     ├── background: bool         // 是否后台执行
  │     └── commands: Vec<Command>   // 管道中的命令序列
  │           ├── Command            // 单个命令
  │           │     ├── argv: Vec<String>     // 命令名 + 参数
  │           │     └── redirections: Vec<Redirection>  // 重定向列表
  │           │           ├── Redirection { src_fd, op, target }
  │           │           └── ...
  │           └── Command
  └── CommandGroup
```

### 解析算法

```text
token 流: [echo, hello, |, wc, -l, &, sleep, 10]

parse_command():
  遍历 token:
    ├── 遇到 Pipe → 保存当前 command，清空 current_tokens
    ├── 遇到 Background → 保存当前 command，创建 CommandGroup { background: true }
    └── 其他 token → 加入 current_tokens
  → [ CommandGroup { commands: [echo hello, wc -l], background: true },
       CommandGroup { commands: [sleep 10], background: false } ]
```

## 命令执行策略模式

`executor/mod.rs` 使用**策略模式**分离内置命令和外部命令的执行路径：

```text
CommandHandlerFactory::create_handler(command)
         │
         ├── command.parse::<BuiltinCommand>() 成功
         │     └── Box::new(BuiltinCommandHandler)
         │           └── BuiltinFactory::create_command() → 9 个内置实现
         │
         └── 失败
               └── Box::new(ExternalCommandHandler)
                     └── utils::find_executable_file_in_paths() → PATH 查找
                     └── std::process::Command::spawn()
```

### `ExecutionContext` — I/O 上下文传递

`ExecutionContext` 是整个执行过程中的核心上下文对象，携带当前命令的 stdin/stdout/stderr 文件描述符：

```rust
pub struct ExecutionContext {
    pub stdin: Option<File>,    // 当前标准输入
    pub stdout: Option<File>,   // 当前标准输出
    pub stderr: Option<File>,   // 当前标准错误
    pub background: bool,       // 是否后台执行
    pub job: Option<usize>,     // 关联的后台作业 ID
}
```

初始创建时，通过 `libc::dup()` 复制当前进程的 fd 0/1/2 到 `File` 包装中。重定向和管道操作通过替换这些 `Option<File>` 实现 I/O 流重定向。

## 管道执行

`execute_pipeline()` 位于 `executor/pipe_handler.rs`，实现了 N 个命令的管道连接：

> **模块划分说明**：管道执行逻辑从 `parse.rs` 抽取到独立的 `pipe_handler.rs` 模块，实现关注点分离。

```text
输入: [cmd1, cmd2, cmd3]

迭代过程:
  第 0 轮 (cmd1):
    libc::pipe() → [read_fd, write_fd]
    context.stdout = write_fd
    执行 cmd1 (写入 write_fd)
    context.stdin = read_fd (下一轮读取)
    context.stdout = dup(1) (恢复标准输出)

  第 1 轮 (cmd2):
    libc::pipe() → [read_fd, write_fd]
    context.stdout = write_fd
    执行 cmd2 (从 stdin 读，写入 write_fd)
    context.stdin = read_fd

  第 2 轮 (cmd3, 最后一轮):
    执行 cmd3 (从 stdin 读，写入原始 stdout)
    context.stdout 未替换

等待所有子进程退出，返回最后一个命令的退出码
```

注意：管道创建使用了 libc 的原始 FFI 调用，辅以 `File::from_raw_fd()` 包装为安全的 Rust `File` 对象。

## 重定向系统

`apply_redirections()` 按顺序处理命令的每个重定向：

| 操作符    | 默认 fd     | 行为                                         |
| --------- | ----------- | -------------------------------------------- |
| `>` file  | 1 (stdout)  | 创建/截断文件，替换 context.stdout           |
| `>>` file | 1 (stdout)  | 创建/追加文件，替换 context.stdout           |
| `<` file  | 0 (stdin)   | 打开文件，替换 context.stdin                 |
| `>&` fd   | 根据 src_fd | 复制文件描述符（如 `2>&1`: stderr → stdout） |
| `<&` fd   | 根据 src_fd | 复制输入文件描述符                           |
| `<<`      | 0 (stdin)   | heredoc（已解析，内容传递暂未实现）          |

### `DupOut` / `DupIn` 的特殊处理

`2>&1` 等操作通过 `ExecutionContext` 字段的 `take()` 转移实现：

```rust
// src/parse.rs:254-260
if src_fd == 2 && *target_fd == 1 {
    // stderr 重定向到 stdout
    context.stderr = context.stdout.take();
} else if src_fd == 1 && *target_fd == 2 {
    // stdout 重定向到 stderr
    context.stdout = context.stderr.take();
}
```

这种设计利用 Rust 的 `Option::take()` 方法，安全地转移 `File` 所有权，避免重复打开文件描述符。

## 变量展开

`expand()` 函数使用正则表达式实现 `$VAR` 和 `${VAR}` 替换：

```rust
let re = Regex::new(r"\$\{([a-zA-Z_][a-zA-Z0-9_]*)\}|\$([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
```

展开来源是 `GLOBAL_COMPLETION_DECLARE`（由 `declare` 内置命令维护的键值存储），而非系统环境变量。这是一个有意的设计选择——shell 变量与环境变量分离。
