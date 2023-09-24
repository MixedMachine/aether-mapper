extern "C" {
    fn initiate_scan();
}

fn main() {
    unsafe {
        initiate_scan();
    }
}

