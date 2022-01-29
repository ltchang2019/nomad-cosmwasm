/// A lightweight incremental merkle, suitable for running on-chain. Stores O
/// (1) data
pub mod incremental;
pub use incremental::*;

pub mod proof;
pub use proof::*;

use ethers_core::types::H256;
use lazy_static::lazy_static;
use sha3::{digest::Update, Digest, Keccak256};

/// Tree depth
pub const TREE_DEPTH: usize = 32;
const EMPTY_SLICE: &[[u8; 32]] = &[];

lazy_static! {
    /// A cache of the zero hashes for each layer of the tree.
    pub static ref ZERO_HASHES: [[u8; 32]; TREE_DEPTH + 1] = {
        let mut hashes = [H256::zero().into(); TREE_DEPTH + 1];
        for i in 0..TREE_DEPTH {
            hashes[i + 1] = hash_concat(hashes[i], hashes[i]);
        }
        hashes
    };

    /// The root of an empty tree
    #[derive(Debug, PartialEq)]
    pub static ref INITIAL_ROOT: [u8; 32] = incremental::IncrementalMerkle::default().root();
}

pub(super) fn hash(preimage: impl AsRef<[u8]>) -> [u8; 32] {
    H256::from_slice(Keccak256::digest(preimage.as_ref()).as_slice()).into()
}

pub(super) fn hash_concat(left: impl AsRef<[u8]>, right: impl AsRef<[u8]>) -> [u8; 32] {
    H256::from_slice(
        Keccak256::new()
            .chain(left.as_ref())
            .chain(right.as_ref())
            .finalize()
            .as_slice(),
    )
    .into()
}

/// Compute a root hash from a leaf and a Merkle proof.
pub(super) fn merkle_root_from_branch(
    leaf: [u8; 32],
    branch: &[[u8; 32]],
    depth: usize,
    index: usize,
) -> [u8; 32] {
    assert_eq!(branch.len(), depth, "proof length should equal depth");

    let mut current = leaf;

    for (i, next) in branch.iter().enumerate().take(depth) {
        let ith_bit = (index >> i) & 0x01;
        if ith_bit == 1 {
            current = hash_concat(next, current);
        } else {
            current = hash_concat(current, next);
        }
    }

    current
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn it_calculates_the_initial_root() {
        assert_eq!(
            H256::from(*INITIAL_ROOT),
            "0x27ae5ba08d7291c96c8cbddcc148bf48a6d68c7974b94356f53754ef6171d757"
                .parse()
                .unwrap()
        );
    }
}
