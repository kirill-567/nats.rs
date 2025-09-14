use async_nats::{AuthError, ConnectOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example demonstrating auth_url_callback functionality
    // This callback would be called when a 401 error is encountered during handshake
    
    let client_result = ConnectOptions::new()
        .auth_url_callback(|()| async move {
            // Simulate getting a new connection string from an auth service
            println!("Authentication error detected! Getting new connection URL from auth service...");
            
            // In a real scenario, this would make a request to your auth service
            // to get a new connection URL with fresh credentials
            // This could handle various auth-related errors:
            // - 401 Unauthorized
            // - Token expired 
            // - IO errors caused by authentication failures
            // - Connection refused due to invalid credentials
            let new_url = "nats://refreshed-server:4222".to_string();
            
            println!("Received new URL: {}", new_url);
            Ok(new_url)
        })
        .connect("nats://demo.nats.io:4222") // This would normally cause auth errors in test scenario
        .await;
    
    match client_result {
        Ok(client) => {
            println!("Successfully connected to NATS server");
            let info = client.server_info();
            println!("Server info: {:?}", info.server_id);
            
            // Test basic functionality
            client.publish("test.subject", "Hello NATS".into()).await?;
            println!("Published test message");
            
            client.flush().await?;
            println!("Flushed connection");
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    Ok(())
}

// Example of an auth_url_callback that might fail
#[allow(dead_code)]
async fn failing_auth_url_callback() -> Result<String, AuthError> {
    // Simulate a scenario where the auth service is temporarily unavailable
    Err(AuthError::new("Auth service temporarily unavailable"))
}

// Example of a more complex auth_url_callback that makes HTTP requests
#[allow(dead_code)]  
async fn http_auth_url_callback() -> Result<String, AuthError> {
    // In a real implementation, you might do something like:
    // let response = reqwest::get("https://your-auth-service.com/new-nats-url").await?;
    // let new_url = response.text().await?;
    // Ok(new_url)
    
    // For this example, just return a mock URL
    Ok("nats://auth-refreshed-server:4222".to_string())
}
