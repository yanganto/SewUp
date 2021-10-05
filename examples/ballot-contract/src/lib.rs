//! This is an example sewup contract for a simple voting scenario
//!
//! Where in,
//! the proposals and the chairman are set in constructor (setup once when the contract on chain)
//! only chairman can give the ballots to voters
//! the voter can vote the proposal once
//! everyone can check out the voting result after everyone voted
//!
use std::convert::TryInto;

use serde_derive::{Deserialize, Serialize};
use sewup::types::Raw;
use sewup_derive::{
    ewasm_constructor, ewasm_fn, ewasm_fn_sig, ewasm_main, ewasm_test, SizedString, Value,
};

mod errors;

static CHARIMAN: &str = "8663DBF0cC68AaF37fC8BA262F2df4c666a41993";

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Value)]
struct Voter {
    voted: bool,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Value)]
struct Proposal {
    name: SizedString!(50),
    vote_count: usize,
}

#[ewasm_constructor]
fn constructor() {
    let mut storage =
        sewup::kv::Store::new().expect("there is no return for constructor currently");

    // TODO: Use Address type, and make sure it compatiable with Key trait of KV
    let voters_bucket = storage
        .bucket::<Raw, Voter>("voters")
        .expect("there is no return for constructor currently");

    // TODO: make usize be compatiable with Key trait of KV
    /// use KV to storage array like data structure
    let mut proposals_bucket = storage
        .bucket::<Raw, Proposal>("proposals")
        .expect("there is no return for constructor currently");

    let proposals = ["carbon neutral in 2021", "safety with Rust in 2022"];

    for (idx, name) in proposals.iter().enumerate() {
        let name = sewup::types::sized_str::SizedString::new(50)
            .from_str(name)
            .unwrap();
        proposals_bucket.set(
            Raw::from(idx),
            Proposal {
                name: name.into(),
                vote_count: 0,
            },
        );
    }

    storage.save(voters_bucket);
    storage.save(proposals_bucket);
    storage
        .commit()
        .expect("there is no return for constructor currently");
}

#[ewasm_fn]
fn give_right_to_vote(voter: String) -> anyhow::Result<sewup::primitives::EwasmAny> {
    let caller = ewasm_api::caller();
    let charman_address = {
        let byte20: [u8; 20] = hex::decode(CHARIMAN)
            .expect("address should be hex format")
            .try_into()
            .expect("address should be byte20");
        ewasm_api::types::Address::from(byte20)
    };

    if caller != charman_address {
        return Err(errors::Error::ChairmanOnly.into());
    }

    let mut storage = sewup::kv::Store::load(None)?;
    let mut voters_bucket = storage.bucket::<Raw, Voter>("voters")?;
    let voter_address = {
        let byte20: [u8; 20] = hex::decode(&voter)
            .expect("address should be hex format")
            .try_into()
            .map_err(|_| errors::Error::VoterAddressIncorrect(voter.clone()))?;
        ewasm_api::types::Address::from(byte20)
    };

    return if voters_bucket.get(Raw::from(voter_address))?.is_some() {
        Err(errors::Error::VoterExist(voter).into())
    } else {
        voters_bucket.set(Raw::from(voter_address), Voter { voted: false });
        storage.save(voters_bucket);
        storage.commit()?;
        Ok(().into())
    };
}

#[ewasm_fn]
fn vote(proposal_id: usize) -> anyhow::Result<sewup::primitives::EwasmAny> {
    let caller = ewasm_api::caller();
    let caller_address = {
        let byte20: [u8; 20] = hex::decode(CHARIMAN)
            .expect("address should be hex format")
            .try_into()
            .expect("address should be byte20");
        ewasm_api::types::Address::from(byte20)
    };

    let mut storage = sewup::kv::Store::load(None)?;
    let mut voters_bucket = storage.bucket::<Raw, Voter>("voters")?;
    let mut proposals_bucket = storage.bucket::<Raw, Proposal>("proposals")?;

    if let Some(mut voter) = voters_bucket.get(Raw::from(caller_address))? {
        if voter.voted {
            return Err(errors::Error::AlreadyVote.into());
        } else {
            if let Some(mut proposal) = proposals_bucket.get(Raw::from(proposal_id))? {
                voter.voted = true;
                voters_bucket.set(Raw::from(caller_address), voter);

                proposal.vote_count += 1;
                proposals_bucket.set(Raw::from(proposal_id), proposal);

                storage.save(voters_bucket);
                storage.save(proposals_bucket);
                storage.commit()?;

                return Ok(().into());
            } else {
                return Err(errors::Error::ProposalNonExist(proposal_id).into());
            }
        }
    } else {
        return Err(errors::Error::LackRightToVote.into());
    }
}

#[ewasm_fn]
fn winning_proposals() -> anyhow::Result<sewup::primitives::EwasmAny> {
    let mut storage = sewup::kv::Store::load(None)?;
    let voters_bucket = storage.bucket::<Raw, Voter>("voters")?;
    let proposals_bucket = storage.bucket::<Raw, Proposal>("proposals")?;
    for (_, voter) in voters_bucket.iter() {
        if !voter.voted {
            return Err(errors::Error::StillVoting.into());
        }
    }
    let mut highest_vote = 0;
    let mut highest_proposals: Vec<Proposal> = vec![];
    for (_, proposal) in proposals_bucket.iter() {
        if proposal.vote_count > highest_vote {
            highest_vote = proposal.vote_count;
            highest_proposals = vec![proposal];
        } else if proposal.vote_count == highest_vote {
            highest_proposals.push(proposal);
        }
    }
    return Ok(highest_proposals.into());
}

#[ewasm_main(auto)]
fn main() -> anyhow::Result<sewup::primitives::EwasmAny> {
    use sewup_derive::ewasm_input_from;

    let contract = sewup::primitives::Contract::new()?;
    return match contract.get_function_selector()? {
        ewasm_fn_sig!(give_right_to_vote) => ewasm_input_from!(contract move give_right_to_vote),
        ewasm_fn_sig!(vote) => ewasm_input_from!(contract move vote),
        ewasm_fn_sig!(winning_proposals) => winning_proposals(),
        _ => panic!("unknown handle"),
    };
}

#[ewasm_test]
mod tests {
    use super::*;
    use sewup_derive::{ewasm_assert_eq, ewasm_auto_assert_eq, ewasm_output_from};

    #[ewasm_test]
    fn test_get_greeting() {
        assert!(true);
    }
}
