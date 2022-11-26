use super::*;
use crate::transaction::{Sign, Transaction1559};
use crate::utils::recover_address;
use ic_cdk::export::Principal;

use futures::executor::block_on;

#[test]
fn create_new_user() {
    let text = "aaaaa-aa";
    let principal_id = Principal::from_text(text).unwrap();

    let res = block_on(create(principal_id)).unwrap();
    assert_eq!(42, res.address.len());
}
// #[test]
// fn sign_legacy_transaction() {
//     let text = "aaaaa-aa";
//     let principal_id = Principal::from_text(text).unwrap();
//     let tx_parm = EVMTransactionLegacy {
//         nonce: 0,
//         gas_price: 36935555629,
//         gas_limit: 31272,
//         to: "0x70997970C51812dc3A010C7d01b50e0d17dc79C8".to_string(),
//         value: 1000000000000000000,
//         data: "0x00".to_string(),
//     };
//     let raw_tx = create_raw_legacy_tx(tx_parm);

//     let chain_id: usize = 1;

//     let res0 = block_on(create(principal_id)).unwrap();

//     let res = block_on(sign(raw_tx.clone(), chain_id, principal_id)).unwrap();

//     let tx = transaction::get_transaction(&res.sign_tx, chain_id).unwrap();

//     let signature = tx.get_signature().unwrap();

//     let msg = tx.get_message_to_sign().unwrap();

//     let recovery_id = tx.get_recovery_id().unwrap();
//     let address = recover_address(signature, recovery_id, msg);

//     assert_eq!(res0.address, address)
// }
// #[test]
// fn sign_eip2930_transaction() {
//     let text = "aaaaa-aa";
//     let principal_id = Principal::from_text(text).unwrap();
//     let tx_parm = EVMTransactionEIP2930 {
//         chain_id: 1,
//         nonce: 0,
//         gas_price: 36935555629,
//         gas_limit: 31272,
//         to: "0x70997970C51812dc3A010C7d01b50e0d17dc79C8".to_string(),
//         value: 1000000000000000000,
//         data: "0x00".to_string(),
//         access_list: vec![],
//     };
//     let raw_tx = create_raw_eip_2930_tx(tx_parm);

//     let chain_id: usize = 1;

//     let res0 = block_on(create(principal_id)).unwrap();

//     let res = block_on(sign(raw_tx.clone(), chain_id, principal_id)).unwrap();

//     let tx = transaction::get_transaction(&res.sign_tx, chain_id).unwrap();

//     let signature = tx.get_signature().unwrap();

//     let message = tx.get_message_to_sign().unwrap();

//     let recovery_id = tx.get_recovery_id().unwrap();
//     let address = recover_address(signature, recovery_id, message);

//     assert_eq!(res0.address, address)
// }

#[test]
fn sign_eip1559_transaction() {
    let tx = transaction::Transaction1559 {
        chain_id: 1,
        nonce: 0,
        max_priority_fee_per_gas: 0,
        gas_limit: 0,
        max_fee_per_gas: 0,
        to: "0x0000000000000000000000000000000000000000".to_string(),
        value: 0,
        data: "0x00".to_string(),
        access_list: vec![],
        v: "0x00".to_string(),
        r: "0x00".to_string(),
        s: "0x00".to_string(),
    };

    let text = "aaaaa-aa";
    let principal_id = Principal::from_text(text).unwrap();
    let res0 = block_on(create(principal_id)).unwrap();

    let raw_tx = tx.serialize().unwrap();
    let chain_id: u64 = 1;
    let res = block_on(sign(raw_tx, chain_id, principal_id)).unwrap();

    let tx = transaction::get_transaction(&res.sign_tx, chain_id).unwrap();
    let signature = tx.get_signature().unwrap();
    let message = tx.get_message_to_sign().unwrap();
    let recovery_id = tx.get_recovery_id().unwrap();
    let address = recover_address(signature, recovery_id, message);

    assert_eq!(res0.address, address)
}
