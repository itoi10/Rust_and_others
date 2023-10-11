// リクエストのパスを取得し、そのままレスポンスボディとして返すHTTPサーバのサンプル
use env_logger;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use log::{error, info};
use std::convert::Infallible;

async fn echo(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // リクエストのパスを取得  to_owned() は String に変換するため
    let path = req.uri().path().to_owned();

    // レスポンスを作成
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(path))
        .unwrap();
    info!("Response: {:?}", response);
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Loggerのセットアップ
    env_logger::builder().format_timestamp(None).init();

    // ローカルホストの8080ポートでリッスンするアドレス  intoはSocketAddrに変換するため
    let addr = ([127, 0, 0, 1], 8080).into();

    // サービスを作成する
    // make_service_fnは、リクエストを受け取った時に呼び出される関数を作成する。
    //    新しいTCP接続が確立されるたびにmake_service_fnは提供したクロージャを呼び出す
    // _connはクライアントからの新しい接続を表すオブジェクト
    let make_svc = make_service_fn(|_conn| {
        // echo関数を呼び出すサービスを作成する
        let svc = service_fn(echo);
        // async moveで非同期ブロック生成. moveしているのでsrvの所有権を奪う
        // Result<_, Infallible>はエラーが発生しないことを表す
        async move { Ok::<_, Infallible>(svc) }
    });

    // サーバインスタンス作成
    let server = Server::bind(&addr).serve(make_svc);
    info!("Listening on http://{}", addr);

    // サーバを実行する
    // エラーが発生した場合はエラーをログ出力して終了. ?はResultのエラーを返す
    server.await.map_err(|e| {
        error!("server error: {}", e);
        e
    })?;

    Ok(())
}
