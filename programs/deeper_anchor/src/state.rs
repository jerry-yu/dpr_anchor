// use borsh::{to_vec, BorshDeserialize, BorshSerialize};
// use solana_program::{program_error::ProgramError, pubkey::Pubkey};

// #[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy, Default)]
// pub struct UserCredit {
//     pub campaign_id: u16,
//     pub level: u8,
//     pub day: u32,
// }

// #[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
// pub struct UserAccount {
//     pub campaign_id: u16,
//     pub credit: u32,
//     // reward data which days clamed
//     pub reward_since: u32,
//     // u32: days,u16:campaign id, u8: credit level
//     pub history: Vec<UserCredit>,
// }

// impl UserAccount {
//     pub fn unpack(data: &[u8]) -> Result<Self, ProgramError> {
//         let credit_bytes: [u8; 4] = data[0..4]
//             .try_into()
//             .map_err(|_| ProgramError::InvalidAccountData)?;

//         let len = u32::from_be_bytes(credit_bytes);
//         UserAccount::try_from_slice(&data[4..4 + len as usize])
//             .map_err(|_| ProgramError::BorshIoError("user account error".to_string()))
//     }

//     pub fn pack(src: Self, dst: &mut [u8]) -> Result<(), ProgramError> {
//         let buf = to_vec(&src)?;
//         let real_len = buf.len();
//         dst[0..4].copy_from_slice(&(real_len as u32).to_be_bytes());
//         dst[4..4 + real_len].copy_from_slice(&buf);
//         Ok(())
//     }
// }

// #[repr(C)]
// #[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default, PartialEq)]
// pub struct TokenAccount {
//     pub token: Pubkey,
// }

// #[repr(C)]
// #[derive(BorshSerialize, BorshDeserialize, Debug, Default, Clone, PartialEq)]
// pub struct CreditSetting {
//     pub campaign_id: u16,
//     pub level: u8,
//     pub daily_reward: u64,
// }

// #[repr(C)]
// #[derive(BorshSerialize, BorshDeserialize, Debug, Default, Clone, PartialEq)]
// pub struct CreditSettings {
//     pub settings: Vec<CreditSetting>,
// }

// #[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
// pub struct PrivelegeUser {
//     pub users: Vec<Pubkey>,
// }


use anchor_lang::prelude::*;

// 用户信用
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
    // 累计奖励的数据（已声明的天数）
    pub reward_since: u32,
    pub history: Vec<UserCredit>,
}

impl UserAccount {
    // 获取账户长度
    pub fn length(&self) -> usize {
        self.history.len() * std::mem::size_of::<UserCredit>()
    }
}

// Token账户
#[account]
#[derive(Default)]
pub struct TokenAccount {
    pub token: Pubkey,
}

// 奖励设置
#[account]
#[derive(Default)]
pub struct CreditSetting {
    pub campaign_id: u16,
    pub level: u8,
    pub daily_reward: u64,
}

// 所有奖励设置
#[account]
#[derive(Default)]
pub struct CreditSettings {
    pub settings: Vec<CreditSetting>,
}

// 管理权限的用户
#[account]
#[derive(Default)]
pub struct PrivelegeUser {
    pub users: Vec<Pubkey>,
}

// 定义程序的上下文（账户集合）
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

// // 实现设置信用
// #[program]
// pub mod solana_program_example {
//     use super::*;

//     // 设置用户账户的奖励
//     pub fn setup_campaign(ctx: Context<SetupCampaign>, campaign_id: u16, level: u8) -> Result<()> {
//         let user_account = &mut ctx.accounts.user_account;
//         user_account.campaign_id = campaign_id;
//         user_account.level = level;
//         user_account.credit = 0;
//         user_account.reward_since = 0;
//         user_account.history = Vec::new();
//         Ok(())
//     }

//     // 添加信用设置
//     pub fn add_credit_setting(ctx: Context<SetupCampaign>, daily_reward: u64) -> Result<()> {
//         let credit_settings = &mut ctx.accounts.credit_settings;
//         credit_settings.settings.push(CreditSetting {
//             campaign_id: 1, // 假设使用默认的campaign_id
//             level: 1,        // 假设为默认level
//             daily_reward,
//         });
//         Ok(())
//     }
// }
