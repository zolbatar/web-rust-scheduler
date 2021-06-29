use std::time::Duration;
use stdweb::js;
use yew::prelude::*;

// This is because the inbuilt TimeoutService appears buggy, copying it here makes it work
fn to_ms(duration: Duration) -> u32 {
    let ms = duration.subsec_millis();
    ms + duration.as_secs() as u32 * 1000
}

pub fn create_timeout(duration: Duration, callback: Callback<()>) {
    let callback = move || {
        callback.emit(());
    };
    let ms = to_ms(duration);
    let _handle = js! {
        var callback = @{callback};
        var action = function() {
            callback();
            callback.drop();
        };
        var delay = @{ms};
        return {
            timeout_id: setTimeout(action, delay),
            callback: callback,
        };
    };
}
