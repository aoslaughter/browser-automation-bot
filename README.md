# 🏸 Tennis Court Reservation Bot

A browser automation bot built in **Rust** to automatically reserve tennis courts at your local club using a booking interface.

> ⚡️ Powered by `thirtyfour` WebDriver, `tokio` async runtime, and robust error handling via `anyhow`.

---

## 📦 Features

- 🖥 Automates court bookings using ChromeDriver (headless or visible)
- 📅 Selects time slots intelligently based on a config file
- 🔐 Loads secrets and credentials from a `.env` file
- 📄 Supports structured config via `config.toml`
- 🔁 Retries intelligently on transient booking failures
- 🪵 Logs to stdout with `env_logger`
