# RideNow-Backend 🚗

A modern ride-sharing backend service built with Rust, featuring robust user and driver management, payment processing

## 📋 Table of Contents

- [Features](#features)
- [Tech Stack](#tech-stack)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Database Setup](#database-setup)
- [Running the Application](#running-the-application)
- [API Endpoints](#api-endpoints)
- [Project Structure](#project-structure)
- [Contributing](#contributing)

## ✨ Features

- **User Management**: Complete CRUD operations for users
- **Driver Management**: Complete CRUD operations for users
- **Payment Processing**: Secure payment handling for users and drivers
- **RESTful API**: Clean, well-structured API endpoints
- **Database Migrations**: Version-controlled schema management
- **Logging**: Comprehensive logging with tracing

## 🛠 Tech Stack

- **Language**: Rust
- **Web Framework**: [Axum](https://github.com/tokio-rs/axum) 0.8.8
- **Database**: PostgreSQL with [SQLx](https://github.com/launchbadge/sqlx) 0.8.6
- **Async Runtime**: [Tokio](https://tokio.rs/) 1.49.0
- **Serialization**: Serde 1.0.228
- **Logging**: Tracing & Tracing-Subscriber
- **UUID Generation**: UUID v4
- **Environment Variables**: dotenvy 0.15.7

## 📦 Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [PostgreSQL](https://www.postgresql.org/download/) (version 12 or higher)
- [SQLx CLI](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (for database migrations)

Install SQLx CLI:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

## 🚀 Installation

1. **Clone the repository**

```bash
git clone https://github.com/AbelOsaretin/RideNow-Backend.git
cd RideNow-Backend
```

2. **Install dependencies**

```bash
cargo build
```

## ⚙️ Configuration

1. **Create a `.env` file** in the project root:

```env
DATABASE_URL=postgres://username:password@localhost/ridenow_db
PORT=3000
```

2. **Adjust the configuration** based on your PostgreSQL setup:
   - Replace `username` with your PostgreSQL username
   - Replace `password` with your PostgreSQL password
   - Replace `ridenow_db` with your desired database name

## 🗄️ Database Setup

1. **Create the database**

```bash
createdb ridenow_db
```

Or using PostgreSQL:

```sql
CREATE DATABASE ridenow_db;
```

2. **Run migrations**

```bash
sqlx migrate run
```

The migrations will create the following tables:

- `users` - Rider information and authentication
- `drivers` - Driver profiles, vehicles, and availability
- `user_payments` - User payment records
- `driver_payments` - Driver payment and earnings records

## 🏃 Running the Application

### Development Mode

```bash
cargo run
```

### Production Mode

```bash
cargo build --release
./target/release/RideNow-Backend
```

The server will start on `http://127.0.0.1:3000` by default.

## 🔌 API Endpoints

### Root

- `GET /` - Health check endpoint

### Users

- `GET /users` - List all users
- `POST /users` - Create a new user
- `GET /users/{id}` - Get a specific user
- `PUT /users/{id}` - Update a user (full update)
- `PATCH /users/{id}` - Update a user (partial update)
- `DELETE /users/{id}` - Delete a user

### Drivers

- `GET /drivers` - List all drivers
- `POST /drivers` - Create a new driver
- `GET /drivers/{id}` - Get a specific driver
- `PUT /drivers/{id}` - Update a driver (full update)
- `PATCH /drivers/{id}` - Update a driver (partial update)
- `DELETE /drivers/{id}` - Delete a driver

### Payments

- `GET /payments/health` - Payments endpoint health check
- `GET /payments` - List all payments
- `POST /payments/initialize` - Process a new payment
- `POST /payments/initialize/redirect` - Redirects straight to payment page.
- `POST /payments/webhook` - To receive payment webhook request.
- `GET /payments/user/{user_id}` - Get payment details of a single user.
- `GET /payments/driver/{driver_id}` - Get payment details of a single driver.

## 📁 Project Structure

```
RideNow-Backend/
├── src/
│   ├── main.rs                    # Application entry point
│   ├── handlers/                  # Request handlers
│   │   ├── mod.rs
│   │   ├── user_handlers.rs
│   │   ├── driver_handlers.rs
│   │   └── payment_handlers.rs
│   ├── models/                    # Data models
│   │   ├── mod.rs
│   │   ├── user_model.rs
│   │   ├── driver_model.rs
│   │   └── payment_model.rs
│   ├── routes/                    # Route definitions
│   │   ├── mod.rs
│   │   ├── user_route.rs
│   │   ├── driver_route.rs
│   │   └── payment_route.rs
│   └── services/                  # Business logic & database operations
│       ├── mod.rs
│       ├── database_service.rs
│       ├── user_service.rs
│       ├── driver_service.rs
│       └── payment_service.rs
├── migrations/                    # Database migrations
│   ├── 202602240001_create_users.sql
│   ├── 202602240002_create_drivers.sql
│   ├── 202602250001_create_user_payments.sql
│   └── 202602250002_create_driver_payments.sql
├── Cargo.toml                     # Rust dependencies
└── README.md
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is part of the Web3Bridge Rust learning curriculum.

## 👨‍💻 Author

**Abel Osaretin**

- GitHub: [@AbelOsaretin](https://github.com/AbelOsaretin)

## 🙏 Acknowledgments

- Web3Bridge Rust Cohort - Week 5 Assignment
- The Rust community for excellent documentation and tooling
- Axum and SQLx teams for their amazing frameworks

---

Built with ❤️ using Rust 🦀
