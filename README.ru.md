# Торговый бот Augur

**🌐 Language / 语言 / Язык:** [English](README.md) · [简体中文](README.zh-CN.md) · [Русский](README.ru.md)

[![Engine](https://img.shields.io/badge/engine-shared_core-6e40c9?style=flat-square)](https://github.com/tarpboi/Prediction-Markets-Trading-Bot-Toolkits)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE)
[![CI](https://github.com/tarpboi/Augur/actions/workflows/ci.yml/badge.svg)](https://github.com/tarpboi/Augur/actions/workflows/ci.yml)

> **Торговый бот Augur** — Ethereum. Часть набора [Prediction Market Toolkits](https://github.com/tarpboi/Prediction-Markets-Trading-Bot-Toolkits): одно ядро исполнения, один слой риска, все площадки.

Площадка **Augur** сопоставлена с общим движком, но её адаптер пока не подключён. Этот репозиторий хранит метаданные площадки; торговый код находится в основном наборе, который сегодня исполняет сделки на Polymarket.

---

## Стратегии на Augur

Это стратегии, сопоставленные с Augur. Они заработают здесь, когда адаптер будет подключён — те же риск-контроли, тот же TUI, полная поддержка dry-run.

| Стратегия |
|----------|
| 🎯 **Снайпер разрешения** — покупка почти определённых исходов перед завершением, удержание до выплаты |
| 📊 **Дисбаланс стакана** — сигнал *и есть* стакан заявок, без внешних фидов |

> Нужна стратегия, не указанная здесь для Augur? Покрытие адаптеров определяется спросом — [напишите](https://t.me/tarpboi).

---

## Быстрый старт

Торговый код находится в основном наборе. Клонируйте его, добавьте ключи и начните с dry-run. Сегодня он работает с Polymarket.

```bash
git clone https://github.com/tarpboi/Prediction-Markets-Trading-Bot-Toolkits.git
cd Prediction-Markets-Trading-Bot-Toolkits
cp config.yaml.example config.yaml        # add your keys
cargo run --release -- run copy-trading   # dry-run: enable_trading is false by default
```

---

## Один движок, все площадки

Ядро исполнения и слой риска находятся в основном наборе:

### 👉 **[Prediction-Markets-Trading-Bot-Toolkits](https://github.com/tarpboi/Prediction-Markets-Trading-Bot-Toolkits)** — полный набор

| | |
|---|---|
| **Исполнение ордеров** | < 100 мс от начала до конца |
| **Обработка событий** | < 1 мс на событие |
| **Безопасность** | Предохранитель · защита глубины · dry-run · минимум сделки |
| **Площадки** | Сопоставлено площадок: 21 |

Добавить площадку — значит написать **один адаптер**, а не пересобирать бота.

---

## Получить доступ

| Платформа | Ссылка |
|----------|------|
| **Полный набор** | [Prediction-Markets-Trading-Bot-Toolkits](https://github.com/tarpboi/Prediction-Markets-Trading-Bot-Toolkits) |
| **Telegram** | [@tarpboi](https://t.me/tarpboi) |
| **Обсуждения** | [GitHub Discussions](https://github.com/tarpboi/Prediction-Markets-Trading-Bot-Toolkits/discussions) |

*Обычно отвечаем в течение нескольких часов.*

---

## Связанные площадки

[Kalshi](https://github.com/tarpboi/Kalshi) · [Limitless](https://github.com/tarpboi/Limitless-Exchange) · [Polymarket](https://github.com/tarpboi/Polymarket) · [Predict.fun](https://github.com/tarpboi/Predict.fun)

> Полный каталог площадок — в [основном наборе →](https://github.com/tarpboi/Prediction-Markets-Trading-Bot-Toolkits#venue-coverage)

---

## Отказ от ответственности

> Торговля на рынках предсказаний сопряжена с реальным финансовым риском. ПО предоставляется «как есть», без гарантий, и не является финансовой консультацией. Всегда тестируйте с `enable_trading: false` перед вводом реального капитала. Соблюдайте условия использования Augur и местное законодательство.

---

<div align="center">

**Торговый бот Augur · построен на движке [Prediction Market Toolkits](https://github.com/tarpboi/Prediction-Markets-Trading-Bot-Toolkits)**

[![Telegram](https://img.shields.io/badge/Telegram-@tarpboi-26A5E4?style=flat-square&logo=telegram)](https://t.me/tarpboi)

</div>
