#[tauri::command]
pub async fn start_login_server() -> bool {
    println!("start_login_server start");

    let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(12);

    tokio::spawn(async move {
        // let listener = tokio::net::TcpListener::bind("127.0.0.1").await.unwrap();
        // let port = listener.local_addr().unwrap().port();
        

        // for stream in listener.incoming() {
        //     let stream = stream.unwrap();
        //     thread::spawn(|| handle_connection(stream));
        // }

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        tx.send(0).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        println!("start_login_server finish");
    });
    if rx.recv().await.is_some() {
        return true;
    }
    false
}
