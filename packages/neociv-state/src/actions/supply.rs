pub struct SupplyActionSimple {
    from: String,
    item: String,
    amount: u64,
}

pub struct SupplyActionAdvanced {
    from: String,
    callback: String,
}

pub enum SupplyAction {
    SupplyActionSimple,
    SupplyActionAdvanced,
}