use crate::error::AppError;
use aws_sdk_sts::{model::Credentials, Client as stsClient};

pub async fn assume_role(
    role_arn: &String,
    duration: i32,
    session_name: &String,
) -> Result<Credentials, AppError> {
    let sts_config = aws_config::from_env().load().await;
    let sts_client: stsClient = stsClient::new(&sts_config);

    let assumed_role = sts_client
        .assume_role()
        .role_arn(role_arn)
        .role_session_name(session_name)
        .duration_seconds(duration)
        .send()
        .await?;

    Ok(assumed_role
        .credentials
        .expect("Couldn't find credentials from assumed role"))
}
