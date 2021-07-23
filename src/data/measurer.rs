pub struct Measurer {
    id: i64,
    measurer_type: MeasurerType,
    user_id: i64,
}

pub enum MeasurerType {
    Hub,
    Farm,
    Rack,
}

pub struct Hub {
    id: i64,
    farms: Vec<Growable>,
}

pub enum Growable {
    Tomato,
    Cherry,
    Strawberry,
    EggPlant,
}
