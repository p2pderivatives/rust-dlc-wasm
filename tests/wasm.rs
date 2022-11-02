use rust_dlc_wasm::{
    create_dlc_transactions, utils::tx_from_string, CreateCetAdaptorSigFromOracleInfoReq,
    CreateDlcTransactionResp, CreateDlcTransactionsReq,
};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_input() {
    let req: CreateCetAdaptorSigFromOracleInfoReq =
        serde_json::from_str(include_str!("../src/test_input/createadaptorsig.json")).unwrap();
    let msgs = rust_dlc_wasm::utils::hash_messages(&req.msgs);
    let cet = tx_from_string(&req.cet);
    let secp = secp256k1_zkp::SECP256K1;
    if req.oracle_infos.is_empty() || msgs.is_empty() {
        panic!("I PANIC");
    }

    let hash = dlc::secp_utils::create_schnorr_hash(
        &msgs[0][0],
        &req.oracle_infos[0].nonces[0],
        &req.oracle_infos[0].public_key,
    );
    let mut pk =
        dlc::secp_utils::schnorr_pubkey_to_pubkey(&req.oracle_infos[0].public_key).unwrap();
    pk.mul_assign(secp, &hash).unwrap();
}
