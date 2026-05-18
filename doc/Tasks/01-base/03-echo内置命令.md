---
created: 2026-05-18T15:25
updated: 2026-05-18T15:41
view_count: 1
---

## echo 内置命令

在这一阶段，你将实现 `echo` 内置命令。

### echo 内置命令

[`echo`](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/echo.html) 内置命令将其参数打印到标准输出，参数之间用空格分隔，最后以换行符（`\n`）结尾。

示例用法：

```bash
$ echo hello world
hello world
$ echo one two three
one two three
```

### 测试

测试程序会这样执行你的程序：

```bash
./your_program.sh
```

然后它会向你的 shell 发送一系列 `echo` 命令：

```bash
$ echo hello world
hello world
$ echo pineapple strawberry
pineapple strawberry
$
```

每次命令执行后，测试程序会验证 `echo` 命令是否正确地回显了提供的文本。

### 注意事项

- 大多数语言的标准输出函数（如 JavaScript 的 `console.log()`、Python 的 `print()` 或 Java 的 `println()`）会自动添加换行符，这正是你需要的。如果你的语言需要显式添加换行符（如 C 的 `printf()`），请确保在末尾添加 `\n`。
