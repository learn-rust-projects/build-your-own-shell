---
created: 2026-05-18T15:25
updated: 2026-05-18T15:47
---

## 注册 jobs 内置命令

在这一阶段，你将注册 `jobs` 内置命令。

### 后台作业

通常，当你在 shell 中运行命令时，shell 会等待命令完成后才再次显示提示符。这称为在前台运行命令。

但是，你也可以通过在末尾添加 `&` 来在后台运行命令。后台命令在不阻塞 shell 的情况下运行，所以你可以在命令执行时继续输入其他命令。

例如：
```bash
$ sleep 10 &
[1] 12345
$ echo "I can run this immediately"
I can run this immediately
$
```

Shell 为每个后台命令分配一个作业号（如 `[1]`）和一个进程 ID（如 `12345`）。

### jobs 内置命令

[`jobs`](https://www.man7.org/linux/man-pages/man1/jobs.1p.html) 内置命令列出当前 shell 已知的所有后台作业。它显示它们的作业号、状态（如 `Running` 或 `Done`）以及命令本身。

在这一阶段，你将把 `jobs` 注册为内置命令但提供空实现。当没有后台作业时，`jobs` 命令应该不产生任何输出，直接返回到提示符。

例如：
```bash
$ type jobs
jobs is a shell builtin
$ jobs
$
```

### 测试

测试程序会这样执行你的程序：
```bash
$ ./your_program.sh
```

然后它会验证 `jobs` 是否注册为内置命令：
```bash
$ type jobs
# 预期输出
jobs is a shell builtin

# 预期空输出
$ jobs
$
```

测试程序会验证：
- `type jobs` 报告它是一个 shell 内置命令
- `jobs` 命令调用时不产生输出

### 注意事项

- `jobs` 的实际实现（列出正在运行的后台作业）将在后续阶段介绍。目前，只需将其注册为具有空实现的内置命令。
