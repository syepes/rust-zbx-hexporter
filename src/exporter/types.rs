extern crate zbx;

// Zabbix History data types
pub enum HistoryTypes {
    Float(zbx::ZBX_HISTORY_FLOAT),
    Integer(zbx::ZBX_HISTORY_INTEGER),
    String(zbx::ZBX_HISTORY_STRING),
    Text(zbx::ZBX_HISTORY_TEXT),
    Log(zbx::ZBX_HISTORY_LOG),
}
