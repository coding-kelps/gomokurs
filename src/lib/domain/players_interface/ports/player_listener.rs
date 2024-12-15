use crate::domain::game_manager::models::{PlayerColor, PlayerAction};
use crate::domain::players_interface::models::ListenError;
use tokio::sync::mpsc::Sender;

pub trait PlayerListener: Send + Sync + 'static {
    fn listen(
        &self,
        tx: Sender<(PlayerColor, PlayerAction)>,
        color: PlayerColor,
    ) -> impl std::future::Future<Output = Result<(), ListenError>> + Send;
}
