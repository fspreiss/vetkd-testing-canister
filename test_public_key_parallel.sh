#!/bin/bash

task(){
for _i in {1..2}
do
    DFX_WARNING=-mainnet_plaintext_identity dfx canister call --network ic irf3a-oaaaa-aaaah-arfaq-cai vetkd_public_key '(
        record {
            context = blob "test-context";
            key_id = record { name = "test_key_1"; curve = variant { bls12_381_g2 } };
            canister_id = null;
        },
    )'
done
}

for _i in {1..2}
do
    task &
done

wait
echo "Done"