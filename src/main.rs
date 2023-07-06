use aws_sdk_rds as rds;

#[::tokio::main]
async fn main() -> Result<(), rds::Error> {
    let config = ::aws_config::load_from_env().await;
    let client = rds::Client::new(&config);

    let req1 = client.describe_pending_maintenance_actions().send().await?;

    println!("{:#?}", req1.pending_maintenance_actions());

    Ok(())
}
