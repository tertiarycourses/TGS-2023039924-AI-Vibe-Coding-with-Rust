// Sample: nested modules, super::, self:: and use ... as.
mod stockpilot {
    pub mod inventory {
        pub fn count() -> u32 {
            super::defaults::STARTING_STOCK + 2
        }
    }

    mod defaults {
        pub const STARTING_STOCK: u32 = 10;
    }

    pub fn summary() -> String {
        format!("stock count = {}", self::inventory::count())
    }
}

use stockpilot::inventory::count as stock_count;

fn main() {
    println!("{}", stockpilot::summary());
    println!("via use-as: {}", stock_count());
}
