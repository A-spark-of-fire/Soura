#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

// Define the keys used to store data in the contract's persistent storage.
#[contracttype]
pub enum DataKey {
    // Tracks the total votes for a specific proposal (Proposal Symbol -> u32)
    ProposalVotes(Symbol),
    // Tracks whether a specific address has already voted (Address -> bool)
    UserVoted(Address),
}

#[contract]
pub struct VotingAnalysisContract;

#[contractimpl]
impl VotingAnalysisContract {
    
    /// Casts a vote for a specific proposal.
    /// Ensures the voter has authorized the transaction and hasn't voted before.
    pub fn vote(env: Env, voter: Address, proposal: Symbol) {
        // Ensure the transaction is authorized by the voter
        voter.require_auth();

        let voted_key = DataKey::UserVoted(voter.clone());
        
        // Check if the user has already voted
        if env.storage().persistent().has(&voted_key) {
            panic!("Error: This address has already cast a vote.");
        }

        // Mark the user as having voted
        env.storage().persistent().set(&voted_key, &true);

        // Increment the vote count for the chosen proposal
        let proposal_key = DataKey::ProposalVotes(proposal.clone());
        let mut current_votes: u32 = env.storage().persistent().get(&proposal_key).unwrap_or(0);
        
        current_votes += 1;
        
        // Save the updated vote tally
        env.storage().persistent().set(&proposal_key, &current_votes);
    }

    /// Retrieves the current total votes for a given proposal for analysis.
    pub fn get_votes(env: Env, proposal: Symbol) -> u32 {
        let proposal_key = DataKey::ProposalVotes(proposal);
        // Return the current vote count, or 0 if no votes have been cast yet
        env.storage().persistent().get(&proposal_key).unwrap_or(0)
    }
}