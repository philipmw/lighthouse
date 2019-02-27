use super::{AggregatePublicKey, AggregateSignature, AttestationData, Bitfield, Hash256};
use crate::test_utils::TestRandom;
use rand::RngCore;
use serde_derive::Serialize;
use ssz::TreeHash;
use ssz_derive::{Decode, Encode, TreeHash};
use test_random_derive::TestRandom;

#[derive(Debug, Clone, PartialEq, Serialize, Encode, Decode, TreeHash, TestRandom)]
pub struct Attestation {
    pub aggregation_bitfield: Bitfield,
    pub data: AttestationData,
    pub custody_bitfield: Bitfield,
    pub aggregate_signature: AggregateSignature,
}

impl Attestation {
    pub fn canonical_root(&self) -> Hash256 {
        Hash256::from(&self.hash_tree_root()[..])
    }

    pub fn signable_message(&self, custody_bit: bool) -> Vec<u8> {
        self.data.signable_message(custody_bit)
    }

    pub fn verify_signature(
        &self,
        group_public_key: &AggregatePublicKey,
        custody_bit: bool,
        domain: u64,
    ) -> bool {
        self.aggregate_signature.verify(
            &self.signable_message(custody_bit),
            domain,
            group_public_key,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{SeedableRng, TestRandom, XorShiftRng};
    use ssz::{ssz_encode, Decodable, TreeHash};

    #[test]
    pub fn test_ssz_round_trip() {
        let mut rng = XorShiftRng::from_seed([42; 16]);
        let original = Attestation::random_for_test(&mut rng);

        let bytes = ssz_encode(&original);
        let (decoded, _) = <_>::ssz_decode(&bytes, 0).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    pub fn test_hash_tree_root_internal() {
        let mut rng = XorShiftRng::from_seed([42; 16]);
        let original = Attestation::random_for_test(&mut rng);

        let result = original.hash_tree_root_internal();

        assert_eq!(result.len(), 32);
        // TODO: Add further tests
        // https://github.com/sigp/lighthouse/issues/170
    }
}