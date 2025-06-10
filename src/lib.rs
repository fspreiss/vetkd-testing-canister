use candid::Principal;
use ic_cdk::management_canister::{
    VetKDCurve, VetKDDeriveKeyArgs, VetKDDeriveKeyResult, VetKDKeyId, VetKDPublicKeyArgs,
    VetKDPublicKeyResult,
};
use ic_cdk::update;

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
            name: "test_key_1".to_string(),
        },
    };

    let mut calls = Vec::new();

    for _i in 1..=count {
        let future = async {
            match ic_cdk::management_canister::vetkd_derive_key(&request).await {
                Ok(_) => 1,
                Err(_) => 0,
            }
        };
        calls.push(future);
    }

    let results = futures::future::join_all(calls).await;
    results.iter().sum()
}
