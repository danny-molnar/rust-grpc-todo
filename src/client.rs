//! The client implementation for the Todo List gRPC service.

use todo::todo_service_client::TodoServiceClient;
use todo::{CreateTodoRequest, GetTodoRequest, ListTodosRequest, DeleteTodoRequest};

/// Generated gRPC code
pub mod todo {
    tonic::include_proto!("todo");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TodoServiceClient::connect("http://[::1]:50051").await?;

    // Create a new Todo
    let request = tonic::Request::new(CreateTodoRequest {
        title: "Learn Rust".into(),
        description: "Study Rust programming language".into(),
    });

    let response = client.create_todo(request).await?;
    println!("CreateTodoResponse={:?}", response);

    // Get a Todo by ID
    let request = tonic::Request::new(GetTodoRequest { id: 1 });
    let response = client.get_todo(request).await?;
    println!("GetTodoResponse={:?}", response);

    // List all Todos
    let request = tonic::Request::new(ListTodosRequest {});
    let response = client.list_todos(request).await?;
    println!("ListTodosResponse={:?}", response);

    // Delete a Todo by ID
    let request = tonic::Request::new(DeleteTodoRequest { id: 1 });
    let response = client.delete_todo(request).await?;
    println!("DeleteTodoResponse={:?}", response);

    Ok(())
}
