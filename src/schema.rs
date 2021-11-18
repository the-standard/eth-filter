table! {
    full_trans (id) {
        id -> Integer,
        txid -> Text,
        to_address -> Text,
        type_handle -> Text,
        value -> Text,
        fee -> Text,
        handle -> Text,
        from_address -> Text,
        block_number -> Integer,
        erc20 -> Bool,
    }
}
