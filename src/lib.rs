use bitcoin::Script;
use dlc::{
    secp_utils::schnorrsig_compute_sig_point, DlcTransactions, OracleInfo, PartyParams, Payout,
};
use secp256k1_zkp::{schnorr::Signature, EcdsaAdaptorSignature, PublicKey, SecretKey, SECP256K1};
use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;
use utils::{dlc_error_to_js_error, hash_messages, tx_from_string};
use wasm_bindgen::prelude::*;

pub mod utils;

use crate::utils::tx_to_string;

#[derive(Serialize, Deserialize, TypeScriptify)]
#[serde(rename_all = "camelCase")]
pub struct CreateDlcTransactionsReq {
    pub offer_params: PartyParams,
    pub accept_params: PartyParams,
    pub payouts: Vec<Payout>,
    pub refund_lock_time: u32,
    pub fee_rate_per_vb: u64,
    pub fund_lock_time: u32,
    pub cet_lock_time: u32,
    pub fund_output_serial_id: u64,
}

#[derive(Serialize, Deserialize, TypeScriptify)]
#[serde(rename_all = "camelCase")]
pub struct CreateDlcTransactionResp {
    pub fund: String,
    pub cets: Vec<String>,
    pub refund: String,
    pub fund_vout: usize,
    pub funding_script_pubkey: Script,
}

impl From<DlcTransactions> for CreateDlcTransactionResp {
    fn from(input: DlcTransactions) -> Self {
        Self {
            fund: tx_to_string(&input.fund),
            cets: input.cets.iter().map(tx_to_string).collect(),
            refund: tx_to_string(&input.refund),
            fund_vout: input.get_fund_output_index(),
            funding_script_pubkey: input.funding_script_pubkey,
        }
    }
}

#[wasm_bindgen]
pub fn create_dlc_transactions(val: JsValue) -> Result<JsValue, JsError> {
    let req: CreateDlcTransactionsReq = serde_wasm_bindgen::from_value(val)?;
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
    .map_err(dlc_error_to_js_error)?;
    // return Ok(JsValue::from_serde(&CreateDlcTransactionResp {
    //     fund: "".to_string(),
    //     cets: vec!["".to_string()],
    //     refund: "".to_string(),
    //     fund_vout: 0,
    //     funding_script_pubkey: Script::default(),
    // })?);
    let resp: CreateDlcTransactionResp = dlc_transactions.into();

    Ok(serde_wasm_bindgen::to_value(&resp)?)
}

#[derive(Serialize, Deserialize, TypeScriptify)]
#[serde(rename_all = "camelCase")]
pub struct VerifyCetAdaptorSignatureFromOracleInfoReq {
    adaptor_sig: EcdsaAdaptorSignature,
    cet: String,
    oracle_infos: Vec<OracleInfo>,
    pubkey: PublicKey,
    funding_script_pubkey: Script,
    total_collateral: u64,
    msgs: Vec<Vec<String>>,
}

pub fn verify_cet_adaptor_sig_from_oracle_info(val: JsValue) -> Result<(), JsError> {
    let req: VerifyCetAdaptorSignatureFromOracleInfoReq = serde_wasm_bindgen::from_value(val)?;
    Ok(dlc::verify_cet_adaptor_sig_from_oracle_info(
        SECP256K1,
        &req.adaptor_sig,
        &tx_from_string(&req.cet),
        &req.oracle_infos,
        &req.pubkey,
        &req.funding_script_pubkey,
        req.total_collateral,
        &hash_messages(&req.msgs),
    )
    .map_err(dlc_error_to_js_error)?)
}

#[derive(Serialize, Deserialize, TypeScriptify)]
#[serde(rename_all = "camelCase")]
pub struct SignCetReq {
    cet: String,
    adaptor_signature: EcdsaAdaptorSignature,
    oracle_signatures: Vec<Vec<Signature>>,
    funding_sk: SecretKey,
    other_pk: PublicKey,
    funding_script_pubkey: Script,
    fund_output_value: u64,
}

#[wasm_bindgen]
pub fn sign_cet(val: JsValue) -> Result<String, JsError> {
    let req: SignCetReq = serde_wasm_bindgen::from_value(val)?;
    let mut cet = tx_from_string(&req.cet);
    dlc::sign_cet(
        SECP256K1,
        &mut cet,
        &req.adaptor_signature,
        &req.oracle_signatures,
        &req.funding_sk,
        &req.other_pk,
        &req.funding_script_pubkey,
        req.fund_output_value,
    )
    .map_err(dlc_error_to_js_error)?;
    Ok(tx_to_string(&cet))
}

#[derive(Serialize, Deserialize, TypeScriptify)]
#[serde(rename_all = "camelCase")]
pub struct CreateCetAdaptorSigFromOracleInfoReq {
    pub cet: String,
    pub oracle_infos: Vec<OracleInfo>,
    pub funding_sk: SecretKey,
    pub funding_script_pubkey: Script,
    pub fund_output_value: u64,
    pub msgs: Vec<Vec<String>>,
}

#[wasm_bindgen]
pub fn create_cet_adaptor_sig_from_oracle_info(val: JsValue) -> Result<String, JsError> {
    let req: CreateCetAdaptorSigFromOracleInfoReq = serde_wasm_bindgen::from_value(val)?;
    let msgs = hash_messages(&req.msgs);
    let cet = tx_from_string(&req.cet);
    let secp = SECP256K1;
    if req.oracle_infos.is_empty() || msgs.is_empty() {
        return Err(JsError::new("bad"));
    }

    let hash = dlc::secp_utils::create_schnorr_hash(
        &msgs[0][0],
        &req.oracle_infos[0].nonces[0],
        &req.oracle_infos[0].public_key,
    );
    let mut pk =
        dlc::secp_utils::schnorr_pubkey_to_pubkey(&req.oracle_infos[0].public_key).unwrap();
    let e = pk.mul_assign(secp, &hash);
    if let Err(e) = e {
        return Ok(e.to_string());
    }
    return Ok("".to_string());
    let npk = dlc::secp_utils::schnorr_pubkey_to_pubkey(&req.oracle_infos[0].nonces[0]).unwrap();

    let res = schnorrsig_compute_sig_point(
        secp,
        &req.oracle_infos[0].public_key,
        &req.oracle_infos[0].nonces[0],
        &msgs[0][0],
    )
    .map_err(dlc_error_to_js_error)?;

    let mut oracle_sigpoints = Vec::with_capacity(msgs[0].len());
    for (i, info) in req.oracle_infos.iter().enumerate() {
        let res = schnorrsig_compute_sig_point(
            secp,
            &req.oracle_infos[0].public_key,
            &req.oracle_infos[0].nonces[0],
            &msgs[0][0],
        )
        .map_err(dlc_error_to_js_error)?;
        oracle_sigpoints.push(res);
    }
    let pk = PublicKey::combine_keys(&oracle_sigpoints.iter().collect::<Vec<_>>());
    let res = dlc::create_cet_adaptor_sig_from_oracle_info(
        secp,
        &cet,
        &req.oracle_infos,
        &req.funding_sk,
        &req.funding_script_pubkey,
        req.fund_output_value,
        &msgs,
    )
    .map_err(dlc_error_to_js_error)?;
    Ok(res.to_string())
}

#[cfg(test)]
mod test {
    use dlc_messages::AcceptDlc;

    use super::*;

    #[test]
    fn parse_test() {
        let _: CreateDlcTransactionsReq =
            serde_json::from_str(include_str!("./test_input/createdlc.json")).unwrap();
    }

    #[test]
    fn parse_accept_test() {
        let _: AcceptDlc =
            serde_json::from_str(include_str!("./test_input/acceptmsg.json")).unwrap();
    }

    #[test]
    fn creat_adaptor_sig_test() {
        let req: CreateCetAdaptorSigFromOracleInfoReq =
            serde_json::from_str(include_str!("./test_input/createadaptorsig.json")).unwrap();
        let msgs = hash_messages(&req.msgs);
        let cet = tx_from_string(&req.cet);
        let secp = SECP256K1;
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

    #[test]
    #[ignore]
    fn generate() {
        use std::fmt::Write;
        use typescript_definitions::TypeScriptifyTrait;
        let mut base = include_str!("../dlc-definitions/rust-dlc.d.ts").to_string();

        macro_rules! generate_types {
            ($($type:ty),*) => {
               $(
                write!(base, "{}\n", <$type>::type_script_ify()).unwrap();
               )*
            };
        }

        generate_types!(
            CreateDlcTransactionsReq,
            CreateDlcTransactionResp,
            VerifyCetAdaptorSignatureFromOracleInfoReq,
            SignCetReq,
            CreateCetAdaptorSigFromOracleInfoReq
        );

        macro_rules! generate_function {
            ($({$fn_name:ident, $in_type:ident, $out_type:ident}),*) => {
               $(
                write!(base, "export function {}(input: {}): {};\n", stringify!($fn_name), stringify!($in_type), stringify!($out_type)).unwrap();
                )*
            };
        }

        generate_function!(
            {create_dlc_transactions, CreateDlcTransactionsReq, CreateDlcTransactionResp},
            {verify_cet_adaptor_sig_from_oracle_info, VerifyCetAdaptorSignatureFromOracleInfoReq, void},
            {sign_cet, SignCetReq, void},
            {create_cet_adaptor_sig_from_oracle_info, CreateCetAdaptorSigFromOracleInfoReq, string}
        );

        macro_rules! replace_to_string {
            ($($type:ty),*) => {
                $(
                    base = base.replace(&format!(" {}", stringify!($type)), " string");
                )*
            };
        }

        replace_to_string!(
            Script,
            PublicKey,
            XOnlyPublicKey,
            EcdsaAdaptorSignature,
            SecretKey,
            Signature
        );

        base = base.replace("Id: number", "Id: BigInt");

        std::fs::write("./pkg/rust_dlc_wasm.d.ts", base).unwrap();
    }
}
