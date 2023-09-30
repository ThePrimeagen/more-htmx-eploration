use askama::Template;

#[derive(Clone, PartialEq)]
pub enum TimingType {
    Movement,
    Consume,
}

#[derive(Clone)]
pub struct Timing {
    pub timing_type: TimingType,
	pub start: u64,
	pub stop: u64,
	pub id: usize,
}

#[derive(Template)]
#[template(path = "timer.html")]
pub struct Timer {
    pub oob: bool,
    pub msg: String
}


#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub timings: Vec<Timing>,
}


