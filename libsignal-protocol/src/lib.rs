//! A Rust interface to the [libsignal-protocol-c] library.
//!
//! A ratcheting forward secrecy protocol that works in synchronous and
//! asynchronous messaging environments.
//!
//! # Key Concepts
//!
//! ## PreKeys
//!
//! This protocol uses a concept called "*PreKeys*". A PreKey is a
//! [`keys::PublicKey`] and an associated unique ID which are stored together by
//! a server. PreKeys can also be signed.
//!
//! At install time, clients generate a single signed PreKey, as well as a large
//! list of unsigned PreKeys, and transmit all of them to the server.
//!
//! ## Sessions
//!
//! The Signal Protocol is session-oriented. Clients establish a "session,"
//! which is then used for all subsequent encrypt/decrypt operations. There is
//! no need to ever tear down a session once one has been established.
//!
//! Sessions are established in one of three ways:
//!
//! 1. [`PreKeyBundle`]. A client that wishes to send a message to a recipient
//!    can establish a session by retrieving a PreKeyBundle for that recipient
//!    from the server.
//! 2. PreKeySignalMessages.  A client can receive a PreKeySignalMessage from a
//!    recipient and use it to establish a session.
//! 3. KeyExchangeMessages.  Two clients can exchange KeyExchange messages to
//!    establish a session.
//!
//! ## State
//!
//! An established session encapsulates a lot of state between two clients. That
//! state is maintained in durable records which need to be kept for the life of
//! the session.
//!
//! State is kept in the following places:
//!
//! 1. Identity State. Clients will need to maintain the state of their own
//!    identity key pair, as well as identity keys received from other clients
//!    (saved in an [`IdentityKeyStore`]).
//! 1. PreKey State. Clients will need to maintain the state of their generated
//!    PreKeys in a [`PreKeyStore`].
//! 1. Signed PreKey States. Clients will need to maintain the state of their
//!    signed PreKeys using a [`SignedPreKeyStore`].
//! 1. Session State. Clients will need to maintain the state of the sessions
//!    they have established using a [`SessionStore`].
//!
//! [libsignal-protocol-c]: https://github.com/signalapp/libsignal-protocol-c

extern crate libsignal_protocol_sys as sys;

pub use crate::{
    address::Address,
    buffer::Buffer,
    context::Context,
    crypto::{CipherMode, Crypto, SignalCipherType, SignalCipherTypeError},
    errors::InternalError,
    hkdf::HMACBasedKeyDerivationFunction,
    identity_key_store::IdentityKeyStore,
    pre_key_bundle::{PreKeyBundle, PreKeyBundleBuilder},
    pre_key_store::PreKeyStore,
    session_builder::SessionBuilder,
    session_store::SessionStore,
    signed_pre_key_store::SignedPreKeyStore,
    store_context::StoreContext,
};

mod address;
mod buffer;
mod context;
pub mod crypto;
mod errors;
mod hkdf;
mod identity_key_store;
pub mod keys;
mod pre_key_bundle;
mod pre_key_store;
mod raw_ptr;
mod session_builder;
mod session_store;
mod signed_pre_key_store;
mod store_context;
