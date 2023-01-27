# keyz_rust_client
Rust client for keyz
# Getting started

- Download from crates [here](https://crates.io/crates/keyz_rust_client)
- Import
  ``` rust
  use crate::client::connection::Keyz;
  ```
- Initialize the connection to running keyz server
  ``` rust
  let keyz = Keyz::new("127.0.0.1".to_owned(), 7667).await;
  ```
- Set value where key is test and value is 1
  ```rust
  let result = keyz.set("test", "1").await.unwrap();
  ```
- Get value with key test
  ``` rust
  let result = keyz.get("test").await.unwrap();
  ```
- Delete value with key test
  ``` rust
  let result = keyz.delete("test").await.unwrap();
  ```
- Dispose connection
  ``` rust
  keyz.dispose().await.unwrap();
  ```
  
 ***!!! Important make sure to dispose of connection when not needed anymore***
 
 # Using direct send message
 ``` rust
 let keyz = Keyz::new("127.0.0.1".to_owned(), 7667).await;
 keyz.send_message("SET test 1").await.unwrap();
 keyz.dispose().await.unwrap();
 ```
 It is not advised to directly use this because some command currently are not fully supported with this method. You can learn more about all the commands in the keyz repo [here](https://github.com/viktor111/keyz)
