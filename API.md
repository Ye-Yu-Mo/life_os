# Life OS - API Documentation

## Authentication

All protected endpoints require a JSON Web Token (JWT) in the `Authorization` header.

**Header Format:**
```
Authorization: Bearer <your_token>
```

## Base URL
```
http://127.0.0.1:3000
```

---

## Auth Endpoints

### 1. Register

Register a new user account.

**Endpoint:** `POST /register`

**Request Body:**
```json
{
  "username": "alice",
  "password": "secret123"
}
```

**Success Response:**
```json
{
  "id": "uuid",
  "username": "alice",
  "token": "eyJhbGciOiJIUzI1Ni..."
}
```

### 2. Login

Authenticate and receive an access token.

**Endpoint:** `POST /login`

**Request Body:**
```json
{
  "username": "alice",
  "password": "secret123"
}
```

**Success Response:**
```json
{
  "id": "uuid",
  "username": "alice",
  "token": "eyJhbGciOiJIUzI1Ni..."
}
```

---

## Account Endpoints

### 1. Create Account

**Endpoint:** `POST /accounts`

**Request Body:**
```json
{
  "name": "My Bank Card",
  "type": "bank_card",
  "currency_code": "USD"
}
```

**Response:**
```json
{
  "id": "uuid",
  "name": "My Bank Card",
  "type": "bank_card",
  "currency_code": "USD",
  "created_at": "2023-10-27T10:00:00Z",
  "updated_at": "2023-10-27T10:00:00Z"
}
```

### 2. List Accounts

**Endpoint:** `GET /accounts`

**Response:**
```json
[
  {
    "id": "uuid",
    "name": "My Bank Card",
    "type": "bank_card",
    "currency_code": "USD",
    ...
  }
]
```

### 3. Get Account

**Endpoint:** `GET /accounts/:account_id`

### 4. Update Account

**Endpoint:** `PUT /accounts/:account_id`

**Request Body:**
```json
{
  "name": "Updated Name",
  "type": "cash", 
  "currency_code": "EUR"
}
```
(All fields are optional)

### 5. Delete Account

**Endpoint:** `DELETE /accounts/:account_id`

---

## Transaction Endpoints

### 1. Create Transaction

**Endpoint:** `POST /transactions`

**Request Body:**
```json
{
  "from_account_id": "uuid", 
  "to_account_id": "uuid",
  "amount": "100.00",
  "currency_code": "USD",
  "txn_type": "expense", 
  "category": "Food",
  "occurred_at": "2023-10-27T10:00:00Z"
}
```

### 2. List Transactions

**Endpoint:** `GET /transactions`

**Query Parameters:**
- `account_id`: Filter by account (optional)
- `start_date`: Filter by start date (optional)
- `end_date`: Filter by end date (optional)

---

## Holdings Endpoints

### 1. Create Holding

**Endpoint:** `POST /holdings`

**Request Body:**
```json
{
  "account_id": "uuid",
  "asset_type": "stock",
  "symbol": "AAPL",
  "quantity": "10",
  "cost_basis_total": "1500.00",
  "currency_code": "USD"
}
```

### 2. List Holdings

**Endpoint:** `GET /holdings`

**Query Parameters:**
- `account_id`: Filter by account (optional)
- `asset_type`: Filter by asset type (optional)