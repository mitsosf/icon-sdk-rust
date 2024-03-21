pub(crate) enum TransactionType {
    LastBlock,
    BlockByHeight,
    BlockByHash,
    TransactionResult,
    TransactionByHash,
    Balance,
    SendTransaction,
    Call,
}

impl TransactionType {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            TransactionType::LastBlock => "icx_getLastBlock",
            TransactionType::BlockByHeight => "icx_getBlockByHeight",
            TransactionType::BlockByHash => "icx_getBlockByHash",
            TransactionType::TransactionResult => "icx_getTransactionResult",
            TransactionType::TransactionByHash => "icx_getTransactionByHash",
            TransactionType::Balance => "icx_getBalance",
            TransactionType::SendTransaction => "icx_sendTransaction",
            TransactionType::Call => "icx_call",
        }
    }
}
