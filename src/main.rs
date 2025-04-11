use bdk::{
    bitcoin::Network, blockchain::esplora::EsploraBlockchain, database::MemoryDatabase,
    wallet::AddressIndex, SignOptions, Wallet,
};

const SEND_AMOUNT: u64 = 50000;
const STOP_GAP: usize = 5;

const NETWORK: Network = Network::Regtest;
const EXTERNAL_DESC: &str = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/1'/0'/0/*)";
const INTERNAL_DESC: &str = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/1'/0'/1/*)";
const ESPLORA_URL: &str = "http://localhost:3002"; // run esplora on localhost:3002

fn main() -> Result<(), anyhow::Error> {
    let db = MemoryDatabase::new();
    let wallet = Wallet::new(EXTERNAL_DESC, Some(INTERNAL_DESC), NETWORK, db)?;

    let address = wallet.get_address(AddressIndex::New)?;
    println!(
        "Next unused address: ({}) {}",
        address.index, address.address
    );

    let balance = wallet.get_balance()?;
    println!("Wallet balance before syncing: {}", balance.get_total());

    print!("Syncing...");
    let client = EsploraBlockchain::new(ESPLORA_URL, STOP_GAP);

    wallet.sync(&client, Default::default())?;

    let balance = wallet.get_balance()?;
    println!("Wallet balance after syncing: {}", balance.get_total());

    if balance.get_total() < SEND_AMOUNT {
        println!(
            "Please send at least {} to the receiving address",
            SEND_AMOUNT
        );
        std::process::exit(0);
    }

    let mut tx_builder = wallet.build_tx();
    tx_builder.add_recipient(address.script_pubkey(), SEND_AMOUNT);

    let (mut psbt, _details) = tx_builder.finish()?;
    let finalise = wallet.sign(&mut psbt, SignOptions::default())?;
    assert!(finalise, "Transaction is finalised");

    let tx = psbt.extract_tx();

    client.broadcast(&tx)?;
    println!("Transaction broadcasted: {}", tx.txid());

    Ok(())
}
