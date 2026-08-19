use std::future::Future;

use etcd_client::{Client, Error, GetResponse, Txn, TxnResponse};

#[allow(dead_code)]
fn get_future<'a>(
    client: &'a mut Client,
) -> impl Future<Output = Result<GetResponse, Error>> + Send + 'a {
    client.get(b"prw-c02f-w-probe".to_vec(), None)
}

#[allow(dead_code)]
fn txn_future<'a>(
    client: &'a mut Client,
) -> impl Future<Output = Result<TxnResponse, Error>> + Send + 'a {
    client.txn(Txn::new())
}

#[test]
fn etcd_client_and_selected_kv_futures_are_send() {
    fn assert_send<T: Send>() {}

    assert_send::<Client>();
}
