pub mod sheet {
    #[derive(Debug)]
    pub struct Sheet {
        name: String,
        level: i64,
        race: String,
        class: String,
        stats: Vec<i64>,
    }
}

