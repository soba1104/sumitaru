use std::net::AddrParseError;

quick_error!{
    #[derive(Debug)]
    pub enum Error {
        AddrParse(err: AddrParseError) {
            from()
            description("addr parse")
            display("AddrParseError: {}", err)
            cause(err)
        }
    }
}
