---
created: 2026-05-18T15:25
updated: 2026-05-18T15:51
---

## 注册 complete 内置命令

在这一阶段，你将把 `complete` 注册为 shell 内置命令。

### complete 内置命令

[`complete`](https://www.gnu.org/software/bash/manual/html_node/Programmable-Completion-Builtins.html#index-complete) 内置命令让用户为命令注册可编程补全。

例如，你可以为 `git` 注册一个补全脚本：

```bash
$ complete -C /path/to/completer_script git
```

一旦注册，当用户在 `git` 后按 TAB 获得建议时，shell 会调用脚本：

```bash
$ git clo<TAB>
$ git clone
```

在这一阶段，你只需要将 `complete` 注册为内置命令，以便 `type` 命令能识别它。我们将在后续阶段实现实际行为。

### 测试

测试程序会这样执行你的程序：

```bash
$ ./your_program.sh
```

然后它会检查 `complete` 是否被识别为内置命令：

```bash
$ type complete
complete is a shell builtin
```

### 注意事项

- 在这一阶段你不需要实现任何补全逻辑。只需注册 `complete` 使其显示为内置命令
- 如果你的 shell 已经使用内置命令列表或映射（如 `echo`、`cd`、`pwd`），你可以将 `complete` 添加到同一结构中
