mod redis;
mod jwt_impl;

use ::clap::{Parser};
use log::{debug,info};
use tonic::{transport::Server, Request, Response, Status};

use jwt::jwt_service_server::{JwtService, JwtServiceServer};
use jwt::{ExchangeTokenRequest, ExchangeTokenResponse, GetTokenInfoRequest, GetTokenInfoResponse, RevokeTokenRequest, RevokeTokenResponse};

pub mod jwt {
    tonic::include_proto!("jwt");
}

#[derive(Debug, Default)]
pub struct JWT {}

#[tonic::async_trait]
impl JwtService for JWT {
    async fn exchange_token(
        &self,
        request: Request<ExchangeTokenRequest>,
    ) -> Result<Response<ExchangeTokenResponse>, Status> {
        debug!("[SERVER]Got a ExchangeTokenRequest: {:?}", request);
        let rq = request.into_inner();
        let access_token = jwt_impl::issue_jwt_token(rq.email.as_str(), rq.password.as_str());
        let response = ExchangeTokenResponse {
           token: access_token,
        };
        Ok(Response::new(response))
    }

    async fn get_token_info(
        &self,
        request: Request<GetTokenInfoRequest>,
    ) -> Result<Response<GetTokenInfoResponse>, Status> {
        debug!("[SERVER]Got a GetTokenInfoRequest: {:?}", request);
        let result = jwt_impl::get_info_from_token(request.into_inner().token.as_str());
        // get email from Result<Claims>
        let claims = match result {
            Ok(result) => result,
            Err(_) => return Err(Status::invalid_argument("Get token info error")),
        };
        let response = GetTokenInfoResponse {
            sub: claims.sub,
            iat: unsigned_to_signed(claims.iat),
            exp: unsigned_to_signed(claims.exp),
            email: claims.email,
            iss: claims.iss,
        };
        Ok(Response::new(response))
    }

    async fn revoke_token(
        &self,
        request: Request<RevokeTokenRequest>,
    ) -> Result<Response<RevokeTokenResponse>, Status> {
        debug!("[SERVER]Got a GetTokenInfoRequest: {:?}", request);
        let bool = jwt_impl::revoke_token(request.into_inner().token.as_str());
        let response = RevokeTokenResponse {
            success: bool,
        };
        Ok(Response::new(response))
    }
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "jwt-server - a simple echo microservice", long_about = None)]
struct ServerCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "50052")]
    port: u16,
}

fn unsigned_to_signed(u: u64) -> i64 {
    u as i64
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    pretty_env_logger::init();
    // Initialize Redis
    let redis = redis::RedisInstance::new();
    let cli = ServerCli::parse();
    let addr = format!("{}:{}", cli.server, cli.port).parse()?;
    let jwt = JWT::default();

    info!("[SERVER]Server listening on {}", addr);

    Server::builder()
        .add_service(JwtServiceServer::new(jwt))
        .serve(addr)
        .await?;

    Ok(())
}

