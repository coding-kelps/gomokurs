use crate::domain::game::models::*;

pub trait PlayerListener: Send + Sync + 'static {
    fn listen_action(
        &mut self,
    ) -> impl std::future::Future<Output = Result<PlayerAction, ListenActionError>> + Send;
}
