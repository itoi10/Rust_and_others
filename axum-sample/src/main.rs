use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // トレーシング(ログ、イベントトラッキング)の初期化
    tracing_subscriber::fmt::init();

    // アプリケーションの構築(ルーティングの設定)
    let app = Router::new()
        // `GET /` に対して `root`ハンドラを設定
        .route("/", get(root))
        // `POST /users` に対して `create_user`ハンドラを設定
        .route("/users", post(create_user));

    // アドレスのバインド
    // これはlet addr: SocketAddr = ([127, 0, 0, 1], 3000).into(); とも書ける
    //   詳しくはFromトレイト、Intoトレイトで調べれば出てくる
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    // hyperを使ってアプリケーション実行
    //   axum::Serverは実際にはhyper::Serverと同等
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// `GET /` に対するハンドラだが、単なる非同期関数であり、Webフレームワークと直接の依存はない
// &'static strはプログラムの生存期間全体で有効な文字列の型を表す
//    "Hello, World!"は文字列リテラルであり、コンパイル時にバイナリに埋め込まれるので、この型になる
async fn root() -> &'static str {
    "Hello, World!"
}

// `POST /users` に対するハンドラ
//   リクエストボディを受け取って、レスポンスボディを返す
// 引数の部分(Json(payload): Json<CreateUser>)はAxumがよしなにやってくれてる
// impl IntoResponseは、IntoResponseトレイトを実装している型を返すという意味
//   この関数の戻り値(StatusCode, impl IntoResponse)もIntoResponseトレイトを実装している
//   IntoResponseトレイトを実装している型は、axumがよしなにレスポンスボディに変換してくれる
//   トレイトは、抽象化というかインターフェース的なもの
async fn create_user(
    // リクエストボディをJSONとしてパースしてCreateUser構造体に変換する
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // リエストボディを受け取って、レスポンスボディを作る
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // (ステータスコード 201 Created, Jsonに変換されたuser)
    // このタプルはIntoResponseトレイトが実装されている
    (StatusCode::CREATED, Json(user))
}

// リクエストボディのでシリアライズ用の構造体
// リクエストボディはJSON形式だが、これを構造体に変換するためにDeserializeを使う
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// レスポンスボディのシリアライズ用の構造体
// 構造体からJSON形式のレスポンスボディを作るためにSerializeを使う
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
