---
created: 2026-05-18T15:25
updated: 2026-05-18T15:50
---

## 参数展开 - ${VAR} 形式

在这一阶段，你将添加使用 `${VAR}` 形式的参数展开支持。

### 使用花括号的展开

没有花括号时，`$FOObar` 被读取为变量 `FOObar`。用花括号包裹名称 — `${FOO}bar` — 告诉 shell 名称在哪里结束，所以 `FOO` 被展开，`bar` 作为字面文本被追加。

```bash
$ declare Var1=foo
$ declare Var2=bar
$ echo ${Var1}end
fooend
$ echo ${Var1}and${Var2}
fooandbar
```

### 测试

测试程序会这样执行你的程序：

```bash
$ ./your_program.sh
```

它会设置变量并如下运行命令：

```bash
$ declare Item=widget
$ declare Foo1=Bar2
$ ./custom_exe_1234 stock_${Item}_id ${Foo1}
Program was passed 3 args (including program name).
Arg #0 (program name): custom_exe_1234
Arg #1: stock_widget_id
Arg #2: Bar2
Program Signature: 5998595441
```

测试程序会验证打印的输出与完全展开的行匹配。
