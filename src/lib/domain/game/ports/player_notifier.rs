use crate::domain::game::models;

pub trait PlayerNotifier {
    fn notify_begin(
        &self,
    ) -> Result<(), NotifyBeginError>;

    fn notify_end(
        &self,
    ) -> Result<(), NotifyEndError>;
}
