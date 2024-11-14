use crate::domain::game::models::{NotifyBeginError, NotifyEndError};

pub trait PlayerNotifier {
    fn notify_begin(
        &self,
    ) -> Result<(), NotifyBeginError>;

    fn notify_end(
        &self,
    ) -> Result<(), NotifyEndError>;
}
