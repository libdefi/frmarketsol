use anchor_lang::prelude::*;
use reclaim::cpi::accounts::VerifyProof;
use reclaim::cpi::verify_proof;
use reclaim::instructions::VerifyProofArgs;
use reclaim::program::Reclaim;
use reclaim::state::ClaimData as ReclaimClaimData;
use reclaim::state::ClaimInfo as ReclaimClaimInfo;
use reclaim::state::SignedClaim as ReclaimSignedClaim;
use reclaim::state::{Epoch, EpochConfig};
use reclaim::state::Epoch as ReclaimEpoch;

#[account]
pub struct EpochWrapper {
    pub inner: ReclaimEpoch,
}

#[account]
pub struct EpochConfigWrapper {
    pub inner: ReclaimEpochConfig,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct VerifyArgs {
    pub claim_info: ClaimInfo,
    pub signed_claim: SignedClaim,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SignedClaim {
    pub claim_data: ClaimData,
    pub signatures: Vec<[u8; 65]>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ClaimInfo {
    pub provider: String,
    pub parameters: String,
    pub context_address: Pubkey,
    pub context_message: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ClaimData {
    pub identifier: [u8; 32],
    pub owner: String,
    pub timestamp: u32,
    pub epoch_index: u32,
}

#[derive(Accounts)]
pub struct Verify<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub epoch_config: Account<'info, EpochConfigWrapper>,
    pub epoch: Account<'info, EpochWrapper>,
    pub reclaim_program: Program<'info, Reclaim>,
    pub system_program: Program<'info, System>,
}

