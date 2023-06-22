use azalea_buf::McBuf;
use azalea_core::ResourceLocation;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSoundPacket {
    pub sound: azalea_registry::CustomRegistry<azalea_registry::SoundEvent, CustomSoundEvent>,
    pub source: SoundSource,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub volume: f32,
    pub pitch: f32,
    pub seed: u64,
}

#[derive(McBuf, Clone, Debug)]
pub struct CustomSoundEvent {
    pub location: ResourceLocation,
    pub range: Option<f32>,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum SoundSource {
    Master = 0,
    Music = 1,
    Records = 2,
    Weather = 3,
    Blocks = 4,
    Hostile = 5,
    Neutral = 6,
    Players = 7,
    Ambient = 8,
    Voice = 9,
}
