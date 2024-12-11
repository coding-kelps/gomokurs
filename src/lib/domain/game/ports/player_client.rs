use crate::domain::game::models::*;
use tokio::sync::mpsc::Sender;

pub trait PlayerClient: Send + Sync + 'static {
    fn listen(
        &self,
        tx: Sender<(PlayerColor, PlayerAction)>,
        player: PlayerColor,
    ) -> impl std::future::Future<Output = Result<(), ()>> + Send;

    fn notify_start(
        &self,
        size: u8,
    ) -> impl std::future::Future<Output = Result<(), NotifyStartError>>;

    fn notify_turn(
        &self,
        position: Position,
    ) -> impl std::future::Future<Output = Result<(), NotifyTurnError>>;
    
    fn notify_begin(
        &self,
    ) -> impl std::future::Future<Output = Result<(), NotifyBeginError>>;

    fn notify_board(
        &self,
        turns: Vec<RelativeTurn>,
    ) -> impl std::future::Future<Output = Result<(), NotifyBoardError>>;

    fn notify_info(
        &self,
        info: Information,
    ) -> impl std::future::Future<Output = Result<(), NotifyInfoError>>;

    fn notify_end(
        &self,
    ) -> impl std::future::Future<Output = Result<(), NotifyEndError>>;

    fn notify_about(
        &self,
    ) -> impl std::future::Future<Output = Result<(), NotifyAboutError>>;

    fn notify_unknown(
        &self,
        content: &str,
    ) -> impl std::future::Future<Output = Result<(), NotifyUnknownError>>;

    fn notify_error(
        &self,
        content: &str,
    ) -> impl std::future::Future<Output = Result<(), NotifyErrorError>>;
}
