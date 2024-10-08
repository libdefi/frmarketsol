use anchor_lang::prelude::*;
declare_id!("2ofxvLdNQMNFmg6WYovFdnfU6GconP7sYhbipYJXUYuw");

#[program]
pub mod market_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let market = &mut ctx.accounts.market;
        market.current_round_id = 0;
        market.rounds = Vec::new();
        market.options = Vec::new();
        Ok(())
    }

    pub fn create_round(ctx: Context<CreateRound>, betting_deadline: i64) -> Result<()> {
        let market = &mut ctx.accounts.market;
        let round_id = market.current_round_id + 1;
        let new_round = Round {
            betting_deadline,
            is_active: true,
            total_invested: 0,
            round_owner: *ctx.accounts.round_owner.key,
            winner_ids: Vec::new(),
        };

        market.rounds.push((round_id, new_round));
        market.current_round_id = round_id;
        Ok(())
    }

    pub fn place_bet(ctx: Context<PlaceBet>, youtube_id: String, amount: u64, round_id: u64) -> Result<()> {
        let market = &mut ctx.accounts.market;

        // Find the round
        let round_index = market.rounds.iter().position(|&(id, _)| id == round_id)
            .ok_or(ErrorCode::RoundNotFound)?;
        
        // Check if the round is active
        if !market.rounds[round_index].1.is_active {
            return Err(ErrorCode::BettingInactive.into());
        }

        // Find or create the option for this round
        let option_index = market.options.iter().position(|&(id, _)| id == round_id)
            .unwrap_or_else(|| {
                market.options.push((round_id, Vec::new()));
                market.options.len() - 1
            });

        // Find or create the specific betting option
        let (_, options) = &mut market.options[option_index];
        let opt_index = options.iter().position(|(id, _)| id == &youtube_id)
            .unwrap_or_else(|| {
                options.push((youtube_id.clone(), BettingOption {
                    youtube_id: youtube_id.clone(),
                    total_invested: 0,
                    total_shares: 0,
                    shares: Vec::new(),
                    resolved: false,
                }));
                options.len() - 1
            });

        let (_, opt) = &mut options[opt_index];

        // Calculate share price and shares to issue
        let share_price = calculate_share_price(opt.total_invested);
        let shares_to_issue = amount as u128 / share_price;

        // Update user's shares
        if let Some(index) = opt.shares.iter().position(|(pubkey, _)| pubkey == ctx.accounts.user.key) {
            opt.shares[index].1 += shares_to_issue;
        } else {
            opt.shares.push((*ctx.accounts.user.key, shares_to_issue));
        }

        // Update option totals
        opt.total_shares += shares_to_issue;
        opt.total_invested += amount as u128;

        // Update round total
        market.rounds[round_index].1.total_invested += amount as u128;

        Ok(())
    }

    pub fn resolve_round(ctx: Context<ResolveRound>, round_id: u64, winner_ids: Vec<String>) -> Result<()> {
        let market = &mut ctx.accounts.market;

        // Search rounds based on round_id
        let round = market
            .rounds
            .iter_mut()
            .find(|(id, _)| *id == round_id)
            .ok_or(ErrorCode::RoundNotFound)?;

        // Deselect round references
        let round = &mut round.1;

        // Get current time
        let current_time = Clock::get()?.unix_timestamp;

        // Check out betting_deadline
        require!(
            current_time >= round.betting_deadline + 48 * 3600,
            ErrorCode::TooEarlyToResolve
        );

        // Resolve rounds
        round.is_active = false;
        round.winner_ids = winner_ids;

        Ok(())
    }

    pub fn claim_reward(ctx: Context<ClaimReward>, youtube_id: String, round_id: u64) -> Result<()> {
        let market = &mut ctx.accounts.market;

        // Search for options based on round_id
        let option_vec = market
            .options
            .iter_mut()
            .find(|(id, _)| *id == round_id)
            .ok_or(ErrorCode::OptionNotFound)?;

        // Get BettingOption based on youtube_id
        let option = option_vec.1
            .iter_mut()
            .find(|(id, _)| *id == youtube_id)
            .ok_or(ErrorCode::OptionNotFound)?;

        // Get your share of users
        let shares = option.1.shares.iter()
            .find(|(pubkey, _)| *pubkey == *ctx.accounts.user.key) 
            .map(|(_, shares)| *shares)
            .unwrap_or(0);
        
        require!(shares > 0, ErrorCode::NoClaimableReward);

        let reward = (option.1.total_invested * shares) / option.1.total_shares;
        
        // Set user share to 0
        option.1.shares.retain(|(pubkey, _)| *pubkey != *ctx.accounts.user.key);
        option.1.shares.push((*ctx.accounts.user.key, 0));

        // Transfer reward to user (you'll need to implement this)
        // ...

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Round {
    pub betting_deadline: i64,
    pub is_active: bool,
    pub total_invested: u128,
    pub round_owner: Pubkey,
    pub winner_ids: Vec<String>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct BettingOption {
    pub youtube_id: String,
    pub total_invested: u128,
    pub total_shares: u128,
    pub shares: Vec<(Pubkey, u128)>,
    pub resolved: bool,
}

#[account]
pub struct Market {
    pub current_round_id: u64,
    pub rounds: Vec<(u64, Round)>,
    pub options: Vec<(u64, Vec<(String, BettingOption)>)>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 1000)]  // Adjust space as needed
    pub market: Account<'info, Market>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateRound<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
    pub round_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResolveRound<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
    pub round_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Round not found")]
    RoundNotFound,
    #[msg("Betting is not active for this round")]
    BettingInactive,
    #[msg("Option not found")]
    OptionNotFound,
    #[msg("No claimable reward in this market")]
    NoClaimableReward,
    #[msg("Too early to resolve the round")]
    TooEarlyToResolve,
}

fn calculate_share_price(total_invested: u128) -> u128 {
    1 + (total_invested / 100)
}
