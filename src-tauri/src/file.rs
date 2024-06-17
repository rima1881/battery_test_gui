use crate::pilot::BatteryBench;

/// Logs all values associated to a battery bench to a csv file.
///
/// The values are logged into a file named after the ID of the battery. If the file exists, the values are appended
///
/// # Arguments
///
/// * `battery_bench` - A BatteryBench object holding information about a battery bench.
///
/// # Returns
///
/// A `Result<(), &'static str>` an error message if the logging failed.
pub fn log_battery(battery_bench: BatteryBench) -> Result<(),  &'static str> {
	todo!()
}
