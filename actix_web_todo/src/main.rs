use actix_web::{get, App, HttpResponse, HttpServer, Responder};
// 日付と時刻の操作のためのライブラリ
use chrono::{DateTime, Utc};
// ログ出力のためのライブラリ
use log::info;
// JSONシリアライズのためのライブラリ
use serde::Serialize;
// UUID生成のためのライブラリ
use uuid::Uuid;

// Serializeトレイトを実装することでJSONシリアライズ可能になる
// TaskIdという新しい方を定義し、その中にUuidを格納する
#[derive(Serialize)]
struct TaskId(Uuid);

#[derive(Serialize)]
struct Todo {
    id: TaskId,
    description: String,
    done: bool,
    // <>はジェネリクスで、DateTime<Utc>のように具体的な型を指定する
    datetime: DateTime<Utc>,
}

// Todo型の要素からなる動的配列
#[derive(Serialize)]
struct TodoList(Vec<Todo>);

// impl Responderは、戻り値がResponderトレイトを実装していることを示す。具体的に何の型かは関係ない。
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/todos")]
async fn todos() -> impl Responder {
    let todo_list = TodoList(vec![
        Todo {
            // ランダムなUUIDを生成
            id: TaskId(Uuid::new_v4()),
            // 文字列リテラル($str型)からString型へ変換
            description: "Task1".to_string(),
            done: false,
            datetime: Utc::now(),
        },
        Todo {
            id: TaskId(Uuid::new_v4()),
            description: "Task2".to_string(),
            done: false,
            datetime: Utc::now(),
        },
        Todo {
            id: TaskId(Uuid::new_v4()),
            description: "Task3".to_string(),
            done: false,
            datetime: Utc::now(),
        },
    ]);

    // 200 OKステータスコードと、JSONシリアライズしたtodo_listを返す
    HttpResponse::Ok().json(todo_list)
}

// #[actix_web::main]マクロは、main関数を非同期関数に変換する
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // RUST_LOGはenv_loggerの環境変数で、ログレベルを指定する
    std::env::set_var("RUST_LOG", "INFO");
    // env_logger::init()は、ログ出力の初期化を行う
    env_logger::init();

    let addr = "127.0.0.1:8080";

    info!("Starting server at {}", addr);

    HttpServer::new(|| App::new().service(health).service(todos))
        .bind(addr)?
        .run()
        .await
}
