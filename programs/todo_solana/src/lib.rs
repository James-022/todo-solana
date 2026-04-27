use anchor_lang::prelude::*;

declare_id!("
8WynjzGuEiRYtpPSTsbXESiDJ38gvELVJytZQLrRcjUH");

#[program]
pub mod todo_solana {
    use super::*;

    /// Crea una nueva tarea para el usuario
    pub fn create_task(ctx: Context<CreateTask>, title: String, description: String) -> Result<()> {
        let task = &mut ctx.accounts.task;
        task.owner = ctx.accounts.user.key();
        task.title = title;
        task.description = description;
        task.completed = false;
        task.created_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// Actualiza el titulo y descripcion de una tarea
    pub fn update_task(ctx: Context<UpdateTask>, title: String, description: String) -> Result<()> {
        let task = &mut ctx.accounts.task;
        task.title = title;
        task.description = description;
        Ok(())
    }

    /// Marca una tarea como completada o no completada
    pub fn toggle_complete(ctx: Context<UpdateTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        task.completed = !task.completed;
        Ok(())
    }

    /// Elimina una tarea (cierra la cuenta y devuelve el SOL al usuario)
    pub fn delete_task(_ctx: Context<DeleteTask>) -> Result<()> {
        Ok(())
    }
}

// --- ESTRUCTURA DE DATOS ---

/// Cuenta PDA que representa una tarea en la blockchain
#[account]
pub struct Task {
    pub owner: Pubkey,       // Dueno de la tarea
    pub title: String,       // Titulo (max 50 caracteres)
    pub description: String, // Descripcion (max 200 caracteres)
    pub completed: bool,     // Estado de la tarea
    pub created_at: i64,     // Timestamp de creacion
}

impl Task {
    // Tamanio en bytes que ocupa la cuenta en la blockchain
    pub const LEN: usize = 8      // discriminator de Anchor
        + 32                       // owner (Pubkey)
        + 4 + 50                   // title (String: 4 bytes longitud + 50 bytes contenido)
        + 4 + 200                  // description
        + 1                        // completed (bool)
        + 8;                       // created_at (i64)
}

// --- CONTEXTOS (definen las cuentas que cada instruccion necesita) ---

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateTask<'info> {
    /// PDA derivada del usuario y el titulo de la tarea
    #[account(
        init,
        payer = user,
        space = Task::LEN,
        seeds = [b"task", user.key().as_ref(), title.as_bytes()],
        bump
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTask<'info> {
    /// Solo el dueno puede modificar su tarea
    #[account(
        mut,
        has_one = owner,
        seeds = [b"task", owner.key().as_ref(), task.title.as_bytes()],
        bump
    )]
    pub task: Account<'info, Task>,

    /// CHECK: Solo se usa como referencia para validar el PDA
    pub owner: AccountInfo<'info>,

    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteTask<'info> {
    /// close = user devuelve el SOL al usuario al cerrar la cuenta
    #[account(
        mut,
        has_one = owner,
        close = user,
        seeds = [b"task", owner.key().as_ref(), task.title.as_bytes()],
        bump
    )]
    pub task: Account<'info, Task>,

    /// CHECK: Solo se usa como referencia para validar el PDA
    pub owner: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
}