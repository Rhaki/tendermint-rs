//! Tendermint RPC definitions and types.
//!
//! ## Client
//!
//! This crate optionally provides access to different types of RPC client
//! functionality and different client transports based on which features you
//! select when using it.
//!
//! Two features are provided at present.
//!
//! | Feature | Description |
//! | ------- | ----------- |
//! | `http-client` | Provides [`HttpClient`], which is a basic RPC client that interacts with
//! remote Tendermint nodes via **JSON-RPC over HTTP**. This client does not provide [`Event`]
//! subscription functionality. See the [Tendermint RPC] for more details. |
//! | `websocket-client` | Provides [`WebSocketClient`], which currently only provides [`Event`]
//! subscription functionality over a WebSocket connection. See the [`/subscribe` endpoint] in the
//! Tendermint RPC for more details. This client does not yet provide access to the RPC methods
//! provided by the [`Client`] trait (this is planned for a future release). |
//!
//! ### Mock Clients
//!
//! Mock clients are included when either of the `http-client` or
//! `websocket-client` features are enabled to aid in testing. This includes
//! [`MockClient`], which only implements [`Client`] (no subscription
//! functionality), and [`MockSubscriptionClient`], which helps you simulate
//! subscriptions to events being generated by a Tendermint node.
//!
//! [`Client`]: trait.Client.html
//! [`HttpClient`]: struct.HttpClient.html
//! [`Event`]: event/struct.Event.html
//! [`WebSocketClient`]: struct.WebSocketClient.html
//! [Tendermint RPC]: https://docs.tendermint.com/master/rpc/
//! [`/subscribe` endpoint]: https://docs.tendermint.com/master/rpc/#/Websocket/subscribe
//! [`MockClient`]: struct.MockClient.html
//! [`MockSubscriptionClient`]: struct.MockSubscriptionClient.html

#[cfg(any(feature = "http-client", feature = "websocket-client"))]
mod client;
#[cfg(any(feature = "http-client", feature = "websocket-client"))]
pub use client::{
    Client, MockClient, MockRequestMatcher, MockRequestMethodMatcher, MockSubscriptionClient,
    Subscription, SubscriptionClient, SubscriptionId,
};

#[cfg(feature = "http-client")]
pub use client::HttpClient;
#[cfg(feature = "websocket-client")]
pub use client::WebSocketClient;

pub mod endpoint;
pub mod error;
pub mod event;
mod id;
mod method;
pub mod query;
pub mod request;
pub mod response;
mod result;
mod version;

pub use self::{
    error::Error, id::Id, method::Method, request::Request, response::Response, result::Result,
    version::Version,
};
