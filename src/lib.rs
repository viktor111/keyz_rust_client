use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc, u64,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};

pub struct Keyz {
    pub host: String,
    pub port: u16,
    pub stream: Arc<Mutex<TcpStream>>,
}

impl Keyz {
    pub async fn new(host: String, port: u16) -> Keyz {
        let addr = Self::socket_address_from_string_ip(format!("{}:{}", host, port)).unwrap();

        let stream = TcpStream::connect(addr)
            .await
            .expect("[-] Failed to connect to server check if server is running on port 7667");

        Keyz {
            host: host,
            port: port,
            stream: Arc::new(Mutex::new(stream)),
        }
    }

    pub async fn set(&self, key: &str, value: &str, expire_secs: Option<u64>) -> Result<String, Box<dyn Error>> {
        let set_command = match expire_secs {
            Some(expire) => format!("SET {} {} EX {}", key, value, expire),
            None => format!("SET {} {}", key, value)
        };
        let response = Self::send_message(self, &set_command).await.unwrap();
        if response == "ok" {
            Ok(response)
        } else {
            Err("[-] Failed to set value".into())
        }
    }
    

    pub async fn get(&self, key: &str) -> Result<String, Box<dyn Error>> {
        let get_command = format!("GET {}", key);
        let response = Self::send_message(self, &get_command).await.unwrap();
        if response != "null" {
            Ok(response)
        } else {
            Err("[-] Failed to get value".into())
        }
    }

    pub async fn delete(&self, key: &str) -> Result<String, Box<dyn Error>> {
        let delete_command = format!("DEL {}", key);
        let response = Self::send_message(self, &delete_command).await.unwrap();
        if response == key {
            Ok(response)
        } else {
            Err("[-] Failed to delete value".into())
        }
    }
    
    pub async fn expires_in(&self, key: &str) -> Result<u64, Box<dyn Error>> {
        let expires_in_command = format!("EXIN {}", key);
        let response = Self::send_message(self, &expires_in_command).await.unwrap();
        if response != key {
            let response: u64 = response.parse().unwrap();
            Ok(response)
        } else {
            Err("[-] Failed to retrieve expiration time".into())
        }
    }

    pub async fn dispose(&self) -> Result<(), Box<dyn Error>> {
        let close_msg = "CLOSE";
        Self::send_message(self, close_msg).await?;
        let response = Self::read_message(self).await?;
        let mut stream = self.stream.lock().await;
        stream.shutdown().await?;
        Ok(())
    }

    async fn read_message(&self) -> Result<String, Box<dyn Error>> {
        let mut stream = self.stream.lock().await;
        let mut len_bytes = [0; 4];
        let bytes_read = stream.read(&mut len_bytes).await?;
        if bytes_read < 4 {
            return Err("[-] Failed to read the length of the message".into());
        }
        let len = u32::from_be_bytes(len_bytes);
        let mut buffer = vec![0; len as usize];
        stream.read_exact(&mut buffer).await?;
        let message = String::from_utf8_lossy(&buffer);
        Ok(message.to_string())
    }

    pub async fn send_message(&self, message: &str) -> Result<String, Box<dyn Error>> {
        let mut stream = self.stream.lock().await;
        //stream.write_all(&[BYTE_PASSWORD]).await?;
        let len = message.len() as u32;
        let len_bytes = len.to_be_bytes();
        stream.write_all(&len_bytes).await?;
        stream.write_all(message.as_bytes()).await?;
        if message != "CLOSE" {
            let mut len_bytes = [0; 4];
            let bytes_read = stream.read(&mut len_bytes).await?;
            if bytes_read < 4 {
                return Err("[-] Failed to read the length of the message".into());
            }
            let len = u32::from_be_bytes(len_bytes);
            let mut buffer = vec![0; len as usize];
            stream.read_exact(&mut buffer).await?;
            let message = String::from_utf8_lossy(&buffer);
            return Ok(message.to_string());
        }
        Ok(message.to_string())
    }

    fn socket_address_from_string_ip(ip: String) -> Result<SocketAddr, Box<dyn Error>> {
        const INVALID_IP_ERROR: &str = "Invalid IP address - should be in format: 127.0.0.1:8080";

        let ip = ip.split(":").collect::<Vec<&str>>();
        let port = ip[1].parse::<u16>().expect(INVALID_IP_ERROR);

        let ip_parts = ip[0].split(".").collect::<Vec<&str>>();

        if ip_parts.len() != 4 {
            return Err(INVALID_IP_ERROR.into());
        }

        let mut ip_parts_u8 = Vec::new();
        for part in ip_parts {
            let part_u8 = part.parse::<u8>();
            if part_u8.is_err() {
                return Err(INVALID_IP_ERROR.into());
            }
            ip_parts_u8.push(part_u8.unwrap());
        }

        let ip_addr = IpAddr::V4(Ipv4Addr::new(
            ip_parts_u8[0],
            ip_parts_u8[1],
            ip_parts_u8[2],
            ip_parts_u8[3],
        ));

        let socket_addr = SocketAddr::new(ip_addr, port);

        return Ok(socket_addr);
    }
}

#[cfg(test)]
mod tests {
    use crate::Keyz;


    #[tokio::test]   
    async fn set_value_using_send_message() {
        let keyz = Keyz::new("127.0.0.1".to_owned(), 7667).await;
        keyz.send_message("SET test 1").await.unwrap();
        let val = keyz.send_message("GET test").await.unwrap();
        keyz.dispose().await.unwrap();

        assert_eq!(val, "1");
    }

    #[tokio::test]
    async fn set_value() {
        let keyz = Keyz::new("127.0.0.1".to_owned(), 7667).await;
        let result = keyz.set("test", "1", None).await.unwrap();
        keyz.dispose().await.unwrap();

        assert_eq!(result, "ok");
    }

    #[tokio::test]
    async fn get_value() {
        let keyz = Keyz::new("127.0.0.1".to_owned(), 7667).await;
        let req = keyz.set("test", "1", None).await.unwrap();
        let result = keyz.get("test").await.unwrap();
        keyz.dispose().await.unwrap();

        assert_eq!(result, "1");
    }

    #[tokio::test]
    async fn delete_value() {
        let keyz = Keyz::new("127.0.0.1".to_owned(), 7667).await;
        let req = keyz.set("test", "1", None).await.unwrap();
        let result = keyz.delete("test").await.unwrap();
        keyz.dispose().await.unwrap();

        assert_eq!(result, "test");
    }

    #[tokio::test]
    async fn expires_in() {
        let keyz = Keyz::new("127.0.0.1".to_owned(), 7667).await;
        let req = keyz.set("test", "1", Some(20)).await.unwrap();
        let result = keyz.expires_in("test").await.unwrap();
        keyz.dispose().await.unwrap();

        assert_ne!(result, 0);
    }
}
