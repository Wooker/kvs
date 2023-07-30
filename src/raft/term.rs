use std::time::Duration;

static ELECTION_TIMEOUT: u32 = 0;

struct Term {
    current: u64,
    len: Duration,
}
