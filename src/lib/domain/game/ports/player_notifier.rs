use crate::domain::game::models::*;

pub trait PlayerNotifier: Send + Sync + 'static {
    fn notify_start(
        &mut self,
        size: u8,
    ) -> impl std::future::Future<Output = Result<(), NotifyStartError>> + Send;

    fn notify_turn(
        &mut self,
        position: Position,
    ) -> impl std::future::Future<Output = Result<(), NotifyTurnError>> + Send;
    
    fn notify_begin(
        &mut self,
    ) -> impl std::future::Future<Output = Result<(), NotifyBeginError>>;

    fn notify_board(
        &mut self,
        turns: Vec<RelativeTurn>,
    ) -> impl std::future::Future<Output = Result<(), NotifyBoardError>>;

    fn notify_info(
        &mut self,
        info: Information,
    ) -> impl std::future::Future<Output = Result<(), NotifyInfoError>>;

    fn notify_end(
        &mut self,
    ) -> impl std::future::Future<Output = Result<(), NotifyEndError>>;

    fn notify_about(
        &mut self,
    ) -> impl std::future::Future<Output = Result<(), NotifyAboutError>>;

    fn notify_unknown(
        &mut self,
        content: &str,
    ) -> impl std::future::Future<Output = Result<(), NotifyAboutError>>;

    fn notify_error(
        &mut self,
        content: &str,
    ) -> impl std::future::Future<Output = Result<(), NotifyAboutError>>;
}
