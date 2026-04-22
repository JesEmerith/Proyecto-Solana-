use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod tienda {
    use super::*;

    /////////////////////////// CREAR TIENDA ///////////////////////////
    pub fn crear_tienda(context: Context<NuevaTienda>, nombre_tienda: String) -> Result<()> {

        let owner_id = context.accounts.owner.key();

        let compuertas = Vec::<Pubkey>::new();

        context.accounts.tienda.set_inner(Tienda { 
            owner: owner_id,
            nombre_tienda: nombre_tienda.clone(),
            compuertas,
        }); 

        Ok(())
    }

    /////////////////////////// NUEVA COMPUERTA LOGICA ///////////////////////////
    pub fn agregar_compuerta(context: Context<NuevaCompuerta>, serie: String, logica: String) -> Result<()> {
        
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElPropietario
        );

        let compuert = Compuerta {
            tienda: context.accounts.tienda.nombre_tienda.clone(),
            serie: serie.clone(),
            logica,
            disponible: true,
        };

        context.accounts.compuert.set_inner(compuert);

        context
            .accounts
            .tienda
            .compuertas
            .push(context.accounts.compuert.key());
    
        Ok(())
    }

    /////////////////////////// VER COMPUERTAS ///////////////////////////
    pub fn ver_compuertas(context: Context<NuevaCompuerta>) -> Result<()> {
        msg!("Lista de compuertas: {:#?}",context.accounts.tienda.compuertas);

        Ok(())
    }

    /////////////////////////// ACTUALIZAR DISPONIBILIDAD ///////////////////////////
    pub fn alternar_estado(context: Context<ModificarCompuerta>, serie: String) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElPropietario
        );

        let compuert = &mut context.accounts.compuert;
        let estado = compuert.disponible;
        let nuevo_estado = !estado;
        compuert.disponible = nuevo_estado;
        
        msg!(
            "La compuerta: {} ahora tiene un valor de disponibilidad: {}",
            serie,
            nuevo_estado
        );

        Ok(())
    }

    /////////////////////////// ELIMINAR COMPUERTA LOGICA ///////////////////////////
    pub fn eliminar_compuerta(context: Context<EliminarCompuerta>, serie: String) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElPropietario
        );

        let tienda = &mut context.accounts.tienda;
        let compuertas = &tienda.compuertas;

        require!(
            context.accounts.compuert.tienda == tienda.nombre_tienda,
            Errores::NoPertenece
        );

        require!(tienda.compuertas.contains(&context.accounts.compuert.key()), Errores::NoExiste);

        let mut pos = 0;

        for i in 0..compuertas.len() {
            if compuertas[i] == context.accounts.compuert.key() {
                pos = i;
                break
            }
        }

        tienda.compuertas.remove(pos);

        Ok(())
    }
}
/////////////////////////// CODIGOS DE ERROR ///////////////////////////
#[error_code]
pub enum Errores {
    #[msg("Acceso Denegado: No eres el propietario de la tienda")]
    NoEresElPropietario,
    #[msg("Error: El número de serie de la compuerta no existe en la tienda")]
    NoExiste,
    #[msg("Error: El número de serie de la compuerta no pertenece a esta tienda")]
    NoPertenece,
}


/////////////////////////// CUENTAS ///////////////////////////

/////////////////////////// TIENDA ///////////////////////////
#[account]
#[derive(InitSpace)]
pub struct Tienda {
    pub owner: Pubkey,

    #[max_len(60)]
    pub nombre_tienda: String,

    #[max_len(10)]
    pub compuertas: Vec<Pubkey>,
}

/////////////////////////// COMPUERTAS LOGICAS ///////////////////////////
#[account]
#[derive(InitSpace, PartialEq, Debug)]
pub struct Compuerta {
    #[max_len(60)]
    pub tienda: String, //// nombre de la tienda o local

    #[max_len(60)]
    pub serie: String, //// 74LS00, 74LS02, 74LS04, 74LS08, 74LS32
    #[max_len(6)]
    pub logica: String, //// AND, OR, NOT, NAND, XOR 

    pub disponible: bool,
}


/////////////////////////// CONTEXTOS ///////////////////////////

/// Instruccion: Crear Tienda
#[derive(Accounts)]
#[instruction(nombre_tienda:String)]
pub struct NuevaTienda<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner, 
        space = 8 + Tienda::INIT_SPACE, 
        seeds = [b"tienda", nombre_tienda.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, Tienda>,

    pub system_program: Program<'info, System>,
}

/// Instruccion: Agregar Compuerta logica
#[derive(Accounts)]
#[instruction(nombre:String)]
pub struct NuevaCompuerta<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner, 
        space = 8 + Compuerta::INIT_SPACE,
        seeds = [b"componente", nombre.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub compuert: Account<'info, Compuerta>,

    #[account(mut)]
    pub tienda: Account<'info, Tienda>,

    pub system_program: Program<'info, System>,
}


/// Instruccion: Modificar Datos de la Compuerta
#[derive(Accounts)]
pub struct ModificarCompuerta<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub compuert: Account<'info, Compuerta>,

    #[account(mut)]
    pub tienda: Account<'info, Tienda>,
}

///  Instruccion: Eliminar Compuerta Logica
#[derive(Accounts)]
pub struct EliminarCompuerta<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        close = owner,
        constraint = compuert.tienda == tienda.nombre_tienda @ Errores::NoPertenece
    )]
    pub compuert: Account<'info, Compuerta>,

    #[account(mut)]
    pub tienda: Account<'info, Tienda>,
}