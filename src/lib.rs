use candid::Principal;
use futures::stream::{FuturesUnordered, StreamExt};
use ic_cdk::management_canister::{
    VetKDCurve, VetKDDeriveKeyArgs, VetKDDeriveKeyResult, VetKDKeyId, VetKDPublicKeyArgs,
    VetKDPublicKeyResult,
};
use ic_cdk::update;
use std::u128;

pub type CanisterId = Principal;

#[update]
async fn vetkd_public_key(request: VetKDPublicKeyArgs) -> VetKDPublicKeyResult {
    ic_cdk::management_canister::vetkd_public_key(&request)
        .await
        .expect("call to vetkd_public_key failed")
}

#[update]
async fn vetkd_derive_key(request: VetKDDeriveKeyArgs) -> VetKDDeriveKeyResult {
    ic_cdk::management_canister::vetkd_derive_key(&request)
        .await
        .expect("call to vetkd_public_key failed")
}

#[update]
async fn vetkd_derive_key_parallel(count: u16) -> u16 {
    let tsk = ic_vetkeys::TransportSecretKey::from_seed(vec![42; 32]).unwrap();

    let request = VetKDDeriveKeyArgs {
        input: b"test-input".to_vec(),
        context: b"test-context".to_vec(),
        transport_public_key: tsk.public_key(),
        key_id: VetKDKeyId {
            curve: VetKDCurve::Bls12_381_G2,
            name: "key_1".to_string(),
        },
    };

    let futures = FuturesUnordered::new();
    for _ in 1..=count {
        futures.push(async {
            match ic_cdk::management_canister::vetkd_derive_key(&request).await {
                Ok(_) => 1,
                Err(_) => 0,
            }
        });
    }

    let results: Vec<_> = futures.collect().await;
    results.iter().sum()
}

#[update]
async fn send_cycles_to_hu6ab(amount: u128) {
    let hu6ab = Principal::from_text("hu6ab-5qaaa-aaaar-qbpkq-cai").unwrap();
    ic_cdk::call::Call::unbounded_wait(hu6ab, "receive_cycles")
        .with_cycles(amount)
        .await
        .expect("call to hu6ab's receive_cycles failed");
}

#[update]
async fn receive_cycles() -> u128 {
    ic_cdk::api::msg_cycles_accept(u128::MAX)
}
