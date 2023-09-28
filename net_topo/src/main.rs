mod messaging;
extern "C" {
    fn initiate_scan();
}

fn main() {
    messaging::server::start().expect("Server crashed");
    unsafe {
        initiate_scan();
    }
}

