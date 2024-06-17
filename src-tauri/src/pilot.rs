#[derive(Debug, Default)]
pub enum BatteryBenchState {
	#[default]
	Standby,
	Charge,
	Discharge
}

#[derive(Debug)]
pub enum CompletionStatus {
	Success,
	Fail,
	InProgress
}

// TODO: Create/Implement the struct that holds all information about a given battery bench. Battery ID, voltage, current, temperature, bench temperature, bench state, electronic load temperature, qualification status, and USB port it's connected to.
#[derive(Debug)]
pub struct BatteryBench;

impl BatteryBench {
	pub fn new() -> Result<BatteryBench, &'static str> {
		todo!()
	}
	
	pub fn start_sequence(&mut self) {
		todo!()
	}
	
	pub fn complete_sequence_step(&mut self) {
		todo!()
	}
	
	pub fn complete_sequence(&mut self) {
		todo!()
	}
}
