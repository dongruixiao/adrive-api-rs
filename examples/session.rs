use adrive_api_rs::ADriveAPI;

#[tokio::main]
async fn main() {
    let mut api = ADriveAPI::new();
    // let res = api.create_session().await.unwrap();
    // println!("{:#?}", res);
    let res = api.renew_session().await.unwrap();
    println!("{:#?}", res);
    // api.signature_crypto();
}

// prikey "9aedd1ee79b40a1fc8df5f57908b290eca8ddce75bfd73702b8c2cbb437fc1a0"
// sig "2f809a62ee0cc9b6d0d4fa499425ed8902dddf5ec084aa0db9eb570e7a2f24ea2d1fe8b470272d10f4cd998d2b813c170e10ac3a5e6dacfd01e8761860f3753001"
// pubkey "03c449089ab84ee5adbf7892ad16cd1a25bdd8cc526c3146b9fa40f9d48629c97d"

// pri_key: 38f32951d438d2ce030d23049692ac0a11c510f5aa2e4359d734519435b83c70
// sig: cd5c4021570fb1fc0553c20f47012296c3c86f63e1364eb0cfd3a8b1a30f72151cfea658c8db2fe8997604b1216f5402b77176b065811f5f331de01f83f7b2de
// sig2: 38ceb670e4b31cc514a55a13647e56037e325ee3b9b3b6289b089417b71938321c1c1807dcbe54bfac70314147a68fed7ee3786a3db6c8197ae6606872381c36
// pub_key: 02d314eefc1d6fdfbd35b72e8c0309ab7c8100309fc5636a1a5e3f504160c7d8b9
