use crate::pilot::{BatteryBench, BatteryBenchState, CompletionStatus};
use rusqlite::{params, Connection, Result};
use chrono::{DateTime, Utc};

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
/// 
/// Testing using a databse instead of a CSV file, might go back to csv if fails:
// Logs battery data into the SQLite database.
pub fn log_battery(conn: &Connection, battery_bench: BatteryBench) -> Result<(), &'static str> {
    // Open or create the SQLite database
    //let conn = Connection::open("battery_logs.db").map_err(|_| "Failed to open database")?;

    // Convert enums to their corresponding string representations
    let state_str = match battery_bench.state {
        crate::pilot::BatteryBenchState::Standby => "Standby",
        crate::pilot::BatteryBenchState::Charge => "Charge",
        crate::pilot::BatteryBenchState::Discharge => "Discharge",
    };

    let status_str = match battery_bench.status {
        crate::pilot::CompletionStatus::Success => "Success",
        crate::pilot::CompletionStatus::Fail => "Fail",
        crate::pilot::CompletionStatus::InProgress => "InProgress",
    };

    // Execute the SQL INSERT statement
    conn.execute(
        "INSERT INTO battery_logs (
            id, port, temperature, battery_temperature,
            electronic_load_temperature, voltage, current,
            state, status, start_date, end_date
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            battery_bench.id,
            battery_bench.port,
            battery_bench.temperature,
            battery_bench.battery_temperature,
            battery_bench.electronic_load_temperature,
            battery_bench.voltage,
            battery_bench.current,
            state_str,
            status_str,
            battery_bench.start_date.to_rfc3339(),
            battery_bench.end_date.to_rfc3339(),
        ],
    ).map_err(|err| {
		eprintln!("Failed to insert data into database: {}", err);
		"Failed to insert data into database"
	})?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::OpenFlags;

    #[test]
    fn test_log_battery() {
        // Setup: Initialize an in-memory SQLite database connection
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");

		// Create the table (if it doesn't exist)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS battery_logs (
                id INTEGER PRIMARY KEY,
                port TEXT NOT NULL,
                temperature INTEGER NOT NULL,
                battery_temperature INTEGER NOT NULL,
                electronic_load_temperature INTEGER NOT NULL,
                voltage INTEGER NOT NULL,
                current INTEGER NOT NULL,
                state TEXT NOT NULL,
                status TEXT NOT NULL,
                start_date TEXT,
                end_date TEXT
            )",
            [],
        ).expect("Failed to create table");

        // Example BatteryBench object for testing
        let battery_bench = BatteryBench {
            id: 99,
            port: "COM4".to_string(),
            temperature: 25,
            battery_temperature: 30,
            electronic_load_temperature: 27,
            voltage: 12,
            current: 1,
            state: BatteryBenchState::Charge,
            status: CompletionStatus::InProgress,
            start_date: Utc::now(),
            end_date: Utc::now(),
        };

        // Exercise: Call the function under test
        let result = log_battery(&conn, battery_bench.clone());  // Only pass the BatteryBench object

        // Assertion: Check the result
    	assert!(result.is_ok(), "Failed to log battery data: {:?}", result.err());

    	// Verification: Query the database to verify the inserted data
    	let mut stmt = conn.prepare("SELECT * FROM battery_logs WHERE id = ?1").expect("Failed to prepare statement");
    	let rows = stmt.query_map(params![battery_bench.id], |row| {
        	Ok(())
    	}).expect("Failed to query database");

    	for row in rows {
        	println!("Row found: {:?}", row);
    	}
    }
}