use crate::{
    api::Namespace,
    helpers::{self, CallFuture},
    transports::ic_http_client::CallOptions,
    types::{Address, ParityPeerType, H256},
    Transport,
};

#[derive(Debug, Clone)]
/// `Parity_Set` Specific API
pub struct ParitySet<T> {
    transport: T,
}

impl<T: Transport> Namespace<T> for ParitySet<T> {
    fn new(transport: T) -> Self {
        ParitySet { transport }
    }

    fn transport(&self) -> &T {
        &self.transport
    }
}

impl<T: Transport> ParitySet<T> {
    /// Set Parity to accept non-reserved peers (default behavior)
    pub fn accept_non_reserved_peers(&self, options: CallOptions) -> CallFuture<bool, T::Out> {
        CallFuture::new(
            self.transport()
                .execute("parity_acceptNonReservedPeers", vec![], options),
        )
    }

    /// Add a reserved peer
    pub fn add_reserved_peer(&self, enode: &str, options: CallOptions) -> CallFuture<bool, T::Out> {
        let enode = helpers::serialize(&enode);
        CallFuture::new(self.transport().execute("parity_addReservedPeer", vec![enode], options))
    }

    /// Set Parity to drop all non-reserved peers. To restore default behavior call parity_acceptNonReservedPeers
    pub fn drop_non_reserved_peers(&self, options: CallOptions) -> CallFuture<bool, T::Out> {
        CallFuture::new(self.transport().execute("parity_dropNonReservedPeers", vec![], options))
    }

    /// Get list of connected/connecting peers.
    pub fn parity_net_peers(&self, options: CallOptions) -> CallFuture<ParityPeerType, T::Out> {
        CallFuture::new(self.transport.execute("parity_netPeers", vec![], options))
    }

    /// Attempts to upgrade Parity to the version specified in parity_upgradeReady
    pub fn execute_upgrade(&self, options: CallOptions) -> CallFuture<bool, T::Out> {
        CallFuture::new(self.transport().execute("parity_executeUpgrade", vec![], options))
    }

    /// Creates a hash of a file at a given URL
    pub fn hash_content(&self, url: &str, options: CallOptions) -> CallFuture<H256, T::Out> {
        let url = helpers::serialize(&url);
        CallFuture::new(self.transport().execute("parity_hashContent", vec![url], options))
    }

    /// Remove a reserved peer
    pub fn remove_reserved_peer(&self, enode: &str, options: CallOptions) -> CallFuture<bool, T::Out> {
        let enode = helpers::serialize(&enode);
        CallFuture::new(
            self.transport()
                .execute("parity_removeReservedPeer", vec![enode], options),
        )
    }

    /// Changes author (coinbase) for mined blocks
    pub fn set_author(&self, author: &Address, options: CallOptions) -> CallFuture<bool, T::Out> {
        let address = helpers::serialize(&author);
        CallFuture::new(self.transport().execute("parity_setAuthor", vec![address], options))
    }

    /// Sets the network spec file Parity is using
    pub fn set_chain(&self, chain: &str, options: CallOptions) -> CallFuture<bool, T::Out> {
        let chain = helpers::serialize(&chain);
        CallFuture::new(self.transport().execute("parity_setChain", vec![chain], options))
    }

    /// Sets an authority account for signing consensus messages
    pub fn set_engine_signer(
        &self,
        address: &Address,
        password: &str,
        options: CallOptions,
    ) -> CallFuture<bool, T::Out> {
        let address = helpers::serialize(&address);
        let password = helpers::serialize(&password);
        CallFuture::new(
            self.transport()
                .execute("parity_setEngineSigner", vec![address, password], options),
        )
    }

    /// Changes extra data for newly mined blocks
    pub fn set_extra_data(&self, data: &H256, options: CallOptions) -> CallFuture<bool, T::Out> {
        let data = helpers::serialize(&data);
        CallFuture::new(self.transport().execute("parity_setExtraData", vec![data], options))
    }

    /// Sets new gas ceiling target for mined blocks
    pub fn set_gas_ceil_target(&self, quantity: &H256, options: CallOptions) -> CallFuture<bool, T::Out> {
        let quantity = helpers::serialize(&quantity);
        CallFuture::new(
            self.transport()
                .execute("parity_setGasCeilTarget", vec![quantity], options),
        )
    }

    /// Sets a new gas floor target for mined blocks
    pub fn set_gas_floor_target(&self, quantity: &H256, options: CallOptions) -> CallFuture<bool, T::Out> {
        let quantity = helpers::serialize(&quantity);
        CallFuture::new(
            self.transport()
                .execute("parity_setGasFloorTarget", vec![quantity], options),
        )
    }

    /// Sets the maximum amount of gas a single transaction may consume
    pub fn set_max_transaction_gas(&self, quantity: &H256, options: CallOptions) -> CallFuture<bool, T::Out> {
        let quantity = helpers::serialize(&quantity);
        CallFuture::new(
            self.transport()
                .execute("parity_setMaxTransactionGas", vec![quantity], options),
        )
    }

    /// Changes minimal gas price for transaction to be accepted to the queue
    pub fn set_min_gas_price(&self, quantity: &H256, options: CallOptions) -> CallFuture<bool, T::Out> {
        let quantity = helpers::serialize(&quantity);
        CallFuture::new(
            self.transport()
                .execute("parity_setMinGasPrice", vec![quantity], options),
        )
    }

    /// Changes the operating mode of Parity.
    pub fn set_mode(&self, mode: &str, options: CallOptions) -> CallFuture<bool, T::Out> {
        let mode = helpers::serialize(&mode);
        CallFuture::new(self.transport().execute("parity_setMode", vec![mode], options))
    }

    /// Changes limit for transactions in queue. (NOT WORKING !)
    pub fn set_transactions_limit(&self, limit: &H256, options: CallOptions) -> CallFuture<bool, T::Out> {
        let limit = helpers::serialize(&limit);
        CallFuture::new(
            self.transport()
                .execute("parity_setTransactionsLimit", vec![limit], options),
        )
    }

    /// Returns a ReleaseInfo object describing the release which is available for upgrade or null if none is available.
    pub fn upgrade_ready(&self, options: CallOptions) -> CallFuture<Option<String>, T::Out> {
        CallFuture::new(self.transport().execute("parity_upgradeReady", vec![], options))
    }
}

#[cfg(test)]
mod tests {
    use super::ParitySet;
    use crate::{
        api::Namespace,
        rpc::Value,
        transports::ic_http_client::CallOptions,
        types::{Address, ParityPeerInfo, ParityPeerType, PeerNetworkInfo, PeerProtocolsInfo, H256},
    };

    rpc_test! (
        ParitySet:accept_non_reserved_peers,CallOptions::default() => "parity_acceptNonReservedPeers", Vec::<String>::new();
        Value::Bool(true) => true
    );

    rpc_test! (
        ParitySet:add_reserved_peer,
        "enode://a979fb575495b8d6db44f750317d0f4622bf4c2aa3365d6af7c284339968eef29b69ad0dce72a4d8db5ebb4968de0e3bec910127f134779fbcb0cb6d3331163c@22.99.55.44:7770",CallOptions::default()
        => "parity_addReservedPeer", vec![r#""enode://a979fb575495b8d6db44f750317d0f4622bf4c2aa3365d6af7c284339968eef29b69ad0dce72a4d8db5ebb4968de0e3bec910127f134779fbcb0cb6d3331163c@22.99.55.44:7770""#];
        Value::Bool(true) => true
    );

    rpc_test! (
        ParitySet:parity_net_peers ,CallOptions::default()=> "parity_netPeers", Vec::<String>::new();
        serde_json::from_str::<Value>(r#"{"active":1,"connected":1,"max":1,"peers":[{"id":"f900000000000000000000000000000000000000000000000000000000lalalaleelooooooooo","name":"","caps":[],"network":{"remoteAddress":"Handshake","localAddress":"127.0.0.1:43128"},"protocols":{"eth":null,"pip":null}}]}"#).unwrap()
            => ParityPeerType {
                active: 1,
                connected: 1,
                max: 1,
                peers: vec![ParityPeerInfo {
                    id: Some(String::from("f900000000000000000000000000000000000000000000000000000000lalalaleelooooooooo")),
                    name: String::from(""),
                    caps: vec![],
                    network: PeerNetworkInfo {
                        remote_address: String::from("Handshake"),
                        local_address: String::from("127.0.0.1:43128"),
                    },
                    protocols: PeerProtocolsInfo {
                        eth: None,
                        pip: None,
                    },
                }],
            }
    );

    rpc_test! (
        ParitySet:drop_non_reserved_peers,CallOptions::default() => "parity_dropNonReservedPeers", Vec::<String>::new();
        Value::Bool(true) => true
    );

    rpc_test! (
        ParitySet:execute_upgrade,CallOptions::default() => "parity_executeUpgrade", Vec::<String>::new();
        Value::Bool(true) => true
    );

    rpc_test! (
        ParitySet:hash_content,
        "https://raw.githubusercontent.com/paritytech/parity-ethereum/master/README.md",CallOptions::default()
        => "parity_hashContent", vec![r#""https://raw.githubusercontent.com/paritytech/parity-ethereum/master/README.md""#];
        Value::String("0x5198e0fc1a9b90078c2e5bfbc6ab6595c470622d3c28f305d3433c300bba5a46".into()) => "5198e0fc1a9b90078c2e5bfbc6ab6595c470622d3c28f305d3433c300bba5a46".parse::<H256>().unwrap()
    );

    rpc_test! (
        ParitySet:remove_reserved_peer,
        "enode://a979fb575495b8d6db44f750317d0f4622bf4c2aa3365d6af7c284339968eef29b69ad0dce72a4d8db5ebb4968de0e3bec910127f134779fbcb0cb6d3331163c@22.99.55.44:7770",CallOptions::default()
        => "parity_removeReservedPeer", vec![r#""enode://a979fb575495b8d6db44f750317d0f4622bf4c2aa3365d6af7c284339968eef29b69ad0dce72a4d8db5ebb4968de0e3bec910127f134779fbcb0cb6d3331163c@22.99.55.44:7770""#];
        Value::Bool(true) => true
    );

    rpc_test! (
        ParitySet:set_author, &"407d73d8a49eeb85d32cf465507dd71d507100c1".parse::<Address>().unwrap(),CallOptions::default()
        => "parity_setAuthor", vec![r#""0x407d73d8a49eeb85d32cf465507dd71d507100c1""#];
        Value::Bool(true) => true
    );

    rpc_test! (
        ParitySet:set_chain, "kovan",CallOptions::default()
        => "parity_setChain", vec![r#""kovan""#];
        Value::Bool(true) => true
    );

    rpc_test! (
        ParitySet:set_engine_signer, &"407d73d8a49eeb85d32cf465507dd71d507100c1".parse::<Address>().unwrap(), "hunter2",CallOptions::default()
        => "parity_setEngineSigner", vec![r#""0x407d73d8a49eeb85d32cf465507dd71d507100c1""#, r#""hunter2""#];
        Value::Bool(true) => true
    );

    rpc_test! (
        ParitySet:set_extra_data,
        &"5198e0fc1a9b90078c2e5bfbc6ab6595c470622d3c28f305d3433c300bba5a46".parse::<H256>().unwrap(),CallOptions::default()
        => "parity_setExtraData", vec![r#""0x5198e0fc1a9b90078c2e5bfbc6ab6595c470622d3c28f305d3433c300bba5a46""#];
        Value::Bool(true) => true
    );

    rpc_test! (
        ParitySet:set_gas_ceil_target, &"0000000000000000000000000000000000000000000000000000000000000123".parse::<H256>().unwrap(),CallOptions::default()
        => "parity_setGasCeilTarget", vec![r#""0x0000000000000000000000000000000000000000000000000000000000000123""#];
        Value::Bool(true) => true
    );

    rpc_test! (
        ParitySet:set_gas_floor_target, &"0000000000000000000000000000000000000000000000000000000000000123".parse::<H256>().unwrap(),CallOptions::default()
        => "parity_setGasFloorTarget", vec![r#""0x0000000000000000000000000000000000000000000000000000000000000123""#];
        Value::Bool(true) => true
    );

    rpc_test! (
        ParitySet:set_max_transaction_gas,
        &"0000000000000000000000000000000000000000000000000000000000000123".parse::<H256>().unwrap(),CallOptions::default()
        => "parity_setMaxTransactionGas", vec![r#""0x0000000000000000000000000000000000000000000000000000000000000123""#];
        Value::Bool(true) => true
    );

    rpc_test! (
        ParitySet:set_min_gas_price,
        &"0000000000000000000000000000000000000000000000000000000000000123".parse::<H256>().unwrap(),CallOptions::default()
        => "parity_setMinGasPrice", vec![r#""0x0000000000000000000000000000000000000000000000000000000000000123""#];
        Value::Bool(true) => true
    );

    rpc_test! (
        ParitySet:set_mode, "offline",CallOptions::default()
        => "parity_setMode", vec![r#""offline""#];
        Value::Bool(true) => true
    );

    rpc_test! (
        ParitySet:set_transactions_limit,
        &"0000000000000000000000000000000000000000000000000000000000000123".parse::<H256>().unwrap(),CallOptions::default()
        => "parity_setTransactionsLimit", vec![r#""0x0000000000000000000000000000000000000000000000000000000000000123""#];
        Value::Bool(true) => true
    );

    rpc_test!(
        ParitySet:upgrade_ready,CallOptions::default() => "parity_upgradeReady", Vec::<String>::new();
        Value::Null => None
    );
}
