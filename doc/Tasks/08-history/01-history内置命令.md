---
created: 2026-05-18T15:25
updated: 2026-05-18T15:48
---

## history 内置命令

在这一阶段，你将实现 `history` 内置命令。

### 测试

测试程序会这样执行你的程序：

```bash
./your_program.sh
```

然后它会向你的 shell 发送多个命令，然后是 `history` 命令：

```bash
$ echo hello
hello
$ echo world
world
$ invalid_command
invalid_command: command not found
$ history
    1  echo hello
    2  echo world
    3  invalid_command
    4  history
$
```

测试程序期望一个历史列表，包含已执行的命令，格式和索引如上例所示。

### 注意事项

- 一些 shell 如 zsh 不会将 `history` 命令添加到历史列表中，但测试程序期望它存在
- 对于此扩展，在内存中存储历史记录就足够了。跨会话的历史记录持久化将在单独的扩展中实现
