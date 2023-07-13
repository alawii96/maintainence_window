use aws_sdk_rds as rds;

#[::tokio::main]
async fn main() -> Result<(), rds::Error> {
    let config = ::aws_config::load_from_env().await;
    let client = rds::Client::new(&config);

    let req1 = client.describe_pending_maintenance_actions().send().await?;

    req1.pending_maintenance_actions()
        .unwrap()
        .iter()
        .for_each(|action| {
            println!(
                "Action: {:?}:{:?}",
                action.resource_identifier().unwrap().to_owned(),
                action
                    .pending_maintenance_action_details()
                    .unwrap()
                    .iter()
                    .map(|detail| {
                        format!(
                            "{:?}:{:?}\n",
                            detail.action().unwrap(),
                            detail.auto_applied_after_date().unwrap()
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(",")
            );
        });
    println!("{:#?}", req1.pending_maintenance_actions());

    Ok(())
}
