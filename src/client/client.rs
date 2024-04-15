pub mod client {
    include!(concat!(env!("ROOT_DIR"), "/protos/pwdgen.rs"));
}
use crate::client::pwd_generator_client::PwdGeneratorClient;
use crate::client::PwdRequest;
use clap::Parser;
#[allow(dead_code)]
#[derive(Parser)]
#[command(version,about,long_about=None)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[arg(short, long, value_name = "USER_NAME")]
    username: Option<String>,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_args = Cli::parse();
    let addr: String = "http://[::1]:50051".parse().unwrap();
    println!("client connecting to:{}", addr.to_string());
    // connect to server
    let mut grpc_client = PwdGeneratorClient::connect(addr).await?;
    let name = cli_args.username.unwrap();
    // build request from cli args
    let request = PwdRequest { user: name };
    // submit request
    let response = grpc_client.update_pwd_validity(request).await?;
    let response_data = response.into_inner();
    println!("user:{:?}", response_data.user);
    println!("pwd:{:?}", response_data.pwd);
    println!("expire_at:{:?}", response_data.expiry_at);

    Ok(())
}
