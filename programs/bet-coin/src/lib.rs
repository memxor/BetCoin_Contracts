use anchor_lang::prelude::*;

declare_id!("5C3b6Qbj7CqQxtHyCnRMzMjTR3dbFTtcYxLREGynRPFX");

pub const GAME_SEED: &[u8] = b"GAME";
pub const PLAYER_SEED: &[u8] = b"PLAYER";

#[program]
pub mod bet_coin 
{
    use super::*;

    pub fn initialize_player(ctx: Context<InitializePlayer>, username: String) -> Result<()> 
    {
        let player = &mut ctx.accounts.player;
        let game = &mut ctx.accounts.game;

        game.total_players = game.total_players.checked_add(1).unwrap();

        player.username = username;
        player.authority = ctx.accounts.signer.key();
        player.games_played = 0;
        player.games_won = 0;
        player.solana_won = 0;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(username: String)]
pub struct InitializePlayer<'info>
{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init_if_needed, payer = signer, seeds = [GAME_SEED], bump, space = 8 + 8 + 8 + 8)]
    pub game: Account<'info, Game>,

    #[account(init, payer = signer, seeds = [PLAYER_SEED, signer.key().as_ref()], bump, space = 8 + 32 + 8 + 8 + 8 + 4 + username.len())]
    pub player: Account<'info, Player>,

    pub system_program: Program<'info, System>
}

#[account]
pub struct Game
{
    pub total_players: u64,
    pub total_solana_won: u64,
    pub total_games_played: u64
}

#[account]
pub struct Player
{
    pub authority: Pubkey,
    pub username: String,
    pub games_played: u64,
    pub games_won: u64,
    pub solana_won: u64
}