# RideNow-Backend 🚗

A modern, scalable ride-sharing backend service built with Rust, featuring robust authentication, user and driver management, payment processing, and transport company administration.

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
- [Security Features](#security-features)
- [Contributing](#contributing)
- [License](#license)
- [Author](#author)
- [Acknowledgments](#acknowledgments)

## ✨ Features

- **Authentication & Authorization**: Secure JWT-based authentication with Argon2 password hashing
- **User Management**: Complete CRUD operations for riders/passengers
- **Driver Management**: Complete CRUD operations for drivers with vehicle assignments
- **Transport Company Management**: Manage transport companies, their vehicles, and driver assignments
- **Vehicle Management**: Track and manage fleet vehicles linked to transport companies
- **Payment Processing**: Secure payment handling for users and drivers with webhook support
- **RESTful API**: Clean, well-structured API endpoints following REST best practices
- **Database Migrations**: Version-controlled schema management with SQLx
- **Comprehensive Logging**: Request tracing and application monitoring with Tracing

## 🛠 Tech Stack

- **Language**: Rust (Edition 2024)
- **Web Framework**: [Axum](https://github.com/tokio-rs/axum) 0.8.8
- **Database**: PostgreSQL with [SQLx](https://github.com/launchbadge/sqlx) 0.8.6
- **Async Runtime**: [Tokio](https://tokio.rs/) 1.49.0
- **Authentication**:
  - [JSON Web Tokens](https://github.com/Keats/jsonwebtoken) (JWT) 9.3
  - [Argon2](https://github.com/RustCrypto/password-hashes) 0.5 for password hashing
- **Serialization**: Serde 1.0.228 with JSON support
- **HTTP Client**: Reqwest 0.12 (for payment processing)
- **Cryptography**:
  - HMAC 0.12.1
  - SHA2 0.10.8
  - Hex 0.4.3
- **Logging & Tracing**: Tracing 0.1.44 & Tracing-Subscriber 0.3.22
- **Date/Time**: Chrono 0.4
- **UUID Generation**: UUID 1.21.0 (v4)
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
JWT_SECRET=your_secure_random_secret_key_here
```

2. **Adjust the configuration** based on your setup:
   - Replace `username` with your PostgreSQL username
   - Replace `password` with your PostgreSQL password
   - Replace `ridenow_db` with your desired database name
   - Replace `your_secure_random_secret_key_here` with a strong, random secret for JWT signing

**Security Note**: Never commit your `.env` file to version control. Ensure it's listed in `.gitignore`.

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

- `users` - User/rider information and authentication
- `drivers` - Driver profiles and information
- `user_payments` - User payment records and transaction history
- `driver_payments` - Driver payment and earnings records
- `transport_companies` - Transport company profiles and details
- `vehicles` - Vehicle information linked to transport companies
- `transport_company_drivers` - Junction table linking drivers to transport companies

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

### Authentication

- `POST /login` - User/driver authentication and JWT token generation

### Users

- `GET /users` - List all users
- `POST /users` - Create a new user
- `GET /users/{id}` - Get a specific user
- `PUT /users/{id}` - Update a user (full update)
- `PATCH /users/{id}` - Update a user (partial update)
- `DELETE /users/{id}` - Delete a user

### Transport Companies

- `GET /transport_companies` - List all transport companies
- `POST /transport_companies` - Create a new transport company
- `GET /transport_companies/{id}` - Get a specific transport company
- `PUT /transport_companies/{id}` - Update a transport company (full update)
- `PATCH /transport_companies/{id}` - Update a transport company (partial update)
- `DELETE /transport_companies/{id}` - Delete a transport company

### Vehicles

- `GET /transport_companies/vehicles` - List all vehicles
- `POST /transport_companies/vehicles` - Add a new vehicle
- `GET /transport_companies/vehicles/{id}` - Get a specific vehicle
- `PUT /transport_companies/vehicles/{id}` - Update a vehicle (full update)
- `PATCH /transport_companies/vehicles/{id}` - Update a vehicle (partial update)
- `DELETE /transport_companies/vehicles/{id}` - Delete a vehicle

### Company-Driver Assignments

- `GET /transport_companies/drivers` - List all company-driver
- `POST /transport_companies/drivers` - Assign a driver to a company
- `GET /transport_companies/drivers/{id}` - Get a specific assignment
- `PUT /transport_companies/drivers/{id}` - Update driver (full update)
- `PATCH /transport_companies/drivers/{id}` - Update driver (partial update)
- `DELETE /transport_companies/drivers/{id}` - Remove a driver

### Payments

- `GET /payments/health` - Payments service health check
- `GET /payments` - List all payments
- `POST /payments/initialize` - Initialize a new payment transaction
- `POST /payments/initialize/redirect` - Initialize payment with redirect to payment gateway
- `POST /payments/webhook` - Webhook endpoint for payment notifications
- `GET /payments/user/{user_id}` - Get payment history for a specific user
- `GET /payments/driver/{driver_id}` - Get payment history for a specific driver

## 📁 Project Structure

```
RideNow-Backend/
├── src/
│   ├── main.rs                          # Application entry point
│   ├── auth/                            # Authentication utilities
│   │   ├── mod.rs
│   │   └── password_utils.rs            # Password hashing with Argon2
│   ├── handlers/                        # Request handlers
│   │   ├── mod.rs
│   │   ├── login_handler.rs             # Authentication handlers
│   │   ├── user_handlers.rs             # User CRUD handlers
│   │   ├── payment_handlers.rs          # Payment processing handlers
│   │   └── transport_company_handlers.rs # Transport company handlers
│   ├── models/                          # Data models & DTOs
│   │   ├── mod.rs
│   │   ├── login_model.rs               # Login request/response models
│   │   ├── user_model.rs                # User data structures
│   │   ├── payment_model.rs             # Payment data structures
│   │   └── transport_company_model.rs   # Transport company models
│   ├── routes/                          # Route definitions
│   │   ├── mod.rs
│   │   ├── login_route.rs               # Authentication routes
│   │   ├── user_route.rs                # User routes
│   │   ├── payment_route.rs             # Payment routes
│   │   └── transport_company_route.rs   # Transport company routes
│   └── services/                        # Business logic layer
│       ├── mod.rs
│       ├── database_service.rs          # Database connection pooling
│       ├── login_service.rs             # Authentication service
│       ├── user_service.rs              # User business logic
│       ├── payment_service.rs           # Payment processing logic
│       └── transport_company_service.rs # Transport company logic
├── migrations/                          # SQLx database migrations
│   ├── 202602240001_create_users.sql
│   ├── 202602240002_create_drivers.sql
│   ├── 202602250001_create_user_payments.sql
│   ├── 202602250002_create_driver_payments.sql
│   ├── 202602260001_create_transport_companies.sql
│   ├── 202602260002_create_vehicles.sql
│   └── 202602260003_create_transport_company_drivers.sql
├── Cargo.toml                           # Project dependencies
├── .env                                 # Environment configuration (not in repo)
└── README.md                            # This file
```

## 🔒 Security Features

- **Password Hashing**: Passwords are hashed using Argon2, a memory-hard password hashing algorithm resistant to GPU cracking attacks
- **JWT Authentication**: Secure token-based authentication for API endpoint protection
- **HMAC Verification**: Webhook signature verification for payment processing
- **Parameterized Queries**: SQLx compile-time checked queries prevent SQL injection
- **Environment Variables**: Sensitive credentials stored securely outside the codebase

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
