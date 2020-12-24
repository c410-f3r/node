#![cfg(feature = "_integration-tests")]

use sp_keyring::AccountKeyring;
use substrate_subxt::{extrinsic::PairSigner, ClientBuilder};
use valiu_node_rpc::{AddMemberCallExt, ValiuRuntime};

#[tokio::test]
async fn add_member() {
    let _ = env_logger::builder().is_test(true).try_init();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());

    let client = ClientBuilder::<ValiuRuntime>::new().build().await.unwrap();

    let rslt = client
        .add_member_and_watch(&signer, AccountKeyring::Bob.to_account_id())
        .await;

    assert!(rslt.is_ok());
}
