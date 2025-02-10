pub const PROTOCOL_VERSION: &str            = "0.2.0";

#[non_exhaustive]
pub struct ActionID;

impl ActionID {
    // Action that can be send from the manager to the player.

    pub const MANAGER_PROTOCOL_COMPATIBLE: u8   = 0x00;
    pub const MANAGER_START: u8                 = 0x01;
    pub const MANAGER_RESTART: u8               = 0x02;
    pub const MANAGER_TURN: u8                  = 0x03;
    pub const MANAGER_BEGIN: u8                 = 0x04;
    pub const MANAGER_BOARD: u8                 = 0x05;
    pub const MANAGER_INFO: u8                  = 0x06;
    pub const MANAGER_RESULT: u8                = 0x07;
    pub const MANAGER_END: u8                   = 0x08;
    pub const MANAGER_ABOUT: u8                 = 0x09;
    #[allow(dead_code)]
    pub const MANAGER_UNKNOWN: u8               = 0x0A;
    pub const MANAGER_ERROR: u8                 = 0x0B;

    // Actions that can be send from the player to the manager.

    pub const PLAYER_PROTOCOL_VERSION: u8       = 0x0C;
    pub const PLAYER_READY: u8                  = 0x0D;
    pub const PLAYER_PLAY: u8                   = 0x0E;
    pub const PLAYER_PLAYER_DESCRIPTION: u8     = 0x0F;
    pub const PLAYER_UNKNOWN: u8                = 0x10;
    pub const PLAYER_ERROR: u8                  = 0x11;
    pub const PLAYER_MESSAGE: u8                = 0x12;
    pub const PLAYER_DEBUG: u8                  = 0x13;
    pub const PLAYER_SUGGESTION: u8             = 0x14;
}
