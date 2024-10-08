use crate::db::DbPool;
use crate::services::todo_service;

use actix_web::{web, HttpResponse};

pub async fn get_todos(pool: web::Data<DbPool>) -> HttpResponse {
    match todo_service::get_all_todos(pool).await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn add_todo(pool: web::Data<DbPool>) -> HttpResponse {
    todo!()
}

pub async fn update_todo(pool: web::Data<DbPool>) -> HttpResponse {
    todo!()
}

pub async fn delete_todo(pool: web::Data<DbPool>) -> HttpResponse {
    todo!()
}

// use actix_web::{web, HttpResponse, Responder};
// use chrono::Utc;
// use uuid::Uuid;

// use super::AppState;
// use crate::models::{CreateTodoItem, TodoItem, UpdateTodoItem};

// pub async fn get_todos(data: web::Data<AppState>) -> impl Responder {
//     let todos = data.todo_list.lock().unwrap();
//     HttpResponse::Ok().json(&*todos)
// }

// pub async fn add_todo(
//     item: web::Json<CreateTodoItem>,
//     data: web::Data<AppState>,
//     // pool: web::Data<db::PgPool>,
// ) -> impl Responder {
//     let mut todos = data.todo_list.lock().unwrap();

//     let new_todo = TodoItem {
//         id: Uuid::new_v4(),
//         title: item.title.clone(),
//         completed: item.completed.clone(),
//         create_at: Utc::now(),
//     };

//     todos.push(new_todo);

//     HttpResponse::Ok().json(&*todos)
// }

// pub async fn update_todo(
//     path: web::Path<Uuid>,
//     item: web::Json<UpdateTodoItem>,
//     data: web::Data<AppState>,
// ) -> impl Responder {
//     let mut todos = data.todo_list.lock().unwrap();

//     if let Some(todo) = todos.iter_mut().find(|todo| todo.id == *path) {
//         if let Some(title) = &item.title {
//             todo.title = title.clone();
//         }

//         if let Some(completed) = item.completed {
//             todo.completed = completed;
//         }

//         HttpResponse::Ok().json(&*todos)
//     } else {
//         HttpResponse::NotFound().body("Todo not found")
//     }
// }

// pub async fn delete_todo(path: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
//     let mut todos = data.todo_list.lock().unwrap();

//     if let Some(index) = todos.iter().position(|todo| todo.id == *path) {
//         todos.remove(index);

//         HttpResponse::Ok().json(&*todos)
//     } else {
//         HttpResponse::NotFound().body("Todo not found")
//     }
// }
