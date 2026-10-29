# Augur 交易机器人

**🌐 Language / 语言 / Язык:** [English](README.md) · [简体中文](README.zh-CN.md) · [Русский](README.ru.md)

![Status](https://img.shields.io/badge/status-🟢_live-2ea44f?style=flat-square)
[![Engine](https://img.shields.io/badge/engine-shared_core-6e40c9?style=flat-square)](https://github.com/tarpboi/Prediction-Markets-Trading-Bot-Toolkits)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE)
[![CI](https://github.com/tarpboi/Augur/actions/workflows/ci.yml/badge.svg)](https://github.com/tarpboi/Augur/actions/workflows/ci.yml)

> 自动化 **Augur 交易机器人** — 以太坊。隶属 [Prediction Market Toolkits](https://github.com/tarpboi/Prediction-Markets-Trading-Bot-Toolkits) 套件：一个执行核心，一个风控层，覆盖所有平台。

**Augur** 现已在生产环境中**正式上线**。

---

## Augur 实盘运行

<div align="center">

<img width="820" alt="Augur trading bot TUI" src="https://github.com/user-attachments/assets/b6c51ba1-14c6-4582-858c-e9441516dd1d" />
<img width="820" alt="Augur trading bot TUI" src="https://github.com/user-attachments/assets/66d9cb72-e14a-414f-93e5-600fb1d3f49f" />

<sub>共享 TUI 在 Augur 上运行 — 实时持仓、盈亏与熔断状态。 <!-- TODO: swap in Augur-specific captures --></sub>

</div>

---

## Augur 上的策略

这些机器人通过单一平台适配器运行在共享引擎之上 — 相同的风控、相同的 TUI、完整的空跑（dry-run）支持。

| 策略 |
|----------|
| 🎯 **结算狙击** — 95 美分接近确定性 → 保证 1.00 美元结算 |
| 📊 **订单簿失衡** — 信号本身*就是*订单簿，无需外部数据源 |

> 想要 Augur 上未列出的策略？适配器覆盖按需驱动 — [联系我们](https://t.me/tarpboi)。

---

## 快速开始

克隆仓库、填入密钥、运行 —— TUI 让你选择策略。

```bash
git clone https://github.com/tarpboi/Augur.git
cd Augur
cp config.example.yaml config.yaml   # add your keys
cargo run --release                  # launch the TUI
# headless: cargo run --release -- run copy-trading
```

---

## 一个引擎，覆盖所有平台

本仓库是 **Augur** 的入口。执行核心、风控层以及全部 20+ 个平台适配器都位于主工具包中：

### 👉 **[Prediction-Markets-Trading-Bot-Toolkits](https://github.com/tarpboi/Prediction-Markets-Trading-Bot-Toolkits)** — 完整套件

| | |
|---|---|
| **订单执行** | 端到端 < 100 毫秒 |
| **事件处理** | 每事件 < 1 毫秒 |
| **安全** | 熔断器 · 深度保护 · 空跑 · 最小交易额 |
| **平台** | 7 个已上线 · 20+ 个平台 |

新增一个平台只需编写**一个适配器** — 而非重建整个机器人。

---

## 获取访问权限

| 平台 | 链接 |
|----------|------|
| **完整工具包** | [Prediction-Markets-Trading-Bot-Toolkits](https://github.com/tarpboi/Prediction-Markets-Trading-Bot-Toolkits) |
| **Telegram** | [@tarpboi](https://t.me/tarpboi) |
| **讨论区** | [GitHub Discussions](https://github.com/tarpboi/Prediction-Markets-Trading-Bot-Toolkits/discussions) |

*通常在数小时内回复。*

---

## 相关平台

[Kalshi](https://github.com/tarpboi/Kalshi) · [Limitless](https://github.com/tarpboi/Limitless-Exchange) · [Polymarket](https://github.com/tarpboi/Polymarket) · [Azuro](https://github.com/tarpboi/Azuro)

> 在[主工具包 →](https://github.com/tarpboi/Prediction-Markets-Trading-Bot-Toolkits#venue-coverage)中浏览完整平台目录

---

## 免责声明

> 交易预测市场涉及真实的财务风险。本软件按“原样”提供，不附带任何担保，亦不构成投资建议。在投入真实资金前，请务必使用 `enable_trading: false` 进行测试。请确保遵守 Augur 的服务条款及您所在地区的法律法规。

---

<div align="center">

**Augur 交易机器人 · 基于 [Prediction Market Toolkits](https://github.com/tarpboi/Prediction-Markets-Trading-Bot-Toolkits) 引擎构建**

[![Telegram](https://img.shields.io/badge/Telegram-@tarpboi-26A5E4?style=flat-square&logo=telegram)](https://t.me/tarpboi)

</div>
