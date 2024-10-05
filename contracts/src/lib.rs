// contracts/src/lib.rs
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::entrypoint::ProgramResult;
use solana_program::{account_info::AccountInfo, entrypoint, pubkey::Pubkey};
use std::collections::HashMap;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Market {
    pub usdc_token: Pubkey,
    pub current_round_id: u64,
    pub rounds: HashMap<u64, Round>,
    pub options: HashMap<u64, HashMap<String, Option>>, // round_id -> (youtube_id -> Option)
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Round {
    pub betting_deadline: u64,
    pub is_active: bool,
    pub total_invested: u64,
    pub round_owner: Pubkey,
    pub winner_ids: Vec<String>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Option {
    pub youtube_id: String,
    // pub creation_time: u64,
    pub total_invested: u64,
    pub total_shares: u64,
    pub shares: HashMap<Pubkey, u64>,
    pub resolved: bool,
}

impl Market {
    pub fn new(usdc_token: Pubkey) -> Self {
        Self {
            usdc_token,
            current_round_id: 0,
            rounds: HashMap::new(),
            options: HashMap::new(),
        }
    }

    pub fn create_round(&mut self, betting_deadline: u64, round_owner: Pubkey) {
        let round_id = self.current_round_id + 1;
        let new_round = Round {
            betting_deadline,
            is_active: true,
            total_invested: 0,
            round_owner,
            winner_ids: Vec::new(),
        };
        self.rounds.insert(round_id, new_round);
        self.current_round_id = round_id;
    }

    pub fn place_bet(&mut self, youtube_id: String, amount: u64, round_id: u64, user: Pubkey) {
        let round = self
            .rounds
            .get_mut(&round_id)
            .expect("Round does not exist");
        assert!(round.is_active, "Betting is not active for this round");
        // assert!(
        //     round.betting_deadline > Clock::get().unwrap().unix_timestamp as u64,
        //     "Betting period has ended"
        // );

        // `option`をミュータブルに借用
        let option = self.options.entry(round_id).or_insert_with(HashMap::new);
        let opt = option.entry(youtube_id.clone()).or_insert(Option {
            youtube_id: youtube_id.clone(),
            // creation_time: Clock::get().unwrap().unix_timestamp as u64,
            total_invested: 0,
            total_shares: 0,
            shares: HashMap::new(),
            resolved: false,
        });

        opt.total_invested += amount;

        // ユーザーのシェアを更新
        *opt.shares.entry(user).or_insert(0) += amount; // ここでミュータブルな参照を使用
        round.total_invested += amount;
    }

    pub fn resolve_round(&mut self, round_id: u64, winner_ids: Vec<String>) {
        let round = self
            .rounds
            .get_mut(&round_id)
            .expect("Round does not exist");
        round.is_active = false;
        round.winner_ids = winner_ids;
    }

    pub fn claim_reward(&mut self, youtube_id: String, round_id: u64, user: Pubkey) -> u64 {
        let option = self
            .options
            .get_mut(&round_id)
            .and_then(|o| o.get_mut(&youtube_id)) // ミュータブルな参照を取得
            .expect("Option does not exist");

        let shares = option.shares.get(&user).unwrap_or(&0);
        assert!(*shares > 0, "No claimable reward in this market");

        // 報酬計算ロジックを追加
        let reward = (option.total_invested * *shares) / option.total_shares; // 簡易的な報酬計算
        option.shares.insert(user, 0); // シェアをリセット
        reward
    }
}

entrypoint!(process_instruction);
// エントリーポイント
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // コントラクトの処理ロジックを実装
    // 例えば、create_roundやplace_betなどのメソッドを呼び出す
    Ok(())
}
