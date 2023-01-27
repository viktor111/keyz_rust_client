mod client;

#[cfg(test)]
mod tests {
    use crate::client::connection::Keyz;

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
        let result = keyz.set("test", "1").await.unwrap();
        keyz.dispose().await.unwrap();

        assert_eq!(result, "ok");
    }

    #[tokio::test]
    async fn get_value() {
        let keyz = Keyz::new("127.0.0.1".to_owned(), 7667).await;
        let req = keyz.set("test", "1").await.unwrap();
        let result = keyz.get("test").await.unwrap();
        keyz.dispose().await.unwrap();

        assert_eq!(result, "1");
    }

    #[tokio::test]
    async fn delete_value() {
        let keyz = Keyz::new("127.0.0.1".to_owned(), 7667).await;
        let req = keyz.set("test", "1").await.unwrap();
        let result = keyz.delete("test").await.unwrap();
        keyz.dispose().await.unwrap();

        assert_eq!(result, "test");
    }
}
