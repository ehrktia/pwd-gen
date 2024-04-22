use std::collections::HashMap;

use crate::gen;
use crate::pg;
use crate::server::pwd_generator_server::PwdGenerator;
use crate::server::{PwdRequest, PwdResponse};
use chrono::NaiveDateTime;
use tokio_postgres::Client;
use tonic::Code;
use tonic::{Request, Response, Result, Status};

pub struct Grpcserver {
    db_client: tokio_postgres::Client,
}
impl Grpcserver {
    pub fn new(client: Client) -> Grpcserver {
        Grpcserver { db_client: client }
    }
}

#[tonic::async_trait]
impl PwdGenerator for Grpcserver {
    async fn update_pwd_validity(
        &self,
        request: Request<PwdRequest>,
    ) -> Result<Response<PwdResponse>, Status> {
        println!(
            "Got a request from {:?}",
            request.remote_addr().take().unwrap()
        );
        let request_message: PwdRequest = request.into_inner();
        let user = request_message.user;
        // skip admin user
        if filter_user(&user) {
            return Err(Status::new(
                Code::Aborted,
                format!("action not allowed user:{} is admin user", user),
            ));
        }
        // check user
        let row = pg::validate_user(&self.db_client, &user).await;
        match row {
            Ok(r) => {
                if r.len() > 0 {
                    println!("user:{} found", user);
                }
            }
            Err(e) => {
                if e.as_db_error().is_some() {
                    return Err(Status::new(
                        Code::Internal,
                        format!("internal error:{}", e.to_string()),
                    ));
                } else {
                    return Err(Status::new(
                        Code::NotFound,
                        format!("data error:{}", e.to_string()),
                    ));
                }
            }
        }
        // generate pwd
        let pwd: String = match gen::generate() {
            Ok(s) => s,
            Err(e) => return Err(Status::new(Code::Internal, e.to_string())),
        };
        // generate new expiry time
        let expire_at: NaiveDateTime = match gen::utc_expiry_time() {
            Some(t) => t,
            None => return Err(Status::new(Code::Internal, "error generating validity")),
        };
        // update user validity
        match pg::update_user_validatity(&self.db_client, &user, &pwd, &expire_at.to_string()).await
        {
            Ok(_) => Ok(Response::new(PwdResponse {
                user: user.to_owned(),
                pwd: pwd,
                expiry_at: expire_at.to_string(),
            })),
            Err(e) => Err(Status::new(
                Code::Internal,
                format!("error:{} updating validity for user:{}", e, &user),
            )),
        }
    }
}

// filter_user checks if incoming user
// is super/admin user
fn filter_user(user: &str) -> bool {
    let mut user_list = HashMap::new();
    user_list.insert("postgres".to_string(), true);
    user_list.get(user).is_some()
}
