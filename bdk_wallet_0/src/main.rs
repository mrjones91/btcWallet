use bdk::bitcoin::Network;
use bdk::database::MemoryDatabase;
use bdk::Wallet;
use bdk::blockchain::EsploraBlockchain;
use bdk::SyncOptions;

fn main() {
    println!("Hello, world!");

    let external_descriptor = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/1'/0'/0/*)";
    let internal_descriptor = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/1'/0'/1/*)";

    let wallet = Wallet::new(
        external_descriptor,
        Some(internal_descriptor),
        Network::Testnet,
        MemoryDatabase::default(),
    ).unwrap();

    let balance_before_sync = wallet.get_balance().unwrap();

    println!("{}", balance_before_sync);

    let client = EsploraBlockchain::new(
        "https://mempool.space/testnet/api", 
        20
    );

    wallet.sync(&client, SyncOptions::default()).unwrap();

    let balance_after_sync = wallet.get_balance().unwrap();

    println!("{}", balance_after_sync);
}
