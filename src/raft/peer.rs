enum PeerState {
    Leader,
    Follower,
    Candidate,
}

impl Default for PeerState {
    fn default() -> Self {
        Self::Follower
    }
}

pub(crate) struct Peer {
    state: PeerState,
}
