// Sample: modules and privacy in one file.
mod money {
    pub fn format_cents(cents: u64) -> String {
        format!("${}.{:02}", cents / 100, cents % 100)
    }

    fn secret_rule() -> &'static str {
        "private: only code inside `money` can call this"
    }

    pub fn explain() -> &'static str {
        secret_rule()
    }
}

mod report {
    use crate::money::format_cents; // path from the crate root

    pub fn line(name: &str, qty: u32, unit: u64) -> String {
        let total = unit * u64::from(qty);
        format!(
            "{name} x{qty} @ {} = {}",
            format_cents(unit),
            format_cents(total)
        )
    }
}

fn main() {
    println!("{}", report::line("Cable", 3, 250));
    println!("{}", money::explain());
    // money::secret_rule(); // error[E0603]: function `secret_rule` is private
}
