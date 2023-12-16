// fn main() {
//     use secp256k1::hashes::sha256;
//     use secp256k1::rand::rngs::OsRng;
//     use secp256k1::{Message, Secp256k1};

//     let secp = Secp256k1::new();
//     let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
//     let message = Message::from_hashed_data::<sha256::Hash>("Hello World!".as_bytes());

//     let sig = secp.sign_ecdsa(&message, &secret_key);
//     println!("{sig}");
//     assert!(secp.verify_ecdsa(&message, &sig, &public_key).is_ok());
// }

use k256::schnorr::VerifyingKey;

fn main() {
    use k256::{
        ecdsa::{signature::Signer, Signature, SigningKey},
        SecretKey,
    };
    use rand_core::OsRng; // requires 'getrandom' feature

    // Signing
    let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
                                                      // let message = b"ECDSA proves knowledge of a secret number in the context of a single message";
    let message = format!(
        "{app_id}:{device_id}:{user_id}:{nonce}",
        app_id = "25dzX3vbYqktVxyX",
        device_id = "797ef840-3d6f-44ff-b58d-0ad9b1104196",
        user_id = "638c401866394d44bc90dff35c39cb92",
        nonce = 0,
    );
    let message2 = format!(
        "{app_id}:{device_id}:{user_id}:{nonce}",
        app_id = "25dzX3vbYqktVxyX",
        device_id = "797ef840-3d6f-44ff-b58d-0ad9b1104196",
        user_id = "638c401866394d44bc90dff35c39cb92",
        nonce = 1,
    );
    let pri_key = hex::encode(signing_key.to_bytes());
    println!("pri_key: {pri_key}");
    // Note: The signature type must be annotated or otherwise inferable as
    // `Signer` has many impls of the `Signer` trait (for both regular and
    // recoverable signature types).
    let signature: Signature = signing_key.sign(message.as_bytes());
    let signature2: Signature = signing_key.sign(message2.as_bytes());

    let sig = hex::encode(signature.to_bytes());
    let sig2 = hex::encode(signature2.to_bytes());

    println!("sig: {sig}");
    println!("sig2: {sig2}");

    // Verification
    use k256::{
        ecdsa::{signature::Verifier, VerifyingKey},
        EncodedPoint,
    };

    let verifying_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`
    let pub_key = hex::encode(verifying_key.to_sec1_bytes());
    println!("pub_key: {pub_key}");
    let verifying_key =
        VerifyingKey::from_sec1_bytes(hex::decode(pub_key).unwrap().as_slice()).unwrap();
    // println!("{pub_key}");
    assert!(verifying_key.verify(message.as_bytes(), &signature).is_ok());
    assert!(verifying_key
        .verify(message2.as_bytes(), &signature2)
        .is_ok());
}
