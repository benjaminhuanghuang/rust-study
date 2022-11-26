use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
  let processing_fee = 1;
  let cost_per_item = 5;

  // return the error
  let qty = item_quantity.parse::<i32>()?;

  Ok(qty * cost_per_item + processing_fee)
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
  let processing_fee = 1;
  let cost_per_item = 5;
  //let qty = item_quantity.parse::<i32>()?;

  let qty = match item_quantity.parse::<i32>() {
    Ok(x) => x,
    Err(e) => return Err(e),
  };

  Ok(qty * cost_per_item + processing_fee)
}
