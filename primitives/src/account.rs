// Copyright 2019 Conflux Foundation. All rights reserved.
// Conflux is free software and distributed under GNU General Public License.
// See http://www.gnu.org/licenses/

use crate::hash::KECCAK_EMPTY;
use cfx_types::{Address, H256, U256};
use rlp::*;
use rlp_derive::{RlpDecodable, RlpEncodable};

#[derive(Clone, Debug, RlpDecodable, RlpEncodable)]
pub struct Account {
    pub address: Address,
    pub balance: U256,
    pub nonce: U256,
    pub code_hash: H256,
    /// This is the number of tokens used to rent storage.
    pub locked_balance: U256,
    /// This is the number of tokens deposited in bank.
    pub bank_balance: U256,
    /// This is the accumulated interest rate at latest deposit.
    pub bank_ar: U256,
    // TODO: check if we need the storage root, and if so, implement.
}

impl Account {
    pub fn new_empty_with_balance(
        address: &Address, balance: &U256, nonce: &U256, locked_balance: &U256,
        bank_balance: &U256, bank_ar: &U256,
    ) -> Account
    {
        Self {
            address: address.clone(),
            balance: balance.clone(),
            nonce: nonce.clone(),
            code_hash: KECCAK_EMPTY,
            locked_balance: locked_balance.clone(),
            bank_balance: bank_balance.clone(),
            bank_ar: bank_ar.clone(),
        }
    }

    pub fn new_from_rlp(
        address: &Address, rlp_bytes: &[u8],
    ) -> Result<Account, DecoderError> {
        let account = rlp::decode::<Account>(rlp_bytes)?;
        if !account.address.eq(address) {
            return Err(DecoderError::Custom("Address mismatch."));
        }

        Ok(account)
    }
}
