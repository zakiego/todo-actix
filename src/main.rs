use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer};

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Serialize)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

#[derive(Serialize)]
enum ResponseStatus {
    Success,
    Error,
}

#[derive(Serialize)]
#[serde(untagged)]
enum ResponseData {
    Single(Todo),
    Multiple(Vec<Todo>),
}

#[derive(Serialize)]
struct ResponseSuccess {
    status: ResponseStatus,
    data: ResponseData,
}

#[derive(Serialize)]
struct ResponseError {
    status: ResponseStatus,
    message: String,
}

#[derive(Deserialize)]
struct TodoUpdate {
    title: Option<String>,
    completed: Option<bool>,
}

#[derive(Deserialize)]
struct TodoCreate {
    title: Option<String>,
    completed: Option<bool>,
}

fn get_dummy_data() -> Vec<Todo> {
    let data: [Todo; 2] = [
        Todo {
            id: 1,
            title: String::from("Belajar Rust"),
            completed: false,
        },
        Todo {
            id: 2,
            title: "Belajar Rust lagi".to_string(),
            completed: false,
        },
    ];

    data.to_vec()
}

#[get("/todos")]
async fn get_all_todos() -> Result<HttpResponse, Error> {
    let data = get_dummy_data();

    Ok(HttpResponse::Ok().json(ResponseSuccess {
        status: ResponseStatus::Success,
        data: ResponseData::Multiple(data.to_vec()),
    }))
}

#[get("/todo/{id}")]
async fn get_todo_by_id(path: web::Path<u64>) -> HttpResponse {
    let data: Vec<Todo> = get_dummy_data();

    let data: Option<&Todo> = data.iter().find(|i| i.id == *path);

    if data.is_none() {
        return HttpResponse::NotFound().json(ResponseError {
            status: ResponseStatus::Error,
            message: "Data not found".to_string(),
        });
    };

    HttpResponse::Ok().json(ResponseSuccess {
        status: ResponseStatus::Success,
        data: ResponseData::Single(data.unwrap().clone()),
    })
}

#[post("/todo/update/{id}")]
async fn update_todo_by_id(path: web::Path<u64>, payload: web::Json<TodoUpdate>) -> HttpResponse {
    let data: Vec<Todo> = get_dummy_data();
    let find_data: Option<&Todo> = data.iter().find(|i| i.id == *path);

    if find_data.is_none() {
        return HttpResponse::NotFound().json(ResponseError {
            status: ResponseStatus::Error,
            message: "Data not found".to_string(),
        });
    };

    let mut find_data = find_data.unwrap().clone();

    if let Some(title) = &payload.title {
        find_data.title = title.to_string();
    }

    if let Some(completed) = &payload.completed {
        find_data.completed = *completed;
    }

    HttpResponse::Ok().json(ResponseSuccess {
        status: ResponseStatus::Success,
        data: ResponseData::Single(find_data.clone()),
    })
}

#[post("/todo/create")]
async fn create_todo(payload: web::Json<TodoCreate>) -> HttpResponse {
    let data = get_dummy_data();
    let latest_id = data.last().unwrap().id + 1;

    let new_todo = Todo {
        id: latest_id,
        title: payload.title.clone().unwrap(),
        completed: payload.completed.clone().unwrap(),
    };

    HttpResponse::Ok().json(ResponseSuccess {
        status: ResponseStatus::Success,
        data: ResponseData::Single(new_todo),
    })
}

#[get("/todo/delete/{id}")]
async fn delete_todo_by_id(path: web::Path<u64>) -> HttpResponse {
    let data: Vec<Todo> = get_dummy_data();
    let find_data: Option<&Todo> = data.iter().find(|i| i.id == *path);

    if find_data.is_none() {
        return HttpResponse::NotFound().json(ResponseError {
            status: ResponseStatus::Error,
            message: "Data not found".to_string(),
        });
    };

    HttpResponse::Ok().json(ResponseSuccess {
        status: ResponseStatus::Success,
        data: ResponseData::Single(find_data.unwrap().clone()),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_all_todos)
            .service(get_todo_by_id)
            .service(update_todo_by_id)
            .service(create_todo)
            .service(delete_todo_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
