use crate::{
    client::classic_rest::KuCoinClient,
    types::{
        requests::deposit_req_type::{DepositQuery, DepositStatus},
        responses::{
            deposit_res_type::{Deposit, DepositList},
            KuCoinResponse,
        },
    },
};

impl DepositQuery {
    /// Create a new Deposit query with the mandatory 'currency' field.
    /// Examples currency: BTC,ETH,USDT
    pub fn new(currency: &str) -> Self {
        DepositQuery {
            currency: currency.to_string(),
            current_page: None,
            end_at: None,
            page_size: None,
            start_at: None,
            status: None,
        }
    }

    /// Set the status (Chainable).
    pub fn set_status(mut self, status: DepositStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Set the current page (Chainable).
    pub fn set_current_page(mut self, page: i64) -> Self {
        self.current_page = Some(page);
        self
    }

    /// Set the page size (Chainable).
    /// Note: API usually requires min 10, max 500.
    pub fn set_page_size(mut self, size: i64) -> Self {
        self.page_size = Some(size);
        self
    }

    /// Set the start timestamp in milliseconds (Chainable).
    pub fn set_start_at(mut self, start_at: i64) -> Self {
        self.start_at = Some(start_at);
        self
    }

    /// Set the end timestamp in milliseconds (Chainable).
    pub fn set_end_at(mut self, end_at: i64) -> Self {
        self.end_at = Some(end_at);
        self
    }

    /// This method **mutates** the provided `client` instance by overwriting its:
    /// - `base_link` to `https://api.kucoin.com`
    /// - `endpoint` to `/api/v1/deposits` + deposit query.
    ///
    /// # Argurments
    /// - 'client' - Mutable instance of 'KuCoinClient'
    ///
    /// # Returns
    /// - Request query for deposit in string.
    fn build(self, client: &mut KuCoinClient) -> String {
        let query = serde_urlencoded::to_string(&self).unwrap();
        client.base_link = "https://api.kucoin.com".to_string();
        client.endpoint = format!("/api/v1/deposits?{}", query);

        let json = serde_json::to_string(&self).unwrap();
        json
    }
}

impl KuCoinClient {
    /// Fetch deposit history filtered by 'DepositQuery'
    pub async fn get_deposit_history(
        &mut self,
        filter: DepositQuery,
    ) -> Result<KuCoinResponse<DepositList>, reqwest::Error> {
        filter.build(self);
        self.send("GET", "").await
    }

    /// Looks up a specific deposit by its transaction hash (TX ID).
    ///
    /// # Arguments
    /// * `signature` - The Transaction Hash (TX ID) to search for.
    ///
    /// # Returns
    /// * `Ok(Some(Item))` - The transaction info if found.
    /// * `Ok(None)` - If match is found.
    /// * `Err(reqwest::Error)` - If the API request fails.
    pub async fn find_deposit_from(
        &mut self,
        signature: &str,
    ) -> Result<Option<Deposit>, reqwest::Error> {
        let filter = DepositQuery::new("");
        let deposit_log = self.get_deposit_history(filter).await?;

        let items = match deposit_log.data {
            Some(data) => data.items,
            None => return Ok(None),
        };

        let target_item = items
            .into_iter()
            .find(|item| item.wallet_tx_id.as_deref() == Some(signature));
        Ok(target_item)
    }
}

#[cfg(test)]
mod test {

    use std::env;

    use super::*;
    use crate::{
        client::classic_rest::Credentials,
        types::requests::deposit_req_type::{DepositQuery, DepositStatus},
    };

    #[tokio::test]
    async fn test_get_deposits() {
        // 1. Setup Credentials
        let api_key = env::var("api_key").unwrap();
        let api_secret = env::var("api_secret").unwrap();
        let api_passphrase = env::var("api_passphrase").unwrap();
        let credentials = Credentials::new(&api_key, &api_secret, &api_passphrase);

        // 2. Initialize Client
        let mut client = KuCoinClient::new(credentials);

        // 3. configure search_filter.
        let search_filter = DepositQuery::new("SOL")
            .set_status(DepositStatus::Success)
            .set_page_size(20); // 20 rows per page.

        // 4. Fetch deposit history for client.
        let deposit_log = client.get_deposit_history(search_filter).await;
        println!("Deposit history: {:#?}", deposit_log);
    }

    #[tokio::test]
    async fn test_transaction_lookup() {
        // 1. Setup Credentials
        let api_key = env::var("api_key").unwrap();
        let api_secret = env::var("api_secret").unwrap();
        let api_passphrase = env::var("api_passphrase").unwrap();
        let credentials = Credentials::new(&api_key, &api_secret, &api_passphrase);

        // 2. Initialize Client
        let mut client = KuCoinClient::new(credentials);

        // 3. Get a target deposit log.
        let deposit_log = client.find_deposit_from("x");
        println!("Deposit history: {:#?}", deposit_log.await.unwrap());
    }
}
