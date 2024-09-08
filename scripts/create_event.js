const anchor = require('@project-serum/anchor');
const { SystemProgram, Keypair } = require('@solana/web3.js');
const fs = require('fs');
// Set up the provider
const wallet = new anchor.Wallet(
  Keypair.fromSecretKey(
    Buffer.from(JSON.parse(fs.readFileSync('/home/os/my-wallet.json', 'utf8')))
  )
);

const connection = new anchor.web3.Connection("http://127.0.0.1:8899", "processed");


const provider = new anchor.AnchorProvider(connection, wallet, anchor.AnchorProvider.defaultOptions());
anchor.setProvider(provider);

// Load your program
const programId = new anchor.web3.PublicKey('GqtdriywSmas3dMgjoVYgk8DDoYyRFXtdg4edx7iSK2j'); // Update with your program's ID
const idl = JSON.parse(fs.readFileSync('/home/os/eventticketingsystem/target/idl/eventticketingsystem.json', 'utf8')); // Update with the path to your program's IDL
const program = new anchor.Program(idl, programId, provider);
async function createEvent() {
  // Generate a new keypair for the event account
  const eventAccount = Keypair.generate();

  // Define event parameters
  const eventParams = {
    event_name: "Concert",
    event_date: Math.floor(Date.now() / 1000),
    ticket_price: 1000, // 1 SOL
    total_tickets: 100,
  };

  try {
    // Create the event
    const tx = await program.methods.createEvent(
      eventParams.event_name,
      eventParams.event_date,
      eventParams.ticket_price,
      eventParams.total_tickets
    ).accounts({
      event: eventAccount.publicKey,
      creator: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    }).signers([eventAccount]).rpc();

    console.log("Event created with transaction:", tx);
    console.log("Event account public key:", eventAccount.publicKey.toBase58());
  } catch (error) {
    console.error("Error creating event:", error);
  }
}

createEvent().catch(err => console.error(err));
