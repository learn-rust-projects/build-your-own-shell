---
created: 2026-05-18T15:25
updated: 2026-05-18T15:50
---

## 注册 declare 内置命令

在这一阶段，你将注册 `declare` 内置命令。

### declare 内置命令

`declare` 内置命令可用于创建和检查 shell 变量。例如：

```bash
$ type declare
declare is a shell builtin
$ declare variable=value
$ declare -p variable
declare -- variable="value"
```

### 测试

测试程序会这样执行你的程序：

```bash
$ ./your_program.sh
```

然后它会验证 `declare` 是否注册为内置命令：

```bash
$ type declare
declare is a shell builtin
```

### 注意事项

- 在这一阶段，你只需要注册 `declare` 内置命令。我们将在后续阶段实现其行为
