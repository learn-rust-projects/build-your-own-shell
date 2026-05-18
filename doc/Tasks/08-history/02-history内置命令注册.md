---
created: 2026-05-18T15:25
updated: 2026-05-18T15:48
---

## history 内置命令注册

在这一阶段，你将添加对 [history](https://www.gnu.org/software/bash/manual/html_node/Bash-History-Builtins.html#index-history) 作为 shell 内置命令的支持。

### history 内置命令

[history](https://www.gnu.org/software/bash/manual/html_node/Bash-History-Builtins.html#index-history) 是列出先前执行命令的 shell 内置命令。示例用法：
```bash
$ history
    1  previous_command_1
    2  previous_command_2
    3  history
```

### 测试

测试程序会这样执行你的程序：

```bash
./your_program.sh
```

然后测试程序会执行 `type history` 命令。

```bash
$ type history
history is a shell builtin
$
```

然后测试程序会执行 `type history` 命令并期望输出为 `history is a shell builtin`。
