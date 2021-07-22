mod error; // code under the same code
pub use error::TransactionError;
use serde_derive::*;
  
#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,   
    to: String, 
    amount: u64,
}

pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, String> {
    let s = std::fs::read_to_string(fname).unwrap();
    let t: Vec<Transaction> = serde_json::from_str(&s).unwrap();

    Ok(t)
}

fn get_transactions_a(fname: &str) -> Result<Vec<Transaction>, String> {
    let s = match std::fs::read_to_string(fname) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    let t: Vec<Transaction> = match serde_json::from_str(&s) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    Ok(t)
}

fn get_transactions_b(fname: &str) -> Result<Vec<Transaction>, String> {
    std::fs::read_to_string(fname)
        .map_err(|e| e.to_string()) // map error to a string
        // only when there is no error
        .and_then(|loaded_result| serde_json::from_str(&loaded_result).map_err(|e| e.to_string()))
}

fn get_transactions_c(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    std::fs::read_to_string(fname)
        .map_err(|e| e.into()) // map error to TransactionError
        // only when there is no error
        .and_then(|loaded_result| serde_json::from_str(&loaded_result).map_err(|e| e.into()))
}

fn get_transactions_d(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    Ok(
        match serde_json::from_str(&match std::fs::read_to_string(fname) {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        }) {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        },
    )
}

// equivalent to get_transactions_d
pub fn get_transactions_e(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    // ? can transfer err to the return type TransactionError
    Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?)
}

pub fn get_first_transaction_for(fname: &str, uname: &str) -> Option<Transaction> {
    let trans = get_transactions_e(fname).ok()?;
    for t in trans {
        if t.from == uname {
            return Some(t);
        }
    }
    None
}