use serde_derive::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}


fn get_transactions(fname: &str) -> Result<Vec<Transaction>, String>{
    let s = std::fs::read_to_string(fname).unwrap();
    let t:Vec<Transaction> = serde_json::from_str(&s).unwrap();
    
    Ok(t)
}


fn main(){
    let trans = get_transactions("test_data/transactions.json").expect("xxxx:::");

    for t in trans {
        println!("{:?}", t);
    }
}