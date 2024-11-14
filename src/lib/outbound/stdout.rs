use crate::domain::game::ports::PlayerNotifier;

#[derive(Debug, Clone)]
pub struct Stdout {
}

impl Stdout {
    pub fn new() -> Self {
        Self {}
    }
}

impl PlayerNotifier for Stdout {
    fn notify_begin(&self) -> Result<(), NotifyBeginError> {
        println("White Player Begin");

        Ok(())
    }

    fn notify_end(&self) -> Result<(), NotifyEndError> {
        println("Game Over");

        Ok(())
    }
}
