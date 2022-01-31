use serde::{Deserialize, Serialize};
use ethers_core::types::H256;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub local_domain: u32,
    pub updater: String,
}

impl From<InstantiateMsg> for ownable::msg::InstantiateMsg {
    fn from(_: InstantiateMsg) -> Self {
        ownable::msg::InstantiateMsg {}
    }
}

impl From<InstantiateMsg> for queue::msg::InstantiateMsg {
    fn from(_: InstantiateMsg) -> Self {
        queue::msg::InstantiateMsg {}
    }
}

impl From<InstantiateMsg> for merkle::msg::InstantiateMsg {
    fn from(_: InstantiateMsg) -> Self {
        merkle::msg::InstantiateMsg {}
    }
}

impl From<InstantiateMsg> for nomad_base::msg::InstantiateMsg {
    fn from(msg: InstantiateMsg) -> Self {
        nomad_base::msg::InstantiateMsg {
            local_domain: msg.local_domain,
            updater: msg.updater,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Dispatch {
        destination: u32,
        recipient: String,
        message: Vec<u8>,
    },
    Update {
        committed_root: H256,
        new_root: H256,
        signature: Vec<u8>,
    },
    DoubleUpdate {
        old_root: H256,
        new_roots: [H256; 2],
        signature: Vec<u8>,
        signature_2: Vec<u8>,
    },
    ImproperUpdate {
        old_root: H256,
        new_root: H256,
        signature: Vec<u8>,
    },
    SetUpdater {
        updater: String,
    },
    SetUpdaterManager {
        updater_manager: String,
    },
    RenounceOwnership {},
    TransferOwnership {
        new_owner: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    CommittedRoot {},
    Count {},
    HomeDomainHash {},
    LocalDomain {},
    Nonces { domain: u32 },
    Owner {},
    QueueContains { item: H256 },
    QueueEnd {},
    QueueLength {},
    Root {},
    State {},
    SuggestUpdate {},
    Tree {},
    Updater {},
    UpdaterManager {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NoncesResponse {
    pub next_nonce: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SuggestUpdateResponse {
    pub committed_root: H256,
    pub new_root: H256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UpdaterManagerResponse {
    pub updater_manager: String,
}
