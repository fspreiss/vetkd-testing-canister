#!/bin/bash

task(){
for _i in {1..5}
do
    DFX_WARNING=-mainnet_plaintext_identity dfx canister call --network ic lhmvx-syaaa-aaaao-qkb6a-cai vetkd_derive_key_parallel '(3 : nat16)'
done
}

for _i in {1..3}
do
    task &
done

wait
echo "Done"