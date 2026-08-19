use std::future::Future;

use etcd_client::{Client, Error, GetResponse, Txn, TxnResponse};

#[allow(dead_code)]
fn get_future(
    client: &mut Client,
) -> impl Future<Output = Result<GetResponse, Error>> + Send + '_ {
    client.get(b"prw-c02f-w-probe".to_vec(), None)
}

#[allow(dead_code)]
fn txn_future(
    client: &mut Client,
) -> impl Future<Output = Result<TxnResponse, Error>> + Send + '_ {
    client.txn(Txn::new())
}

#[test]
fn etcd_client_and_selected_kv_futures_are_send() {
    fn assert_send<T: Send>() {}

    assert_send::<Client>();
}
