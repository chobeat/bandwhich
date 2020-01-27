use crate::network::{display_connection_string, display_ip_or_host, Connection, Utilization};
use ::std::collections::{BTreeMap, HashMap};
use ::std::net::Ipv4Addr;
use chrono::DateTime;
use chrono::Local;

pub trait Bandwidth {
    fn get_total_bytes_downloaded(&self) -> u128;
    fn get_total_bytes_uploaded(&self) -> u128;
}

#[derive(Default)]
pub struct NetworkData {
    pub total_bytes_downloaded: u128,
    pub total_bytes_uploaded: u128,
    pub connection_count: u128,
}

#[derive(Default)]
pub struct ConnectionData {
    pub total_bytes_downloaded: u128,
    pub total_bytes_uploaded: u128,
    pub process_name: String,
    pub interface_name: String,
}

impl Bandwidth for ConnectionData {
    fn get_total_bytes_uploaded(&self) -> u128 {
        self.total_bytes_uploaded
    }
    fn get_total_bytes_downloaded(&self) -> u128 {
        self.total_bytes_downloaded
    }
}

impl Bandwidth for NetworkData {
    fn get_total_bytes_uploaded(&self) -> u128 {
        self.total_bytes_uploaded
    }
    fn get_total_bytes_downloaded(&self) -> u128 {
        self.total_bytes_downloaded
    }
}

#[derive(Default)]
pub struct UIState {
    pub processes: BTreeMap<String, NetworkData>,
    pub remote_addresses: BTreeMap<Ipv4Addr, NetworkData>,
    pub connections: BTreeMap<Connection, ConnectionData>,
    pub total_bytes_downloaded: u128,
    pub total_bytes_uploaded: u128,
    pub ip_to_host: HashMap<Ipv4Addr, String>,
}

impl UIState {
    pub fn new(
        connections_to_procs: HashMap<Connection, String>,
        mut network_utilization: Utilization,
        ip_to_host: HashMap<Ipv4Addr, String>,
    ) -> Self {
        let mut processes: BTreeMap<String, NetworkData> = BTreeMap::new();
        let mut remote_addresses: BTreeMap<Ipv4Addr, NetworkData> = BTreeMap::new();
        let mut connections: BTreeMap<Connection, ConnectionData> = BTreeMap::new();
        let mut total_bytes_downloaded: u128 = 0;
        let mut total_bytes_uploaded: u128 = 0;
        for (connection, process_name) in connections_to_procs {
            if let Some(connection_info) = network_utilization.connections.remove(&connection) {
                let data_for_remote_address = remote_addresses
                    .entry(connection.remote_socket.ip)
                    .or_default();
                let connection_data = connections.entry(connection).or_default();
                let data_for_process = processes.entry(process_name.clone()).or_default();

                data_for_process.total_bytes_downloaded += connection_info.total_bytes_downloaded;
                data_for_process.total_bytes_uploaded += connection_info.total_bytes_uploaded;
                data_for_process.connection_count += 1;
                connection_data.total_bytes_downloaded += connection_info.total_bytes_downloaded;
                connection_data.total_bytes_uploaded += connection_info.total_bytes_uploaded;
                connection_data.process_name = process_name;
                connection_data.interface_name = connection_info.interface_name;
                data_for_remote_address.total_bytes_downloaded +=
                    connection_info.total_bytes_downloaded;
                data_for_remote_address.total_bytes_uploaded +=
                    connection_info.total_bytes_uploaded;
                data_for_remote_address.connection_count += 1;
                total_bytes_downloaded += connection_info.total_bytes_downloaded;
                total_bytes_uploaded += connection_info.total_bytes_uploaded;
            }
        }
        UIState {
            processes,
            remote_addresses,
            connections,
            total_bytes_downloaded,
            total_bytes_uploaded,
            ip_to_host,
        }
    }
}

impl ToString for UIState {
    fn to_string(&self) -> String {
        //let ip_to_host = &self.ip_to_host;
        let local_time: DateTime<Local> = Local::now();
        let timestamp = local_time.timestamp();
        let mut lines = Vec::<String>::new();
        for (process, process_network_data) in &self.processes {
            lines.push(format!(
                "process: <{}> \"{}\" up/down Bps: {}/{} connections: {}",
                timestamp,
                process,
                process_network_data.total_bytes_uploaded,
                process_network_data.total_bytes_downloaded,
                process_network_data.connection_count
            ));
        }
        for (connection, connection_network_data) in &self.connections {
            lines.push(format!(
                "connection: <{}> {} up/down Bps: {}/{} process: \"{}\"",
                timestamp,
                display_connection_string(
                    connection,
                    &self.ip_to_host,
                    &connection_network_data.interface_name
                ),
                connection_network_data.total_bytes_uploaded,
                connection_network_data.total_bytes_downloaded,
                connection_network_data.process_name
            ));
        }
        for (remote_address, remote_address_network_data) in &self.remote_addresses {
            lines.push(format!(
                "remote_address: <{}> {} up/down Bps: {}/{} connections: {}",
                timestamp,
                display_ip_or_host(*remote_address, &self.ip_to_host),
                remote_address_network_data.total_bytes_uploaded,
                remote_address_network_data.total_bytes_downloaded,
                remote_address_network_data.connection_count
            ));
        }

        lines.join("\n")
    }
}
