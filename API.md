# Life OS - API 文档

## 身份验证 (Authentication)

所有受保护的接口都需要在 `Authorization` 请求头中携带 JSON Web Token (JWT)。

**Header 格式:**
```
Authorization: Bearer <your_token>
```

## 基础 URL (Base URL)
```
http://127.0.0.1:3000
```

---

## 认证接口 (Auth Endpoints)

### 1. 注册 (Register)

注册一个新的用户账号。

**接口:** `POST /register`

**请求体:**
```json
{
  "username": "alice",
  "password": "secret123"
}
```

**成功响应:**
```json
{
  "id": "uuid",
  "username": "alice",
  "token": "eyJhbGciOiJIUzI1Ni..."
}
```

### 2. 登录 (Login)

验证用户身份并获取访问令牌 (Token)。

**接口:** `POST /login`

**请求体:**
```json
{
  "username": "alice",
  "password": "secret123"
}
```

**成功响应:**
```json
{
  "id": "uuid",
  "username": "alice",
  "token": "eyJhbGciOiJIUzI1Ni..."
}
```

---

## 账户接口 (Account Endpoints)

### 1. 创建账户 (Create Account)

**接口:** `POST /accounts`

**请求体:**
```json
{
  "name": "我的银行卡",
  "type": "bank_card",
  "currency_code": "USD"
}
```

**响应:**
```json
{
  "id": "uuid",
  "name": "我的银行卡",
  "type": "bank_card",
  "currency_code": "USD",
  "created_at": "2023-10-27T10:00:00Z",
  "updated_at": "2023-10-27T10:00:00Z"
}
```

### 2. 获取账户列表 (List Accounts)

**接口:** `GET /accounts`

**响应:**
```json
[
  {
    "id": "uuid",
    "name": "我的银行卡",
    "type": "bank_card",
    "currency_code": "USD",
    ...
  }
]
```

### 3. 获取账户详情 (Get Account)

**接口:** `GET /accounts/:account_id`

### 4. 更新账户 (Update Account)

**接口:** `PUT /accounts/:account_id`

**请求体:**
```json
{
  "name": "更新后的名称",
  "type": "cash", 
  "currency_code": "EUR"
}
```
(所有字段均为可选)

### 5. 删除账户 (Delete Account)

**接口:** `DELETE /accounts/:account_id`

---

## 交易/流水接口 (Transaction Endpoints)

### 1. 创建交易 (Create Transaction)

**接口:** `POST /transactions`

**请求体:**
```json
{
  "from_account_id": "uuid", 
  "to_account_id": "uuid",
  "amount": "100.00",
  "currency_code": "USD",
  "txn_type": "expense", 
  "category": "餐饮",
  "occurred_at": "2023-10-27T10:00:00Z"
}
```

### 2. 获取交易列表 (List Transactions)

**接口:** `GET /transactions`

**查询参数 (Query Parameters):**
- `account_id`: 按账户筛选 (可选)
- `start_date`: 按开始日期筛选 (可选)
- `end_date`: 按结束日期筛选 (可选)

---

## 资产/持仓接口 (Holdings Endpoints)

### 1. 创建持仓 (Create Holding)

**接口:** `POST /holdings`

**请求体:**
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

### 2. 获取持仓列表 (List Holdings)

**接口:** `GET /holdings`

**查询参数 (Query Parameters):**
- `account_id`: 按账户筛选 (可选)
- `asset_type`: 按资产类型筛选 (可选)
