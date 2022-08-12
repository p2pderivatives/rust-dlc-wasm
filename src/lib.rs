use dlc::{PartyParams, Payout};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDlcTransactionReq {
    offer_params: PartyParams,
    accept_params: PartyParams,
    payouts: Vec<Payout>,
    refund_lock_time: u32,
    fee_rate_per_vb: u64,
    fund_lock_time: u32,
    cet_lock_time: u32,
    fund_output_serial_id: u64,
}
#[wasm_bindgen]
pub fn create_dlc_transaction(val: &JsValue) -> JsValue {
    let req: CreateDlcTransactionReq = val.into_serde().unwrap();
    let dlc_transactions = dlc::create_dlc_transactions(
        &req.offer_params,
        &req.accept_params,
        &req.payouts,
        req.refund_lock_time,
        req.fee_rate_per_vb,
        req.fund_lock_time,
        req.cet_lock_time,
        req.fund_output_serial_id,
    )
    .unwrap();

    JsValue::from_serde(&dlc_transactions).unwrap()
}
