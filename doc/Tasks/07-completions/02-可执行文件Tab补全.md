---
created: 2026-05-18T15:48
updated: 2026-05-18T15:48
---
created: 2026-05-18T15:25
updated: 2026-05-18T15:25
---

## 可执行文件 Tab 补全

在这一阶段，你将扩展 shell 的 tab 补全以包含用户 `PATH` 中的外部可执行文件。

### 可执行文件的 Tab 补全

在前面的阶段中，你为内置命令（`echo` 和 `exit`）实现了 tab 补全。现在你将把补全扩展到包含在 PATH 环境变量中找到的外部可执行文件。

当用户输入可执行文件名的开头并按 `<TAB>` 时，你的 shell 应该将其补全为完整的可执行文件名。

例如，如果 `custom_executable` 存在于 PATH 中列出的目录中，输入 `custom` 并按 tab 会补全为 `custom_executable `（带尾随空格）

### 测试

测试程序会创建一个名为 `custom_executable` 的可执行文件，并将其目录添加到 `PATH`。

然后它会这样执行你的程序：

```bash
./your_program.sh
```

接下来，测试程序会模拟用户输入外部命令的开头并按 `<TAB>`：

```bash
$ custom<TAB>
$ custom_executable
```

测试程序会验证你的 shell 正确地将命令补全为外部可执行文件名。

### 注意事项

- PATH 可能包含磁盘上不存在的目录，所以你的代码应该优雅地处理这种情况
