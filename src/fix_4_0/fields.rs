#[derive(Debug)]
pub struct Field {
    id: u8,
    name: &'static str,
}

pub static ACCOUNT: Field = Field {
    id: 1,
    name: "Account",
};

pub static ADV_ID: Field = { 
    id: 2,
    name: "AdvId"
}