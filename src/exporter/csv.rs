//! Exporter `CSV`
//!
//!

use exporter::types::HistoryTypes;
use std::fs::OpenOptions;
use std::io::{Write, BufWriter};
use std::path::Path;


pub fn writer(htype: &str, data: &[HistoryTypes]) -> Result<(), String> {
    let file: &str = &format!("data_{}.csv", htype);
    let path = Path::new(file);

    let file = match OpenOptions::new().append(true).create(true).open(&path) {
        Ok(f) => f,
        Err(e) => return Err(format!("Opening file {:?} - {}", path, e)),
    };

    let mut wtr = BufWriter::new(&file);

    for data_type in data {
        match *data_type {
            HistoryTypes::Float(m) => {
                if let Err(e) = wtr.write_fmt(format_args!("{},{},{},{}\n", m.itemid, m.clock, m.ns, m.value)) {
                    return Err(format!("CSV Writer error {:?} ({:?}) - {}", path, m, e));
                };
            },
            HistoryTypes::Integer(m) => {
                if let Err(e) = wtr.write_fmt(format_args!("{},{},{},{}\n", m.itemid, m.clock, m.ns, m.value)) {
                    return Err(format!("CSV Writer error {:?} ({:?}) - {}", path, m, e));
                };
            },
            HistoryTypes::String(m) => {
                if let Err(e) = wtr.write_fmt(format_args!("{},{},{},{}\n", m.itemid, m.clock, m.ns, m.value().unwrap())) {
                    return Err(format!("CSV Writer error {:?} ({:?}) - {}", path, m, e));
                };
            },
            HistoryTypes::Text(m) => {
                if let Err(e) = wtr.write_fmt(format_args!("{},{},{},{}\n", m.itemid, m.clock, m.ns, m.value().unwrap())) {
                    return Err(format!("CSV Writer error {:?} ({:?}) - {}", path, m, e));
                };
            },
            HistoryTypes::Log(m) => {
                if let Err(e) = wtr.write_fmt(format_args!("{},{},{},{},{},{},{},{}\n", m.itemid, m.clock, m.ns, m.value().unwrap(), m.source().unwrap(), m.timestamp, m.logeventid, m.severity)) {
                    return Err(format!("CSV Writer error {:?} ({:?}) - {}", path, m, e));
                };
            },
        }
    }

    if let Err(e) = wtr.flush() {
        return Err(format!("Flushing file {:?} - {}", path, e));
    };

    Ok(())
}
