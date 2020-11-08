use reqwest::{blocking::Client, blocking::Response, Error, Url};

pub mod album;

pub struct LastfmError {
    error_code: u8,
    error_message: String,
}

pub struct LastfmClient {
    api_key: String,
    api_url: String,
    http_client: Client,
}

impl LastfmClient {
    pub fn new(api_key: &str, api_url: &str) -> LastfmClient {
        LastfmClient { api_key: api_key.to_owned(), api_url: api_url.to_owned(), http_client: Client::new() }
    }

    pub fn send_request(&self, method: &str, params: Vec<(&str, &str)> ) -> Result<Response, Error> {
        let mut request_url = Url::parse(&*self.api_url).unwrap();

        // Insert method
        request_url.query_pairs_mut().append_pair("method", method);

        // Insert provided Last.fm API key and ensure that the response is in JSON
        request_url.query_pairs_mut().append_pair("api_key", &*self.api_key)
            .append_pair("format", "json");

        // Insert provided URL parameters into request_url
        for (key, value) in params {
            request_url.query_pairs_mut().append_pair(key, value);
        }

        self.http_client.get(request_url).send()
    }
}

#[cfg(test)]
mod tests {
    use crate::LastfmClient;
    use std::env;

    #[test]
    fn test_request() {
        let client = LastfmClient{
            api_key: env::var("API_KEY").unwrap(),
            api_url: "http://ws.audioscrobbler.com/2.0/".to_string(),
            http_client: Default::default()
        };

        // Method is missing so this will result in a 400 error
        assert_eq!(client.send_request("", vec![]).unwrap().status().as_u16(), 400);
    }
}
