use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
  let hc = httpc_test::new_client("http://localhost:8964")?;

  hc.do_get("/hello").await?.print().await?;

  Ok(())
}
