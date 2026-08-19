use std::future::{Future, ready};

use prw_connectivity::{PeerConnectivityIdentity, TransportIdentity};
use prw_core::DeviceId;
use prw_remote_bridge::reachability_live_owner::{
    ReachabilityLiveOwnerAcquisition, ReachabilityLiveOwnerAuthorityError,
    ReachabilityLiveOwnerCurrentness, ReachabilityLiveOwnerFence, ReachabilityLiveOwnerGrant,
    ReachabilityLiveOwnerRelease,
};

trait AsyncReachabilityLiveOwnerAuthority {
    fn acquire<'a>(
        &'a mut self,
        peer: &'a PeerConnectivityIdentity,
    ) -> impl Future<
        Output = Result<ReachabilityLiveOwnerAcquisition, ReachabilityLiveOwnerAuthorityError>,
    > + Send
    + 'a;

    fn currentness<'a>(
        &'a mut self,
        grant: &'a ReachabilityLiveOwnerGrant,
    ) -> impl Future<
        Output = Result<ReachabilityLiveOwnerCurrentness, ReachabilityLiveOwnerAuthorityError>,
    > + Send
    + 'a;

    fn release<'a>(
        &'a mut self,
        grant: &'a ReachabilityLiveOwnerGrant,
    ) -> impl Future<Output = Result<ReachabilityLiveOwnerRelease, ReachabilityLiveOwnerAuthorityError>>
           + Send
           + 'a;
}

struct ReferenceAuthority;

impl AsyncReachabilityLiveOwnerAuthority for ReferenceAuthority {
    fn acquire<'a>(
        &'a mut self,
        _peer: &'a PeerConnectivityIdentity,
    ) -> impl Future<
        Output = Result<ReachabilityLiveOwnerAcquisition, ReachabilityLiveOwnerAuthorityError>,
    > + Send
    + 'a {
        ready(Err(
            ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous,
        ))
    }

    fn currentness<'a>(
        &'a mut self,
        _grant: &'a ReachabilityLiveOwnerGrant,
    ) -> impl Future<
        Output = Result<ReachabilityLiveOwnerCurrentness, ReachabilityLiveOwnerAuthorityError>,
    > + Send
    + 'a {
        ready(Err(
            ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous,
        ))
    }

    fn release<'a>(
        &'a mut self,
        _grant: &'a ReachabilityLiveOwnerGrant,
    ) -> impl Future<Output = Result<ReachabilityLiveOwnerRelease, ReachabilityLiveOwnerAuthorityError>>
           + Send
           + 'a {
        ready(Err(
            ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous,
        ))
    }
}

fn peer() -> PeerConnectivityIdentity {
    PeerConnectivityIdentity::new(
        DeviceId::new("c02f-w-device").expect("valid device id"),
        TransportIdentity::new([7_u8; 32]).expect("non-zero transport identity"),
    )
}

fn assert_send_future<F: Future + Send>(future: F) {
    drop(future);
}

#[test]
fn static_dispatch_async_authority_port_supports_send_borrowing_futures() {
    let peer = peer();
    let fence = ReachabilityLiveOwnerFence::new(1).expect("non-zero fence");
    let grant = ReachabilityLiveOwnerGrant::from_authority(peer.clone(), fence);
    let mut authority = ReferenceAuthority;

    assert_send_future(authority.acquire(&peer));
    assert_send_future(authority.currentness(&grant));
    assert_send_future(authority.release(&grant));
}
