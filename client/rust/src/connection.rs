use std::net::{Shutdown, TcpListener, TcpStream};

pub struct Connection {
    address: String,
    port: String,
    stream: Option<TcpStream>,
    used_up: bool,
}

impl Connection {
    pub fn new(address: String, port: String) -> Self {
        Connection {
            address,
            port,
            stream: None,
            used_up: false,
        }
    }

    pub fn connect(&mut self) -> std::io::Result<()> {
        if self.used_up {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Cannot reuse a `Connection`.",
            ));
        }

        let listener = TcpListener::bind(format!("{}:{}", self.address, self.port))?;
        if self.port == "0" {
            self.port = listener.local_addr().unwrap().port().to_string();
        }

        let (stream, _) = listener.accept()?;
        self.stream = Some(stream);
        Ok(())
    }

    pub fn close(&mut self) -> std::io::Result<()> {
        match &self.stream {
            Some(stream) => match stream.shutdown(Shutdown::Both) {
                Ok(()) => {
                    self.used_up = true;
                    self.stream = None;
                    Ok(())
                }
                Err(e) => Err(e),
            },
            None => Ok(()),
        }
    }
}

#[allow(unused_must_use)] // TODO: handle possible error
impl Drop for Connection {
    fn drop(&mut self) {
        self.close();
    }
}
