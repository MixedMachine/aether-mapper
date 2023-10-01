mod messaging;
extern "C" {
    fn initiate_scan();
}

fn main() {
    messaging::server::start(8080).expect("Server crashed");
    unsafe {
        initiate_scan();
    }
}
