---
created: 2026-05-18T15:25
updated: 2026-05-18T17:00
view_count: 1
update_count: 1
---

## Type 内置命令

在这一阶段，你将为你的 shell 实现 `type` 内置命令。

### Type 内置命令

[`type`](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/type.html) 内置命令用于确定一个命令将被如何解释。它会检查一个命令是内置命令、可执行文件还是无法识别的命令。

例如：

```bash
$ type echo
echo is a shell builtin
$ type exit
exit is a shell builtin
$ type invalid_command
invalid_command: not found
```

在这一阶段，你需要处理两种情况：
- 对于内置命令（如 `echo`、`exit`、`type`），打印 `<command> is a shell builtin`
- 对于无法识别的命令（不匹配任何内置命令），打印 `<command>: not found`

我们将在后续阶段处理可执行文件。

### 测试

测试程序会这样执行你的程序：

```bash
./your_program.sh
```

然后它会向你的 shell 发送一系列 `type` 命令：

```bash
$ type echo
echo is a shell builtin
$ type exit
exit is a shell builtin
$ type type
type is a shell builtin
$ type invalid_command
invalid_command: not found
$
```

测试程序会验证：
- 内置命令打印：`<command> is a shell builtin`
- 无法识别的命令打印：`<command>: not found`

### 注意事项

- 测试程序在这一阶段只检查内置命令和无法识别的命令。
- `type` 本身是一个 shell 内置命令，所以 `$ type type` 应该打印 `type is a shell builtin`。
