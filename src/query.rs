use cosmwasm_std::{to_json_binary, to_json_vec, Binary, Deps, Empty, Env, QueryRequest, StdError, StdResult};

use crate::slinky_query_proto::GetAllCurrencyPairsRequest;
use crate::state::Contract;
use crate::msgs::QueryMsg;
use crate::slinky_query_proto::{GetPriceRequest, get_price_request::Currency_pair_selector, };
use protobuf::Message;

impl<'a> Contract {
    fn get_price(&self, deps: Deps, _env: Env, pair_id: String) -> StdResult<GetPriceResponse> {
        let request = GetPriceRequest { 
            currency_pair_selector: Some(Currency_pair_selector::CurrencyPairId(pair_id)),
            special_fields: ::protobuf::SpecialFields::new()
        };
        let bytes = request.write_to_bytes().unwrap();
        
        let data = Binary::from(bytes);
        let request = QueryRequest::Stargate{path: "/slinky.oracle.v1.Query/GetPrice".to_string(), data};
        let res: GetPriceResponse = deps.querier.query(&request)?;
        Ok(res)
    }
    fn get_price_raw(&self, deps: Deps, _env: Env, pair_id: String) -> StdResult<String> {
        let request = GetPriceRequest { 
            currency_pair_selector: Some(Currency_pair_selector::CurrencyPairId(pair_id)),
            special_fields: ::protobuf::SpecialFields::new()
        };
        let bytes = request.write_to_bytes().unwrap();
        
        let data = Binary::from(bytes);
        let request = QueryRequest::Stargate{path: "/slinky.oracle.v1.Query/GetPrice".to_string(), data};
        let res = deps.querier.raw_query(&to_bin_request(&request));
        Ok(res.unwrap().unwrap().to_base64())
    }
    fn get_all_currency_pairs(&self, deps: Deps, _env: Env) -> StdResult<GetAllCurrencyPairsResponse> {
        let request = GetAllCurrencyPairsRequest { 
            special_fields: ::protobuf::SpecialFields::new()
        };
        let bytes = request.write_to_bytes().unwrap();

        let data = Binary::from(bytes);
        let request = QueryRequest::<Empty>::Stargate{path: "/slinky.oracle.v1.Query/GetAllCurrencyPairs".to_string(), data};
        let res: GetAllCurrencyPairsResponse = deps.querier.query(&request)?;
        Ok(res)
    }
    fn get_all_currency_pairs_raw(&self, deps: Deps, _env: Env) -> StdResult<String> {
        let request = GetAllCurrencyPairsRequest { 
            special_fields: ::protobuf::SpecialFields::new()
        };
        let bytes = request.write_to_bytes().unwrap();

        let data = Binary::from(bytes);
        let request = QueryRequest::<Empty>::Stargate{path: "/slinky.oracle.v1.Query/GetAllCurrencyPairs".to_string(), data};
        let res = deps.querier.raw_query(&to_bin_request(&request));
        Ok(res.unwrap().unwrap().to_base64())
    }
}


impl<'a> Contract {
    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetPrice { pair_id } => to_json_binary(&self.get_price(deps, env, pair_id)?),
            QueryMsg::GetPriceRaw { pair_id } => to_json_binary(&self.get_price_raw(deps, env, pair_id)?),
            QueryMsg::GetAllCurrencyPairs {} => to_json_binary(&self.get_all_currency_pairs(deps, env)?),
            QueryMsg::GetAllCurrencyPairsRaw {} => to_json_binary(&self.get_all_currency_pairs_raw(deps, env)?),
        }
    }
}

fn to_bin_request(request: &QueryRequest<Empty>) -> Vec<u8> {
    to_json_vec(request).map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {serialize_err}"))
    }).unwrap()
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GetPriceResponse {
    pub price: Option<QuotePrice>,
    pub nonce: u64,
    pub decimals: u64,
    pub id: u64,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct QuotePrice {
    pub price: String,
    pub block_timestamp: Timestamp,
    pub block_height: u64,
}
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanos: i32,
}
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GetAllCurrencyPairsResponse {
    pub currency_pairs: Vec<CurrencyPair>,
}
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CurrencyPair {
    pub base: String,
    pub quote: String,
}