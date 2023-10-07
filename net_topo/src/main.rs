mod messaging;
extern "C" {
    fn initiate_scan();
}
#[tokio::main]
async fn main() {
    messaging::server::start("127.0.0.1", 8080).await.expect("Server crashed");
    unsafe {
        initiate_scan();
    }
}
