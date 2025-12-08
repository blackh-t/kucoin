use uuid::Uuid;

use crate::{
    client::classic_rest::KuCoinClient,
    types::{
        requests::{
            spot_cancel_req_type::SpotQuery,
            spot_contract_req_type::{
                BatchSpotContract, Side, SpotContract, Stp, TimeInForce, TradeType,
            },
        },
        responses::{
            spot_cancel_res_type::SpotCanceledData,
            spot_contract_res_type::{BatchOrderResult, SpotData},
            KuCoinResponse,
        },
    },
    utils::errors::KucoinResults,
};

impl SpotContract {
    /// Create a new payload for spottrade.
    ///
    /// # Attributes
    /// * trade_type - Market/Limit, if limit the 'price' must be set.
    /// * symbol - Trading symbol : BTC-USDT...
    /// * side - Buy/Sell
    ///
    /// # Returns
    /// * A spot contract with undefined fund/size, this can be set with 'set_fund' method.
    pub fn new(trade_type: TradeType, symbol: &str, side: Side) -> Self {
        SpotContract {
            client_oid: Some(Uuid::new_v4().to_string()),
            spot_contract_type: trade_type,
            symbol: symbol.to_string(),
            side,
            // Initialize all other Option fields to None
            allow_max_time_window: None,
            cancel_after: None,
            client_timestamp: None,
            funds: None,
            hidden: None,
            iceberg: None,
            post_only: None,
            price: None,
            remark: None,
            size: None,
            stp: None,
            tags: None,
            time_in_force: None,
            visible_size: None,
        }
    }

    /// Sets the quantity for the order.
    /// Usually required for Limit orders.
    pub fn set_size(mut self, size: f64) -> Self {
        self.size = Some(size.to_string());
        self
    }

    /// Sets the price for the order.
    /// Required for Limit orders.
    pub fn set_price(mut self, price: f64) -> Self {
        self.price = Some(price.to_string());
        self
    }

    /// Sets the funds (quote currency amount) for the order.
    /// Often used for Market Buy orders (e.g., "Buy 100 USDT worth of BTC").
    pub fn set_funds(mut self, funds: f64) -> Self {
        self.funds = Some(funds.to_string());
        self
    }

    /// Sets the Time In Force (e.g., "GTC", "IOC", "FOK").
    pub fn set_time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = Some(time_in_force);
        self
    }

    /// Sets the Post Only flag.
    /// If true, the order will not execute immediately against the market.
    pub fn set_post_only(mut self, post_only: bool) -> Self {
        self.post_only = Some(post_only);
        self
    }

    /// Sets the hidden flag.
    /// If true, the order is not displayed in the order book.
    pub fn set_hidden(mut self, hidden: bool) -> Self {
        self.hidden = Some(hidden);
        self
    }

    /// Sets the Iceberg flag.
    /// If true, only a part of the order is visible.
    pub fn set_iceberg(mut self, iceberg: bool) -> Self {
        self.iceberg = Some(iceberg);
        self
    }

    /// Sets the visible size for Iceberg orders.
    pub fn set_visible_size(mut self, visible_size: f64) -> Self {
        self.visible_size = Some(visible_size.to_string());
        self
    }

    /// Sets a remark/comment for the order.
    pub fn set_remark(mut self, remark: &str) -> Self {
        self.remark = Some(remark.to_string());
        self
    }

    /// Sets Self-Trade Prevention (STP) mode.
    pub fn set_stp(mut self, stp: Stp) -> Self {
        self.stp = Some(stp);
        self
    }

    /// Sets the cancel_after timeout (usually in seconds or milliseconds).
    pub fn set_cancel_after(mut self, cancel_after: i64) -> Self {
        self.cancel_after = Some(cancel_after);
        self
    }

    async fn build(self, client: &mut KuCoinClient) -> KucoinResults<String> {
        client.base_link = "https://api.kucoin.com".to_string();
        client.endpoint = "/api/v1/hf/orders".to_string();

        let json = serde_json::to_string(&self)?;
        Ok(json)
    }
}

impl BatchSpotContract {
    pub fn new() -> Self {
        BatchSpotContract {
            order_list: Vec::new(),
        }
    }

    pub fn add_order(mut self, contract: SpotContract) -> Self {
        self.order_list.push(contract);
        self
    }

    async fn build(self, client: &mut KuCoinClient) -> KucoinResults<String> {
        client.base_link = "https://api.kucoin.com".to_string();
        client.endpoint = "/api/v1/hf/orders/multi".to_string();

        let json = serde_json::to_string(&self)?;
        Ok(json)
    }
}

impl SpotQuery {
    /// Generate cancel partial order contact.
    pub fn new(order_id: &str, cancel_size: f64, symbol: &str) -> Self {
        SpotQuery {
            order_id: order_id.to_string(),
            cancel_size: cancel_size.to_string(),
            symbol: symbol.to_string(),
        }
    }

    async fn build(self, client: &mut KuCoinClient) -> KucoinResults<String> {
        client.base_link = "https://api.kucoin.com".to_string();
        client.endpoint = format!(
            "/api/v1/hf/orders/cancel/{}?symbol={}&cancelSize={}",
            self.order_id, self.symbol, self.cancel_size
        );

        let json = serde_json::to_string(&self)?;
        Ok(json)
    }
}

impl KuCoinClient {
    /// Place a new spot-contract.
    ///
    /// # Attributes
    /// * contract - is a type of 'SpotContract'
    ///
    /// # Returns
    /// * KucoinResults, if 'data' field is None, the order did not went throught
    ///
    /// # Example om creating a contract
    /// ```no_run
    /// use kucoin::types::requests::spot_contract_req_type::{SpotContract, TradeType, Side};
    /// let contract = SpotContract::new(TradeType::Market, "BTC-USDT", Side::Buy)
    ///                     .set_funds(1000.00)
    ///                     .set_remark("syndicate");
    /// ```
    pub async fn send_order(
        &mut self,
        contract: SpotContract,
    ) -> KucoinResults<KuCoinResponse<SpotData>> {
        let payload = contract.build(self).await?;
        let res = self
            .send::<KuCoinResponse<SpotData>>("POST", &payload)
            .await?;
        Ok(res)
    }

    /// Place a batch of spot orders.
    ///
    /// # Attributes
    /// * contracts - A collection of 'SpotContract'.
    ///
    /// # Returns
    /// * 'BatchOrderResult'
    pub async fn send_multi_orders(
        &mut self,
        contracts: BatchSpotContract,
    ) -> KucoinResults<BatchOrderResult> {
        let payload = contracts.build(self).await?;
        let res = self.send::<BatchOrderResult>("POST", &payload).await?;
        Ok(res)
    }

    /// This interface can cancel the specified quantity of the order according to the orderId.
    ///
    /// # Attributes
    /// * contract - SpotQuery.
    ///
    /// # Returns
    /// * order id and the canceled size on success, else error msg.
    pub async fn cancel_partial_order(
        &mut self,
        contract: SpotQuery,
    ) -> KucoinResults<KuCoinResponse<SpotCanceledData>> {
        let _ = contract.build(self).await?;
        let res = self
            .send::<KuCoinResponse<SpotCanceledData>>("DELETE", "")
            .await?;
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::client::classic_rest::Credentials;
    use std::env;

    use super::*;

    #[tokio::test]
    async fn test_send_order() {
        // 1. Setup Credentials
        let credentials = Credentials::new(
            &env::var("api_key").unwrap(),
            &env::var("api_secret").unwrap(),
            &env::var("api_passphrase").unwrap(),
        );

        // 2. Initialize Client
        let mut client = KuCoinClient::new(credentials);

        // 3. Generate SpotContract.
        let open_long_btc = SpotContract::new(TradeType::Market, "BTC-USDT", Side::Buy)
            .set_funds(0.0)
            .set_remark("syndicate");

        // 4. Execute.
        match client.send_order(open_long_btc).await {
            Ok(res) => println!("Trade Order: {:#?}", res),
            Err(e) => println!("Err: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_send_multi_orders() {
        // 1. Setup Credentials
        let credentials = Credentials::new(
            &env::var("api_key").unwrap(),
            &env::var("api_secret").unwrap(),
            &env::var("api_passphrase").unwrap(),
        );

        // 2. Initialize Client
        let mut client = KuCoinClient::new(credentials);

        // 3. Generate SpotContracts.
        let btc_contract = SpotContract::new(TradeType::Market, "BTC-USDT", Side::Buy)
            .set_funds(0.0)
            .set_remark("syndicate");
        let sol_contract = SpotContract::new(TradeType::Market, "SOL-USDT", Side::Buy)
            .set_funds(0.0)
            .set_remark("syndicate2");

        let orders = BatchSpotContract::new()
            .add_order(btc_contract)
            .add_order(sol_contract);

        // 4. Execute
        match client.send_multi_orders(orders).await {
            Ok(res) => println!("Trade Orders: {:#?}", res),
            Err(e) => println!("Multi Orders Err: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_cancel_partial_order() {
        // 1. Setup Credentials
        let credentials = Credentials::new(
            &env::var("api_key").unwrap(),
            &env::var("api_secret").unwrap(),
            &env::var("api_passphrase").unwrap(),
        );

        // 2. Initialize Client
        let mut client = KuCoinClient::new(credentials);

        // 3. Generate query and execute.
        let query = SpotQuery::new("x", 0.01, "BTC-USDT");
        match client.cancel_partial_order(query).await {
            Ok(res) => println!("Spot Canceled res: {:#?}", res),
            Err(e) => println!("Spot Canceled Err: {:?}", e),
        }
    }
}
