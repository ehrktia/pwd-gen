pub mod server {
    include!(concat!(env!("ROOT_DIR"), "/protos/pwdgen.rs"));
}
mod gen;
mod pg;
mod pg_grpc;
use crate::server::pwd_generator_server::PwdGeneratorServer;
use pg_grpc::Grpcserver;
use tonic::transport::Server;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    println!("server listening at:{}", addr);
    // create a new pg client
    let pg_config = pg::create_config();
    let (client, connection) = pg_config.connect(tokio_postgres::NoTls).await?;
    tokio::spawn(async move {
        #[allow(irrefutable_let_patterns)]
        if let e = connection.await {
            eprintln!("connection error:{:?}", e);
        }
    });
    // initialize grpc server
    let grpc_server = Grpcserver::new(client);
    Server::builder()
        .add_service(PwdGeneratorServer::new(grpc_server))
        .serve(addr)
        .await?;
    Ok(())
}
