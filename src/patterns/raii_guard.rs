use std::io;

struct NetworkConnection {
    connected: bool,
}

impl NetworkConnection {
    fn connect() -> Self {
        NetworkConnection { connected: true }
    }

    fn send_data(&self, data: &str) -> io::Result<()> {
        if self.connected {
            println!("Sending data over the network: {}", data);
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "Connection closed",
            ))
        }
    }

    fn close(&mut self) {
        println!("Closing network connection");
        self.connected = false;
    }
}

pub struct ConnectionGuard {
    network: Option<NetworkConnection>,
}

impl ConnectionGuard {
    pub fn new() -> Self {
        ConnectionGuard {
            network: Some(NetworkConnection::connect()),
        }
    }

    pub fn send_data(&self, data: &str) -> io::Result<()> {
        if let Some(network) = &self.network {
            network.send_data(data)
        } else {
            Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "Connection closed",
            ))
        }
    }

    fn close(&mut self) {
        if let Some(mut network) = self.network.take() {
            network.close();
        }
    }
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        println!("ConnectionGuard is dropping");
        self.close();
    }
}

#[cfg(test)]
mod test {
    use crate::patterns::raii_guard::ConnectionGuard;

    #[test]
    fn test_raii_guard() {
        let connection_guard = ConnectionGuard::new();

        assert!(connection_guard.send_data("Something New!").is_ok());
    }
}
