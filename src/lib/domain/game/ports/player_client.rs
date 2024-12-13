use crate::domain::game::models::*;
use tokio::sync::mpsc::Sender;

pub trait PlayerClient: Send + Sync + 'static {
    fn listen(
        &self,
        tx: Sender<(PlayerColor, PlayerAction)>,
        player: PlayerColor,
    ) -> impl std::future::Future<Output = Result<(), ListenError>> + Send;

    fn notify_start(
        &self,
        size: u8,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    fn notify_turn(
        &self,
        position: Position,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;
    
    fn notify_begin(
        &self,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    fn notify_board(
        &self,
        turns: Vec<RelativeTurn>,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    fn notify_info(
        &self,
        info: Information,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    fn notify_end(
        &self,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    fn notify_about(
        &self,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    fn notify_unknown(
        &self,
        content: &str,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    fn notify_error(
        &self,
        content: &str,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;
}
