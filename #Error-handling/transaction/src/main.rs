extern crate transaction_lib;

use transaction_lib::*;
use serde_derive::*;


fn main() -> Result<(), TransactionError> {
    let trans = get_transactions_e("test_data/transactions.json")?;

    for t in trans {
        println!("{:?}", t);
    }

    let t = get_first_transaction_for("test_data/transactions.json", "A")
        .ok_or("Could not get first transaction")?;
    println!("A's first transaction: {:?}", t);
    Ok(())
}
