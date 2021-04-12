# reqwest-debug-json

Provides `inspect_json` method for reqwest's response object.
Can be usefull for response json-tracing purposes.

Example:

```
use reqwest_inspect_json::InspectJson;

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
```