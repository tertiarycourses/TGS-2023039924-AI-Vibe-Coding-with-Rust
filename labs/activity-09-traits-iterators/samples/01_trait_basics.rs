// Sample: one trait, two types, generic and dynamic dispatch.
trait Priced {
    fn unit_cents(&self) -> u64;
    fn quantity(&self) -> u32;
    fn line_cents(&self) -> u64 {
        self.unit_cents() * u64::from(self.quantity())
    }
}

struct Product {
    unit_cents: u64,
    qty: u32,
}

struct Service {
    rate_cents: u64,
    hours: u32,
}

impl Priced for Product {
    fn unit_cents(&self) -> u64 {
        self.unit_cents
    }
    fn quantity(&self) -> u32 {
        self.qty
    }
}

impl Priced for Service {
    fn unit_cents(&self) -> u64 {
        self.rate_cents
    }
    fn quantity(&self) -> u32 {
        self.hours
    }
}

fn show<T: Priced>(label: &str, line: &T) {
    println!("{label}: {} cents", line.line_cents());
}

fn main() {
    let cable = Product {
        unit_cents: 899,
        qty: 2,
    };
    let setup = Service {
        rate_cents: 5000,
        hours: 2,
    };
    show("cable", &cable);
    show("setup", &setup);
    let order: Vec<&dyn Priced> = vec![&cable, &setup];
    let total: u64 = order.iter().map(|line| line.line_cents()).sum();
    println!("mixed order total: {total} cents");
}
