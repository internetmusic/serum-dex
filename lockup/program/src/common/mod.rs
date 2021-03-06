use serum_lockup::accounts::vault;
use serum_lockup::accounts::Vesting;
use solana_sdk::account_info::AccountInfo;
use solana_sdk::entrypoint::ProgramResult;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

pub mod access_control;

pub fn whitelist_cpi(
    instruction: Instruction,
    safe: &Pubkey,
    beneficiary_acc_info: &AccountInfo,
    vesting: &Vesting,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let signer_seeds = vault::signer_seeds(safe, beneficiary_acc_info.key, &vesting.nonce);
    solana_sdk::program::invoke_signed(&instruction, accounts, &[&signer_seeds])
}
