# 后台作业管理设计

## 概述

后台作业系统处理以 `&` 结尾的命令，支持作业的生命周期管理、状态追踪和 bash 兼容格式输出。

## 核心数据结构

```rust
pub struct JobList {
    pub jobs: Vec<Job>,          // 当前作业列表（按插入顺序）
    pub list: Vec<usize>,        // 可回收的作业 ID 集合（有序）
}

pub struct Job {
    pub id: usize,              // 作业 ID（从 1 开始）
    pub pid: u32,               // 子进程 PID
    pub command: String,        // 命令字符串
    pub status: JobStatus,      // Running / Done
    pub child: Option<std::process::Child>,  // 子进程句柄
}
```

## ID 管理 — 回收复用机制

作业 ID 采用**回收复用**策略：

```text
JobList.next_id():
  1. 检查 list（已完成的作业 ID 集合）是否有可用 ID
  2. 有 → 返回复用的 ID
  3. 无 → 从原子计数器 JOB_ID 获取递增新 ID

JobList.insert(id):
  当作业完成时，其 ID 被插入 list（保持排序）
  供后续新作业复用
```

这种设计避免 ID 无限增长，但存在一个已知 bug：`update_pid()` 使用 `id - 1` 作为 Vec 索引，而复用的 ID 会导致越界 panic。

## 作业状态追踪

作业状态在每次命令执行后通过 `try_wait()` 非阻塞检查更新：

```text
用户输入命令
  │
  ├── 包含 & → 后台执行，注册到 JobList
  │               ↓
  │             parse_and_handle_line() 返回前调用
  │             GLOBAL_JOB.lock().list_done_jobs()
  │               ↓
  │             遍历所有作业，对每个 job.child 调用 try_wait()
  │             如果子进程已退出 → status = Done
  │             输出 Done 作业信息，回收其 ID
  │
  └── 无 & → 前台同步执行
```

## 输出格式（bash 兼容）

```text
[1]+  Running                 sleep 10 &
[2]-  Done                    echo hello
```

格式说明：

- `[n]` — 作业 ID
- `+` — 最近操作的作业（当前作业）
- `-` — 次近操作的作业（上一个作业）
- 状态占 24 个字符宽度，左对齐

标记算法：

```rust
let marker = if idx + 1 == len { '+' }       // 最后一个 = 当前
       else if idx + 2 == len { '-' }        // 倒数第二个 = 上一个
       else { ' ' };
```

## 已知限制

1. **无 `fg`/`bg` 命令** — 当前不支持将后台作业调回前台或切换状态
2. **无 SIGCHLD 处理器** — 作业完成仅在同步检查时发现
3. **`update_pid` 索引 bug** — 使用 `id - 1` 索引而非搜索匹配 ID，复用 ID 会导致越界
