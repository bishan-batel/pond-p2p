pub mod client;
pub mod server;

pub(crate) mod message {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ChatMessage {
        pub user: String,
        pub contents: String,
    }
}
