use askama::Template;

pub struct Timing {
    pub timing_type: &'static str,
	pub start: u64,
	pub stop: u64,
	pub id: i32,
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


