use crate::domain::game_manager::models::*;

pub trait PlayerListener: Send + Sync + Clone + 'static {
    fn listen_command(
        &mut self,
    ) -> impl std::future::Future<Output = Result<PlayerCommands, ListenCommandError>> + Send;
}
