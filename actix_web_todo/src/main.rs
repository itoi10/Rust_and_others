use actix_web::web::{Data, Json};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
// 日付と時刻の操作のためのライブラリ
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
// ログ出力のためのライブラリ
use log::info;
// JSONシリアライズのためのライブラリ
use serde::{Deserialize, Serialize};
// UUID生成のためのライブラリ
use derive_new::new;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Row};
use uuid::Uuid;

const DATETIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

// テーブル作成
// $ sqlite3 db.sqlite3 "CREATE TABLE IF NOT EXISTS todo (id TEXT PRIMARY KEY, description TEXT, done INTEGER, datetime TEXT);"

// Serializeトレイトを実装することでJSONシリアライズ可能になる
// TaskIdという新しい方を定義し、その中にUuidを格納する
#[derive(Serialize)]
struct TaskId(Uuid);

#[derive(Serialize, new)]
struct Todo {
    id: TaskId,
    description: String,
    done: bool,
    // <>はジェネリクスで、DateTime<Utc>のように具体的な型を指定する
    datetime: DateTime<Utc>,
}

// FromトレイトはRust標準のトレイトで、ある型から別の型への変換を定義する
// From<&Row<'stmt>>はRow型からTodo型への変換を定義している
// 'stmtはライフタイムで、Row型の参照が有効な期間を表す
impl<'stmt> From<&Row<'stmt>> for Todo {
    // 変換元の型を受け取り、変換先の型(Selfはトレイトを実装しているTodo型)を返す
    // またfromが実装されていると、intoメソッドが自動的に実装される
    fn from(row: &Row) -> Self {
        let uuid: String = row.get_unwrap(0);
        let datetime: String = row.get_unwrap(3);

        Todo::new(
            TaskId(Uuid::parse_str(uuid.as_str()).unwrap()),
            row.get_unwrap(1),
            matches!(row.get_unwrap(2), 1),
            Utc.from_local_datetime(
                &NaiveDateTime::parse_from_str(datetime.as_str(), DATETIME_FORMAT).unwrap(),
            )
            .unwrap(),
        )
    }
}

// Vec<Todo>はTodo型の要素からなる動的配列
#[derive(Serialize)]
struct TodoList(Vec<Todo>);

// DeserializeはJSON形式データを構造体に変換するためのトレイト
#[derive(Deserialize)]
struct RegisterTodo {
    description: String,
}

// impl Responderは、戻り値がResponderトレイトを実装していることを示す。具体的に何の型かは関係ない。
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/todo")]
async fn todos(db: Data<Pool<SqliteConnectionManager>>) -> impl Responder {
    let conn = db.get().unwrap();

    let mut stmt = conn
        .prepare("SELECT id, description, done, datetime FROM todo")
        .unwrap();

    // SQLの結果をfromメソッドでTodo型に変換し、Vec<Todo>に格納する
    let results: Vec<Todo> = stmt
        .query_map([], |row| Ok(Todo::from(row)))
        .unwrap()
        .into_iter()
        .map(|r| r.unwrap())
        .collect();

    // 200 OKステータスコードと、JSONシリアライズしたtodo_listを返す
    HttpResponse::Ok().json(TodoList(results))
}

#[post("/todo")]
async fn register_todo(
    req: Json<RegisterTodo>,
    db: Data<Pool<SqliteConnectionManager>>,
) -> impl Responder {
    let id = Uuid::new_v4();
    let todo = Todo::new(TaskId(id), req.0.description, false, Utc::now());

    let conn = db.get().unwrap();
    conn.execute(
        "INSERT INTO todo (id, description, done, datetime) VALUES (?1, ?2, ?3, ?4)",
        params![
            todo.id.0.to_string(),
            todo.description,
            todo.done,
            todo.datetime.format(DATETIME_FORMAT).to_string()
        ],
    )
    .unwrap();

    let mut stmt = conn
        .prepare("SELECT id, description, done, datetime FROM todo WHERE id = ?1")
        .unwrap();

    let results: Vec<Todo> = stmt
        .query_map(params![id.to_string()], |row| Ok(Todo::from(row)))
        .unwrap()
        .into_iter()
        .map(|r| r.unwrap())
        .collect();

    HttpResponse::Ok().json(TodoList(results))
}

// #[actix_web::main]マクロは、main関数を非同期関数に変換する
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // RUST_LOGはenv_loggerの環境変数で、ログレベルを指定する
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    // コネクションプールの作成
    let manager = SqliteConnectionManager::file("db.sqlite3");
    let pool = Pool::new(manager).unwrap();

    let addr = "127.0.0.1:8080";
    info!("Starting server at {}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone())) // dataメソッドでアプリケーション全体で使用するデータを保存できる
            .service(health)
            .service(todos)
            .service(register_todo)
    })
    .bind(addr)?
    .run()
    .await
}
