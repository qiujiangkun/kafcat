mod consume;
mod copy;
mod produce;

pub use consume::*;
pub use copy::*;
pub use produce::*;

use tokio::time::Duration;

fn get_delay(exit: bool) -> Duration {
    if exit {
        Duration::from_millis(3000)
    } else {
        Duration::from_secs(3600)
    }
}
