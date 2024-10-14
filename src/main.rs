use std::net::TcpListener;
use womscp::womscp::WOMSCP;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream_res in listener.incoming() {
        match stream_res {
            Ok(stream) => {
                match WOMSCP::try_from(stream) {
                    Ok(res) => { 
                        dbg!(res);
                    },
                    Err(e) => eprintln!("TCP stream read error: {:?}", e),
                }
            },
            Err(e) => {
                eprintln!("TCP stream error: {:?}", e);
            }
        } 
    }
}
