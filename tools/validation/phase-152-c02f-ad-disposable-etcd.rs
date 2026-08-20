//! Phase 152 C02f-AD disposable etcd integration harness.
//!
//! This executable is deliberately outside the Cargo workspace targets. The validation script links
//! it against the already-built locked workspace artifacts so production `prw-control-plane` does
//! not gain a Tokio/runtime dependency. It is permitted to connect only to the disposable endpoint
//! supplied through `PRW_C02F_AD_ETCD_ENDPOINT`.

use std::{error::Error, io, num::NonZeroU128};

use etcd_client::{Client, KvClient};
use prw_connectivity::{PeerConnectivityIdentity, TransportIdentity};
use prw_control_plane::{
    reachability_live_owner_codec::{
        AuthorityAttemptId, ReachabilityLiveOwnerAuthorityRecord, encode_live_owner_key,
        encode_live_owner_record,
    },
    reachability_live_owner_etcd::{
        ReachabilityLiveOwnerEtcdError, ReachabilityLiveOwnerEtcdStore,
    },
    reachability_live_owner_txn::{
        LiveOwnerDefinitiveMutation, LiveOwnerProviderCurrentness, LiveOwnerTxnError,
        plan_acquisition, plan_release,
    },
};
use prw_core::DeviceId;

fn main() -> Result<(), Box<dyn Error>> {
    let endpoint = std::env::var("PRW_C02F_AD_ETCD_ENDPOINT")?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    runtime.block_on(run(&endpoint))
}

async fn run(endpoint: &str) -> Result<(), Box<dyn Error>> {
    if endpoint != "http://127.0.0.1:2379" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("refusing non-disposable endpoint: {endpoint}"),
        )
        .into());
    }

    let client = Client::connect([endpoint], None).await?;
    let mut fixture_kv = client.kv_client();
    let mut store = ReachabilityLiveOwnerEtcdStore::new(client.kv_client());

    validate_absence_fails_closed(&mut store).await?;
    validate_real_get_txn_and_release(&mut fixture_kv, &mut store).await?;

    println!("C02F_AD_DISPOSABLE_ETCD_INTEGRATION_PASS");
    Ok(())
}

async fn validate_absence_fails_closed(
    store: &mut ReachabilityLiveOwnerEtcdStore,
) -> Result<(), Box<dyn Error>> {
    let absent_peer = peer("c02f-ad-disposable-absent", 0x11);
    let error = store
        .currentness(&absent_peer, fence(1))
        .await
        .expect_err("missing established state must fail closed");

    assert!(matches!(
        error,
        ReachabilityLiveOwnerEtcdError::Transaction(LiveOwnerTxnError::MissingEstablishedState)
    ));
    println!("C02F_AD_DISPOSABLE_ETCD_ABSENCE_FAIL_CLOSED_PASS");
    Ok(())
}

async fn validate_real_get_txn_and_release(
    fixture_kv: &mut KvClient,
    store: &mut ReachabilityLiveOwnerEtcdStore,
) -> Result<(), Box<dyn Error>> {
    let peer = peer("c02f-ad-disposable-txn", 0x21);
    let key = encode_live_owner_key(&peer)?;

    let initial_current =
        ReachabilityLiveOwnerAuthorityRecord::current(peer.clone(), fence(10), attempt(0x31));
    let initial_released = initial_current.released_successor();
    fixture_kv
        .put(
            key.clone(),
            encode_live_owner_record(&initial_released)?,
            None,
        )
        .await?;

    let before = store
        .linearizable_observation(&peer)
        .await?
        .expect("fixture seed must be visible");
    assert_eq!(before.key(), key.as_slice());
    assert_eq!(before.record(), &initial_released);

    let current_11 =
        ReachabilityLiveOwnerAuthorityRecord::current(peer.clone(), fence(11), attempt(0x32));
    let acquisition_11 = plan_acquisition(&before, current_11.clone())?;
    assert_eq!(
        store.execute(&acquisition_11).await?,
        LiveOwnerDefinitiveMutation::Committed
    );
    assert_eq!(
        store.currentness(&peer, fence(11)).await?,
        LiveOwnerProviderCurrentness::Current
    );

    let observed_11 = store
        .linearizable_observation(&peer)
        .await?
        .expect("committed acquisition must remain observable");
    assert_eq!(observed_11.record(), &current_11);
    assert_eq!(
        observed_11.value(),
        encode_live_owner_record(&current_11)?.as_slice()
    );
    println!("C02F_AD_DISPOSABLE_ETCD_ACQUISITION_COMMIT_PASS");

    let current_12 =
        ReachabilityLiveOwnerAuthorityRecord::current(peer.clone(), fence(12), attempt(0x33));
    let stale_acquisition_12 = plan_acquisition(&observed_11, current_12)?;

    let current_13 =
        ReachabilityLiveOwnerAuthorityRecord::current(peer.clone(), fence(13), attempt(0x34));
    fixture_kv
        .put(key.clone(), encode_live_owner_record(&current_13)?, None)
        .await?;

    match store.execute(&stale_acquisition_12).await? {
        LiveOwnerDefinitiveMutation::CompareFailed(observed) => {
            assert_eq!(observed.record(), &current_13);
        }
        LiveOwnerDefinitiveMutation::Committed => {
            panic!("stale acquisition unexpectedly committed over newer authority");
        }
    }
    assert_eq!(
        store.currentness(&peer, fence(11)).await?,
        LiveOwnerProviderCurrentness::Stale
    );
    assert_eq!(
        store.currentness(&peer, fence(13)).await?,
        LiveOwnerProviderCurrentness::Current
    );
    println!("C02F_AD_DISPOSABLE_ETCD_COMPARE_FAILURE_GET_PASS");

    let observed_13 = store
        .linearizable_observation(&peer)
        .await?
        .expect("current fence 13 must be observable");
    let stale_release_13 = plan_release(&peer, fence(13), Some(&observed_13))?
        .into_transaction()
        .expect("current owner must produce a release transaction");

    let current_14 =
        ReachabilityLiveOwnerAuthorityRecord::current(peer.clone(), fence(14), attempt(0x35));
    fixture_kv
        .put(key.clone(), encode_live_owner_record(&current_14)?, None)
        .await?;

    match store.execute(&stale_release_13).await? {
        LiveOwnerDefinitiveMutation::CompareFailed(observed) => {
            assert_eq!(observed.record(), &current_14);
        }
        LiveOwnerDefinitiveMutation::Committed => {
            panic!("stale release unexpectedly overwrote newer authority");
        }
    }
    assert_eq!(
        store.currentness(&peer, fence(14)).await?,
        LiveOwnerProviderCurrentness::Current
    );
    println!("C02F_AD_DISPOSABLE_ETCD_STALE_RELEASE_FENCING_PASS");

    let observed_14 = store
        .linearizable_observation(&peer)
        .await?
        .expect("current fence 14 must be observable");
    let release_14 = plan_release(&peer, fence(14), Some(&observed_14))?
        .into_transaction()
        .expect("current owner must produce a release transaction");
    assert_eq!(
        store.execute(&release_14).await?,
        LiveOwnerDefinitiveMutation::Committed
    );

    let released_14 = current_14.released_successor();
    let final_observation = store
        .linearizable_observation(&peer)
        .await?
        .expect("released tombstone must remain present");
    assert_eq!(final_observation.record(), &released_14);
    assert_eq!(
        final_observation.value(),
        encode_live_owner_record(&released_14)?.as_slice()
    );
    assert_eq!(
        store.currentness(&peer, fence(14)).await?,
        LiveOwnerProviderCurrentness::Stale
    );
    println!("C02F_AD_DISPOSABLE_ETCD_RELEASE_COMMIT_PASS");

    Ok(())
}

fn peer(device: &str, marker: u8) -> PeerConnectivityIdentity {
    PeerConnectivityIdentity::new(
        DeviceId::new(device).expect("valid DeviceId"),
        TransportIdentity::new([marker; 32]).expect("non-zero TransportIdentity"),
    )
}

fn fence(value: u128) -> NonZeroU128 {
    NonZeroU128::new(value).expect("non-zero fence")
}

fn attempt(marker: u8) -> AuthorityAttemptId {
    AuthorityAttemptId::new([marker; 32]).expect("non-zero authority attempt ID")
}
