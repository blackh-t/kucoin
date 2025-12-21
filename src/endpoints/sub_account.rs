use crate::{
    client::rest::KuCoinClient,
    types::{
        KuCoinResponse,
        sup_account::{Expire, SubAccBalance, SubAccData, SubAccListData, SubAccRequest},
    },
};

impl SubAccRequest {
    /// Request to add a new SubAccRequest API
    pub fn new(name: &str, remark: &str, passphrase: &str) -> Self {
        SubAccRequest {
            expire: None,
            ip_whitelist: None,
            passphrase: passphrase.to_string(),
            permission: None,
            remark: remark.to_string(),
            sub_name: name.to_string(),
        }
    }

    pub fn set_expire(mut self, day: Expire) -> Self {
        self.expire = Some(day);
        self
    }

    /// Add up to 20 IPs, one by one.
    pub fn add_ipwhitelist(mut self, ip: &str) -> Self {
        match self.ip_whitelist.as_mut() {
            Some(ips) => {
                ips.push(',');
                ips.push_str(ip);
            }
            None => self.ip_whitelist = Some(ip.to_string()),
        };
        self
    }

    /// Only General, Spot, Futures, Margin, Unified
    /// InnerTransfer (Flex Transfer) permissions can be set.
    ///
    /// # Examples
    /// * "General,Spot,Futures,Unified")
    pub fn set_permission(mut self, permission: &str) -> Self {
        self.permission = Some(permission.to_string());
        self
    }
}

pub struct SubAccHander<'a> {
    pub client: &'a KuCoinClient,
}

impl<'a> SubAccHander<'a> {
    /// Creates a new sub-account.
    ///
    /// # Arguments
    ///
    /// * `request` - The configuration details for the new sub-account.
    ///
    /// # Returns
    /// * KuCoinResponse deserialized into SubAccData.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # // Assuming SubAccRequest is in scope
    /// let request = SubAccRequest::new("user01", "vip", "pass456");
    /// ```
    pub async fn add_api(
        &self,
        request: SubAccRequest,
    ) -> Result<KuCoinResponse<SubAccData>, reqwest::Error> {
        let enpoint: &str = "/api/v1/sub/api-key";
        let payload = serde_json::to_string(&request).unwrap();

        self.client
            .send::<KuCoinResponse<SubAccData>>("POST", &payload, enpoint)
            .await
    }

    /// Get every sub-account summary info.
    pub async fn fetchall(&self) -> Result<KuCoinResponse<SubAccListData>, reqwest::Error> {
        let endpoint = "/api/v2/sub/user";
        self.client
            .send::<KuCoinResponse<SubAccListData>>("GET", "", endpoint)
            .await
    }

    pub async fn balance(
        &self,
        user_id: &str,
    ) -> Result<KuCoinResponse<SubAccBalance>, reqwest::Error> {
        let endpoint = &format!("/api/v1/sub-accounts/{}", user_id);
        self.client
            .send::<KuCoinResponse<SubAccBalance>>("GET", "", endpoint)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_basic_request() {
        let req = SubAccRequest::new("myuser", "remark", "pass123");

        assert_eq!(req.sub_name, "myuser");
        assert_eq!(req.passphrase, "pass123");
        assert_eq!(req.ip_whitelist, None); // Should be None initially
    }

    #[test]
    fn test_add_ip_whitelist() {
        let req = SubAccRequest::new("u", "r", "p")
            .add_ipwhitelist("192.168.1.1")
            .add_ipwhitelist("10.0.0.1");

        // Verify it handles the comma correctly
        assert_eq!(req.ip_whitelist, Some("192.168.1.1,10.0.0.1".to_string()));
    }

    #[test]
    fn test_full_builder_chain() {
        let req = SubAccRequest::new("user", "remark", "pass")
            .set_permission("General,Spot")
            .add_ipwhitelist("1.1.1.1");

        assert_eq!(req.permission, Some("General,Spot".to_string()));
        assert_eq!(req.ip_whitelist, Some("1.1.1.1".to_string()));
    }
}
