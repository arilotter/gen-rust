pub mod gen;
use crate::gen::{ExampleService, WebRPCError, GetUserArgs};
#[tokio::main]
async fn main() -> Result<(), WebRPCError> {
    let hostname = "http://localhost:3000".to_string();
    let service = ExampleService::new(hostname);

    let ping_response = service.ping().await?;
    println!("Ping response: {:?}", ping_response);

    let get_user_response = service.get_user(GetUserArgs { user_id: 0 }).await?;
    println!("Get user 0 response: {:?}", get_user_response);
    
    let get_user_response = service.get_user(GetUserArgs { user_id: 1 }).await?;
    println!("Get user 1 response: {:?}", get_user_response);
    Ok(())
}
