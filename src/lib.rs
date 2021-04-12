use async_trait::async_trait;
use serde::Deserialize;

#[async_trait(?Send)]
pub trait DebugJson<C> 
where
    C: FnOnce(&str) + 'static
{
    async fn inspect_json<T, E>(self, callback: C) -> Result<T, E>
    where
        T: for<'de> Deserialize<'de>,
        E: From<reqwest::Error> + 'static,
        E: From<serde_json::Error> + 'static;
}

#[async_trait(?Send)]
impl<C> DebugJson<C> for reqwest::Response
where
    C: FnOnce(&str) + 'static
{
    async fn inspect_json<T, E>(self, callback: C) -> Result<T, E>
    where
        T: for<'de> Deserialize<'de>,
        E: From<reqwest::Error> + 'static,
        E: From<serde_json::Error> + 'static        
    {
        let full = self
            .text()
            .await?;
        
        callback(&full);

        let json = serde_json::from_str(&full)?;

        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::error::Error;
    
    #[derive(Debug)]
    enum DebugError {
        Reqwest(reqwest::Error),
        ParseError(serde_json::Error),
    }
    impl std::fmt::Display for DebugError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:#?}", self)
        }
    }
    impl From<reqwest::Error> for DebugError{
        fn from(err: reqwest::Error) -> Self {
            DebugError::Reqwest(err)
        }
    }
    impl From<serde_json::Error> for DebugError{
        fn from(err: serde_json::Error) -> Self {
            DebugError::ParseError(err)
        }
    }    
    impl std::error::Error for DebugError {
    }

    #[tokio::test]
    async fn debug_json_async_test() -> Result<(), Box<dyn Error>> {
        
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
        struct TestDataClass {
            key1: String,
            key2: String,
        }
        #[derive(Serialize, Deserialize, Debug)]
        struct Response {
            json: TestDataClass,
        }

        let test_data = TestDataClass {
            key1: "asdada".to_owned(),
            key2: "asdagfdgdf".to_owned(),
        };
        let test_data_copy = test_data.clone();

        let client = reqwest::Client::new();
        let response = client
            .post("http://httpbin.org/post")
            .json(&test_data)
            .send()
            .await
            .expect("Request failed")
            .inspect_json::<Response, DebugError>(move |text| {
                // println!("Json content: {}", text);
                let text_data = serde_json::from_str::<Response>(text).expect("Parsing failed");
                assert_eq!(text_data.json, test_data_copy);
            })
            .await
            .expect("Response parse failed");

        assert_eq!(response.json, test_data);

        Ok(())
    }
}
