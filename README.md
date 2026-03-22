# 🏦 Giba Bank API | Rust Backend

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-Framework-blue?style=for-the-badge)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)

A **Giba Bank API** é o motor de processamento financeiro desenvolvido em **Rust**. Este projeto foi desenvolvido com o objetivo para explorar a robustez da linguagem em sistemas críticos, focando em segurança de memória, alta performance e precisão decimal absoluta para transações bancárias.

## 🚀 Tecnologias e Bibliotecas

* **Runtime:** [Tokio](https://tokio.rs/) (I/O assíncrono de alta performance)
* **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
* **Banco de Dados:** [PostgreSQL](https://www.postgresql.org/)
* **SQL Toolkit:** [SQLx](https://github.com/launchbadge/sqlx) (Queries verificadas em tempo de compilação)
* **Cálculos Financeiros:** [rust_decimal](https://crates.io/crates/rust_decimal) (Prevenção de erros de arredondamento de ponto flutuante)
* **Autenticação:** JWT (JSON Web Tokens) com `jsonwebtoken`
* **Log & Observabilidade:** `tracing`

## 📋 Pré-requisitos

* [Rust](https://rustup.rs/) (Stable 2021 edition ou superior)
* [Docker](https://www.docker.com/) e Docker Compose
* [SQLx CLI](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (`cargo install sqlx-cli --no-default-features --features native-tls,postgres`)