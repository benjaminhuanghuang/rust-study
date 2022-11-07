async fn main() {
  let args: Args = Args::parse();

  match args.action {
    Action::Run(arg:: RunArgs) => run(args).await?,
    _ => panic("Not implemented")
  }

  Ok(())
}



async fn run(args: RunArgs) ->Result<()> {
  let config_file: String =args.config.unwrap_or_else(|| "./xdiff.yml".to_string());
  let config = DiffConfig::load_yaml(&config_file).await?;
  let profile = config.get_profile(&args.profile).ok_or_else(||{
    anyhow::anyhow!(
      "Profile {} not found in config file {}",
      args.profile,
      config_file
    )
  })
}