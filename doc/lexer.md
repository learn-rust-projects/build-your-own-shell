# 词法分析器设计

## 概述

词法分析器实现为 **5 状态有限状态机（FSM）**，逐字符扫描输入行，将原始字符串转换为 `RawToken` 枚举流。

## 核心类型

```rust
pub enum RawToken {
    Word(String),      // 普通单词
    Pipe,              // |
    IoNumber(u8),      // 0, 1, 2 ...（仅重定向前有意义）
    Redirect(RedirectOp), // >, >>, <, <<, >&, <&
    Background,        // &
}

pub enum RedirectOp {
    Out,       // >
    OutAppend, // >>
    In,        // <
    Heredoc,   // <<
    DupOut,    // >&
    DupIn,     // <&
}
```

## 状态机设计

### 五状态定义

```text
┌─────────────────────────────────────────────────────────────────────┐
│                          LexerState                                 │
├─────────────────────────────────────────────────────────────────────┤
│  Normal (普通)        — 默认状态，处理普通字符和特殊符号            │
│  SingleQuote (单引号)  — 遇到 ' 进入，遇到 ' 退出，期间所有字符原样 │
│  DoubleQuote (双引号)  — 遇到 " 进入，遇到 " 退出，部分字符需转义   │
│  Escaping (转义)       — 遇到 \ 进入，消费下一字符后回到 Normal     │
│  DoubleQuoteEscaping   — 双引号内遇到 \ 进入，仅转义特定字符        │
└─────────────────────────────────────────────────────────────────────┘
```

### 状态转移图

```text
                              ┌─────────┐
                    ┌────────►│ Escaping │──(任意字符)──┐
                    │         └─────────┘              │
                    │                                   ▼
    ┌──────────┐  '\'    ┌──────────┐ 未转义字符   ┌──────────┐
    │          │────────►│          │◄────────────┤          │
    │  Normal  │         │  Normal  │────────────►│  Normal  │
    │          │◄────────│          │   '\'        │          │
    └────┬─┬───┘  '"'    └──────────┘             └──────────┘
         │ │
    '\'' │ │ '"'
         │ │
         ▼ ▼      '"'             '\'          特定字符
    ┌──────────┐────────►┌──────────────┐────────►┌────────────────┐
    │SingleQuote│         │ DoubleQuote  │         │DoubleQuoteEscping│
    └──────────┘          └──────┬───────┘◄───────└────────────────┘
                                 │   非特殊字符: 保留反斜杠原样
                                 ▼
                           (留在 DoubleQuote)
```

### 关键设计决策

#### 1. 双引号内转义规则（POSIX 兼容）

在双引号内，反斜杠仅对以下字符进行转义：

- `"` — 防止结束双引号
- `\` — 字面反斜杠
- `$` — 防止变量展开
- `` ` `` — 防止命令替换

其他字符前的反斜杠**保留原样**（反斜杠 + 字符均作为字面量）。

```rust
// lexer.rs:116-124
LexerState::DoubleQuoteEscaping => {
    match ch {
        '"' | '\\' | '$' | '`' => current_word.push(ch),
        _ => {
            current_word.push('\\');  // 保留反斜杠
            current_word.push(ch);    // 保留字符
        }
    }
    state = LexerState::DoubleQuote;
}
```

#### 2. IoNumber 的延迟识别

`IoNumber`（如 `2` 在 `2>&1` 中）不是由词法分析器直接识别的。它首先被识别为 `Word("2")`，在**语法分析阶段**（`parse_simple_command`）中，如果 `Word` 后跟 `Redirect` token，且该 word 可以解析为数字，才被转换为 `IoNumber`。

```rust
// lexer.rs:137-143
fn parse_redirect_word(word: &str) -> RawToken {
    if let Ok(num) = word.parse::<u8>() {
        RawToken::IoNumber(num)
    } else {
        RawToken::Word(word.to_string())
    }
}
```

这种设计避免了词法分析阶段就需要前瞻判断的复杂性。

#### 3. 重定向操作符的多字符匹配

`parse_redirect_op` 函数通过 `peekable()` 迭代器前瞻一个字符，以区分单字符和多字符操作符：

| 起始字符 | 下一个字符 | 结果        |
| -------- | ---------- | ----------- |
| `>`      | (无)       | `Out`       |
| `>`      | `>`        | `OutAppend` |
| `>`      | `&`        | `DupOut`    |
| `<`      | (无)       | `In`        |
| `<`      | `<`        | `Heredoc`   |
| `<`      | `&`        | `DupIn`     |

## Token 处理流程

```text
输入: echo "hello world" > output.txt 2>&1

分词过程:
  Word("echo")
  Word("hello world")     ← 双引号内空格被保留
  Redirect(Out)
  Word("output.txt")
  IoNumber(2)             ← 识别为 I/O 编号
  Redirect(DupOut)
  Word("1")
```
