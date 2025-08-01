use crate::error::Error;
use crate::protocol::Base64Buffer;
use crate::protocol::WireMessage;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Transport {
    async fn create_queue(&self, queue_uuid: Uuid) -> Result<(), Error>;
    async fn send(
        &self,
        device_token: Option<String>,
        queue_uuid: Uuid,
        message: WireMessage,
    ) -> Result<(), Error>;
    async fn receive<T, F>(&self, queue_uuid: Uuid, on_messages: F) -> Result<T, Error>
    where
        F: Fn(&[WireMessage]) -> Result<Option<T>, Error> + Send;

    async fn health_check(&self) -> Result<(), Error>;
}

pub mod queue {
    use super::*;
    use uuid::Uuid;

    #[derive(Clone)]
    pub struct QueueClient {
        client: reqwest::Client,
    }

    pub struct QueueName(Uuid);

    impl QueueName {
        pub fn send(&self) -> String {
            self.0.to_string().to_uppercase().replace("-", "")
        }

        pub fn receive(&self) -> String {
            format!("{}_responder", self.send())
        }
    }

    impl QueueClient {
        const URL: &'static str = "https://mfa.akamai.com/api/v1/device/krypton/channel";

        pub fn new() -> Self {
            Self {
                client: reqwest::Client::new(),
            }
        }

        async fn send_inner(
            &self,
            queue_name: &str,
            device_token: Option<String>,
            message: WireMessage,
        ) -> Result<(), Error> {
            let query = device_token
                .map(|t| format!("?device_token={}", t))
                .unwrap_or("".to_string());
            let url = format!("{}/{}{}", Self::URL, queue_name, query);

            let message = Base64Buffer(message.into_wire()).to_string();
            let _ = self.client.post(url).body(message).send().await?;
            Ok(())
        }

        async fn receive_inner<T, F>(&self, queue_name: &str, on_messages: F) -> Result<T, Error>
        where
            F: Fn(&[WireMessage]) -> Result<Option<T>, Error> + Send,
        {
            let url = format!("{}/{}?poll_wait_secs=10", Self::URL, queue_name);

            // only try for 60s
            let timeout = 60i64;
            let mut duration = 0i64;
            while duration < timeout {
                let now = chrono::Utc::now().timestamp();
                let res: Res<Messages> = self.client.get(&url).send().await?.json().await?;
                let wire: Vec<WireMessage> = res
                    .result
                    .messages
                    .into_iter()
                    .filter_map(|m| WireMessage::new(m.0).ok())
                    .collect();

                duration += chrono::Utc::now().timestamp() - now;

                if let Some(res) = on_messages(&wire)? {
                    return Ok(res);
                }
            }

            Err(Error::ResponseTimedOut)
        }
    }

    #[derive(Debug, serde::Deserialize)]
    struct Res<T> {
        result: T,
    }

    #[derive(Debug, serde::Deserialize)]
    struct Messages {
        messages: Vec<Base64Buffer>,
    }

    #[async_trait]
    impl Transport for QueueClient {
        async fn create_queue(&self, _: Uuid) -> Result<(), Error> {
            Ok(())
        }

        async fn send(
            &self,
            device_token: Option<String>,
            queue_uuid: Uuid,
            message: WireMessage,
        ) -> Result<(), Error> {
            let queue = QueueName(queue_uuid);
            self.send_inner(&queue.send(), device_token, message).await
        }

        async fn receive<T, F>(&self, queue_uuid: Uuid, on_messages: F) -> Result<T, Error>
        where
            F: Fn(&[WireMessage]) -> Result<Option<T>, Error> + Send,
        {
            let queue = QueueName(queue_uuid);
            self.receive_inner(&queue.receive(), on_messages).await
        }


        impl AwsClient {
        /// Note to reader: it is *COMPLETELY OK* for these values to be publicly known.
        /// These are heavily restricted credentials: create a queue, add messages to it, and read messages from it.
        /// For example, listing queues is not permitted.
        /// All messages are end-to-end encrypted and authenticated via the Krypton protocol.
        /// This is a legacy bridge to old krypton and will be removed soon.
        const ACCESS_KEY: &'static str = "AKIAJMZJ3X6MHMXRF7QQ";
        const SECRET_KEY: &'static str = "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";
        const QUEUE_URL_BASE: &'static str = "https://sqs.us-east-1.amazonaws.com/911777333295";
        pub fn new() -> Result<Self, Error> {
            let provider = StaticProvider::new(Self::ACCESS_KEY.into(), Self::SECRET_KEY.into(), None, None);
            let sqs = SqsClient::new_with(HttpClient::new()?, provider.clone(), Region::UsEast1);
            let sns = SnsClient::new_with(HttpClient::new()?, provider.clone(), Region::UsEast1);
            Ok(Self { sqs, sns })
        }

        async fn health_check(&self) -> Result<(), Error> {
            let queue_uuid = Uuid::new_v4();
            self.create_queue(queue_uuid).await?;
            let fake_message: Vec<u8> = sodiumoxide::randombytes::randombytes(4);
            let msg = WireMessage::SealedMessage(fake_message.clone());

            let queue = QueueName(queue_uuid);
            self.send_inner(&queue.receive(), None, msg).await?;

            self.receive(queue_uuid, |msg| {
                for wire_message in msg {
                    if wire_message.clone().data().eq(&fake_message) {
                        return Ok(Some(fake_message.clone()));
                    }
                }
                Err(Error::UnexpectedResponse)
            })
            .await?;

            Ok(())
        }
    }
}
