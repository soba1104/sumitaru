use std::io::Error as IoError;
use std::net::AddrParseError;

quick_error!{
    #[derive(Debug)]
    pub enum Error {
        Io(err: IoError) {
            from()
            description("io")
            display("IoError: {:?}", err)
            cause(err)
        }
        AddrParse(err: AddrParseError) {
            from()
            description("addr parse")
            display("AddrParseError: {:?}", err)
            cause(err)
        }
    }
}
