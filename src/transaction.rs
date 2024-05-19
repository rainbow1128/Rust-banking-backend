pub struct Transaction {
    sender_uuid: uuid::Uuid,
    reciver_uuid: uuid::Uuid,
    amount: f64,
    message: String,
    transaction_id: uuid::Uuid,
}

impl Transaction {
    pub fn new(
        sender_uuid: uuid::Uuid,
        reciver_uuid: uuid::Uuid,
        amount: f64,
        message: String,
    ) -> Transaction {
        Transaction {
            sender_uuid,
            reciver_uuid,
            amount,
            message,
            transaction_id: uuid::Uuid::new_v4(),
        }
    }
    pub fn get_sender_uuid(&self) -> &uuid::Uuid {
        &self.sender_uuid
    }

    pub fn get_reciver_uuid(&self) -> &uuid::Uuid {
        &self.reciver_uuid
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn get_transaction_id(&self) -> &uuid::Uuid {
        &self.transaction_id
    }
}
