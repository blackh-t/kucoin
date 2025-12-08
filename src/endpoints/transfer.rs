use crate::{
    client::classic_rest::KuCoinClient,
    types::{
        requests::transfer_req_type::{AccountType, TransferRequest, TransferType},
        responses::{transfer_res_type::TransferData, KuCoinResponse},
    },
    utils::errors::{KucoinErrors, KucoinResults},
};
use uuid::Uuid;

impl TransferRequest {
    /// Creates a new transfer request with an auto-generated unique ID (`client_oid`).
    ///
    /// # Arguments
    /// * `currency` - The asset to transfer (e.g., "USDT").
    /// * `amount` - The amount to transfer.
    /// * `src_type` - The source account type (e.g., `Main`, `Trade`).
    /// * `dest_type` - The destination account type.
    /// * `transfer_type` - The nature of the transfer (e.g., `Internal`).
    pub fn new(
        currency: &str,
        amount: f64,
        src_type: AccountType,
        dest_type: AccountType,
        transfer_type: TransferType,
    ) -> Self {
        TransferRequest {
            amount: amount.to_string(),
            client_oid: Uuid::new_v4().to_string(),
            currency: currency.to_string(),
            from_account_tag: None,
            from_account_type: src_type,
            from_user_id: None,
            to_account_tag: None,
            to_account_type: dest_type,
            to_user_id: None,
            transfer_request_type: transfer_type,
        }
    }

    /// Sets the source trading pair symbol (e.g., "BTC-USDT").
    /// Required only when the source is an `ISOLATED` margin account.
    pub fn set_from_account_tag(mut self, symbol: &str) -> Self {
        self.from_account_tag = Some(symbol.to_string());
        self
    }

    /// Sets the destination trading pair symbol (e.g., "BTC-USDT").
    /// Required only when the destination is an `ISOLATED` margin account.
    pub fn set_to_account_tag(mut self, symbol: &str) -> Self {
        self.to_account_tag = Some(symbol.to_string());
        self
    }

    /// Sets the source User ID.
    /// Required when transferring **from** a sub-account to a master account.
    pub fn set_from_user_id(mut self, id: &str) -> Self {
        self.from_user_id = Some(id.to_string());
        self
    }

    /// Sets the destination User ID.
    /// Required when transferring **to** a sub-account from a master account.
    pub fn set_to_user_id(mut self, id: &str) -> Self {
        self.to_user_id = Some(id.to_string());
        self
    }

    /// This method **mutates** the provided `client` instance by overwriting its:
    /// - `base_link` to `https://api.kucoin.com`
    /// - `endpoint` to `/api/v3/accounts/universal-transfer`
    ///
    /// # Argurments
    /// - 'client' - Mutable instance of 'KuCoinClient'
    ///
    /// # Returns
    /// - Request Body in json-string.
    fn build(self, client: &mut KuCoinClient) -> KucoinResults<String> {
        // Validate payload.
        let check_tag = |tag: &Option<String>, acc_type: &AccountType, name: &str| {
            if tag.is_none() && matches!(&acc_type, AccountType::Isolated | AccountType::IsolatedV2)
            {
                return Err(KucoinErrors::MissingIsolatedTag(name.to_string()));
            }
            Ok(())
        };

        check_tag(&self.from_account_tag, &self.from_account_type, "Sender")?;
        check_tag(&self.to_account_tag, &self.to_account_type, "Receiver")?;

        client.base_link = "https://api.kucoin.com".to_string();
        client.endpoint = "/api/v3/accounts/universal-transfer".to_string();

        let json = serde_json::to_string(&self)?;
        Ok(json)
    }
}

impl KuCoinClient {
    /// Executes a universal transfer between accounts.
    ///
    /// # Panics
    /// Panics immediately if request validation fails (e.g., missing tags for Isolated Margin).
    ///
    /// # Returns
    /// The transaction receipt on success, or a `reqwest::Error` if the network request fails.
    pub async fn transfer(
        &mut self,
        reqwest: TransferRequest,
    ) -> KucoinResults<KuCoinResponse<TransferData>> {
        let payload = reqwest.build(self);
        let body = match payload {
            Ok(res) => res,
            Err(e) => panic!("Err: {}", e),
        };

        let res = self
            .send::<KuCoinResponse<TransferData>>("POST", &body)
            .await?;
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client::classic_rest::Credentials;
    use std::env;

    #[tokio::test]
    async fn test_transfer_internal() {
        // 1. Setup Credentials
        let credentials = Credentials::new(
            &env::var("api_key").unwrap(),
            &env::var("api_secret").unwrap(),
            &env::var("api_passphrase").unwrap(),
        );

        // 2. Initialize Client
        let mut client = KuCoinClient::new(credentials);

        // 3. Generate request.
        let request = TransferRequest::new(
            "BTC",
            1.0,
            AccountType::Main,
            AccountType::Trade,
            TransferType::Internal,
        );

        // 4. Execute tranaction.
        match client.transfer(request).await {
            Ok(result) => println!("Transfer: {:#?}", result),
            Err(e) => panic!("Transfer failed: {}", e),
        }
    }
}
