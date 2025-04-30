use std::net::TcpListener;
use std::io::{Read, Write};
use crate::{Router, Request, Response};

pub struct Server {
    address: String,
    router: Router,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address: address.to_string(),
            router: Router::new(),
        }
    }

    pub fn with_router(mut self, router: Router) -> Self {
        self.router = router;
        return self;
    }

    pub fn run(&self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(&self.address)?;

        println!("[s] http://{}", self.address);

        for stream in listener.incoming() {
            let mut stream = stream?;

            let mut buffer = [0; 1024];
            stream.read(&mut buffer)?;

            let request = Request::from_buffer(&buffer);
            println!("[+] {:#?}", request); // Pretty debug :3

            let response = self.router.handle_request(request);

            stream.write_all(response.to_bytes().as_slice())?;
        }

        Ok(())
    }
}