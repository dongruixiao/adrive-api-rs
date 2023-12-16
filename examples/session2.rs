use k256::ecdsa::RecoveryId;

fn main() {
    let message = format!(
        "{app_id}:{device_id}:{user_id}:{nonce}",
        app_id = "25dzX3vbYqktVxyX",
        device_id = "797ef840-3d6f-44ff-b58d-0ad9b1104196",
        user_id = "638c401866394d44bc90dff35c39cb92",
        nonce = 0,
    );

    use k256::ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey};
    use sha3::{Digest, Keccak256};

    use rand_core::OsRng;
    let private_key = SigningKey::random(&mut OsRng);

    // let digest = Keccak256::new_with_prefix(hex::encode(message));
    // let (signature, recid) = private_key.sign_digest_recoverable(digest).unwrap();
    // let sig = format!("{}0{}", hex::encode(signature.to_bytes()), recid.to_byte());
    // let verifying_key = VerifyingKey::from(&private_key); // Serialize with `::to_encoded_point()`
    // let public_key = hex::encode(verifying_key.to_sec1_bytes());

    // let sig = "5430c85dc652dae764e2ec4abb9613054f7dbad957eede56f19611ba4b1ebf6301849f2b78c6e24dee50de4bd67ad02e03c2cf5ea06fa09965a0b6c0030bf44e";
    // let signature = Signature::try_from(hex::decode(sig).unwrap().as_slice()).unwrap();
    // let recid = RecoveryId::try_from(0u8).unwrap();
    // let rec_key =
    //     VerifyingKey::recover_from_digest(Keccak256::new_with_prefix(message), &signature, recid)
    //         .unwrap();
    // let except = VerifyingKey::from_sec1_bytes(
    //     hex::decode("033201553edd7a47f11f048e8c9f6237445e636c9a8130fb46d6a68176e56ad580")
    //         .unwrap()
    //         .as_slice(),
    // )
    // .unwrap();
    // assert_eq!(rec_key, except);
}

// prikey "9aedd1ee79b40a1fc8df5f57908b290eca8ddce75bfd73702b8c2cbb437fc1a0"
// sig "2f809a62ee0cc9b6d0d4fa499425ed8902dddf5ec084aa0db9eb570e7a2f24ea2d1fe8b470272d10f4cd998d2b813c170e10ac3a5e6dacfd01e8761860f3753001"
// pubkey "03c449089ab84ee5adbf7892ad16cd1a25bdd8cc526c3146b9fa40f9d48629c97d"

// "a97739a271898a48a976283883d3abcdec02360e06bd51d488caa9fb8893fb8a"
// "cc9e70881e5c4f9756967e19e9b7e8ca2917dc02b86b0ba22486c0e3a94f34de184d21f0a0a2e69349fd159923f7b37beaecc8a02fb9817a9f35220d14a92b7e01"
// "033201553edd7a47f11f048e8c9f6237445e636c9a8130fb46d6a68176e56ad580"
// {
//     "x-device-id": "797ef840-3d6f-44ff-b58d-0ad9b1104196",
//     "x-signature": "5430c85dc652dae764e2ec4abb9613054f7dbad957eede56f19611ba4b1ebf6301849f2b78c6e24dee50de4bd67ad02e03c2cf5ea06fa09965a0b6c0030bf44e00",
// }
