---
created: 2026-05-18T15:25
updated: 2026-05-18T15:50
---

## 参数展开 - $VAR 形式

在这一阶段，你将添加使用 `$VAR` 形式的参数展开支持。

### 展开

当命令行包含 `$NAME` 且 `NAME` 是一个变量时，shell 在调用内置命令或外部程序之前会用变量的值替换 `$NAME`。

替换发生在 shell 作为参数传递的单词中。它不会改变变量在内部存储的方式。每个展开的值成为 shell 运行的程序的单独参数。

例如：

```bash
$ declare Variable_1=value
$ declare Variable_2=value2
$ echo $Variable_1 $Variable_2
value value2
```

这里 `echo` 在展开后接收到两个单词（`value` 和 `value2`），而不是字面字符串 `$Variable_1` 和 `$Variable_2`。

### 测试

测试程序会这样执行你的程序：

```bash
$ ./your_program.sh
```

它会设置变量，然后使用包含 `$VAR` 展开的参数运行程序。例如：

```bash
$ declare Variable_1=Value_1
$ declare Variable_2=Value2
$ custom_exe_1234 $Variable_1 $Variable_2
Program was passed 3 args (including program name).
Arg #0 (program name): custom_exe_1234
Arg #1: Value_1
Arg #2: Value_2
Program Signature: 5998595441
```

上面显示的输出来自可执行文件本身。测试程序会验证参数列表与展开的值匹配。

### 注意事项

- 在这一阶段，只支持简单的 `$VAR` 语法。我们将在后续阶段支持 `${VAR}` 形式
