#![allow(unused_must_use, dead_code, unused_imports, unused_variables, unused_mut)]
#![feature(plugin)]
#![plugin(clippy)]

use std::{str, slice};
use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, Mutex};

use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

#[macro_use]
extern crate log;
extern crate log4rs;

#[macro_use]
extern crate lazy_static;

extern crate zbx;
extern crate csv;

extern crate influent;
use influent::create_client;
use influent::client::{Client, Credentials};
use influent::measurement::{Measurement, Value};


lazy_static! {
    static ref HQ: Mutex<HashMap<&'static str, Vec<Vec<u64>>>> = {
        Mutex::new(HashMap::new())
    };
}

enum HistoryTypes {
    Float(zbx::ZBX_HISTORY_FLOAT),
    Integer(zbx::ZBX_HISTORY_INTEGER),
}


#[no_mangle]
pub extern "C" fn zbx_module_api_version() -> i32 { zbx::ZBX_MODULE_API_VERSION_ONE }

#[no_mangle]
pub extern "C" fn zbx_module_init() -> i32 {
    log4rs::init_file("log4rs.toml", Default::default()).expect("Failed to initialize logger");
    zbx::ZBX_MODULE_OK
}

#[no_mangle]
pub extern "C" fn zbx_module_uninit() -> i32 {
    {
        let hq = HQ.lock().expect("Failed to obtain HQ lock");
        println!("DBG: Flush HQ.keys: {:?}", hq.keys().collect::<Vec<_>>());
        println!("DBG: Flush HQ.values: {:?}", hq.values().collect::<Vec<_>>());
    }
    zbx::ZBX_MODULE_OK
}


#[no_mangle]
#[allow(unused_variables)]
pub unsafe extern "C" fn dummy_history_float_cb(history: *const zbx::ZBX_HISTORY_FLOAT, history_num: i32) {
    let mut f = vec![];

    if !history.is_null() && history_num > 1 {
        let histories = slice::from_raw_parts(history, history_num as usize);
        //println!("DBG: dummy_history_float_cb: {} - {:?}", history_num, histories);

        for h in histories {
            let v = vec![h.itemid, h.clock as u64, h.ns as u64, h.value as u64];
            f.push(HistoryTypes::Float(*h))
        }

    } else {
        //println!("DBG: dummy_history_float_cb: {} - {:?}", history_num, *history);
        let v = vec![(*history).itemid, (*history).clock as u64, (*history).ns as u64, (*history).value as u64];
        f.push(HistoryTypes::Float(*history))
    }

    println!("DBG: dummy_history_float_cb: #{}", f.len());
    if let Err(e) = writer_csv("float", &f) {
        println!("DBG: dummy_history_float_cb: Err: {}", e);
    };
}


#[no_mangle]
#[allow(unused_variables)]
pub unsafe extern "C" fn dummy_history_integer_cb(history: *const zbx::ZBX_HISTORY_INTEGER, history_num: i32) {
    let mut f = vec![];

    if !history.is_null() && history_num > 1 {
        let histories = slice::from_raw_parts(history, history_num as usize);
        //println!("DBG: dummy_history_integer_cb: {} - {:?}", history_num, histories);
        for h in histories {
            let v = vec![h.itemid, h.clock as u64, h.ns as u64, h.value as u64];
            f.push(HistoryTypes::Integer(*h))
        }
    } else {
        //println!("DBG: dummy_history_integer_cb: {} - {:?}", history_num, *history);
        let v = vec![(*history).itemid, (*history).clock as u64, (*history).ns as u64, (*history).value as u64];
        f.push(HistoryTypes::Integer(*history))
    }

    println!("DBG: dummy_history_integer_cb: #{}", f.len());
    if let Err(e) = writer_csv("integer", &f) {
        println!("DBG: dummy_history_integer_cb: Err: {}", e);
    };
}

#[no_mangle]
#[allow(unused_variables)]
pub unsafe extern "C" fn dummy_history_string_cb(history: *const zbx::ZBX_HISTORY_STRING, history_num: i32) {
    if !history.is_null() && history_num > 1 {
        let histories = slice::from_raw_parts(history, history_num as usize);
        println!("DBG: dummy_history_string_cb: {} - {:?}", history_num, histories);
    } else {
        println!("DBG: dummy_history_string_cb: {} - {:?}", history_num, *history);
    }
}

#[no_mangle]
#[allow(unused_variables)]
pub unsafe extern "C" fn dummy_history_text_cb(history: *const zbx::ZBX_HISTORY_TEXT, history_num: i32) {
    if !history.is_null() && history_num > 1 {
        let histories = slice::from_raw_parts(history, history_num as usize);
        println!("DBG: dummy_history_text_cb: {} - {:?}", history_num, histories);
    } else {
        println!("DBG: dummy_history_text_cb: {} - {:?}", history_num, *history);
    }
}

#[no_mangle]
#[allow(unused_variables)]
pub unsafe extern "C" fn dummy_history_log_cb(history: *const zbx::ZBX_HISTORY_LOG, history_num: i32) {
    if !history.is_null() && history_num > 1 {
        let histories = slice::from_raw_parts(history, history_num as usize);
        println!("DBG: dummy_history_log_cb: {} - {:?}", history_num, histories);
    } else {
        println!("DBG: dummy_history_log_cb: {} - {:?}", history_num, *history);
    }
}

#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn zbx_module_history_write_cbs() -> zbx::ZBX_HISTORY_WRITE_CBS {
    zbx::ZBX_HISTORY_WRITE_CBS {
        history_float_cb: Some(dummy_history_float_cb),
        history_integer_cb: Some(dummy_history_integer_cb),
        history_string_cb: Some(dummy_history_string_cb),
        history_text_cb: Some(dummy_history_text_cb),
        history_log_cb: Some(dummy_history_log_cb),
    }
}


/*
   fn writer_csv(htype: &str, records: &[u64]) {
   let path = Path::new("westerns.csv");

   let mut wtr = csv::Writer::from_file("/tmp/csv");
//let mut wtr = csv::Writer::from_memory();
for record in records.into_iter() {
let result = wtr.encode(record);
println!("{:?}: {:?}", result, wtr.as_string());
}
}
*/

fn writer_csv(htype: &str, data: &[HistoryTypes]) -> Result<(), String> {
    let file: &str = &format!("data_{}.csv", htype);
    let path = Path::new(file);

    let mut file = match OpenOptions::new().append(true).create(true).open(&path) {
        Ok(f)  => f,
        Err(e) => return Err(format!("Opening file {:?} - {}", path, e)),
    };

    let mut wtr = csv::Writer::from_memory();
    for r in data.into_iter() {
        match *r {
            HistoryTypes::Float(m) => {
                if let Err(e) = wtr.encode(m) {
                    return Err(format!("CSV Writer error {:?} ({:?}) - {}", path, m, e))
                };
            },
            HistoryTypes::Integer(m) => {
                if let Err(e) = wtr.encode(m) {
                    return Err(format!("CSV Writer error {:?} ({:?}) - {}", path, m, e))
                };
            },
        }
    }

    let str_res = wtr.as_string();
    //println!("Encoded output: {}", str_res);

    // Easiest way to get a &str into &[u8] is to go &str->String->Vec[u8]->&[u8]
    file.write_all(&str_res.to_string().into_bytes());
    file.flush();

    Ok(())
}

/*
   fn writer_influx(htype: &str, records: &[Vec<u64>]) {
// prepare client
let hosts = vec!["http://localhost:8086"];
let credentials = Credentials {
username: "gobwas",
password: "xxx",
database: "mydb"
};

match htype {
"integer" => {
let mut measurements: Vec<Measurement> = vec![];

for v in records.iter() {
let mut m = Measurement::new("history_integer");
m.add_field("value", Value::Float(v[2] as f64) );
m.add_field("ns", Value::Integer(v[2] as i64) );
m.set_timestamp(v[1] as i64);
//m.add_tag("itemid",  format!("{}",v[0]).as_str() );
//m.add_tag("itemid", Value::String( t ) );
measurements.push(m);
}
},
_ => { println!("Noo"); }
}

}

*/
