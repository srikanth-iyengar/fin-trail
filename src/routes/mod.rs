pub mod splash;
pub mod init {
    pub mod postgres;
    pub mod sqlite3;
}

pub mod home {
    pub mod account;
    pub mod reccuring_transaction;
    pub mod root;
    pub mod transaction;
}
