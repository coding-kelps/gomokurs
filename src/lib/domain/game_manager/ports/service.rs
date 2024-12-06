pub trait GameManagerService
{
    fn new_game(
        &mut self,
    ) -> impl std::future::Future<Output = Result<(), ()>>;
}
