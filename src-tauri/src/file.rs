use crate::pilot::{BatteryBench, BatteryBenchState, CompletionStatus};
use rusqlite::{params, Connection, Result};
use chrono::{DateTime, Utc};

use std::fs::File;
use std::io::Write;
use csv::Writer;

/// Generates a CSV file from the SQLite database "battery_logs.db".
///
/// # Arguments
///
/// * `conn` - A reference to the SQLite database connection.
/// * `csv_path` - The path where the CSV file will be saved.
///
/// # Returns
///
/// A `Result<(), String>` indicating success or failure.
pub fn export_to_csv(conn: &Connection, csv_path: &str) -> Result<(), String> {
    // Prepare the SQL query to select all rows from the battery_logs table.
    let mut stmt = conn.prepare("SELECT * FROM battery_logs")
        .map_err(|err| format!("Failed to prepare query: {}", err))?;
    
    // Query the database and map rows to a tuple.
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, i64>(3)?,
            row.get::<_, i64>(4)?,
            row.get::<_, i64>(5)?,
            row.get::<_, i64>(6)?,
            row.get::<_, i64>(7)?,
            row.get::<_, String>(8)?,
            row.get::<_, String>(9)?,
            row.get::<_, String>(10)?,
            row.get::<_, String>(11)?,
        ))
    }).map_err(|err| format!("Failed to query database: {}", err))?;

    // Create a CSV writer that writes to the specified file path.
    let mut wtr = Writer::from_path(csv_path)
        .map_err(|err| format!("Failed to create CSV writer: {}", err))?;
    
    // Write the header record to the CSV file.
    wtr.write_record(&["record_id", "id", "port", "temperature", "battery_temperature", "electronic_load_temperature", "voltage", "current", "state", "status", "start_date", "end_date"])
        .map_err(|err| format!("Failed to write CSV header: {}", err))?;
    
    // Iterate over the rows and write each record to the CSV file.
    for row in rows {
        let row = row.map_err(|err| format!("Failed to fetch row: {}", err))?;
        wtr.write_record(&[
            row.0.to_string(),
            row.1.to_string(),
            row.2,
            row.3.to_string(),
            row.4.to_string(),
            row.5.to_string(),
            row.6.to_string(),
            row.7.to_string(),
            row.8,
            row.9,
            row.10,
            row.11,
        ]).map_err(|err| format!("Failed to write CSV record: {}", err))?;
    }

    // Flush the writer to ensure all data is written to the file.
    wtr.flush().map_err(|err| format!("Failed to flush CSV writer: {}", err))?;
    Ok(())
}

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

// Testing using a databse instead of a CSV file, might go back to csv if fails.
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
                record_id INTEGER PRIMARY KEY AUTOINCREMENT,
                id INTEGER NOT NULL,
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