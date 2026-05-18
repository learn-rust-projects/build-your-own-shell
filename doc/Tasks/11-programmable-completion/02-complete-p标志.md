---
created: 2026-05-18T15:25
updated: 2026-05-18T15:51
---

## complete -p 标志

在这一阶段，你将为 `complete` 内置命令添加 `-p` 标志支持。

### -p 标志

`-p` 标志打印为给定命令注册的补全规范。

```bash
$ complete -p git
complete -C '/path/to/git/completer' git
```

当没有注册规范时，它以以下格式打印错误消息：

```bash
$ complete -p git
complete: git: no completion specification
```

在这一阶段，你只需要返回错误输出。你还不需要跟踪任何规范，只需识别 `-p` 并为跟随它的任何命令名打印错误消息。

### 测试

测试程序会这样执行你的程序：

```bash
$ ./your_program.sh
```

它会使用随机命令名运行 `complete -p`：

```bash
$ complete -p <command>
complete: <command>: no completion specification
```

测试程序会验证：
- 输出匹配格式 `complete: <command>: no completion specification`
- 输出中的命令名与传递给 `-p` 的命令名匹配
