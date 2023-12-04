use anchor_lang::prelude::*;

use anchor_spl::{
    metadata::{MetadataAccount, MasterEditionAccount, Metadata},
    token::Mint,
};

use crate::state::StakingRules;

#[derive(Accounts)]
pub struct StakingRuleCreate<'info> {
    #[account(
        init, 
        payer = initializer, 
        seeds = [b"rules", collection_mint.key().as_ref()], 
        bump,
        space = StakingRules::space()
    )]
    pub rules: Account<'info, StakingRules>,

    #[account(
        mint::authority = collection_master_edition,
    )]
    pub collection_mint: Account<'info, Mint>,
    #[account(
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref()
        ],
        seeds::program = token_metadata_program.key(),
        bump,
    )]
    pub collection_metadata: Account<'info, MetadataAccount>,
    #[account(
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref(),
            b"edition",
            ],
        seeds::program = token_metadata_program.key(),
        bump,
    )]
    pub collection_master_edition: Account<'info, MasterEditionAccount>,
    pub token_metadata_program: Program<'info, Metadata>,

    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> StakingRuleCreate<'info> {
    pub fn create(
        &mut self,
        reward_per_unix: f64,
        bumps: &StakingRuleCreateBumps
    ) -> Result<()> {

        self.rules.authority = self.initializer.key();
        self.rules.collection_address = self.collection_mint.key();
        self.rules.reward_per_unix = reward_per_unix;
        self.rules.bump = bumps.rules;

        Ok(())
    }
}