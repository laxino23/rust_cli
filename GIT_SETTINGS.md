# Git 配置文档

本文档总结了 rcli 项目中所有与 Git 相关的配置和设置。

## 目录

- [Git 仓库配置](#git-仓库配置)
- [.gitignore 配置](#gitignore-配置)
- [Git Hooks 配置](#git-hooks-配置)
- [Changelog 生成配置](#changelog-生成配置)
- [提交规范](#提交规范)

---

## Git 仓库配置

### 远程仓库

- **URL**: `https://github.com/laxino23/rust_cli`
- **主分支**: `main`

### 本地配置

```ini
core.bare=false
core.repositoryformatversion=0
core.filemode=true
core.ignorecase=true
core.precomposeunicode=true
core.logallrefupdates=true
remote.origin.url=https://github.com/laxino23/rust_cli
remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
branch.main.remote=origin
branch.main.merge=refs/heads/main
```

---

## .gitignore 配置

项目忽略以下文件/目录：

```gitignore
/target
/output.json
/output.yaml
/output.toml
```

**说明**:

- `/target` - Rust 编译输出目录
- `/output.*` - 程序生成的输出文件（JSON、YAML、TOML 格式）

---

## Git Hooks 配置

项目使用 **pre-commit** 框架来管理 Git hooks，在每次提交前自动执行代码检查。

### Pre-commit 安装

项目中的 `.git/hooks/pre-commit` 文件由 pre-commit 框架自动生成。

### Pre-commit 配置文件

配置文件位于 `.pre-commit-config.yaml`，包含以下检查：

#### 1. 通用检查 (pre-commit-hooks)

- **check-byte-order-marker**: 检查 BOM 标记
- **check-case-conflict**: 检查文件名大小写冲突
- **check-merge-conflict**: 检查合并冲突标记
- **check-symlinks**: 检查符号链接
- **check-yaml**: 验证 YAML 文件格式
- **end-of-file-fixer**: 确保文件以换行符结尾
- **mixed-line-ending**: 检查混合的行尾符
- **trailing-whitespace**: 移除行尾空格

#### 2. Python 代码格式化

- **black**: Python 代码格式化工具

#### 3. Rust 代码检查（本地 hooks）

##### cargo fmt

- **描述**: 使用 rustfmt 格式化代码
- **命令**: `cargo fmt -- --check`
- **作用范围**: `.rs` 文件

##### cargo deny

- **描述**: 检查 cargo 依赖项
- **命令**: `cargo deny check -d`
- **作用范围**: `.rs` 文件

##### typos

- **描述**: 检查拼写错误
- **命令**: `typos`
- **作用范围**: 所有文件

##### cargo check

- **描述**: 检查包是否有编译错误
- **命令**: `cargo check --all`
- **作用范围**: `.rs` 文件

##### cargo clippy

- **描述**: Rust 代码 lint 检查
- **命令**: `cargo clippy --all-targets --all-features --tests --benches -- -D warnings`
- **作用范围**: `.rs` 文件

##### cargo test

- **描述**: 运行单元测试
- **命令**: 使用 `cargo nextest` 运行测试（如果存在测试）
- **作用范围**: `.rs` 文件
- **智能跳过**: 如果没有测试，会自动跳过

### 配置特点

- `fail_fast: false` - 即使某个检查失败，也会继续运行其他检查
- 使用 cargo nextest 作为测试运行器
- 严格的代码质量标准（clippy 警告会导致失败）

---

## Changelog 生成配置

项目使用 **git-cliff** 工具自动生成 changelog，配置文件为 `cliff.toml`。

### 基本配置

#### Changelog 头部

```markdown
# Changelog

All notable changes to this project will be documented in this file.
See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.
```

#### 仓库链接

- **GitHub 仓库**: `https://github.com/Laxino94/rust-tyr-01-rcli`

### Commit 解析规则

项目使用 **Conventional Commits** 规范解析提交信息：

#### 提交类型分组

| 提交前缀 | 分组名称 | 说明 |
|---------|---------|------|
| `feat` | Features | 新功能 |
| `fix` | Bug Fixes | 错误修复 |
| `doc` | Documentation | 文档更新 |
| `perf` | Performance | 性能优化 |
| `refactor` | Refactoring | 代码重构 |
| `style` | Style | 代码风格 |
| `revert` | Revert | 回退更改 |
| `test` | Tests | 测试相关 |
| `chore` | Miscellaneous Chores | 杂项任务 |
| 其他 | Other | 其他更改 |

#### 特殊规则

- **跳过规则**:
  - 包含 `[skip` 的提交
  - 包含中文字符 (`\p{Han}`) 的提交
  - `chore(version):` 版本更新提交

- **安全相关**: 提交信息 body 中包含 "security" 的会被归类到 Security 组

### Tag 配置

- **Tag 模式**: `v[0-9].*` (如 v1.0.0, v2.1.3)
- **跳过的 tag**: `v0.1.0-beta.1`
- **提交排序**: 按时间从旧到新排序

### 生成的 Changelog 格式

每个版本包含：

- 版本号和日期
- GitHub 比较链接
- 按类型分组的提交
- 提交信息、短 hash、作者名称
- Breaking changes 标记

**示例格式**:

```markdown
## [1.0.0](repo/compare/v0.1.0..v1.0.0) - 2024-01-01

### Features

- **(cli)** add new command - ([abc1234](repo/commit/abc1234)) - Author Name
```

---

## 提交规范

### Conventional Commits 格式

```text
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### 推荐的提交类型

- `feat`: 新功能
- `fix`: 错误修复
- `docs`: 文档更新
- `style`: 代码格式（不影响代码运行）
- `refactor`: 代码重构
- `perf`: 性能优化
- `test`: 测试相关
- `chore`: 构建过程或辅助工具的变动

### Breaking Changes

在提交信息 footer 中添加 `BREAKING CHANGE:` 来标记破坏性更改：

```text
feat(api): change authentication method

BREAKING CHANGE: JWT tokens are now required for all API calls
```

### Scope 示例

- `cli`: 命令行接口
- `csv`: CSV 处理功能
- `base64`: Base64 编解码
- `text`: 文本处理
- `genpass`: 密码生成

---

## 开发工作流

### 1. 安装 pre-commit

```bash
pip install pre-commit
# 或
brew install pre-commit
```

### 2. 安装 hooks

```bash
pre-commit install
```

### 3. 手动运行所有检查

```bash
pre-commit run --all-files
```

### 4. 生成 changelog

```bash
git cliff -o CHANGELOG.md
```

### 5. 提交代码

```bash
git add .
git commit -m "feat(cli): add new feature"
# pre-commit hooks 会自动运行
```

---

## 相关依赖工具

项目的 Git 工作流依赖以下工具：

1. **pre-commit** - Git hooks 管理框架
2. **git-cliff** - Changelog 生成工具
3. **rustfmt** - Rust 代码格式化
4. **cargo-clippy** - Rust linter
5. **cargo-deny** - 依赖检查工具
6. **typos** - 拼写检查工具
7. **cargo-nextest** - 下一代 Rust 测试运行器

---

## 文件清单

与 Git 相关的配置文件：

- `.gitignore` - Git 忽略文件配置
- `.pre-commit-config.yaml` - Pre-commit hooks 配置
- `cliff.toml` - Git-cliff changelog 生成配置
- `CHANGELOG.md` - 项目变更日志
- `.git/hooks/pre-commit` - Pre-commit hook 执行脚本（自动生成）
- `_typos.toml` - 拼写检查配置（可能存在）

---

最后更新: 2025-10-26
