# GitHub Actions 快速参考卡

## 📋 6个工作流一览表

| 工作流 | 文件 | 触发条件 | 主要功能 | 平台 | 运行时间 |
|------|------|--------|--------|------|--------|
| **Tests** | tests.yml | push/PR | 单元+集成+覆盖+Miri | 3 | 3-5分钟 |
| **Benchmarks** | benchmarks.yml | push/PR main | 30个基准+回归检测 | 1 | 5-7分钟 |
| **Quality** | quality.yml | push/PR | 7个质量检查 | 1 | 2-3分钟 |
| **Release** | release.yml | v*.*.* tag | 创建Release+二进制 | 3 | 5-8分钟 |
| **Publish** | publish.yml | Release | 发布crates.io | 1 | 2-3分钟 |
| **Docs** | docs.yml | push main | GitHub Pages | 1 | 1-2分钟 |

## 🚀 如何发布新版本

```bash
# 1. 更新版本
sed -i '' 's/version = "0.1.0"/version = "0.2.0"/' Cargo.toml

# 2. 提交
git add Cargo.toml
git commit -m "Bump version to 0.2.0"

# 3. 创建标签（触发自动发布）
git tag -a v0.2.0 -m "Release version 0.2.0"

# 4. 推送
git push origin main --tags

# 5. 观察自动工作流
# - Release 工作流创建 Release
# - Publish 工作流发布到 crates.io
# - Docs 工作流更新文档
```

## 📌 关键配置

### 必须设置的 Secret
```
CARGO_REGISTRY_TOKEN = <from https://crates.io/me>
GITHUB_TOKEN = (自动)
```

### 分支保护规则（main）
- ✓ Require status checks to pass before merging
- ✓ Require tests.yml + quality.yml to pass

### GitHub Pages 配置
- Source: GitHub Actions
- Deploy from: main branch

## ✅ 常见任务

### 运行特定工作流
```bash
# 手动触发文档生成
gh workflow run docs.yml -r main

# 查看所有运行
gh run list --workflow tests.yml

# 查看特定运行的日志
gh run view <RUN_ID> --log
```

### 本地验证（推荐在PR前运行）
```bash
cargo test --lib
cargo test --test '*'
cargo clippy -- -D warnings
cargo fmt -- --check
cargo doc --no-deps
```

## 📊 监控性能

### 查看基准结果
1. GitHub Actions → Benchmarks → 最新运行
2. 下载构件 → benchmark-results
3. 打开 `target/criterion/report/index.html`

### 对比性能
1. 在 PR 中查看 Benchmarks 注释
2. 寻找红色警告（> 200% 回归）
3. 查看详细报告链接

## 🐛 故障排查

### 测试失败
```bash
# 本地复现
git checkout <branch>
cargo test --verbose

# 查看工作流日志
# Actions → 失败的工作流 → 展开 "Run tests" 步骤
```

### 发布失败
- [ ] 检查版本格式 (v0.1.0)
- [ ] 检查 CARGO_REGISTRY_TOKEN 是否有效
- [ ] 确保所有测试通过
- [ ] 查看 publish.yml 的日志

### 文档未更新
- [ ] 运行 `git push origin main`（非标签）
- [ ] 检查 Actions → Docs 工作流
- [ ] 等待 5 分钟后刷新 GitHub Pages

## 🔗 重要链接

- **仓库 Actions**: https://github.com/USER/file-search/actions
- **文档主页**: https://user.github.io/file-search/
- **crates.io**: https://crates.io/crates/file-search
- **API 文档**: https://docs.rs/file-search

## 📖 详细文档

完整配置指南请参考 `CI_CD.md`
