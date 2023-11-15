use derive_builder::Builder;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use serde_json::Value;

#[derive(Debug, Builder, Default)]
pub struct SingleResultTransformProcessor {
    pub transaction_index: bool,
}

#[derive(Debug, Builder, Default)]
pub struct ArrayResultTransformProcessor {
    pub transaction_index: bool,
    pub log_index: bool,
}

pub trait TransformProcessor {
    fn transform(&self, raw: TransformArgs) -> HttpResponse {
        let mut res = HttpResponse {
            status: raw.response.status.clone(),
            ..Default::default()
        };
        if res.status == 200 {
            res.body = self.process_body(&raw.response.body);
        } else {
            ic_cdk::api::print(format!("Received an error from blockchain: err = {:?}", raw));
        }
        res
    }
    fn process_body(&self, body: &[u8]) -> Vec<u8>;
}

impl TransformProcessor for ArrayResultTransformProcessor {
    fn process_body(&self, body: &[u8]) -> Vec<u8> {
        let mut body: Value = serde_json::from_slice(body).unwrap();
        let elements = body.get_mut("result").unwrap().as_array_mut().unwrap();
        for element in elements.iter_mut() {
            if self.transaction_index {
                element
                    .as_object_mut()
                    .unwrap()
                    .insert("transactionIndex".to_string(), Value::from("0x0"));
            }
            if self.log_index {
                element
                    .as_object_mut()
                    .unwrap()
                    .insert("logIndex".to_string(), Value::from("0x0"));
            }
        }
        serde_json::to_vec(&body).unwrap()
    }
}

impl TransformProcessor for SingleResultTransformProcessor {
    fn process_body(&self, body: &[u8]) -> Vec<u8> {
        let mut body: Value = serde_json::from_slice(body).unwrap();
        if self.transaction_index {
            body.get_mut("result")
                .unwrap()
                .as_object_mut()
                .unwrap()
                .insert("transactionIndex".to_string(), Value::from("0x0"));
        }
        serde_json::to_vec(&body).unwrap()
    }
}

#[cfg(test)]
pub mod tests {
    use std::{str::FromStr, vec};

    use candid::Nat;
    use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};

    use crate::transforms::transform::{
        ArrayResultTransformProcessor, SingleResultTransformProcessor, TransformProcessor,
    };

    #[test]
    fn test_single_transform() {
        struct Cases {
            input: String,
            want: String,
        }

        let cases = vec![Cases {
            input: r#"{
                    "id":1,
                    "jsonrpc":"2.0",
                    "result": {
                        "transactionHash": "0xb903239f8543d04b5dc1ba6579132b143087c68db1b2168786408fcbce568238",
                        "transactionIndex": "0x10",
                        "blockNumber": "0xb",
                        "blockHash": "0xc6ef2fc5426d6ad6fd9e2a26abeab0aa2411b7ab17f30a99d3cb96aed1d1055b",
                        "cumulativeGasUsed": "0x33bc",
                        "gasUsed": "0x4dc",
                        "contractAddress": "0xb60e8dd61c5d32be8058bb8eb970870f07233155",
                        "logs": [],
                        "logsBloom": "0x00...0",
                        "status": "0x1"
                      }
                    }"#
            .to_string(),
            want: r#"{
                    "id":1,
                    "jsonrpc":"2.0",
                    "result": {
                        "transactionHash": "0xb903239f8543d04b5dc1ba6579132b143087c68db1b2168786408fcbce568238",
                        "transactionIndex":  "0x0",
                        "blockNumber": "0xb",
                        "blockHash": "0xc6ef2fc5426d6ad6fd9e2a26abeab0aa2411b7ab17f30a99d3cb96aed1d1055b",
                        "cumulativeGasUsed": "0x33bc",
                        "gasUsed": "0x4dc",
                        "contractAddress": "0xb60e8dd61c5d32be8058bb8eb970870f07233155",
                        "logs": [],
                        "logsBloom": "0x00...0",
                        "status": "0x1"
                      }
                    }"#
            .to_string(),
        }];
        for case in cases {
            let response = HttpResponse {
                status: Nat::from_str("200".to_string().as_str()).unwrap(),
                headers: Vec::default(),
                body: case.input.into_bytes(),
            };
            let args = TransformArgs {
                response: response,
                context: vec![],
            };
            let got = SingleResultTransformProcessor {
                transaction_index: true,
            }
            .transform(args);
            let want_json = serde_json::from_str::<serde_json::Value>(&case.want).unwrap();
            let got_json = serde_json::from_slice::<serde_json::Value>(&got.body).unwrap();
            assert_eq!(got_json, want_json);
        }
    }

    #[test]
    fn test_array_transform() {
        struct Cases {
            input: String,
            want: String,
        }

        let cases = vec![Cases {
            input: r#"{
                "id":1,
                "jsonrpc":"2.0",
                "result": [{
                  "logIndex": "0x11",
                  "blockNumber":"0x1b4",
                  "blockHash": "0x8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcfdf829c5a142f1fccd7d",
                  "transactionHash":  "0xdf829c5a142f1fccd7d8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcf",
                  "transactionIndex": "0x12",
                  "address": "0x16c5785ac562ff41e2dcfdf829c5a142f1fccd7d",
                  "data":"0x0000000000000000000000000000000000000000000000000000000000000000",
                  "topics": ["0x59ebeb90bc63057b6515673c3ecf9438e5058bca0f92585014eced636878c9a5"]
                  },
                  {
                    "logIndex": "0x10",
                    "blockNumber":"0x1b4",
                    "blockHash": "0x8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcfdf829c5a142f1fccd7d",
                    "transactionHash":  "0xdf829c5a142f1fccd7d8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcf",
                    "transactionIndex": "0x13",
                    "address": "0x16c5785ac562ff41e2dcfdf829c5a142f1fccd7d",
                    "data":"0x0000000000000000000000000000000000000000000000000000000000000000",
                    "topics": ["0x59ebeb90bc63057b6515673c3ecf9438e5058bca0f92585014eced636878c9a5"]
                  }
                ]
              }"#
            .to_string(),
            want: r#"{
                "id":1,
                "jsonrpc":"2.0",
                "result": [{
                  "logIndex": "0x0",
                  "blockNumber":"0x1b4",
                  "blockHash": "0x8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcfdf829c5a142f1fccd7d",
                  "transactionHash":  "0xdf829c5a142f1fccd7d8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcf",
                  "transactionIndex": "0x0",
                  "address": "0x16c5785ac562ff41e2dcfdf829c5a142f1fccd7d",
                  "data":"0x0000000000000000000000000000000000000000000000000000000000000000",
                  "topics": ["0x59ebeb90bc63057b6515673c3ecf9438e5058bca0f92585014eced636878c9a5"]
                  },{
                    "logIndex": "0x0",
                    "blockNumber":"0x1b4",
                    "blockHash": "0x8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcfdf829c5a142f1fccd7d",
                    "transactionHash":  "0xdf829c5a142f1fccd7d8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcf",
                    "transactionIndex": "0x0",
                    "address": "0x16c5785ac562ff41e2dcfdf829c5a142f1fccd7d",
                    "data":"0x0000000000000000000000000000000000000000000000000000000000000000",
                    "topics": ["0x59ebeb90bc63057b6515673c3ecf9438e5058bca0f92585014eced636878c9a5"]
                  }]
              }"#
            .to_string(),
        }];
        for case in cases {
            let response = HttpResponse {
                status: Nat::from_str("200".to_string().as_str()).unwrap(),
                headers: Vec::default(),
                body: case.input.into_bytes(),
            };
            let args = TransformArgs {
                response: response,
                context: vec![],
            };
            let got = ArrayResultTransformProcessor {
                transaction_index: true,
                log_index: true,
            }
            .transform(args);
            let want_json = serde_json::from_str::<serde_json::Value>(&case.want).unwrap();
            let got_json = serde_json::from_slice::<serde_json::Value>(&got.body).unwrap();
            assert_eq!(got_json, want_json);
        }
    }
}
