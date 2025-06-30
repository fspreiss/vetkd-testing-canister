# vetKD testing canister

The vetKD testing canister is deployed on two subnets:

* On subnet [pzp6e](https://dashboard.internetcomputer.org/subnet/pzp6e-ekpqk-3c5x7-2h6so-njoeq-mt45d-h3h6c-q3mxf-vpeq5-fk5o7-yae) in canister [hu6ab-5qaaa-aaaar-qbpkq-cai](https://dashboard.internetcomputer.org/canister/hu6ab-5qaaa-aaaar-qbpkq-cai) ([Candid UI](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=hu6ab-5qaaa-aaaar-qbpkq-cai)) with production key `key_1` as the default
* On subnet [fuqsr](https://dashboard.internetcomputer.org/subnet/fuqsr-in2lc-zbcjj-ydmcw-pzq7h-4xm2z-pto4i-dcyee-5z4rz-x63ji-nae) in canister [lhmvx-syaaa-aaaao-qkb6a-cai](https://dashboard.internetcomputer.org/canister/lhmvx-syaaa-aaaao-qkb6a-cai) ([Candid UI](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=lhmvx-syaaa-aaaao-qkb6a-cai)) with test key `test_key_1`.
* On subnet: [gmq5v](https://dashboard.internetcomputer.org/subnet/gmq5v-hbozq-uui6y-o55wc-ihop3-562wb-3qspg-nnijg-npqp5-he3cj-3ae) in canister [irf3a-oaaaa-aaaah-arfaq-cai](https://dashboard.internetcomputer.org/canister/irf3a-oaaaa-aaaah-arfaq-cai) ([Candid UI](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=irf3a-oaaaa-aaaah-arfaq-cai)) with test key `test_key_1`.

## Example calls

### vetkd_derive_key
```
dfx canister call --network ic hu6ab-5qaaa-aaaar-qbpkq-cai vetkd_derive_key '(
  record {
    context = blob "test-context";
    key_id = record { name = "key_1"; curve = variant { bls12_381_g2 } };
    input = blob "test-input";
    transport_public_key = blob "\91\19\69\d5\6f\42\87\5d\37\a9\2d\7e\aa\5d\43\29\3e\ff\9f\9a\20\ba\4c\60\52\3e\70\a6\95\ea\ea\de\b7\21\65\9b\52\a4\9d\74\e6\78\41\ad\19\03\3a\12";
  },
)'
```

### vetkd_public_key
```
dfx canister call --network ic hu6ab-5qaaa-aaaar-qbpkq-cai vetkd_public_key '(
  record {
    context = blob "test-context";
    key_id = record { name = "key_1"; curve = variant { bls12_381_g2 } };
    canister_id = null;
  },
)'
```

### vetkd_derive_key_parallel
```
dfx canister call --network ic lhmvx-syaaa-aaaao-qkb6a-cai vetkd_derive_key_parallel '(2 : nat16)'
```