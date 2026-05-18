---
created: 2026-05-18T15:25
updated: 2026-05-18T16:59
view_count: 1
---

## 退出内置命令

在这一阶段，你将实现 `exit` 内置命令。

### exit 内置命令

[`exit`](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#exit) 内置命令是一个特殊命令，用于终止 shell。

当你的 shell 收到 `exit` 命令时，应该立即终止。

### 测试

测试程序会这样执行你的程序：

```bash
./your_program.sh
```

然后它会向你的 shell 发送一个无效命令，然后是 `exit` 命令：

```bash
$ invalid_command_1
invalid_command_1: command not found
$ exit
```

测试程序会验证你的 shell 在收到 `exit` 命令后是否正确终止。
