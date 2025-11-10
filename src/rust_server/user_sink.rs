use crate::mt5_apiserver::IMTUserSink;

#[derive(Debug)]
pub struct UserSink {
    mt_user_sink: *mut IMTUserSink
}

impl UserSink {
    pub fn new(mt_user_sink: *mut IMTUserSink) -> UserSink {
        UserSink {
            mt_user_sink
        }
    }
}
