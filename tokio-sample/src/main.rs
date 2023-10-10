// tokioを使ったTCPエコーサーバのサンプル
use log::{error, info};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

// #[tokio::main] はmain関数を非同期関数にするためのアトリビュート
// Box<dyn std::error::Error>は、さまざまなエラー型を一つの型として抽象化するための書き方
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Loggerのセットアップ
    // ログを表示したい場合は ```$ RUST_LOG=debug cargo run```で実行する
    env_logger::builder().format_timestamp(None).init();

    info!("Starting server on 127.0.0.1:8080");
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    // TPC接続を待って、接続のたびに新しい非同期タスクを生成するループ
    // 生成されたタスクは並行して実行されるので、複数のクライアントからの接続を同時に処理できる
    loop {
        // TPC接続を待ち受ける
        // awaitキーワードで一時停止して接続があるまで待ってる
        let (mut socket, addr) = listener.accept().await?;
        info!("Accepted connection from {}", addr);

        // tokio::spawn(async move { /*非同期で実行する処理*/ }); で非同期タスクを生成する
        // moveをつけとクロージャ内で使用される変数の所有権が、そのクロージャに移動することを示す
        //    この場合だと、socketとaddrの所有権がクロージャに移動する
        //    moveを使う理由は次の２点が考えられる
        //      1.クロージャと変数のライフタイムを一致させることでデータがタスク終了まで有効であることを保証する
        //      2.タスクが実行されている間に安全にアクセスできデータ競合のない並行処理を実現する
        tokio::spawn(async move {
            // 1024バイトのバッファを0で初期化して用意する
            let mut buf = [0; 1024];

            loop {
                // socket.readは非同期関数なのでawaitキーワードで一時停止してデータが届くのを待つ
                let n = match socket.read(&mut buf).await {
                    // 読み込みバイト数が0、つまり接続が閉じられたらループを抜ける
                    Ok(n) if n == 0 => {
                        info!("Connection from {} closed", addr);
                        return;
                    }
                    // 読み込み成功
                    Ok(n) => {
                        info!("Received {} bytes from {}", n, addr);
                        n
                    }
                    // 読み込み失敗
                    Err(e) => {
                        error!("Failed to read from {}; err = {:?}", addr, e);
                        return;
                    }
                };

                // socket.write_allはソケットに書き込む非同期処理
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    error!("Failed to write to {}; err = {:?}", addr, e);
                    return;
                } else {
                    info!("Sent {} bytes to {}", n, addr);
                }
            }
        });
    }
}
