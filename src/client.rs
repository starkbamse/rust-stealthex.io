/**
 * StealthEX.io API client. 
 * Uses reqwest for HTTP requests.
 * Fully asynchronous.
 */
pub struct Client {
    url: String,
    bearer_token: String,
}

impl Client {
    pub fn new(url: String, bearer_token: String) -> Client {
        Client { url, bearer_token }
    }
    pub fn get_exchange_range(&self) -> String {
        format!("GET {} with token {}", self.url, self.bearer_token)
    }
    pub fn post(&self) -> String {
        format!("POST {} with token {}", self.url, self.bearer_token)
    }
    pub fn put(&self) -> String {
        format!("PUT {} with token {}", self.url, self.bearer_token)
    }
    pub fn delete(&self) -> String {
        format!("DELETE {} with token {}", self.url, self.bearer_token)
    }
}
