---
created: 2026-05-18T15:25
updated: 2026-05-18T15:50
---

## declare -p 标志

在这一阶段，你将实现 declare 内置命令的 `-p` 标志，当请求的变量不存在时。

### -p 标志

`declare -p NAME` 打印变量 `NAME` 的描述。如果 shell 的变量存储中不存在这样的变量，shell 会打印错误。

```bash
$ declare -p variable
declare: variable: not found
```

### 测试

测试程序会这样执行你的程序：

```bash
$ ./your_program.sh
```

然后它会运行 `declare -p`，变量名未被定义。

```bash
$ declare -p missing_variable
declare: missing_variable: not found
```

### 注意事项

- 对于这一阶段，你可以硬编码 `declare` 内置命令的输出。我们将在后续阶段实现存储 shell 变量
