# KuCoin Rust Client

A robust and asynchronous Rust library for interacting with the KuCoin API. This crate provides typed wrappers for endpoints, allowing you to easily manage spot orders, track deposits, and handle authentication with the KuCoin exchange.

## Features

- **Async/Await Support**: Built on `tokio` and `reqwest` for non-blocking I/O.
- **Spot Trading**: Place market and limit orders, support for batch orders, order cancellation, and **retrieving active orders**.
- **Wallet Management**: Query deposit history and look up deposits by transaction hash.
- **Sub-Account Management**: Get all sub-account info, **check sub-account balances**, create new sub-accounts, configure permissions, and manage IP whitelists programmatically.
- **Typed Requests**: Uses builder patterns for creating requests (e.g., `SpotOrderRequest`, `DepositHistoryRequest`) to ensure type safety.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
kucoin = "0.4.0"
tokio = { version = "1.0", features = ["full"] }
dotenv = "0.15" # Optional: for managing environment variables
```

## Usage

### 1. Client Initialization

To start, you need to initialize the `KuCoinClient` with your API credentials. It is recommended to load these safely from environment variables.

```rust
use std::env;
use kucoin::client::rest::{Credentials, KuCoinClient};

#[tokio::main]
async fn main() {
    // Load credentials from environment variables
    let api_key = env::var("API_KEY").expect("API_KEY not set");
    let api_secret = env::var("API_SECRET").expect("API_SECRET not set");
    let api_passphrase = env::var("API_PASSPHRASE").expect("API_PASSPHRASE not set");

    let credentials = Credentials::new(&api_key, &api_secret, &api_passphrase);
    let client = KuCoinClient::new(credentials);

    // Now you can use `client` to access various endpoints
}

```

### 2. Spot Trading

#### Placing a Single Order

Create a `SpotOrderRequest` and send it using the spot handler.

```rust
use kucoin::types::spot::{SpotOrderRequest, TradeType, Side};

async fn place_spot_order(client: &KuCoinClient) {
    // Create a Market Buy order for BTC-USDT
    let order = SpotOrderRequest::new(TradeType::Market, "BTC-USDT", Side::Buy)
        .set_funds(100.0) // Buy 100 USDT worth
        .set_remark("Bot Order 01");

    match client.spot().place_order(order).await {
        Ok(response) => println!("Order placed successfully: {:#?}", response),
        Err(e) => eprintln!("Error placing order: {:?}", e),
    }
}

```

#### Placing Batch Orders

You can place multiple orders efficiently in a single request.

```rust
use kucoin::types::spot::{BatchSpotContract, SpotOrderRequest, TradeType, Side};

async fn place_batch_orders(client: &KuCoinClient) {
    let btc_order = SpotOrderRequest::new(TradeType::Market, "BTC-USDT", Side::Buy)
        .set_funds(50.0);

    let sol_order = SpotOrderRequest::new(TradeType::Market, "SOL-USDT", Side::Buy)
        .set_funds(20.0);

    let batch = BatchSpotContract::new()
        .add_order(btc_order)
        .add_order(sol_order);

    match client.spot().place_multi_orders(batch).await {
        Ok(response) => println!("Batch orders placed: {:#?}", response),
        Err(e) => eprintln!("Batch order error: {:?}", e),
    }
}

```

#### Canceling an Order

Cancel an existing order using its ID.

```rust
use kucoin::types::spot::SpotCancelRequest;

async fn cancel_order(client: &KuCoinClient, order_id: &str) {
    let cancel_req = SpotCancelRequest::new(order_id, 0.0, "BTC-USDT");

    match client.spot().cancel_order(cancel_req).await {
        Ok(response) => println!("Order canceled: {:#?}", response),
        Err(e) => eprintln!("Cancellation failed: {:?}", e),
    }
}

```

#### Retrieving Open Orders

Fetch a list of active orders for a specific symbol (e.g., BTC-USDT).

```rust
async fn get_open_orders(client: &KuCoinClient) {
    let ticker = "BTC-USDT";

    match client.spot().list_orders_open(ticker).await {
        Ok(response) => {
            if let Some(orders) = response.data {
                println!("Found {} open orders for {}:", orders.len(), ticker);
                for order in orders {
                    println!("Order ID: {}, Price: {}, Size: {}", order.id, order.price, order.size);
                }
            } else {
                println!("No open orders found.");
            }
        },
        Err(e) => eprintln!("Failed to fetch open orders: {:?}", e),
    }
}

```

### 3. Deposit History

Query your deposit history with filters for currency, status, and time range.

```rust
use kucoin::types::deposit::{DepositHistoryRequest, DepositStatus};

async fn get_deposit_history(client: &KuCoinClient) {
    // Search for successful SOL deposits
    let filter = DepositHistoryRequest::new("SOL")
        .set_status(DepositStatus::Success)
        .set_page_size(20);

    match client.deposit().history(filter).await {
        Ok(response) => println!("Deposit History: {:#?}", response),
        Err(e) => eprintln!("Failed to fetch history: {:?}", e),
    }
}

```

### 4. Transaction Lookup

Find a specific deposit record by its wallet transaction hash.

```rust
async fn lookup_tx(client: &KuCoinClient, tx_hash: &str) {
    match client.deposit().by_tx_hash(tx_hash).await {
        Ok(Some(deposit)) => println!("Found deposit: {:#?}", deposit),
        Ok(None) => println!("No deposit found with that hash."),
        Err(e) => eprintln!("Lookup error: {:?}", e),
    }
}

```

### 5. Sub-Account Management

Create new sub-accounts and generate API keys for them directly.

```rust
use kucoin::types::sup_account::{SubAccRequest, Expire};

async fn create_sub_account(client: &KuCoinClient) {
    // Configure the new sub-account
    let request = SubAccRequest::new("SubUser01", "High Freq Bot", "StrongPass123!")
        .set_permission("General,Spot") // Set permissions
        .add_ipwhitelist("192.168.1.1") // Whitelist IP
        .add_ipwhitelist("10.0.0.5")    // Add another IP
        .set_expire(Expire::Never);     // API Key never expires

    // Send the request
    match client.sub_account().add_api(request).await {
        Ok(response) => println!("Sub-account created: {:#?}", response),
        Err(e) => eprintln!("Failed to create sub-account: {:?}", e),
    }
}

```

### 5.1 Listing Sub-Accounts

Use fetchall to retrieve all sub-accounts and find specific details like the user_id (UID) required for fund transfers.

```rust
async fn list_sub_accounts(client: &KuCoinClient) {
    match client.sub_account().fetchall().await {
        Ok(response) => {
            // Assuming the response data contains a list of accounts
            if let Some(sub_accounts) = response.data {
                println!("Found {} sub-accounts:", sub_accounts.len());

                for account in sub_accounts {
                    println!("Name: {:<15} | UID: {}", account.sub_name, account.user_id);

                    // Example: Capture the UID for a specific bot
                    if account.sub_name == "High Freq Bot" {
                        println!(">> Target UID for transfer: {}", account.user_id);
                    }
                }
            }
        }
        Err(e) => eprintln!("Failed to fetch sub-accounts: {:?}", e),
    }
}

```

### 5.2 Checking Sub-Account Balance

Retrieve the balance details for a specific sub-account using its User ID.

```rust
async fn check_sub_balance(client: &KuCoinClient, sub_user_id: &str) {
    match client.sub_account().balance(sub_user_id).await {
        Ok(response) => {
            println!("Balance info: {:#?}", response.data);
        },
        Err(e) => eprintln!("Failed to fetch sub-account balance: {:?}", e),
    }
}

```

## Project Structure

- `src/client`: Handles authentication and HTTP request logic.
- `src/endpoints`: Contains specific API implementation (Spot, Deposit, etc.).
- `src/types`: Request and response data structures.
- `src/utils`: Helper functions for error handling and data formatting.

## Contributing

Contributions are welcome!
