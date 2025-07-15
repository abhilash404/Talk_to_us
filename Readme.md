# 🛠️ Ticket Support System API

A RESTful API for a customer support ticketing system, built using **Rust**, **Axum**, **SeaORM**, and **PostgreSQL**. It handles user registration, login with JWT authentication, ticket creation, and admin/agent ticket management.

---

## 📚 Table of Contents

- [📦 Tech Stack](#-tech-stack)
- [🚀 Getting Started](#-getting-started)
  - [🔧 Prerequisites](#-prerequisites)
  - [📥 Installation](#-installation)
  - [🧬 Database Setup](#-database-setup)
- [📂 Project Structure](#-project-structure)
- [🔐 Authentication & Roles](#-authentication--roles)
- [🌐 API Endpoints](#-api-endpoints)
  - [👥 Auth (Login & Register)](#-auth-login--register)
  - [🎫 Ticket Operations](#-ticket-operations)
- [🛡️ CORS Configuration](#-cors-configuration)
- [📌 Notes](#-notes)
- [📖 License](#-license)

---

## 📦 Tech Stack

| Component        | Technology          |
|------------------|---------------------|
| Language         | Rust                |
| Web Framework    | Axum                |
| ORM              | SeaORM              |
| DB               | PostgreSQL          |
| Auth             | JWT + Argon2        |
| Password Hashing | Argon2              |
| HTTP Server      | Tokio + Hyper       |
| Middleware       | Tower HTTP CORS     |

---

## 🚀 Getting Started

### 🔧 Prerequisites

- Rust (latest stable)
- PostgreSQL
- `sqlx-cli` or `sea-orm-cli` (optional)
- Node.js (optional, for frontend)

### 📥 Installation

```bash
git clone https://github.com/your-username/ticket-support-system.git
cd ticket-support-system
cargo build
```

🧬 Database Setup

1. Create a database named Talk in PostgreSQL:
```
CREATE DATABASE "Talk";
```
2. Set your credentials properly in main.rs:

Database::connect("postgres://postgres:%40bhil%40%24h12@localhost:5432/Talk")

3. Generate entities (SeaORM):



sea-orm-cli generate entity -u postgres://postgres:@bhil@$h12@localhost:5432/Talk -o src/entities

4. Create required tables:



-users

-tickets

-replies


Refer to the entities module for schema definitions.


---

📂 Project Structure

src/
├── main.rs              # Entry point
├── login.rs             # Auth logic (JWT, register/login)
├── handler.rs           # Ticket retrieval and filtering
├── ticket.rs            # Ticket and reply creation
├── entities/            # SeaORM generated entity files


---

🔐 Authentication & Roles

Users authenticate via JWT tokens.

Passwords are hashed using Argon2.

Roles:

customer: Can create tickets.

agent, admin: Can respond to tickets, filter by status, view summaries.



JWT token payload structure:

{
  "sub": 1,
  "role": "admin",
  "exp": 1716762892
}

Include the token in requests via:

Authorization: Bearer <your-jwt-token>


---

🌐 API Endpoints

👥 Auth (Login & Register)

POST /register

Registers a new user.

Body:

{
  "username": "John",
  "email": "john@example.com",
  "password": "securepassword",
  "role": "customer"
}

POST /login

Returns a JWT token for valid credentials.

Body:

{
  "email": "john@example.com",
  "password": "securepassword"
}


---

🎫 Ticket Operations

POST /create

Create a new ticket. (Role: customer)

{
  "title": "Can't access my account",
  "description": "I've tried resetting password",
  "priority": "high"
}

POST /reply

Reply to a ticket. (Role: agent/admin)

{
  "reply_text": "Please try resetting via OTP",
  "ticket_id": 3
}

GET /tickets

Returns all tickets. No auth required.

GET /tickets/:user_id

Returns ticket summaries (title, priority, status, created_at) for a given user. (Role: agent/admin)

GET /tickets/:user_id/resolved

Returns only resolved (closed) tickets for a user. (Role: agent/admin)

GET /tickets/:user_id/open

📌 Not yet implemented


---

🛡️ CORS Configuration

Allows frontend access from:

http://127.0.0.1:5501

Configured in:

CorsLayer::new()
    .allow_origin("http://127.0.0.1:5501".parse::<HeaderValue>().unwrap())
    .allow_methods(Any)
    .allow_headers(Any);


---

📌 Notes

Ensure your-secret-key is securely stored using .env or a secret manager.

DB credentials should never be hardcoded in production.

The open_tickets endpoint is currently empty – you may implement it similar to resolved() by filtering status = "open".



---

📖 License

MIT © Abhilash Praharaj

---

