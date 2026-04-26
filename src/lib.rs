use anchor_lang::prelude::*;

// ID del programa (Generado para Solana Playground)
declare_id!("Bno9cTvX5nVhbiQSpVyyrXhiL3YTbbf22xcjneSZRRE2");

#[program]
pub mod habit_builder_solana {
    use super::*;

    // 1. CREATE (PDA): Inicializa el gestor de hábitos personal
    pub fn inicializar_gestor(ctx: Context<CrearGestor>, usuario: String) -> Result<()> {
        let gestor = &mut ctx.accounts.gestor;
        gestor.owner = ctx.accounts.owner.key();
        gestor.nombre_usuario = usuario;
        gestor.lista_habitos = Vec::new();
        
        msg!("Gestor de hábitos para '{}' inicializado.", gestor.nombre_usuario);
        Ok(())
    }

    // 2. CREATE (Dato): Registra un nuevo hábito con métricas iniciales manuales
    pub fn agregar_habito(
        ctx: Context<GestionarHabito>, 
        nombre: String, 
        racha: u16, 
        minutos: u8
    ) -> Result<()> {
        let gestor = &mut ctx.accounts.gestor;
        require!(gestor.owner == ctx.accounts.owner.key(), Errores::NoAutorizado);

        let nuevo_habito = Habito {
            nombre_habito: nombre,
            racha_dias: racha,
            minutos_invertidos: minutos,
        };

        gestor.lista_habitos.push(nuevo_habito);
        msg!("Hábito registrado en la blockchain.");
        Ok(())
    }

    // 3. UPDATE: Modifica el progreso (Búsqueda por nombre de hábito)
    pub fn actualizar_progreso(
        ctx: Context<GestionarHabito>, 
        nombre_buscado: String, 
        nueva_racha: u16, 
        nuevos_minutos: u8
    ) -> Result<()> {
        let gestor = &mut ctx.accounts.gestor;
        require!(gestor.owner == ctx.accounts.owner.key(), Errores::NoAutorizado);

        let lista = &mut gestor.lista_habitos;
        for i in 0..lista.len() {
            if lista[i].nombre_habito == nombre_buscado {
                lista[i].racha_dias = nueva_racha;
                lista[i].minutos_invertidos = nuevos_minutos;
                msg!("Progreso de '{}' actualizado exitosamente.", nombre_buscado);
                return Ok(());
            }
        }
        Err(Errores::HabitoNoEncontrado.into())
    }

    // 4. DELETE: Elimina el hábito de la lista
    pub fn eliminar_habito(ctx: Context<GestionarHabito>, nombre_buscado: String) -> Result<()> {
        let gestor = &mut ctx.accounts.gestor;
        require!(gestor.owner == ctx.accounts.owner.key(), Errores::NoAutorizado);

        let lista = &mut gestor.lista_habitos;
        let index = lista.iter().position(|h| h.nombre_habito == nombre_buscado);

        if let Some(i) = index {
            lista.remove(i);
            msg!("Hábito '{}' eliminado del registro.", nombre_buscado);
            Ok(())
        } else {
            Err(Errores::HabitoNoEncontrado.into())
        }
    }

    // 5. READ: Consulta todos los hábitos y su rendimiento
    pub fn consultar_habitos(ctx: Context<GestionarHabito>) -> Result<()> {
        msg!("Usuario: {}", ctx.accounts.gestor.nombre_usuario);
        msg!("Lista de Hábitos: {:#?}", ctx.accounts.gestor.lista_habitos);
        Ok(())
    }
}

// --- ESTADO DEL PROGRAMA ---

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Habito {
    #[max_len(40)]
    pub nombre_habito: String,
    pub racha_dias: u16,
    pub minutos_invertidos: u8,
}

#[account]
#[derive(InitSpace)]
pub struct GestorHabitos {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_usuario: String,
    #[max_len(15)] // Capacidad para 15 hábitos activos
    pub lista_habitos: Vec<Habito>,
}

// --- CONTEXTOS ---

#[derive(Accounts)]
pub struct CrearGestor<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + GestorHabitos::INIT_SPACE,
        seeds = [b"habitos", owner.key().as_ref()],
        bump
    )]
    pub gestor: Account<'info, GestorHabitos>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarHabito<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub gestor: Account<'info, GestorHabitos>,
}

// --- ERRORES ---

#[error_code]
pub enum Errores {
    #[msg("No tienes permisos para modificar este gestor.")]
    NoAutorizado,
    #[msg("El hábito solicitado no existe en tu lista.")]
    HabitoNoEncontrado,
}
