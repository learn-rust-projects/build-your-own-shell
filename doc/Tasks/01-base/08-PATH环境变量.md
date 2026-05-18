---
created: 2026-05-18T15:25
updated: 2026-05-18T17:02
view_count: 1
---

## PATH 环境变量

在这一阶段，你将扩展 `type` 内置命令，使用 PATH 搜索可执行文件。

### PATH 环境变量

[PATH](https://en.wikipedia.org/wiki/PATH_(variable)) 环境变量指定了 shell 查找可执行程序的目录列表。

例如，如果 PATH 设置为 `/dir1:/dir2:/dir3`，shell 会按顺序在 `/dir1`、`/dir2` 和 `/dir3` 中搜索可执行文件。

### 搜索可执行文件

当 `type` 收到一个命令输入时，你的 shell 必须遵循以下步骤：
1. 检查该命令是否是内置命令（如 `exit` 或 `echo`）。如果是，报告为内置命令（`<command> is a shell builtin`）并停止
2. 如果该命令不是内置命令，你的 shell 必须遍历 PATH 中的每个目录。对于每个目录：
   1. 检查是否存在以该命令名命名的文件
   2. 检查该文件是否具有**执行权限**
   3. 如果文件存在且有执行权限，打印 `<command> is <full_path>` 并停止
   4. 如果文件存在但**缺少执行权限**，跳过它并继续到下一个目录
3. 如果在任何目录中都找不到可执行文件，打印 `<command>: not found`

例如：

```bash
$ type grep
grep is /usr/bin/grep
$ type invalid_command
invalid_command: not found
$ type echo
echo is a shell builtin
```

### 测试

测试程序会使用自定义的 `PATH` 这样执行你的程序：

```bash
PATH="/usr/bin:/usr/local/bin:$PATH" ./your_program.sh
```

然后它会向你的 shell 发送一系列 `type` 命令：

```bash
$ type ls
ls is /usr/bin/ls
$ type valid_command
valid_command is /usr/local/bin/valid_command
$ type invalid_command
invalid_command: not found
$
```

测试程序会验证 `type` 命令正确识别 PATH 中的可执行文件：
- PATH 中的可执行文件会报告其完整路径（`<command> is <full_path>`）
- 没有执行权限的文件会被跳过
- 不存在的命令会打印 `<command>: not found` 消息

### 注意事项

- PATH 可能包含磁盘上不存在的目录，所以你的代码应该优雅地处理这种情况
- 解析 PATH 环境变量时，记住分隔符（通常是 `:` 或 `;`）可能因操作系统而异。使用你的语言提供的与操作系统无关的路径处理（如 Python 的 `os.pathsep`、Java 的 `File.pathSeparator` 或 Node.js 的 `path.delimiter`）来正确分割目录
