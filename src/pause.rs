// Copyright 2019 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use crate::{
    chain::{Chain, GenesisPfxInfo},
    id::FullId,
    messages::RoutingMessage,
    parsec::ParsecMap,
    peer_manager::PeerManager,
    peer_map::PeerMap,
    routing_message_filter::RoutingMessageFilter,
    signature_accumulator::SignatureAccumulator,
    NetworkEvent, NetworkService,
};
use crossbeam_channel as mpmc;
use std::collections::VecDeque;

/// A type that wraps the internal state of a node while it is paused in order to be upgraded and/or
/// restarted. A value of this type is obtained by pausing a node and can be then used to resume
/// it.
#[allow(unused)]
pub struct PausedState {
    pub(super) chain: Chain,
    pub(super) full_id: FullId,
    pub(super) gen_pfx_info: GenesisPfxInfo,
    pub(super) msg_filter: RoutingMessageFilter,
    pub(super) msg_queue: VecDeque<RoutingMessage>,
    // TODO: instead of storing both network_service and network_rx, store only the network config.
    pub(super) network_service: NetworkService,
    pub(super) network_rx: Option<mpmc::Receiver<NetworkEvent>>,
    pub(super) parsec_map: ParsecMap,
    pub(super) peer_map: PeerMap,
    pub(super) peer_mgr: PeerManager,
    pub(super) sig_accumulator: SignatureAccumulator,
}
