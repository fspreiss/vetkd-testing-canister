use candid::Principal;
use ic_cdk::management_canister::{
    VetKDDeriveKeyArgs, VetKDDeriveKeyResult, VetKDPublicKeyArgs, VetKDPublicKeyResult,
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
