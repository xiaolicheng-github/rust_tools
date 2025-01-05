use sysinfo::Networks;
use serde::Serialize;

#[derive(Serialize)]
pub struct NetReceivedTransmitted {
  received: Vec<u64>,
  transmitted: Vec<u64>,
  unit: String,
}

pub fn get_net_received_transmitted() -> NetReceivedTransmitted {
  let networks = Networks::new_with_refreshed_list();
  let mut received_list = vec![];
  let mut transmitted_list = vec![];
  for (_interface_name, network) in &networks {
    received_list.push(network.total_received());
    transmitted_list.push(network.total_transmitted());
  }
  return NetReceivedTransmitted {
    received: received_list,
    transmitted: transmitted_list,
    unit: "B".to_string()
  }
}