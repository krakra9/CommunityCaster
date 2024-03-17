#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;


use stylus_sdk::{alloy_primitives::U256, prelude::*, storage::StorageU256};

use stylus_sdk::alloy_primitives::Address;
use std::fs::copy;
use std::ops::{Deref, DerefMut};
use std::result::Result::Ok;
use stylus_sdk::{alloy_primitives::*, prelude::*};
use stylus_sdk::{block, contract, crypto, evm, msg, tx};
use stylus_sdk::storage::*;

use alloy_sol_types::sol;
use stylus_sdk::{console, stylus_proc::entrypoint, ArbResult};

sol! {
    event Vote(address indexed Voter, uint256 Choice);
    event Attestation(uint32 winningID);
}

sol_storage! {
#[entrypoint]
pub struct Polls {
    StorageAddress _proposer_address;
    StorageU64 timestamp_target;
    StorageBool init;
    StorageArray<StorageU64,2> votes;
    StorageVec<StorageAddress> voters;
    StorageAddress aids;
}
}

#[external]
impl Polls {

    pub fn proposer_of(&self,) -> Result<Address, Vec<u8>> {
        let owner = self._proposer_address.get();
        Ok(owner)
        
    }

    pub fn init(&mut self, proposer:Address, timestamp:U256) -> Result<(), Vec<u8>> { 
        if (!bool::from(*self.init)) {
            self.timestamp_target.set(U64::from(timestamp));
            self._proposer_address.set(proposer);
            self.init.set(true);
        }
        return Ok(());
    }

    pub fn call_vote(&mut self, option_id: U256, _voter: Address) -> Result<(), Vec<u8>> {
    if (U64::from(block::timestamp()) < self.timestamp_target.get()) {
        if (option_id <= U256::from(2) && option_id > U256::from(0)) {
        for i in 0..self.voters.len()-1 {
            let _ = match self.voters.get(i) {
                Some(x) => return Err(("User already voted".as_bytes().to_vec())),
                None => todo!()
            };
        }
            let setter = self.votes.setter(option_id);
            let mut result: StorageGuardMut<'_, StorageUint<64, 1>> = match setter {Some(x)=>x,None => todo!()};
            let value: &mut StorageUint<64, 1> = result.deref_mut();
            value.set(value.get()+U64::from(1));
            evm::log(Vote {
                Voter: _voter,
                Choice: option_id
            });
            return Ok(());

        }
        {return Err(("Invalid option id | Either 0 or 1".as_bytes().to_vec()))}
    } else {return Err(("Voting Concluded".as_bytes().to_vec()))}
}

    pub fn attest_outcome(&self) -> Result<U256, Vec<u8>> {
        if (self.timestamp_target.get() < U64::from(block::timestamp())) {
            let votesone = self.votes.get(0).unwrap();
            let votestwo = self.votes.get(1).unwrap();
            let _winning_id = if votesone > votestwo {1} else if votesone == votestwo {0} else {2};
            evm::log(Attestation{
                winningID: _winning_id
            });
            return Ok(U256::from(_winning_id));
        }
        return Err(("Voting has not ended".as_bytes().to_vec()))
    }


}

