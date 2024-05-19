use crate::transaction::Transaction;

type MobileAppId = String;

enum TransactionError {
    TransactionNotApplicableForUser,
    AmountTooLow,
}

pub enum TwoFaBankUser {
    TextMessage,
    PhoneCall,
    MobileApp(MobileAppId),
}

pub struct BankUser<'a> {
    pub name: String,
    pub surname: String,
    pub phone: String,
    bank_number: uuid::Uuid,
    email: String,
    current_money: f64,
    debit: f64,
    history: Vec<Transaction>,
}

impl<'a> BankUser<'a> {
    pub fn new(name: String, surname: String, phone: String, email: String) -> BankUser<'a> {
        BankUser {
            name,
            surname,
            phone,
            email,
            bank_number: uuid::Uuid::new_v4(),
            current_money: 0.0,
            debit: 0.0,
            history: Vec::new(),
        }
    }
    pub fn apply_transaction(&mut self, trans: Transaction) -> Result<(), TransactionError> {
        let sender_id = trans.get_sender_uuid();
        let reciver_id = trans.get_reciver_uuid();
        match self.bank_number {
            sender_id => {
                if self.current_money + self.debit < trans.get_amount() {
                    return Err(TransactionError::AmountTooLow);
                }
                self.current_money -= trans.get_amount();
                self.history.push(trans);
                Ok(())
            }
            reciver_id => {
                self.current_money += trans.get_amount();
                self.history.push(trans);
                Ok(())
            }
            _ => Err(TransactionError::TransactionNotApplicableForUser),
        }
    }
}
