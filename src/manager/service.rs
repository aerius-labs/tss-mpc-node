use crate::common::{
    Key, KeyGenRequest, MessageToSignStored, SignerResult, SigningRequest, SigningRoom,
};
use crate::queue::rabbitmq::RabbitMQService;
use crate::storage::mongodb::MongoDBStorage;
use anyhow::Result;
use futures::future::join_all;
use std::collections::HashMap;
use std::result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task;
use tracing::info;

use super::keygen::{self, run_keygen};

pub struct ManagerService {
    pub storage: MongoDBStorage,
    pub queue: RabbitMQService,
    pub(crate) signing_rooms: Arc<RwLock<HashMap<Key, String>>>,
    pub threshold: u16,
    pub total_parties: u16,
}

impl ManagerService {
    pub async fn new(
        mongodb_uri: &str,
        rabbitmq_uri: &str,
        signing_timeout: u64,
        threshold: u16,
        total_parties: u16,
    ) -> Result<Self> {
        let storage = MongoDBStorage::new(mongodb_uri, "tss_network").await?;
        let queue = RabbitMQService::new(rabbitmq_uri).await?;

        Ok(Self {
            storage,
            queue,
            signing_rooms: Arc::new(RwLock::new(HashMap::new())),
            threshold,
            total_parties,
        })
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting ManagerService");
        loop {
            // match self.queue.receive_signing_request().await {
            //     Ok(request) => {
            //         if let Err(e) = self.handle_signing_request(request).await {
            //             error!("Failed to handle signing request: {:?}", e);
            //         }
            //     }
            //     Err(e) => {
            //         error!("Failed to receive signing request: {:?}", e);
            //     }
            // }
        }
    }

    pub async fn handle_signing_request(&self, request: SigningRequest) -> Result<()> {
        info!("Received signing request: {}", request.id);
        self.storage.insert_request(&request).await?;

        let room_id = self.initiate_signing(&request.message).await?;

        // Notify all registered signers about the new signing request
        let signing_rooms = self.signing_rooms.read().await;
        if let Some(room) = signing_rooms.get(&room_id) {
            let room: SigningRoom = serde_json::from_str(room)?;
            for party_number in room.member_info.keys() {
                self.notify_signer(*party_number, &request).await?;
            }
        }

        Ok(())
    }

    async fn initiate_signing(&self, message: &[u8]) -> Result<String> {
        let room_id = crate::common::sha256_digest(message);
        let mut signing_rooms = self.signing_rooms.write().await;
        let signing_room =
            serde_json::to_string(&SigningRoom::new(room_id.clone(), self.total_parties))?;
        signing_rooms.insert(room_id.clone(), signing_room);
        Ok(room_id)
    }

    async fn notify_signer(&self, party_number: u16, request: &SigningRequest) -> Result<()> {
        // Implement logic to notify a signer about a new signing request
        // This could involve sending a message via RabbitMQ or another communication method
        todo!("Implement signer notification")
    }

    pub async fn get_signing_result(
        &self,
        request_id: &str,
    ) -> Result<Option<MessageToSignStored>> {
        self.storage.get_signing_result(request_id).await
    }

    pub async fn update_signing_result(&self, result: SignerResult) -> Result<()> {
        self.storage.update_signing_result(&result).await
    }

    pub async fn process_signing_request(&self, request: SigningRequest) -> Result<()> {
        self.storage.insert_request(&request).await?;
        self.queue.publish_signing_request(&request).await?;
        Ok(())
    }

    pub async fn process_keygen_request(
        &self,
        request: KeyGenRequest,
        manager_addr: &String,
    ) -> Result<Vec<String>> {
        self.storage.insert_key_gen_request(&request).await?;
        let total_parties = request.keygen_params.parties;
        let tasks: Vec<_> = (0..total_parties)
            .map(|party_id| {
                let manager_addr = manager_addr.clone();
                let request = request.clone();
                task::spawn(async move { run_keygen(&manager_addr, &request).await })
            })
            .collect();

        let results = join_all(tasks).await;

        // Collect successful results and aggregate errors
        let mut successful_results = Vec::new();
        let mut errors = Vec::new();

        for (index, res) in results.into_iter().enumerate() {
            match res {
                Ok(Ok(json_str)) => successful_results.push(json_str),
                Ok(Err(e)) => errors.push(format!("Error in party {}: {}", index, e)),
                Err(e) => errors.push(format!("Task error in party {}: {}", index, e)),
            }
        }

        if !errors.is_empty() {
            eprintln!("Errors occurred during key generation: {:?}", errors);
        }

        self.storage
            .update_key_gen_result(&request.id, successful_results.clone())
            .await?;
        Ok(successful_results)
    }
}
