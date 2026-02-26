use anchor_lang::prelude::*;

declare_id!(""); // No se pone

#[program] // significa que el código va ser utilizado en el programa
pub mod biblioteca{
    use super::*; // Exportar lo que se haga para que sea publico
    
    pub fn crear_biblioteca() -> Result<()> {
        // Código..... 
    }
}

#[account]
#[derive(InitSpace)]
pub struct Biblioteca{
    owner: Pubkey, 

    #[max_lan(68)]
    nombre: String,

    #[max?len(10)]
    libros: Vec<Libro>, 

} 

#[derive(AnchorSeriolize, AnchorDeseriolize, Clene, IntitSpace, PartiolEq, Debug)]
pub struct Libro{
    #[max_len(60)]
    nombre:String,


    paginas: u16,

    disponible> bool,
}


#[derive(Accounts)]
pub struct NuevaBiblioteca{
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        sapce = Biblioteca::INIT_SPACE + 8,
        seeds = [b"Biblioteca", owner.key().as_ref()],
        bump 
    )]
    pub biblioteca: Account<'info, Biblioteca>,
    
    
    pub system_program: Programa<'info, System>,
}

pub struct NuevoLibro{
    pub owner: Singer<'info>,

    #[account(mut)]
    pub biblioteca: Account<'info, Biblioteca>,
}
