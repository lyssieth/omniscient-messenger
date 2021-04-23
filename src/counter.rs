pub struct Counter {
    amount: i64,
}

impl Counter {
    pub const fn new(amt: i64) -> Counter {
        Counter { amount: amt }
    }
}
