use derive_builder::Builder;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use serde_json::Value;

#[derive(Debug, Builder, Default)]
pub struct TransformProcessor {
    pub transaction_index: bool,
}

impl TransformProcessor {
    pub fn send_transaction_processor() -> Self {
        TransformProcessorBuilder::default()
            .transaction_index(true)
            .build()
            .unwrap()
    }
}

impl TransformProcessor {
    pub fn transform(&self, raw: TransformArgs) -> HttpResponse {
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

    fn process_body(&self, body: &[u8]) -> Vec<u8> {
        //ic_cdk::api::print(format!("Got decoded result: {}", body));
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

    use ic_cdk::{
        api::management_canister::http_request::{HttpResponse, TransformArgs},
        export::candid::Nat,
    };

    use crate::transforms::transform::TransformProcessor;

    #[test]
    fn test_transform_eth_get_transaction_count() {
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
            let got = TransformProcessor::send_transaction_processor().transform(args);
            let want_json = serde_json::from_str::<serde_json::Value>(&case.want).unwrap();
            let got_json = serde_json::from_slice::<serde_json::Value>(&got.body).unwrap();
            assert_eq!(got_json, want_json);
        }
    }
}
