pub trait GameManagerService: Clone + Send + Sync + 'static {
    fn new_game(
        &self,
    ) -> impl std::future::Future<Output = Result<(), ()>> + Send;
}
