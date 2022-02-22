#[derive(serde::Serialize, serde::Deserialize)]
pub enum UartRequest {
    Range { to: u8 },
    Ping,
    Health,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum UartResponse {
    Range { distance_mm: u32 },
    Health,
}
