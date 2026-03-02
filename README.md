# Collaborative Task App Backend

Backend part of the collaborative task app, to refer to frontend part please use this link: [Frontend](https://github.com/AlonsoAcunaGuerrero/colaborative-task-app-frontend)

This project was built for performance so I chose Rust as programming language and Actix-Web as Backend Framework thanks to the documentation and the community behind them. For data persistence, I selected PostgreSQL as the database.

## Technologies

- Actix Web
- SQLX
- JsonWebToken (JWT)
- PostgreSQL 

## Install and Configure

### 1. Prerequisites

Before you begin, ensure you have the following installed:
* **Rust** (Latest stable version): [Install Rust](https://www.rust-lang.org/tools/install)
* **PostgreSQL** (v15 or higher): [Download PostgreSQL](https://www.postgresql.org/download/)
* **Cargo**: (Usually comes with Rust)

### 2. Installation & Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/AlonsoAcunaGuerrero/colaborative-task-app-backend.git
   cd colaborative-task-app-backend
   ```

2. **Database Setup:**

   Use the sql file *`setup.sql`* to create all the needed tables.

3. **Environment Variables:**

   Use the next code to configure the backend:

      ```bash
      DATABASE_URL=postgres://postgres:pass@localhost:5433/postgres
      PASSWORD_ENCRYPTION_KEY=YOUR_ENCRYPTION_KEY
      SECRET_KEY=YOUR_SECRET_KEY
      ```
