//! IC http client

use candid::CandidType;
use candid::{candid_method, Principal};
use derive_builder::Builder;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformContext, TransformFunc,
};
use jsonrpc_core::Request;
use serde::{self, Deserialize, Serialize};

// #[derive(CandidType, Deserialize, Debug)]
// pub struct CanisterHttpRequestArgs {
//     pub url: String,
//     pub max_response_bytes: Option<u64>,
//     pub headers: Vec<HttpHeader>,
//     pub body: Option<Vec<u8>>,
//     pub http_method: HttpMethod,
//     pub transform_method_name: Option<String>,
// }

#[derive(Clone, Debug)]
pub struct ICHttpClient {
    pub max_response_bytes: u64,
}

#[derive(Builder, Default, Clone, Debug, PartialEq, Eq)]
pub struct CallOptions {
    max_resp: Option<u64>,
    cycles: Option<u64>,
    transform: Option<TransformContext>,
}

impl ICHttpClient {
    pub fn new(max_resp: Option<u64>) -> Self {
        ICHttpClient {
            max_response_bytes: if let Some(v) = max_resp { v } else { 500_000 },
        }
    }

    pub fn set_max_response_bytes(&mut self, v: u64) {
        self.max_response_bytes = v;
    }

    async fn request(
        &self,
        url: String,
        req_type: HttpMethod,
        req_headers: Vec<HttpHeader>,
        payload: &Request,
        options: CallOptions,
    ) -> Result<Vec<u8>, String> {
        let request = CanisterHttpRequestArgument {
            url: url.clone(),
            max_response_bytes: if let Some(v) = options.max_resp {
                Some(v)
            } else {
                Some(self.max_response_bytes)
            },
            method: req_type,
            headers: req_headers,
            body: Some(serde_json::to_vec(&payload).unwrap()),
            // transform: Some(TransformType::Function(TransformFunc(candid::Func {
            //     principal: ic_cdk::api::id(),
            //     method: "transform".to_string(),
            // }))),
            transform: match options.transform {
                Some(t) => Some(t),
                None => Some(TransformContext {
                    function: TransformFunc(candid::Func {
                        principal: ic_cdk::api::id(),
                        method: "transform".to_string(),
                    }),
                    context: vec![],
                }),
            },
        };

        let cycles = http_request_required_cycles(&request);
        match http_request(request.clone(), cycles).await {
            Ok((result,)) => Ok(result.body),
            Err((r, m)) => {
                let message = format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
                ic_cdk::api::print(message.clone());
                Err(message)
            }
        }
    }

    pub async fn get(&self, url: String, payload: &Request, options: CallOptions) -> Result<Vec<u8>, String> {
        let request_headers = vec![HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        }];

        self.request(url, HttpMethod::GET, request_headers, payload, options)
            .await
    }

    pub async fn post(&self, url: String, payload: &Request, options: CallOptions) -> Result<Vec<u8>, String> {
        let request_headers = vec![HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        }];

        self.request(url, HttpMethod::POST, request_headers, payload, options)
            .await
    }
}

// Calcurate cycles for http_request
// NOTE:
//   v0.11: https://github.com/dfinity/cdk-rs/blob/0b14facb80e161de79264c8f88b1a0c8e18ffcb6/examples/management_canister/src/caller/lib.rs#L7-L19
//   v0.8: https://github.com/dfinity/cdk-rs/blob/a8454cb37420c200c7b224befd6f68326a01442e/src/ic-cdk/src/api/management_canister/http_request.rs#L290-L299
fn http_request_required_cycles(arg: &CanisterHttpRequestArgument) -> u128 {
    let max_response_bytes = match arg.max_response_bytes {
        Some(ref n) => *n as u128,
        None => 2 * 1024 * 1024u128, // default 2MiB
    };
    let arg_raw = candid::utils::encode_args((arg,)).expect("Failed to encode arguments.");
    // The fee is for a 13-node subnet to demonstrate a typical usage.
    (3_000_000u128
        + 60_000u128 * 13
        + (arg_raw.len() as u128 + "http_request".len() as u128) * 400
        + max_response_bytes * 800)
        * 13
}
