use aws_sdk_rds as rds;

#[derive(Debug)]
struct MaintainenceWindow {
    resource_identifier: String,
    action: String,
    auto_applied_after_date: String,
}

#[::tokio::main]
async fn main() -> Result<(), rds::Error> {
    let config = ::aws_config::load_from_env().await;
    let client = rds::Client::new(&config);

    let req1 = client.describe_pending_maintenance_actions().send().await?;
    let list_of_maintainenace_window: Vec<MaintainenceWindow> = req1
        .pending_maintenance_actions()
        .unwrap()
        .iter()
        .map(|action| MaintainenceWindow {
            resource_identifier: action.resource_identifier().unwrap().to_owned(),
            action: action
                .pending_maintenance_action_details()
                .unwrap()
                .iter()
                .map(|detail| format!("{:?}", detail.action().unwrap()))
                .collect::<Vec<String>>()
                .join(","),
            auto_applied_after_date: action
                .pending_maintenance_action_details()
                .unwrap()
                .iter()
                .map(|detail| format!("{:?}", detail.auto_applied_after_date().unwrap()))
                .collect::<Vec<String>>()
                .join(","),
        })
        .collect();

    println!("{:#?}", list_of_maintainenace_window);

    Ok(())
}
