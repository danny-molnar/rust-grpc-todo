//! The server implementation for the Todo List gRPC service.

use tonic::{transport::Server, Request, Response, Status};
use todo::todo_service_server::{TodoService, TodoServiceServer};
use todo::{
    CreateTodoRequest, CreateTodoResponse, GetTodoRequest, GetTodoResponse, ListTodosRequest,
    ListTodosResponse, DeleteTodoRequest, DeleteTodoResponse, Todo
};

/// Generated gRPC code
pub mod todo {
    tonic::include_proto!("todo");
}

/// A struct representing the Todo service.
#[derive(Debug, Default)]
pub struct MyTodoService {
    /// A mutex-protected vector of Todo items.
    todos: std::sync::Mutex<Vec<Todo>>,
}

#[tonic::async_trait]
impl TodoService for MyTodoService {
    /// Creates a new Todo item.
    async fn create_todo(
        &self,
        request: Request<CreateTodoRequest>,
    ) -> Result<Response<CreateTodoResponse>, Status> {
        let req = request.into_inner();
        let mut todos = self.todos.lock().unwrap();
        let id = todos.len() as i32 + 1;
        let todo = Todo {
            id,
            title: req.title,
            description: req.description,
            completed: false,
        };
        todos.push(todo.clone());
        Ok(Response::new(CreateTodoResponse { todo: Some(todo) }))
    }

    /// Gets a Todo item by ID.
    async fn get_todo(
        &self,
        request: Request<GetTodoRequest>,
    ) -> Result<Response<GetTodoResponse>, Status> {
        let req = request.into_inner();
        let todos = self.todos.lock().unwrap();
        if let Some(todo) = todos.iter().find(|t| t.id == req.id) {
            Ok(Response::new(GetTodoResponse { todo: Some(todo.clone()) }))
        } else {
            Err(Status::not_found("Todo not found"))
        }
    }

    /// Lists all Todo items.
    async fn list_todos(
        &self,
        _request: Request<ListTodosRequest>,
    ) -> Result<Response<ListTodosResponse>, Status> {
        let todos = self.todos.lock().unwrap();
        Ok(Response::new(ListTodosResponse { todos: todos.clone() }))
    }

    /// Deletes a Todo item by ID.
    async fn delete_todo(
        &self,
        request: Request<DeleteTodoRequest>,
    ) -> Result<Response<DeleteTodoResponse>, Status> {
        let req = request.into_inner();
        let mut todos = self.todos.lock().unwrap();
        if let Some(pos) = todos.iter().position(|t| t.id == req.id) {
            todos.remove(pos);
            Ok(Response::new(DeleteTodoResponse {}))
        } else {
            Err(Status::not_found("Todo not found"))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let todo_service = MyTodoService::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(TodoServiceServer::new(todo_service))
        .serve(addr)
        .await?;

    Ok(())
}
