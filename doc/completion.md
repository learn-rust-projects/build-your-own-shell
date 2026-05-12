# 自动补全系统设计

## 概述

Tab 补全系统基于 `rustyline` 的 `Completer` trait 实现，结合 `radix_trie` 前缀树和自定义 completion 脚本，支持多层次补全。

## 架构

```text
MyCompleter (rustyline::Completer)
  │
  ├── 补全来源 1: GLOBAL_TRIES (radix_trie::Trie)
  │     ├── 所有内置命令名称 (strum::EnumIter)
  │     └── PATH 中所有可执行文件名称
  │
  ├── 补全来源 2: 文件系统路径
  │     └── 当补全词包含 / 时，读取目录列表
  │
  └── 补全来源 3: 自定义 completion 脚本
        └── 通过 complete -C 注册的外部脚本
```

## 三层补全策略

### 1. 命令名补全（Trie 前缀匹配）

对于行首的命令名，使用 `radix_trie::Trie` 进行前缀匹配查询：

```rust
static GLOBAL_TRIES: LazyLock<Trie<String, ()>> = LazyLock::new(|| {
    // 收集所有内置命令名 + PATH 可执行文件名
    // 构建静态前缀树
});
```

`update()` 方法额外检查补全结果是否为叶子节点（唯一匹配），是则自动追加空格。

### 2. 文件路径补全

当补全词存在空格前缀时（非命令首词），调用 `find_completed_file()`：

```rust
fn find_completed_file(original: &str) -> Result<(Vec<Pair>, usize), ReadlineError> {
    // 分离目录和文件名前缀
    // 扫描目录 → 匹配前缀 → 添加 / 或 空格后缀
    // 排序后返回
}
```

目录项自动追加 `/`，文件项自动追加空格，提升交互体验。

### 3. 自定义补全脚本

通过 `complete -C /path/to/script command_name` 注册补全脚本。当补全触发时，调用 `find_complete_and_executable_file()`：

```rust
fn find_complete_and_executable_file(..., word: &str) -> Option<...> {
    let cmd = std::process::Command::new(path)
        .args([arg1, word, arg2])
        .env("COMP_LINE", env_line)    // 当前完整行
        .env("COMP_POINT", pos)        // 光标位置
        .output()?;
    // 解析脚本输出作为补全建议
}
```

环境变量 `COMP_LINE` 和 `COMP_POINT` 模拟了 bash 的补全接口协议，使现有 bash 补全脚本可以兼容使用。

## 补全决策流程

```text
MyCompleter::complete(line, pos, _ctx)
  │
  ├── 行中有空格?
  │     ├── 是: 尝试自定义补全脚本 → 成功则返回
  │     │          └── 失败: find_completed_file() 文件路径补全
  │     └── 否: GLOBAL_TRIES 前缀匹配命令名
  │
  └── 返回 (起始位置, Vec<Pair>)
```

## rustyline 集成

`MyCompleter` 同时实现了 `Helper`、`Hinter`、`Highlighter`、`Validator` 四个 rustyline trait：

| Trait         | 实现                      |
| ------------- | ------------------------- |
| `Completer`   | 核心补全逻辑              |
| `Helper`      | 空实现（标记组合）        |
| `Hinter`      | 返回 `None`（不提供提示） |
| `Highlighter` | 空实现                    |
| `Validator`   | 始终返回 `Valid(None)`    |
