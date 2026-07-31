# PR-Agent 配置指南

本文档说明如何为 AlphaForge 配置 PR-Agent AI 代码审查助手。

## 📋 目录

- [概述](#概述)
- [架构说明](#架构说明)
- [配置步骤](#配置步骤)
- [Secrets 配置](#secrets-配置)
- [可用命令](#可用命令)
- [自动触发规则](#自动触发规则)
- [费用估算](#费用估算)
- [故障排查](#故障排查)
- [安全最佳实践](#安全最佳实践)

---

## 概述

PR-Agent 是一个 AI 驱动的代码审查助手，可以：

- ✅ 自动审查 PR 代码（安全、性能、质量）
- ✅ 生成 PR 描述和标签
- ✅ 提供代码改进建议
- ✅ 回答关于 PR 的问题
- ✅ 生成测试建议
- ✅ 更新 CHANGELOG

### 本配置使用的 API

- **提供商**: 随风AI (api.sfkey.cn)
- **兼容性**: OpenAI API 兼容
- **模型**: GPT-4o（主） / GPT-4o-mini（备选）
- **优势**: 国内访问稳定，无需翻墙

---

## 架构说明

```
┌─────────────────────────────────────────────────────────────┐
│                     GitHub Repository                        │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  .github/workflows/pr-agent.yml                      │    │
│  │  - Trigger: PR opened, comment with /review, etc.   │    │
│  │  - Uses: the-pr-agent/pr-agent@main                 │    │
│  └──────────────────────┬──────────────────────────────┘    │
│                         │                                    │
│  ┌──────────────────────▼──────────────────────────────┐    │
│  │  GitHub Secrets (安全存储)                            │    │
│  │  - OPENAI_KEY: API 密钥                              │    │
│  │  - OPENAI_API_BASE: API 端点 URL                    │    │
│  └──────────────────────┬──────────────────────────────┘    │
└─────────────────────────┼───────────────────────────────────┘
                          │
                          │ HTTPS
                          ▼
          ┌───────────────────────────────┐
          │   api.sfkey.cn (随风AI)        │
          │   OpenAI-Compatible Endpoint   │
          │   Model: gpt-4o, gpt-4o-mini  │
          └───────────────────────────────┘
```

---

## 配置步骤

### 第一步：确认配置文件已创建

以下文件应已在仓库中创建：

1. **`.pr_agent.toml`** - PR-Agent 配置文件
2. **`.github/workflows/pr-agent.yml`** - GitHub Actions 工作流

### 第二步：获取 API 密钥

从 [随风AI](https://api.sfkey.cn) 获取：

1. 注册并登录账号
2. 进入控制台获取 API Key
3. 确认可用余额充足

### 第三步：配置 GitHub Secrets

**重要**: 这些配置将在另一个 GitHub 账号执行，你需要将密钥安全地传递给该账号的维护者。

#### 在目标仓库配置 Secrets

前往：**Settings → Secrets and variables → Actions → New repository secret**

需要配置以下 Secrets：

| Secret 名称 | 值 | 说明 |
|------------|-----|------|
| `OPENAI_KEY` | `sk-xxxxx` | 随风AI 的 API 密钥 |
| `OPENAI_API_BASE` | `https://api.sfkey.cn/v1` | API endpoint URL |

> ⚠️ **Security Warning**:
> - Never commit API keys to the repository
> - Use GitHub Secrets for storage, don't hardcode in workflow files
> - Rotate keys regularly

### 第四步：启用 GitHub Actions

1. 进入仓库 **Settings → Actions → General**
2. 选择 **Allow all actions and reusable workflows**
3. 保存设置

### 第五步：测试配置

创建一个测试 PR，PR-Agent 应该会自动：

1. 分析代码变更
2. 发布审查评论
3. 生成 PR 描述建议

---

## Secrets 配置

### OPENAI_KEY

随风AI 的 API 密钥，格式通常为 `sk-` 开头的字符串。

**获取方式**:
1. 登录 https://api.sfkey.cn
2. 进入「API 密钥」页面
3. 创建或复制密钥

**配置示例**:
```
Name: OPENAI_KEY
Value: sk-xxxxxxxxxxxxxxxxxxxxxxxx
```

### OPENAI_API_BASE

API 端点 URL，使用随风AI 的 OpenAI 兼容端点。

**配置值**:
```
Name: OPENAI_API_BASE
Value: https://api.sfkey.cn/v1
```

**注意**: URL 末尾不要加 `/`，正确是 `/v1` 而不是 `/v1/`

### GITHUB_TOKEN

这是 GitHub 自动提供的，无需手动配置。

---

## 可用命令

在 PR 评论区输入以下命令触发相应功能：

| 命令 | 功能 | 示例 |
|------|------|------|
| `/review` | 完整代码审查 | `/review` |
| `/describe` | 生成 PR 描述 | `/describe` |
| `/improve` | 代码改进建议 | `/improve` |
| `/ask "问题"` | 提问 | `/ask "这个函数的作用是什么？"` |
| `/test` | 生成测试建议 | `/test` |
| `/update_changelog` | 更新变更日志 | `/update_changelog` |
| `/help` | 查看帮助 | `/help` |

### 命令参数

```
# 只审查特定文件
/review --files="src/**/*.ts"

# 指定测试数量
/test --num_tests=5

# 指定测试框架
/test --testing_framework=vitest

# 自定义改进建议
/improve --extra_instructions="关注性能优化"
```

---

## 自动触发规则

PR-Agent 会在以下情况自动运行：

### 自动触发

| 事件 | 动作 | 执行命令 |
|------|------|----------|
| PR 创建 (`opened`) | 自动审查 | `/review` |
| PR 重新打开 (`reopened`) | 自动审查 | `/review` |
| PR 标记为可审查 (`ready_for_review`) | 自动审查 | `/review` |

### 手动触发

在 PR 评论区输入命令（如 `/describe`）手动触发。

### 不触发的情况

- PR 是草稿状态 (`draft: true`)
- PR 被 `ignore_pr_title` 规则匹配
- 作者在 `ignore_pr_authors` 列表中

---

## 费用估算

### Token 消耗估算

| 操作 | 预估 Token | 预估费用 (GPT-4o) |
|------|-----------|------------------|
| 小型 PR (< 100 行) | ~5,000 tokens | ~$0.03 |
| 中型 PR (100-500 行) | ~15,000 tokens | ~$0.08 |
| 大型 PR (500-1500 行) | ~40,000 tokens | ~$0.20 |
| 超大 PR (> 1500 行) | 可能分段处理 | 变化较大 |

### 随风AI 价格参考

请查看 [随风AI 价格页面](https://api.sfkey.cn/#pricing) 获取最新价格。

**建议**:
- 设置 API 消费限额
- 监控使用量
- 大型 PR 谨慎使用 `/improve --extended`

---

## 故障排查

### 问题：PR-Agent 没有响应

**可能原因**:
1. API 密钥未配置或无效
2. API 余额不足
3. GitHub Actions 未启用
4. PR 是草稿状态

**解决方法**:
```bash
# 检查 GitHub Actions 日志
# 进入 Actions 标签页查看错误信息

# 手动触发测试
# 在 PR 评论: /review
```

### 问题：API 调用失败

**常见错误**:

| 错误 | 原因 | 解决方法 |
|------|------|----------|
| `401 Unauthorized` | API Key 无效 | 检查 OPENAI_KEY Secret |
| `402 Payment Required` | 余额不足 | 充值或检查配额 |
| `404 Not Found` | API Base URL 错误 | 检查 OPENAI_API_BASE |
| `429 Rate Limit` | 请求过快 | 稍后重试 |
| `500 Server Error` | 服务端问题 | 联系随风AI 支持 |

### 问题：响应质量不佳

**可能原因**:
- 模型选择不合适
- Token 限制过低
- 上下文不足

**解决方法**:
```toml
# 在 .pr_agent.toml 中调整
[config]
model = "gpt-4o"  # 使用更强的模型
max_model_tokens = 128000  # 增加 token 限制
temperature = 0.1  # 降低随机性

[pr_reviewer]
num_max_findings = 10  # 增加发现数量
extra_instructions = "重点关注安全性和性能"  # 添加自定义指令
```

---

## 安全最佳实践

### 🔐 API 密钥安全

1. **永远不要**将 API 密钥提交到代码仓库
2. **使用** GitHub Secrets 存储敏感信息
3. **定期轮换** API 密钥（建议每 90 天）
4. **限制** API 密钥的权限范围（如果提供商支持）

### 🛡️ 仓库安全

1. **审查** PR-Agent 的建议，不要盲目接受
2. **限制** 对敏感文件的自动审查（通过配置）
3. **启用** 分支保护规则
4. **监控** GitHub Actions 执行日志

### 📋 Secrets 传递指南

当需要将配置交给其他维护者时：

1. **使用安全的通信渠道**传递密钥：
   - 使用端到端加密的消息应用（如 Signal）
   - 使用密码管理器的共享功能
   - 不要通过邮件/即时通讯直接发送明文密钥

2. **传递内容**:
   - API 密钥: `sk-xxxxx`
   - API 端点: `https://api.sfkey.cn/v1`
   - 配置文件位置: `.pr_agent.toml`

3. **确认清单**:
   - [ ] 接收方已创建 GitHub Secrets
   - [ ] 接收方已测试 PR-Agent 工作正常
   - [ ] 传输渠道已销毁消息记录

### ⚠️ 紧急情况处理

如果 API 密钥泄露：

1. **立即**在随风AI 控制台撤销密钥
2. **更新** GitHub Secrets 中的密钥
3. **检查** API 使用日志是否有异常
4. **通知**仓库维护者

---

## 配置文件说明

### `.pr_agent.toml`

仓库根目录的配置文件，定义 PR-Agent 的行为：

```toml
[config]
model = "gpt-4o"  # 使用的模型
response_language = "zh-CN"  # 响应语言

[pr_reviewer]
require_security_review = true  # 启用安全审查
num_max_findings = 5  # 最多发现 5 个问题

[pr_description]
publish_labels = true  # 自动发布标签
generate_ai_title = false  # 不自动生成标题
```

### `.github/workflows/pr-agent.yml`

GitHub Actions 工作流文件，定义触发条件和环境变量。

---

## 相关链接

- [PR-Agent GitHub](https://github.com/Codium-ai/pr-agent)
- [PR-Agent 文档](https://github.com/Codium-ai/pr-agent/blob/main/README.md)
- [随风AI 文档](https://api.sfkey.cn/#docs)
- [AlphaForge CI/CD 文档](./CI_CD.md)

---

## 更新日志

| 日期 | 变更 |
|------|------|
| 2026-08-01 | 初始配置，使用 api.sfkey.cn 端点 |

---

**需要帮助?** 在 [GitHub Discussions](https://github.com/BerryUIKI/alpha-forge/discussions) 提问。