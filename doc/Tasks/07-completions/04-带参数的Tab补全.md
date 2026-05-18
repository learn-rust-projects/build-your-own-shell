---
created: 2026-05-18T15:25
updated: 2026-05-18T15:48
---

## 带参数的 Tab 补全

在这一阶段，你将扩展 shell 的 tab 补全以处理带参数的命令。

### 带参数的 Tab 补全

在前面的阶段中，你为内置命令实现了基本的 tab 补全。现在，你将确保在命令被补全后，用户可以继续输入参数并执行完整命令。

例如：

```bash
# 1. 用户输入部分命令
$ ech

# 2. 用户按 <TAB>。Shell 自动补全（注意尾随空格）
$ echo


# 3. 用户输入参数 'hello'
$ echo hello

# 4. 用户按 <ENTER>。Shell 执行命令
hello
```

### 测试

测试程序会这样执行你的程序：

```bash
./your_program.sh
```

测试会模拟带 tab 按键的用户输入，并执行内置命令，类似于前面的阶段，但带有附加参数：

```bash
$ ech<TAB>
$ echo
$ echo hello<ENTER>
hello

$ ech<TAB>
$ echo
$ echo foo bar<ENTER>
foo bar
```

测试程序会验证：
- Tab 补全正常工作
- 补全后输入的参数被保留
- 带有所有参数的完整命令正确执行

