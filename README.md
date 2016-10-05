## rust-zbx-hexporter

## Functionality
`rust-zbx-hexporter` is a [loadable module](https://www.zabbix.com/documentation/3.4/manual/config/items/loadablemodules) that uses the [Zabbix](https://www.zabbix.com) callbacks [ZBXNEXT-3353](https://support.zabbix.com/browse/ZBXNEXT-3353) to expose all the received historical item data

The main goal will be to offload all the stored data of the `history*` tables to a more suited solution

## Features
 - Export CSV

## Usage
    git clone https://github.com/syepes/rust-zbx-hexporter.git && cd rust-zbx-hexporter
    cargo build --release (nightly)
    cp target/release/libzbx_hexporter.so /usr/lib/zabbix/modules/ `LoadModulePath=`
    cp -r cfg/log4rs.toml /etc/zabbix/
    vi /etc/zabbix/zabbix_server.cfg `LoadModule=libzbx_hexporter.so`
    restart zabbix_server

## Todo
- [ ] Add configuration file: Paths, Buffer Size, Select exporter...
- [ ] Find a way to correctly buffer data on each of the forked process Zabbix `StartDBSyncers=`
- [ ] Resolve [ZBX-11295]( https://support.zabbix.com/browse/ZBX-11295) to be able to flush the buffered data before unloading the module `zbx_module_uninit`

## Note
Very early POC version that currently only supports exporting data to CSV, but will be working on also adding some TSDB or NoSQL exporters

