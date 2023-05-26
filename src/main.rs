use chrono::Utc;
use clap::{arg, Parser};
use error::AppError;
use ini::Ini;

mod error;
mod sts;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Save credentials to this profile. Print to stdout using shell export syntax if not provided
    #[arg(short, long)]
    profile: Option<String>,

    /// Role arn to assume
    #[arg(short, long)]
    role_arn: String,

    /// Unique role session name. Will automatically add a random string as the suffix
    #[arg(short, long)]
    session_name: String,

    /// Duration of the assumed role in seconds, minimum value is 900
    #[arg(short, long, default_value_t = 3600)]
    duration: i32,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let args = Args::parse();

    let current_time = Utc::now().format("%Y-%m-%d%H%M%SZ");
    let session_name = format!("{}@{}", args.session_name, current_time);

    let home_folder = dirs::home_dir().expect("Failed to get home folder");

    let credentials = sts::assume_role(&args.role_arn, args.duration, &session_name).await?;

    let access_key_id = credentials
        .access_key_id
        .expect("Couldn't find access_key_id in assumed role");
    let secret_access_key = credentials
        .secret_access_key
        .expect("Couldn't find secret_access_key in assumed role");
    let session_token = credentials
        .session_token
        .expect("Couldn't find aws_session_token in assumed role");

    match args.profile {
        Some(profile) => {
            let credentials_file = home_folder.join(".aws/credentials");
            let mut credentials_config = Ini::load_from_file(&credentials_file)?;

            let mut profile_credentials = credentials_config.with_section(Some(&profile));

            profile_credentials
                .set("aws_access_key_id", &access_key_id)
                .set("aws_secret_access_key", &secret_access_key)
                .set("aws_session_token", &session_token);

            credentials_config.write_to_file(&credentials_file)?;
            println!(
                "AWS CLI profile [{}] credentials updated but not used in ENV",
                &profile
            );
            println!(
                "Use --profile {} in aws cli to use this profile, for example:",
                &profile
            );
            println!("aws --profile {} s3 ls", &profile);
        }

        None => {
            println!("export AWS_ACCESS_KEY_ID={}", &access_key_id);
            println!("export AWS_SECRET_ACCESS_KEY={}", &secret_access_key);
            println!("export AWS_SESSION_TOKEN={}", &session_token);
        }
    }

    Ok(())
}
