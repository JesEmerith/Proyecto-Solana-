import { PublicKey } from "@solana/web3.js";

////////////////// CONSTANTES ////////////////////
const nombre_tienda = "InovaTec"; // Nombre de la Tienda
const owner = pg.wallet.publicKey;

console.log("My address:", owner.toString());
const balance = await pg.connection.getBalance(owner);
console.log(`My balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);


//////////////////// TIENDA ////////////////////
function pdaTienda(nombre_tienda) {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("tienda"),
      Buffer.from(nombre_tienda),
      owner.toBuffer(),
    ],
    pg.PROGRAM_ID
  );
}
//////////////////// COMPUERTAS ////////////////////
function pdaCompuerta(nombre_serie) {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("componente"),
      Buffer.from(nombre_serie),
      owner.toBuffer(),
    ],
    pg.PROGRAM_ID
  );
}

//////////////////// CREAR TIENDA////////////////////
async function crearTienda(nombre_tienda) {
  const [pda_tienda] = pdaTienda(nombre_tienda);

  const txHash = await pg.program.methods
    .crearTienda(nombre_tienda) 
    .accounts({
      owner: owner,
      tienda: pda_tienda,
    })
    .rpc();

  console.log("txHash: ", txHash);
}

//////////////////// AGREGAR COMPUERTA LOGICA ////////////////////
async function agregarCompuerta(nombre_serie, logica) {

  const [pda_compuert] = pdaCompuerta(nombre_serie);
  const [pda_tienda] = pdaTienda(nombre_tienda);

  const txHash = await pg.program.methods
    .agregarCompuerta(nombre_serie, logica)
    .accounts({
      owner: owner,
      compuert: pda_compuert,
      tienda: pda_tienda,
    })
    .rpc();

  console.log("txHash: ", txHash);
}

//////////////////// ALTERNAR ESTADO ////////////////////
async function cambiarEstado(nombre_serie) {

  const [pda_compuert] = pdaCompuerta(nombre_serie);
  const [pda_tienda] = pdaTienda(nombre_tienda);

  const txHash = await pg.program.methods
    .alternarEstado(nombre_serie)
    .accounts({
      owner: owner,
      compuert: pda_compuert,
      tienda: pda_tienda,
    })
    .rpc();

  console.log("txHash: ", txHash);
}

//////////////////// ELIMINAR COMPUERTA LOGICA ////////////////////
async function eliminarCompuerta(nombre_serie) {

  const [pda_compuert] = pdaCompuerta(nombre_serie); 
  const [pda_tienda] = pdaTienda(nombre_tienda);
  const txHash = await pg.program.methods
    .eliminarCompuerta(nombre_serie)
    .accounts({
      owner: owner,
      compuert: pda_compuert,
      tienda: pda_tienda,
    })
    .rpc();

  console.log("txHash: ", txHash);
}

//////////////////// VER COMPUERTAS LOGICAS ////////////////////
async function verCompuertas(nombre_tienda) {
  const [pda_tienda] = pdaTienda(nombre_tienda);

  try {
    const tiendaAccount = await pg.program.account.tienda.fetch(
      pda_tienda
    );
    const numero_compuertas = tiendaAccount.compuertas.length;

    if (!tiendaAccount.compuertas || numero_compuertas === 0) {
      console.log("La Tienda esta vacía");
      return;
    }

    console.log("Número de Compuertas:", numero_compuertas);

    for (let i = 0; i < numero_compuertas; i++) {
      const compuertKey = tiendaAccount.compuertas[i];

      const compuertAccount = await pg.program.account.compuerta.fetch(compuertKey);

      console.log(
        `Compuerta #${i + 1}: \n * Serie: ${compuertAccount.serie} \n * Logica: ${
          compuertAccount.logica
        } \n * Tienda: ${
          compuertAccount.tienda
        } \n * Disponible: ${
          compuertAccount.disponible
        } \n * Dirección(PDA): ${compuertKey.toBase58()}`
      );
    }
  } catch (error) {
    console.error("Error viendo Compuertas:", error);

    if (error.message) {
      console.error("Mensaje de error:", error.message);
    }
    if (error.logs) {
      console.error("Logs del programa:", error.logs);
    }
  }
}

//crearTienda("InovaTec");
//agregarCompuerta("74LS99", "XOR");
//cambiarEstado("74LS99");
//eliminarCompuerta("74LS99");
//verCompuertas(nombre_tienda);