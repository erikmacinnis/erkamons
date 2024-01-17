use {
    anchor_lang::{
        prelude::*,
        solana_program::program::invoke,
        system_program,
    },
    anchor_spl::{
        associated_token,
        token,
    },
    mpl_token_metadata::{
        ID as TOKEN_METADATA_ID,
        instructions as token_instruction,
        types as metaplex_types,
    },
};


declare_id!("BQ31J31Mb1qK1dVjYyxq6Q6B2NrW72qaGQrTxuTPQJGM");


#[program]
pub mod mint_nft {
    use super::*;

    pub fn mint(
        ctx: Context<MintNft>, 
        metadata_title: String, 
        metadata_symbol: String, 
        metadata_uri: String,
    ) -> Result<()> {

        msg!("Creating mint account...");
        msg!("Mint: {}", &ctx.accounts.mint.key());
        system_program::create_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.mint_authority.to_account_info(),
                    to: ctx.accounts.mint.to_account_info(),
                },
            ),
            10000000,
            82,
            &ctx.accounts.token_program.key(),
        )?;

        msg!("Initializing mint account...");
        msg!("Mint: {}", &ctx.accounts.mint.key());
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            0,
            &ctx.accounts.mint_authority.key(),
            Some(&ctx.accounts.mint_authority.key()),
        )?;

        msg!("Creating token account...");
        msg!("Token Address: {}", &ctx.accounts.token_account.key());    
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
                },
            ),
        )?;

        msg!("Minting token to token account...");
        msg!("Mint: {}", &ctx.accounts.mint.to_account_info().key());   
        msg!("Token Address: {}", &ctx.accounts.token_account.key());     
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            1,
        )?;

        msg!("Creating metadata account...");
        msg!("Metadata account address: {}", &ctx.accounts.metadata.to_account_info().key());
        let meta_v3_account = &token_instruction::CreateMetadataAccountV3 {
            metadata: ctx.accounts.metadata.key(), 
            mint: ctx.accounts.mint.key(), 
            mint_authority: ctx.accounts.mint_authority.key(), 
            payer: ctx.accounts.mint_authority.key(), 
            update_authority: (ctx.accounts.mint_authority.key(), false), 
            system_program: ctx.accounts.system_program.key(),
            rent: Some(ctx.accounts.rent.key()),
        };

        let meta_v3_instructions = meta_v3_account.instruction(token_instruction::CreateMetadataAccountV3InstructionArgs { 
            data: metaplex_types::DataV2 {
                name: metadata_title,
                symbol: metadata_symbol,
                uri: metadata_uri,
                seller_fee_basis_points: 0,
                collection: None,
                creators: Some(vec![
                        metaplex_types::Creator {
                            address: ctx.accounts.mint_authority.key(),
                            verified: true, 
                            share: 100,
                    },]
                ),
                uses: None,
            }, 
            is_mutable: true, 
            collection_details: None, 
        });

        invoke(
            &meta_v3_instructions,
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        msg!("Creating master edition metadata account...");
        msg!("Master edition metadata account address: {}", &ctx.accounts.master_edition.to_account_info().key());
        let master_edition = &token_instruction::CreateMasterEditionV3 {
            metadata: ctx.accounts.metadata.key(), 
            mint: ctx.accounts.mint.key(), 
            mint_authority: ctx.accounts.mint_authority.key(), 
            payer: ctx.accounts.mint_authority.key(), 
            update_authority: ctx.accounts.mint_authority.key(), 
            system_program: ctx.accounts.system_program.key(),
            rent: Some(ctx.accounts.rent.key()),
            edition: ctx.accounts.master_edition.key(),
            token_program: ctx.accounts.token_program.key(),
        };

        let master_edition_instructions = master_edition.instruction(token_instruction::CreateMasterEditionV3InstructionArgs{
            max_supply: Some(10000),
        });

        invoke(
            &master_edition_instructions,
            &[
                ctx.accounts.master_edition.to_account_info(),
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        msg!("Token mint process completed successfully.");

        Ok(())
    }
}


#[derive(Accounts)]
pub struct MintNft<'info> {
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint: Signer<'info>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
}