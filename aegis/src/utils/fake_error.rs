use win_msgbox::{CancelTryAgainContinue, Error, Okay};

pub async fn show_error(title: &str, message: &str) -> Result<(), ()> {
    if cfg!(feature = "fake_error") {
        let response = win_msgbox::error::<CancelTryAgainContinue>(message)
            .title(title)
            .show();

        match response {
            Ok(_) => Ok(()),
            Err(e) => Err(()),
        }
    } else {
        Ok(())
    }
}
