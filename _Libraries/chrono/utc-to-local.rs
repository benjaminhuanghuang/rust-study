use chrono::{serde::ts_seconds, DateTime, Local, Utc};


let created_at: DateTime<Utc> = Utc::now();
let created_at = created_at.with_timezone(&Local).format("%F %H:%M");

println!("{}", created_at)