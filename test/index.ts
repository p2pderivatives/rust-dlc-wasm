import { create_dlc_transactions, CreateDlcTransactionsReq, CreateDlcTransactionResp } from "../pkg/rust_dlc_wasm";

export function printResponse() {
  let req: CreateDlcTransactionsReq = {
    offerParams: {
      fundPubkey: "d63c43632dcfc29c3843072ecca4173869a2892a21d4e5272b1a338e5eaefe05eda577266d14f2cfa843257a9875d20ccc4bb74f2b49a85a71c055fba175f6c3",
      changeScriptPubkey: "bcrt1qlgmznucxpdkp5k3ktsct7eh6qrc4tju7ktjukn",
      changeSerialId: 1,
      payoutScriptPubkey: "bcrt1qlgmznucxpdkp5k3ktsct7eh6qrc4tju7ktjukn",
      payoutSerialId: 2,
      inputs: [
        {
          outpoint: {
            txid: "bc92a22f07ef23c53af343397874b59f5f8c0eb37753af1d1a159a2177d4bb98",
            vout: 0,
          },
          maxWitnessLen: 108,
          redeemScript: "",
          serialId: 4,
        }
      ],
      inputAmount: 200000000,
      collateral: 100000000,
    },
    acceptParams: {
      fundPubkey: "d63c43632dcfc29c3843072ecca4173869a2892a21d4e5272b1a338e5eaefe05eda577266d14f2cfa843257a9875d20ccc4bb74f2b49a85a71c055fba175f6c3",
      changeScriptPubkey: "bcrt1qlgmznucxpdkp5k3ktsct7eh6qrc4tju7ktjukn",
      changeSerialId: 1,
      payoutScriptPubkey: "bcrt1qlgmznucxpdkp5k3ktsct7eh6qrc4tju7ktjukn",
      payoutSerialId: 2,
      inputs: [
        {
          outpoint: {
            txid: "bc92a22f07ef23c53af343397874b59f5f8c0eb37753af1d1a159a2177d4bb98",
            vout: 0,
          },
          maxWitnessLen: 108,
          redeemScript: "",
          serialId: 4,
        }
      ],
      inputAmount: 200000000,
      collateral: 100000000,
    },
    payouts: [
      {
        offer: 200000000,
        accept: 0,
      },
      {
        offer: 0,
        accept: 200000000,
      }
    ],
    refundLockTime: 100,
    feeRatePerVb: 4,
    fundLockTime: 10,
    cetLockTime: 10,
    fundOutputSerialId: 0
  }

  let res = create_dlc_transactions(req);

  console.log(res)
}