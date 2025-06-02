#!/bin/bash

task(){
for _i in {1..1}
do
    DFX_WARNING=-mainnet_plaintext_identity dfx canister call --network ic irf3a-oaaaa-aaaah-arfaq-cai vetkd_derive_key '(
        record {
            context = blob "test-context";
            key_id = record { name = "test_key_1"; curve = variant { bls12_381_g2 } };
            input = blob "test-input";
            transport_public_key = blob "\91\19\69\d5\6f\42\87\5d\37\a9\2d\7e\aa\5d\43\29\3e\ff\9f\9a\20\ba\4c\60\52\3e\70\a6\95\ea\ea\de\b7\21\65\9b\52\a4\9d\74\e6\78\41\ad\19\03\3a\12";
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