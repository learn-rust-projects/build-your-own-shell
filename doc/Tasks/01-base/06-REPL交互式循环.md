---
created: 2026-05-18T15:25
updated: 2026-05-18T17:01
view_count: 1
update_count: 1
---

## REPL（读取 - 求值 - 打印循环）

在这一阶段，你将实现一个 [REPL（读取-求值-打印循环）](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop)。

### REPL

[REPL（读取-求值-打印循环）](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop) 是一个交互式循环，构成了 shell 的核心。它遵循一个重复的周期：

1. **读取**：显示提示符并等待用户输入
2. **求值**：解析并执行命令
3. **打印**：显示输出或错误消息
4. **循环**：返回步骤 `1` 并等待下一条命令

这个周期会无限重复，直到 shell 进程被终止。

你的 shell 应该遵循相同的周期：
1. 显示提示符 `$ `，然后等待一行输入
2. 对于用户输入的任何命令，打印 `<command_name>: command not found`
3. 返回步骤 `1`

例如，如果用户输入 `hello`，你的 shell 应该打印 `hello: command not found`，然后再次显示提示符（`$ `）。

### 测试

测试程序会这样执行你的程序：

```bash
./your_program.sh
```

然后它会向你的 shell 发送一系列命令：

```bash
$ invalid_command_1
invalid_command_1: command not found
$ invalid_command_2
invalid_command_2: command not found
$ invalid_command_3
invalid_command_3: command not found
$
```

每次命令后，测试程序会验证你的 shell：
- 打印消息 `<command_name>: command not found`
- 在测试程序发送下一条命令前显示新的提示符（`$ `）

### 注意事项

- 发送的命令数量和命令名称是随机的
- 循环应该无限运行。测试完成后，测试程序会终止你的程序
