#![feature(plugin)]
#![plugin(clippy)]

use std::{str, slice};

#[macro_use]
extern crate log;
extern crate log4rs;

extern crate zbx;

mod exporter;
use exporter::types::HistoryTypes;


#[no_mangle]
pub extern "C" fn zbx_module_api_version() -> i32 { zbx::ZBX_MODULE_API_VERSION }

#[no_mangle]
pub extern "C" fn zbx_module_init() -> i32 {
    log4rs::init_file("log4rs.toml", Default::default()).expect("module [hexporter]: func [zbx_module_init]: Failed to initialize logger");
    info!("module [hexporter]: func [zbx_module_init]: OK");
    zbx::ZBX_MODULE_OK
}

#[no_mangle]
pub extern "C" fn zbx_module_uninit() -> i32 {
    info!("module [hexporter]: func [zbx_module_uninit]: OK");
    zbx::ZBX_MODULE_OK
}

#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn zbx_module_history_write_cbs() -> zbx::ZBX_HISTORY_WRITE_CBS {
    info!("module [hexporter]: func [zbx_module_history_write_cbs]: OK");
    zbx::ZBX_HISTORY_WRITE_CBS { history_float_cb: Some(history_float_cb),
                                 history_integer_cb: Some(history_integer_cb),
                                 history_string_cb: Some(history_string_cb),
                                 history_text_cb: Some(history_text_cb),
                                 history_log_cb: Some(history_log_cb), }
}


#[no_mangle]
#[allow(unused_variables)]
pub unsafe extern "C" fn history_float_cb(history: *const zbx::ZBX_HISTORY_FLOAT, history_num: i32) {
    let mut buffer = vec![];

    if !history.is_null() && history_num > 1 {
        let histories = slice::from_raw_parts(history, history_num as usize);
        trace!("module [hexporter]: func [history_float_cb]: {} - {:?}", history_num, histories);

        for h in histories {
            buffer.push(HistoryTypes::Float(*h))
        }

    } else {
        trace!("module [hexporter]: func [history_float_cb]: {} - {:?}", history_num, *history);
        buffer.push(HistoryTypes::Float(*history))
    }

    debug!("module [hexporter]: func [history_float_cb]: #{}", buffer.len());
    if let Err(e) = exporter::csv::writer("float", &buffer) {
        error!("module [hexporter]: func [history_float_cb]: {}", e);
    };
}

#[no_mangle]
#[allow(unused_variables)]
pub unsafe extern "C" fn history_integer_cb(history: *const zbx::ZBX_HISTORY_INTEGER, history_num: i32) {
    let mut buffer = vec![];

    if !history.is_null() && history_num > 1 {
        let histories = slice::from_raw_parts(history, history_num as usize);
        trace!("module [hexporter]: func [history_integer_cb]: {} - {:?}", history_num, histories);
        for h in histories {
            buffer.push(HistoryTypes::Integer(*h))
        }
    } else {
        trace!("module [hexporter]: func [history_integer_cb]: {} - {:?}", history_num, *history);
        buffer.push(HistoryTypes::Integer(*history))
    }

    debug!("module [hexporter]: func [history_integer_cb]: #{}", buffer.len());
    if let Err(e) = exporter::csv::writer("integer", &buffer) {
        error!("module [hexporter]: func [history_integer_cb]: {}", e);
    };
}

#[no_mangle]
#[allow(unused_variables)]
pub unsafe extern "C" fn history_string_cb(history: *const zbx::ZBX_HISTORY_STRING, history_num: i32) {
    let mut buffer = vec![];

    if !history.is_null() && history_num > 1 {
        let histories = slice::from_raw_parts(history, history_num as usize);
        trace!("module [hexporter]: func [history_string_cb]: {} - {:?}", history_num, histories);
        for h in histories {
            buffer.push(HistoryTypes::String(*h));
        }
    } else {
        trace!("module [hexporter]: func [history_string_cb]: {} - {:?}", history_num, *history);
        buffer.push(HistoryTypes::String(*history));
    }

    debug!("module [hexporter]: func [history_string_cb]: #{}", buffer.len());
    if let Err(e) = exporter::csv::writer("string", &buffer) {
        error!("module [hexporter]: func [history_string_cb]: {}", e);
    };
}

#[no_mangle]
#[allow(unused_variables)]
pub unsafe extern "C" fn history_text_cb(history: *const zbx::ZBX_HISTORY_TEXT, history_num: i32) {
    let mut buffer = vec![];

    if !history.is_null() && history_num > 1 {
        let histories = slice::from_raw_parts(history, history_num as usize);
        trace!("module [hexporter]: func [history_text_cb]: {} - {:?}", history_num, histories);
        for h in histories {
            buffer.push(HistoryTypes::Text(*h));
        }
    } else {
        trace!("module [hexporter]: func [history_text_cb]: {} - {:?}", history_num, *history);
        buffer.push(HistoryTypes::Text(*history));
    }

    debug!("module [hexporter]: func [history_text_cb]: #{}", buffer.len());
    if let Err(e) = exporter::csv::writer("text", &buffer) {
        error!("module [hexporter]: func [history_text_cb]: {}", e);
    };
}

#[no_mangle]
#[allow(unused_variables)]
pub unsafe extern "C" fn history_log_cb(history: *const zbx::ZBX_HISTORY_LOG, history_num: i32) {
    let mut buffer = vec![];

    if !history.is_null() && history_num > 1 {
        let histories = slice::from_raw_parts(history, history_num as usize);
        trace!("module [hexporter]: func [history_log_cb]: {} - {:?}", history_num, histories);
        for h in histories {
            buffer.push(HistoryTypes::Log(*h));
        }
    } else {
        trace!("module [hexporter]: func [history_log_cb]: {} - {:?}", history_num, *history);
        buffer.push(HistoryTypes::Log(*history));
    }

    debug!("module [hexporter]: func [history_log_cb]: #{}", buffer.len());
    if let Err(e) = exporter::csv::writer("text", &buffer) {
        error!("module [hexporter]: func [history_log_cb]: {}", e);
    };
}
