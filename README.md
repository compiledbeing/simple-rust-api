Simple Rust API — Axum JWT Backend

A clean, modular backend API written in Rust using Axum, featuring JWT authentication, password hashing, and a scalable project structure suitable for real-world backend services.

This project is designed as a portfolio-grade backend demonstrating secure authentication, middleware usage, and idiomatic Rust architecture.

✨ Features:

✅ Axum-based HTTP API

🔐 JWT Authentication (Access Token)

🔑 Secure password hashing (Argon2)

🧱 Middleware-based auth protection

🧩 Modular architecture (auth / handlers / models / middleware)

📦 Environment-based configuration

🚀 Ready to scale into a production backend

src/
├── app.rs              # App initialization
├── main.rs             # Entry point
├── router.rs           # Route definitions
├── state.rs            # Shared application state
│
├── auth/               # Authentication logic
│   ├── jwt.rs          # JWT creation & verification
│   ├── password.rs     # Password hashing & verification
│   └── mod.rs
│
├── middleware/
│   ├── auth.rs         # JWT auth middleware
│   └── mod.rs
│
├── handlers/           # HTTP request handlers
│   ├── auth.rs         # Login / register
│   ├── users.rs        # User-related endpoints
│   └── mod.rs
│
├── models/             # Domain models
│   ├── user.rs
│   └── mod.rs
│
└── error.rs             # Centralized error handling
