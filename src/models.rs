#[derive(Queryable)]

pub struct FullTrans {
    pub id: i32,
    pub txid: String,
    pub to_address: String,
    pub from_address: String,
    pub type_handle: String,
    pub value: String,
    pub fee: String,
    pub handle: String,
    pub block_number: i32,
    pub erc20: bool,
}

