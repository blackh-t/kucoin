# KuCoin Rust Client

An **asynchronous, strongly typed Rust client** for the KuCoin REST API.

Designed for reliability and production use, this crate provides ergonomic wrappers for KuCoin endpoints covering **spot trading**, **wallet activity**, and **sub-account management**.

---

## Features

- **Async-first** — built on `tokio` and `reqwest`
- **Spot trading**
  - Market & limit orders
  - Batch orders
  - Order cancellation
  - Retrieve open orders

- **Wallet & deposits**
  - Deposit history with filters
  - Lookup by transaction hash

- **Sub-accounts**
  - List sub-accounts
  - Create API keys
  - Configure permissions & IP whitelists
  - Query sub-account balances

- **Typed requests**
  - Builder-pattern request structs
  - Compile-time parameter validation

---

## Installation

```toml
[dependencies]
kucoin = "0.4.0"
tokio = { version = "1", features = ["full"] }
dotenv = "0.15" # optional
```

---

## Quick Start

### Client Initialization

```rust
use std::env;
use kucoin::client::rest::{Credentials, KuCoinClient};

#[tokio::main]
async fn main() {
    let credentials = Credentials::new(
        &env::var("API_KEY").expect("API_KEY not set"),
        &env::var("API_SECRET").expect("API_SECRET not set"),
        &env::var("API_PASSPHRASE").expect("API_PASSPHRASE not set"),
    );

    let client = KuCoinClient::new(credentials);
}
```

---

## Spot Trading

### Place an Order

```rust
use kucoin::types::spot::{SpotOrderRequest, TradeType, Side};

async fn place_order(client: &KuCoinClient) {
    let order = SpotOrderRequest::new(TradeType::Market, "BTC-USDT", Side::Buy)
        .set_funds(100.0)
        .set_remark("example-order");

    client.spot().place_order(order).await.unwrap();
}
```

---

### Batch Orders

```rust
use kucoin::types::spot::{BatchSpotContract, SpotOrderRequest, TradeType, Side};

async fn batch_orders(client: &KuCoinClient) {
    let batch = BatchSpotContract::new()
        .add_order(
            SpotOrderRequest::new(TradeType::Market, "BTC-USDT", Side::Buy)
                .set_funds(50.0),
        )
        .add_order(
            SpotOrderRequest::new(TradeType::Market, "SOL-USDT", Side::Buy)
                .set_funds(20.0),
        );

    client.spot().place_multi_orders(batch).await.unwrap();
}
```

---

### Cancel & List Orders

```rust
use kucoin::types::spot::SpotCancelRequest;

async fn cancel(client: &KuCoinClient, order_id: &str) {
    let req = SpotCancelRequest::new(order_id, 0.0, "BTC-USDT");
    client.spot().cancel_order(req).await.unwrap();
}

async fn open_orders(client: &KuCoinClient) {
    let res = client.spot().list_orders_open("BTC-USDT").await.unwrap();
    println!("{:#?}", res.data);
}
```

---

## Deposits

```rust
use kucoin::types::deposit::{DepositHistoryRequest, DepositStatus};

async fn deposits(client: &KuCoinClient) {
    let req = DepositHistoryRequest::new("SOL")
        .set_status(DepositStatus::Success)
        .set_page_size(20);

    client.deposit().history(req).await.unwrap();
}
```

```rust
async fn lookup(client: &KuCoinClient, tx: &str) {
    let res = client.deposit().by_tx_hash(tx).await.unwrap();
    println!("{:#?}", res);
}
```

---

## Sub-Accounts

```rust
use kucoin::types::sup_account::{SubAccRequest, Expire};

async fn create_sub(client: &KuCoinClient) {
    let req = SubAccRequest::new("SubUser01", "Trading Bot", "StrongPass123!")
        .set_permission("General,Spot")
        .add_ipwhitelist("192.168.1.1")
        .set_expire(Expire::Never);

    client.sub_account().add_api(req).await.unwrap();
}
```

```rust
async fn list_subs(client: &KuCoinClient) {
    let res = client.sub_account().fetchall().await.unwrap();
    println!("{:#?}", res.data);
}
```

```rust
async fn balance(client: &KuCoinClient, uid: &str) {
    let res = client.sub_account().balance(uid).await.unwrap();
    println!("{:#?}", res.data);
}
```

---

## Real-World Usage

This crate is used in **CoinBot**, an automated crypto trading platform built in Rust.

- Website: [https://coinbot.locker](https://coinbot.locker)
- Project: CoinBot (private / production system)

This serves as a production reference for high-frequency and automated trading workloads.

---

## Project Structure

- `client/` — authentication & HTTP layer
- `endpoints/` — KuCoin API endpoints
- `types/` — request and response models
- `utils/` — shared helpers

---

## Contributing

Issues and pull requests are welcome.

---
