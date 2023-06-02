//! `Net` namespace

use crate::{api::Namespace, helpers::CallFuture, transports::ic_http_client::CallOptions, types::U256, Transport};

/// `Net` namespace
#[derive(Debug, Clone)]
pub struct Net<T> {
    transport: T,
}

impl<T: Transport> Namespace<T> for Net<T> {
    fn new(transport: T) -> Self
    where
        Self: Sized,
    {
        Net { transport }
    }

    fn transport(&self) -> &T {
        &self.transport
    }
}

impl<T: Transport> Net<T> {
    /// Returns the network id.
    pub fn version(&self, options: CallOptions) -> CallFuture<String, T::Out> {
        CallFuture::new(self.transport.execute("net_version", vec![], options))
    }

    /// Returns number of peers connected to node.
    pub fn peer_count(&self, options: CallOptions) -> CallFuture<U256, T::Out> {
        CallFuture::new(self.transport.execute("net_peerCount", vec![], options))
    }

    /// Whether the node is listening for network connections
    pub fn is_listening(&self, options: CallOptions) -> CallFuture<bool, T::Out> {
        CallFuture::new(self.transport.execute("net_listening", vec![], options))
    }
}

#[cfg(test)]
mod tests {
    use super::Net;
    use crate::{api::Namespace, rpc::Value, transports::ic_http_client::CallOptions, types::U256};

    rpc_test! (
      Net:version,CallOptions::default() => "net_version", Vec::<String>::new();
      Value::String("Test123".into()) => "Test123"
    );

    rpc_test! (
      Net:peer_count,CallOptions::default() => "net_peerCount",Vec::<String>::new();
      Value::String("0x123".into()) => U256::from(0x123)
    );

    rpc_test! (
      Net:is_listening,CallOptions::default() => "net_listening",Vec::<String>::new();
      Value::Bool(true) => true
    );
}
