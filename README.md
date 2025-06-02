# vetKD testing canister

* Canister ID: [irf3a-oaaaa-aaaah-arfaq-cai](https://dashboard.internetcomputer.org/canister/irf3a-oaaaa-aaaah-arfaq-cai) ([Candid UI](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=irf3a-oaaaa-aaaah-arfaq-cai))

* Subnet: [gmq5v-hbozq-uui6y-o55wc-ihop3-562wb-3qspg-nnijg-npqp5-he3cj-3ae](https://dashboard.internetcomputer.org/subnet/gmq5v-hbozq-uui6y-o55wc-ihop3-562wb-3qspg-nnijg-npqp5-he3cj-3ae)

## Deploy

```
dfx deploy --network ic --identity fsp
```

## Examples for calling the canister from the command line
```
DFX_WARNING=-mainnet_plaintext_identity dfx canister call --network ic irf3a-oaaaa-aaaah-arfaq-cai vetkd_derive_key '(
  record {
    context = blob "test-context";
    key_id = record { name = "test_key_1"; curve = variant { bls12_381_g2 } };
    input = blob "test-input";
    transport_public_key = blob "\91\19\69\d5\6f\42\87\5d\37\a9\2d\7e\aa\5d\43\29\3e\ff\9f\9a\20\ba\4c\60\52\3e\70\a6\95\ea\ea\de\b7\21\65\9b\52\a4\9d\74\e6\78\41\ad\19\03\3a\12";
  },
)'
```

```
DFX_WARNING=-mainnet_plaintext_identity dfx canister call --network ic irf3a-oaaaa-aaaah-arfaq-cai vetkd_public_key '(
  record {
    context = blob "test-context";
    key_id = record { name = "test_key_1"; curve = variant { bls12_381_g2 } };
    canister_id = null;
  },
)'
```