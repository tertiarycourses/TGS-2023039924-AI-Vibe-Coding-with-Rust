// Sample: an enum with data and an exhaustive match with a guard.
enum Movement {
    Receive(u32),
    Ship(u32),
    Adjust(i64),
}

fn describe(movement: &Movement) -> String {
    match movement {
        Movement::Receive(qty) => format!("receive {qty}"),
        Movement::Ship(qty) if *qty > 100 => {
            format!("ship {qty} (large order, needs approval)")
        }
        Movement::Ship(qty) => format!("ship {qty}"),
        Movement::Adjust(delta) => format!("adjust by {delta:+}"),
    }
}

fn main() {
    let moves = [
        Movement::Receive(10),
        Movement::Ship(3),
        Movement::Ship(250),
        Movement::Adjust(-2),
    ];
    for movement in &moves {
        println!("{}", describe(movement));
    }
}
