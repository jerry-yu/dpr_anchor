use anchor_lang::prelude::*;

declare_id!("CjbzEcNr2ZtqJBMFRHFCcCGnzi8WGR7jbTbBVoF1hq8U");

#[program]
pub mod deeper_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(init,payer = payer,space = 8 + 32)]
    dpr_token: Account<'info, TokenAccount>,
    system_program: Program<'info, System>,
}


#[account]
#[derive(Default)]
pub struct UserCredit {
    pub campaign_id: u16,
    pub level: u8,
    pub day: u32,
}

// 用户账户
#[account]
#[derive(Default)]
pub struct UserAccount {
    pub campaign_id: u16,
    pub credit: u32,
    pub reward_since: u32,
    pub history: Vec<UserCredit>,
}

impl UserAccount {
    pub fn length(&self) -> usize {
        self.history.len() * std::mem::size_of::<UserCredit>()
    }
}

#[account]
#[derive(Default)]
pub struct TokenAccount {
    pub token: Pubkey,
}

#[account]
#[derive(Default)]
pub struct CreditSetting {
    pub campaign_id: u16,
    pub level: u8,
    pub daily_reward: u64,
}

#[account]
#[derive(Default)]
pub struct CreditSettings {
    pub settings: Vec<CreditSetting>,
}

#[account]
#[derive(Default)]
pub struct PrivelegeUser {
    pub users: Vec<Pubkey>,
}

#[derive(Accounts)]
pub struct SetupCampaign<'info> {
    #[account(init, payer = user, space = 8 + std::mem::size_of::<UserAccount>())]
    pub user_account: Account<'info, UserAccount>,
    
    #[account(mut)]
    pub user: Signer<'info>, // 作为交易发起者
    
    #[account(init, payer = user, space = 8 + std::mem::size_of::<CreditSettings>())]
    pub credit_settings: Account<'info, CreditSettings>,
    
    pub system_program: Program<'info, System>,
}
