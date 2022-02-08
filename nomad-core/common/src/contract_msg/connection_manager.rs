use ethers_core::types::H256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UnenrollReplica {
        domain: u32,
        updater: H256,
        signature: Vec<u8>,
    },
    OwnerEnrollReplica {
        domain: u32,
        replica: String,
    },
    OwnerUnenrollReplica {
        replica: String,
    },
    SetWatcherPermission {
        domain: u32,
        watcher: H256,
        access: bool,
    },
    SetHome {
        home: String,
    },
    RenounceOwnership {},
    TransferOwnership {
        new_owner: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    DomainToReplica { domain: u32 },
    ReplicaToDomain { replica: String },
    WatcherPermission { domain: u32, watcher: String },
    IsReplica { replica: String },
    LocalDomain {},
    Owner {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LocalDomainResponse {
    pub local_domain: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WatcherPermissionResponse {
    pub has_permission: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IsReplicaResponse {
    pub is_replica: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DomainToReplicaResponse {
    pub replica: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ReplicaToDomainResponse {
    pub domain: u32,
}
