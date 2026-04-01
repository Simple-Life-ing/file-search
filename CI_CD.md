# CI/CD 配置指南

## 📋 概述

file-search 项目自动化工作流包含6个GitHub Actions工作流，覆盖测试、质量检查、基准测试、发布和文档生成。

## 🔄 工作流概览

### 1️⃣ Tests Workflow (`tests.yml`)

**触发条件：** Push 到 main/develop 或 PR

**执行内容：**
- ✅ 跨平台测试（Linux, macOS, Windows）
- ✅ Rust stable 和 beta 版本测试
- ✅ 单元测试 (`cargo test --lib`)
- ✅ 集成测试 (`cargo test --test '*'`)
- ✅ 文档测试 (`cargo test --doc`)
- ✅ 代码覆盖率生成（Tarpaulin）
- ✅ 内存安全检查（Miri）

**输出：**
- Test reports
- Code coverage reports (Codecov)
- Memory safety analysis

**关键指标：**
```yaml
Jobs:
  - test (6 combinations: 3 OS × 2 Rust versions)
  - test-coverage (Linux + Codecov upload)
  - miri (Nightly + Memory safety)
```

### 2️⃣ Benchmarks Workflow (`benchmarks.yml`)

**触发条件：** Push 到 main 或 PR 针对 main

**执行内容：**
- ✅ 运行完整基准测试套件
- ✅ 对比性能基线
- ✅ 检测性能回归（> 200%）
- ✅ 自动注释变更

**输出：**
- Benchmark results (HTML reports)
- Regression alerts
- Performance trend graphs

**关键指标：**
```yaml
Benchmarks Run:
  - pattern_matching (8 benchmarks)
  - file_processing (9 benchmarks)
  - directory_traversal (8 benchmarks)
  - e2e (5 benchmarks)
Total: 30 benchmark functions
```

### 3️⃣ Quality Workflow (`quality.yml`)

**触发条件：** Push 到 main/develop 或 PR

**执行内容：**
- ✅ 代码格式检查 (rustfmt)
- ✅ 代码蚊虫检查 (clippy)
- ✅ 安全审计 (rustsec)
- ✅ 依赖检查 (cargo-deny)
- ✅ MSRV 验证 (1.70+)
- ✅ 文档生成
- ✅ 未使用依赖检查

**输出：**
- Lint reports
- Security vulnerabilities
- Formatting violations

**关键指标：**
```yaml
Quality Checks:
  - Formatting: PASS/FAIL
  - Clippy: 0 warnings
  - Security: No CVEs
  - MSRV: 1.70+
  - Docs: No warnings
```

### 4️⃣ Release Workflow (`release.yml`)

**触发条件：** 创建新 git tag (v*.*.*)

**执行内容：**
- ✅ 生成变更日志
- ✅ 创建 GitHub Release
- ✅ 跨平台验证测试
- ✅ 编译发布二进制
- ✅ 上传构件到 Release

**输出：**
- GitHub Release
- Platform-specific binaries (Linux, macOS, Windows)
- Release notes with changelog

**关键指标：**
```yaml
Release Artifacts:
  - file-search-linux-x86_64
  - file-search-macos-x86_64
  - file-search-windows-x86_64.exe
```

### 5️⃣ Publish Workflow (`publish.yml`)

**触发条件：** Release 创建（自动触发）

**执行内容：**
- ✅ 验证版本号匹配
- ✅ 再次运行测试
- ✅ 检查文档编译
- ✅ 发布到 crates.io
- ✅ 验证发布成功

**输出：**
- Package on crates.io
- Release comment

**关键指标：**
```yaml
Publishing:
  - Version check: PASS
  - Tests: PASS
  - Docs: Build success
  - crates.io: Published
```

### 6️⃣ Documentation Workflow (`docs.yml`)

**触发条件：** Push 到 main（当源文件改动）

**执行内容：**
- ✅ 构建 Rust API 文档
- ✅ 复制 markdown 文档
- ✅ 生成导航页面
- ✅ 发布到 GitHub Pages

**输出：**
- GitHub Pages 站点
- API documentation
- Documentation index

**关键指标：**
```yaml
Documentation Coverage:
  - API docs: ✓
  - README: ✓
  - Performance guide: ✓
  - Benchmarks: ✓
```

---

## 🚀 使用工作流

### 本地测试（推荐在提交前）

```bash
# 运行所有单元测试
cargo test --lib

# 运行集成测试
cargo test --test '*'

# 运行代码检查
cargo clippy --all-targets -- -D warnings

# 检查格式
cargo fmt -- --check

# 运行基准测试（可选）
cargo bench --no-fail-fast
```

### 创建发布

#### 步骤 1: 创建发行版本标签

```bash
# 更新 Cargo.toml 中的版本
# 例如: version = "0.2.0"

git add Cargo.toml
git commit -m "Bump version to 0.2.0"
git tag -a v0.2.0 -m "Release version 0.2.0"
git push origin main
git push origin v0.2.0
```

#### 步骤 2: 自动工作流流程

1. **Release 工作流** 自动触发：
   - 生成变更日志
   - 创建 GitHub Release
   - 编译三个平台的二进制
   - 上传构件

2. **Publish 工作流** 自动触发：
   - 验证版本和测试
   - 发布到 crates.io
   - 注释 Release

3. **Documentation 工作流** 自动触发：
   - 更新 GitHub Pages

### 手动触发工作流

#### 重新运行失败的工作流

在 GitHub Actions 选项卡中，选择失败的工作流，点击 "Re-run jobs"

#### 手动触发文档生成

```bash
# 使用 GitHub CLI
gh workflow run docs.yml -r main

# 或者在 GitHub UI 中点击 "Run workflow"
```

---

## 📊 工作流状态和徽章

### 在 README 中添加状态徽章

```markdown
![Tests](https://github.com/YOUR_ORG/file-search/workflows/Tests/badge.svg?branch=main)
![Code Quality](https://github.com/YOUR_ORG/file-search/workflows/Code%20Quality/badge.svg?branch=main)
![Benchmarks](https://github.com/YOUR_ORG/file-search/workflows/Benchmarks/badge.svg?branch=main)
![Release](https://github.com/YOUR_ORG/file-search/workflows/Release/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/file-search.svg)](https://crates.io/crates/file-search)
[![docs.rs](https://docs.rs/file-search/badge.svg)](https://docs.rs/file-search)
```

---

## 🔑 所需的 Secrets

在 GitHub Pages 设置中配置以下 secrets：

### `CARGO_REGISTRY_TOKEN`
用于发布到 crates.io

**设置方式：**
1. 生成 token: https://crates.io/me
2. 复制 token
3. 在 GitHub repo settings → Secrets → 添加 `CARGO_REGISTRY_TOKEN`

### `GITHUB_TOKEN`
自动生成，无需手动配置

---

## ⚙️ 配置和自定义

### 修改触发条件

编辑 `.github/workflows/*.yml` 中的 `on:` 部分：

```yaml
on:
  push:
    branches: [main]  # 追踪的分支
  pull_request:
    branches: [main]
  schedule:           # 计划运行
    - cron: '0 0 * * 0'  # 每周日午夜运行
```

### 改变测试矩阵

编辑工作流中的 `strategy.matrix`：

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]
    rust: [stable, beta, nightly]  # 添加 nightly
```

### 调整超时

```yaml
jobs:
  test:
    timeout-minutes: 30  # 默认360分钟
```

---

## 📈 监控和调试

### 查看工作流运行日志

1. 进入 GitHub repo → Actions
2. 选择要查看的工作流
3. 点击特定的运行
4. 查看详细日志

### 常见问题排查

#### ❌ 测试失败

```bash
# 本地复现失败
git checkout <branch>
cargo test --verbose

# 查看工作流日志了解更多信息
```

#### ❌ 发布失败

1. 检查 `CARGO_REGISTRY_TOKEN` 是否正确配置
2. 验证 Cargo.toml 版本格式
3. 确保所有依赖都是 public crate

#### ❌ 文档生成失败

1. 检查 rustdoc 警告：`cargo doc --no-deps`
2. 修复文档注释中的问题
3. 确保所有代码示例都能编译

---

## 📚 最佳实践

### ✅ 发布前检查清单

- [ ] 所有测试通过：`cargo test`
- [ ] 代码格式正确：`cargo fmt`
- [ ] 无 Clippy 警告：`cargo clippy -- -D warnings`
- [ ] 基准结果稳定：`cargo bench`
- [ ] 文档生成无警告：`cargo doc --no-deps`
- [ ] Cargo.toml 版本已更新
- [ ] 更新了 CHANGELOG（如有）

### ✅ PR 审查清单

- [ ] CI/CD 所有检查通过
- [ ] 代码覆盖率 ≥ 80%
- [ ] 无性能回归（基准对比）
- [ ] 文档已更新
- [ ] 提交信息清晰

### ✅ 发布清单

- [ ] 版本号遵循语义化版本（SemVer）
- [ ] 所有平台二进制编译成功
- [ ] Release notes 完整
- [ ] crates.io 发布成功
- [ ] 文档已更新

---

## 🔗 相关资源

- [GitHub Actions 文档](https://docs.github.com/en/actions)
- [Rust GitHub Actions](https://docs.github.com/en/actions/guides/building-and-testing-rust)
- [crates.io 发布指南](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Semantic Versioning](https://semver.org/)
- [Conventional Commits](https://www.conventionalcommits.org/)

---

## 🤝 贡献指南

### 为工作流做贡献

1. **修改工作流文件**：创建 PR，更新 `.github/workflows/*.yml`
2. **测试工作流**：在本地分支上推送更改，查看 GitHub Actions 结果
3. **获取反馈**：等待维护者审查和测试

### 添加新工作流

1. 在 `.github/workflows/` 中创建新 YAML 文件
2. 定义触发条件和 jobs
3. 添加到此文档
4. 提交 PR 进行审查

---

**最后更新**: 2026年4月1日  
**维护者**: file-search Team
