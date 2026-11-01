//! The real ends of [`crate::acting`]'s two seams that are this module's own.
//!
//! **This is the only file in `Servers` that opens a socket or waits on a
//! clock.** Everything else in the acting half is behaviour driven against the
//! traits next door, which is why the sequence, the period and the whole
//! standing repair are exercised at the desk.
//!
//! **The other three seams are other modules'** and arrive with them: `Keys`
//! supplies the credential, `State, identity and recovery` and `Rented-server
//! runtime` read the installation identity at step 6 of
//! `Preparation/Build/Plan.md`, and the desktop application publishes what this
//! module supplies.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` C16.

use std::time::{Duration, SystemTime};

use smoozy_server_protocol::transport::ssh;
use smoozy_server_protocol::transport::{
    Credential, DecideHostTrust, Ending, Endpoint, Figures, OpenFailure,
};

use crate::acting::{OpenConnections, Wait};
use crate::keepalive::{KEEPALIVE_INTERVAL, UNANSWERED_COUNT};

/// **K3's figures, handed to the transport as figures of Smoozy's own
/// driving** (K1a).
///
/// **They are read from [`crate::keepalive`] rather than written again**, so
/// the one type that holds the whole rule — the interval, the count, **and the
/// further interval that fixes the 45-to-60-second window** — stays the only
/// place any of it is stated.
pub const K3S_FIGURES: Figures = Figures {
    interval: KEEPALIVE_INTERVAL,
    unanswered_before_the_break: UNANSWERED_COUNT as usize,
};

/// **K1: the connection is held by the SSH transport Smoozy itself drives**,
/// and K1a forbids running `ssh`, `sftp` or `scp` for it at any moment. This is
/// the seam's real end.
pub struct TheTransport;

impl OpenConnections for TheTransport {
    type Connection = ssh::Connection;

    async fn open(
        &mut self,
        endpoint: Endpoint,
        credential: Credential,
        decide: Box<dyn DecideHostTrust>,
        deadline: Duration,
    ) -> Result<ssh::Connection, OpenFailure> {
        // **N2's connect deadline is a waiter's deadline under `Server protocol
        // and transfers` C16 and changes no durable state when it expires.**
        // It is its own outcome rather than a claim that the address did not
        // answer.
        match tokio::time::timeout(deadline, ssh::open(&endpoint, credential, K3S_FIGURES, decide))
            .await
        {
            Ok(opened) => opened,
            Err(_) => Err(OpenFailure::TheDeadlineExpired),
        }
    }

    fn ended(&mut self, connection: &ssh::Connection) -> Option<Ending> {
        connection.ended()
    }

    async fn wait_until_it_ends(&mut self, connection: &mut ssh::Connection) -> Ending {
        connection.wait_until_it_ends().await
    }

    async fn tear_down(&mut self, connection: ssh::Connection) -> Ending {
        connection.tear_down().await
    }
}

/// The desktop's own clock and the ordinary wait.
///
/// **§16 item 21 is met here rather than worked around: a computer that sleeps
/// is not handled, and no document handles it.** Nothing corrects a pause that
/// the machine slept through, and N10a's reading is what the repair intends
/// rather than what it will do.
pub struct TheDesktopsClock;

impl Wait for TheDesktopsClock {
    async fn until(&mut self, how_long: Duration) {
        tokio::time::sleep(how_long).await;
    }

    fn now(&mut self) -> SystemTime {
        SystemTime::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_figures_handed_to_the_transport_are_k3s_own() {
        // The bound a builder composes wrongly is not composed here at all: it
        // is the one type next door that holds the whole rule.
        assert_eq!(K3S_FIGURES.interval, KEEPALIVE_INTERVAL);
        assert_eq!(
            K3S_FIGURES.the_declaration_window(),
            (
                crate::keepalive::EARLIEST_DECLARATION,
                crate::keepalive::LATEST_DECLARATION
            )
        );
    }

    #[tokio::test]
    async fn a_connect_that_runs_past_its_deadline_claims_nothing_about_the_address() {
        // N2's 20 seconds, and C16's *changes no durable state*. The address
        // here is one nothing answers at, so the deadline is what ends it —
        // and what comes back says the deadline expired rather than that the
        // machine was silent.
        let mut transport = TheTransport;
        let failure = transport
            .open(
                Endpoint {
                    // A documented test-net address, which nothing routes to.
                    host: "192.0.2.1".to_string(),
                    port: 22,
                    username: "nobody".to_string(),
                },
                Credential {
                    private_key_file: std::path::PathBuf::from("/nowhere/id_ed25519"),
                    passphrase: None,
                },
                Box::new(Refuses),
                Duration::from_millis(50),
            )
            .await
            .expect_err("nothing answers there");
        assert!(matches!(failure, OpenFailure::TheDeadlineExpired));
    }

    struct Refuses;
    impl DecideHostTrust for Refuses {
        fn decide(
            &mut self,
            _offered: &[u8],
        ) -> smoozy_server_protocol::transport::HostTrust {
            smoozy_server_protocol::transport::HostTrust::Stop
        }
    }
}
