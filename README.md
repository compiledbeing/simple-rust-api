Simple Rust API — Axum + JWT + PostgreSQL

A clean, modular backend API written in Rust using Axum, featuring JWT authentication, secure password hashing, and PostgreSQL integration via SQLx.

This project is built as a portfolio-grade backend service demonstrating real-world backend patterns: authentication, middleware, database access, and idiomatic Rust architecture.

✨ Features

✅ Axum-based HTTP API

🔐 JWT Authentication (Access Token)

🔑 Secure password hashing (Argon2)

🧱 Middleware-based route protection

🗄️ PostgreSQL database integration

⚡ Async SQL access with SQLx

🧩 Modular, scalable project structure

📦 Environment-based configuration

🚀 Production-oriented backend foundation

## Tech Stack

Language: Rust 🦀

Web Framework: Axum

Async Runtime: Tokio

Authentication: JSON Web Tokens (JWT)

Password Hashing: Argon2

Database: PostgreSQL

ORM / Query Layer: SQLx

Configuration: dotenv + environment variables

## Current Scope & Limitations
This project focuses on backend fundamentals:

❌ No refresh tokens yet

❌ No role-based authorization

❌ No pagination or filtering

These are intentional and can be added incrementally.

## Possible Future Improvements
Refresh token rotation

Role-Based Access Control (RBAC)

Database migrations with SQLx

Request validation layer

OpenAPI / Swagger documentation

Rate limiting and logging middleware
