# keyz_rust_client
Rust client for keyz
# Getting started

- Download from crates [here](https://crates.io/crates/keyz_rust_client)
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
