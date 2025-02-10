use crate::domain::coordinator::models::*;
use tokio::sync::mpsc::Sender;

pub trait PlayerInterface: Send + Sync + 'static {
    /// Listens for player actions and sends them to the specified channel.
    ///
    /// # Arguments
    ///
    /// * `color` - The `PlayerColor` representing the player being listened to.
    /// * `tx` - A `Sender` channel for forwarding `(PlayerColor, PlayerAction)`
    /// tuples.
    fn listen(
        &self,
        color: PlayerColor,
        tx: Sender<(PlayerColor, PlayerAction)>,
    ) -> impl std::future::Future<Output = Result<(), ListenError>> + Send;

    /// Notifies the player of the initial gomoku board configuration.
    /// 
    /// # Arguments
    /// 
    /// * `size` - The size of the gomoku board, both width and height.
    fn notify_start(
        &self,
        size: u8,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notifies the player of the to initialize its board with the same
    /// configuration as the previous game.
    fn notify_restart(
        &self
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notifies the player about the opponent's turn.
    /// 
    /// # Arguments
    /// 
    /// * `position` - The position where the opponent placed their piece.
    fn notify_turn(
        &self,
        position: Position,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;
    
    /// Notifies the player that it is their turn to make the first move.
    fn notify_begin(
        &self,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notifies the player of the initial gomoku board state, including
    /// any moves that have already been played.
    /// 
    /// # Arguments
    /// 
    /// * `turns` - A collection of moves (`RelativeTurn`) that have been played.
    fn notify_board(
        &self,
        turns: Vec<RelativeTurn>,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Sends general game-related information to the player.
    /// 
    /// # Arguments
    /// 
    /// * `info` - The game information to send.
    fn notify_info(
        &self,
        info: Information,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notifies the game result to the player.
    /// 
    /// # Arguments
    /// 
    /// + `result` - The game result.
    fn notify_result(
        &self,
        result: RelativeGameEnd,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notifies the player that the game has finished.
    fn notify_end(
        &self,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Requests a description of the player from the player.
    fn notify_about(
        &self,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notifies the player that their last action was unrecognized.
    /// 
    /// # Arguments
    /// 
    /// * `content` - A message explaining the error or unrecognized action.
    fn notify_unknown(
        &self,
        content: &str,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notifies the player that their last action or its arguments were unexpected.
    /// 
    /// # Arguments
    /// 
    /// * `content` - A message explaining the unexpected action or arguments.
    fn notify_error(
        &self,
        content: &str,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;
}
