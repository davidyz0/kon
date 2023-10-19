use std::sync::Mutex;

use crate::{
	fiber::{Fiber, Start},
	task::env::Global,
	trace
};

pub struct Pool {
	pool: Mutex<Vec<Fiber>>,
	count: u64
}

impl Global for Pool {}

impl Pool {
	pub const fn new() -> Self {
		Self { pool: Mutex::new(Vec::new()), count: 0 }
	}

	pub fn new_fiber(&mut self, start: Start) -> Fiber {
		let mut pool = self.pool.lock().unwrap();

		self.count += 1;

		match pool.pop() {
			None => (),
			Some(mut fiber) => {
				trace!(target: self, "== Reusing fiber");

				unsafe { fiber.set_start(start) }

				return fiber;
			}
		}

		Fiber::new(start)
	}

	fn calculate_ideal(count: u64) -> u64 {
		const RATIO: u64 = 20;

		count * RATIO / 100 + 16
	}

	pub fn exit_fiber(&mut self, fiber: Fiber) {
		let mut pool = self.pool.lock().unwrap();

		self.count -= 1;

		let ideal = Self::calculate_ideal(self.count);

		if pool.len() < ideal as usize {
			pool.push(fiber);
		}
	}
}
