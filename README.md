# ChronoH ⏰

> 为长时间运行的 AI Agent 提供时间感知能力的开发框架

## 概述

ChronoH 是一个专为 AI Agent 设计的时间感知开发框架，通过状态持久化、生命周期钩子和会话管理，帮助 AI Agent 实现可靠的长时间开发任务。

## 核心特性

### 🗄️ 状态持久化
- 基于 Sled 嵌入式 KV 数据库
- 自动记录所有开发事件（初始化、检查点、会话开始/结束）
- 支持断点续传，会话状态不丢失

### 🔄 生命周期钩子
- Clean State 协议：每次会话结束前自动检查
  - 测试是否通过
  - Git 仓库是否干净
  - 进度是否更新
  - 交接文档是否生成
- 可扩展的钩子系统，支持自定义检查逻辑

### 🛠️ 4-原子工具
- `read`: 读取文件（支持 offset/limit）
- `write`: 原子写入文件
- `edit`: 精确替换文本
- `bash`: 执行 Shell 命令（支持超时控制）

### 👥 角色化 Agent
- **Initializer**: 项目初始化，创建项目骨架
- **Coder**: 开发 Agent，负责编码任务
- **Reviewer**: 代码审查
- **Compactor**: 上下文压缩

### 📊 阶段管理
- `InfrastructureReady`: 基础设施就绪
- `AuthReady`: 认证模块就绪
- `CoreApiReady`: 核心 API 就绪
- `ProductionReady`: 生产就绪

## 安装

```bash
# 克隆仓库
git clone git@gitcode.com:haomintsai/chronoh.git
cd chronoh

# 构建
cargo build --release
```

## 快速开始

### 1. 初始化项目

```bash
cargo run -- init --name my-project
```

这将创建以下结构：
```
my-project/
├── Cargo.toml
├── src/main.rs
├── .git/
└── .pi/state/          # ChronoH 状态目录
    ├── state.sled      # Sled 数据库
    └── handoff.md      # 交接文档
```

### 2. 开始开发

```bash
cd my-project
cargo run -- dev
```

这将启动一个 Coder 会话，默认 50 轮次限制。

### 3. 查看状态

```bash
cargo run -- status
```

输出示例：
```
📊 Project Status
═══════════════════════════════════════
Current phase: InfrastructureReady
Total events: 4
Last activity: 2026-02-28 10:27:51 UTC
```

### 4. 其他命令

```bash
cargo run -- review              # 代码审查
cargo run -- compact             # 上下文压缩
```

## 项目结构

```
chrono-h/
├── src/
│   ├── agents/          # Agent 实现
│   │   ├── initializer.rs
│   │   └── coder.rs
│   ├── cli/             # 命令行接口
│   ├── git/             # Git 集成
│   ├── hooks/           # 生命周期钩子
│   ├── state/           # 状态引擎
│   │   ├── engine.rs    # Sled 持久化
│   │   └── handoff.rs   # 交接文档
│   ├── tools/           # 4-原子工具
│   ├── types.rs         # 核心类型定义
│   └── error.rs         # 错误处理
├── tests/               # 测试套件
└── Cargo.toml
```

## 事件流

```
┌─────────────┐
│   Init      │  ← 项目初始化事件
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Checkpoint │  ← 里程碑事件
└──────┬──────┘
       │
       ▼
┌─────────────┐
│SessionStart │  ← 会话开始
└──────┬──────┘
       │
       ▼
    [开发循环]
       │
       ▼
┌─────────────┐
│ CleanState  │  ← 钩子检查
│   Hook      │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ SessionEnd  │  ← 会话结束
└─────────────┘
```

## 配置

### 会话配置

在 `.pi/config.json` 中配置：

```json
{
  "max_turns": 50,
  "session_cap": 3,
  "auto_compact": true,
  "clean_state_required": true
}
```

## 测试

```bash
# 运行所有测试
cargo test

# 运行集成测试
cargo test --test integration_test
```

## 技术栈

- **语言**: Rust
- **数据库**: Sled (嵌入式 KV)
- **Git**: git2-rs
- **CLI**: clap
- **异步**: tokio

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！

---

*ChronoH - 让 AI Agent 的长时间开发变得可靠可控*
