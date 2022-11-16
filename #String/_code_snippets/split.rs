fn test() {
  let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

  if cfg!(debug_assertions) {
    eprintln!("debug: {:?} -> {:?}", record, fields);
  }
}
