use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token::Token};

declare_id!("CmzBSk4bbNWb74X7HGAFyHjLzyrTsEPtd6Jky3SBLt9i");

#[program]
pub mod mint_nft {
    use anchor_lang::{solana_program::program::invoke, system_program};
    use anchor_spl::token::{self, Mint};
    use mpl_token_metadata::{instructions::{CreateMasterEditionV3Builder, CreateMetadataAccountV3Builder}, types::DataV2};

    use super::*;

    pub fn mint(
        ctx: Context<MintNFT>,
        name: String,
        symbol: String,
        uri: String
    ) -> Result<()> {
        // msg!("Creating mint account: {}", ctx.accounts.mint.key());
        system_program::create_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.mint_authority.to_account_info(),
                    to: ctx.accounts.mint.to_account_info()
                }
            ),
            ctx.accounts.rent.minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            &ctx.accounts.token_program.key()
        )?;
        
        // msg!("Initializint mint account: {}", ctx.accounts.mint.key());
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info()
                }
            ),
            0,
            &ctx.accounts.mint_authority.key(),
            Some(&ctx.accounts.mint_authority.key())
        )?;

        // msg!("Creating token account: {}", ctx.accounts.token_account.key());
        associated_token::create(
            CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info(),
                associated_token::Create {
                    payer: ctx.accounts.mint_authority.to_account_info(),
                    associated_token: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                }
            )
        )?;

        // msg!("Minting {} to token account: {}", ctx.accounts.mint.to_account_info().key(), ctx.accounts.token_account.key());
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                }    
            ),
            1
        )?;

        // msg!("Creating metadata account: {}", ctx.accounts.metadata.to_account_info().key());
        
        let creators = None;
        let seller_fee_basis_points = 1;
        let update_authority_is_signer = true;
        let is_mutable = true;
        let collection = None;
        let uses = None;
        invoke(
            &CreateMetadataAccountV3Builder::default()
                .metadata(ctx.accounts.metadata.key())
                .mint(ctx.accounts.mint.key())
                .mint_authority(ctx.accounts.mint_authority.key())
                .payer(ctx.accounts.mint_authority.key())
                .update_authority(ctx.accounts.mint_authority.key(), update_authority_is_signer)
                .system_program(ctx.accounts.system_program.key())
                .rent(Some(ctx.accounts.rent.key()))
                .is_mutable(is_mutable)
                .data(DataV2 { collection, creators, name, symbol, uri, seller_fee_basis_points, uses })
                .instruction(),
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.rent.to_account_info()
            ]
        )?;

        // msg!("Creating master edition account: {}", ctx.accounts.master_edition.to_account_info().key());
        invoke(
            &CreateMasterEditionV3Builder::default()
                .metadata(ctx.accounts.metadata.key())
                .mint(ctx.accounts.mint.key())
                .payer(ctx.accounts.mint_authority.key())
                .edition(ctx.accounts.master_edition.key())
                .mint_authority(ctx.accounts.mint_authority.key())
                .update_authority(ctx.accounts.mint_authority.key())
                .max_supply(0)
                .instruction(),
            &[
                ctx.accounts.master_edition.to_account_info(),
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.rent.to_account_info()
            ]
        )?;

        msg!("Token mint process completed successfully.");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    /// CHECK: We're about to create this with Metaplex.
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: We're about to create this with Metaplex.
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint: Signer<'info>,
    /// CHECK: We're about to create this with Anchor.
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    /// CHECK: Metaplex will check this.
    pub token_metadata_program: UncheckedAccount<'info>,
}
