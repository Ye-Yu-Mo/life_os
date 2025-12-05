# Life OS - API Documentation

## Base URL
```
http://127.0.0.1:3000
```

## Endpoints

### 1. User Registration

Register a new user account.

**Endpoint:** `POST /register`

**Request Headers:**
```
Content-Type: application/json
```

**Request Body:**
```json
{
  "username": "string",
  "password": "string"
}
```

**Success Response:**
- **Status Code:** `200 OK`
- **Body:**
```json
{
  "id": "uuid",
  "username": "string"
}
```

**Error Responses:**

| Status Code | Description | Response Body |
|-------------|-------------|---------------|
| 400 Bad Request | Registration failed (username already exists) | `{"error": "Registration failed"}` |
| 500 Internal Server Error | Server error | `{"error": "Internal server error"}` |

**Example:**
```bash
curl -X POST http://127.0.0.1:3000/register \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","password":"secret123"}'
```

**Notes:**
- Usernames are case-insensitive (automatically converted to lowercase)
- Username whitespace is trimmed
- Password is hashed using Argon2 before storage

---

### 2. User Login

Authenticate a user and retrieve their information.

**Endpoint:** `POST /login`

**Request Headers:**
```
Content-Type: application/json
```

**Request Body:**
```json
{
  "username": "string",
  "password": "string"
}
```

**Success Response:**
- **Status Code:** `200 OK`
- **Body:**
```json
{
  "id": "uuid",
  "username": "string"
}
```

**Error Responses:**

| Status Code | Description | Response Body |
|-------------|-------------|---------------|
| 401 Unauthorized | Authentication failed (invalid username or password) | `{"error": "Authentication failed"}` |
| 500 Internal Server Error | Server error | `{"error": "Internal server error"}` |

**Example:**
```bash
curl -X POST http://127.0.0.1:3000/login \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","password":"secret123"}'
```

**Notes:**
- Username lookup is case-insensitive
- Failed login does not distinguish between "user not found" and "wrong password" to prevent user enumeration attacks

---

## Security Considerations

1. **Password Hashing**: All passwords are hashed using Argon2 with random salts
2. **Case-Insensitive Usernames**: Stored as lowercase with unique constraint on `LOWER(username)`
3. **No User Enumeration**: Registration and login errors are intentionally vague
4. **Database Constraints**: Unique username constraint enforced at database level

---

## Error Response Format

All error responses follow this format:
```json
{
  "error": "error message"
}
```

---

## Database Schema

### User Table
```sql
CREATE TABLE "user" (
    id            UUID PRIMARY KEY,
    username      VARCHAR(50) NOT NULL,
    password_hash VARCHAR(256) NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_user_username_lower ON "user" (LOWER(username));
```

---

## Development

### Start Server
```bash
cd server
cargo run
```

Server will start on `http://127.0.0.1:3000`

### Environment Variables
Create a `.env` file in the `server/` directory:
```
DATABASE_URL=postgres://localhost/life_os
```

### Database Migrations
```bash
sea-orm-cli migrate up
```
