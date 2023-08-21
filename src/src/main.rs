use std::fmt::Debug;
use std::path::Path;
use ubus::Connection;
use ubus::IO;
use ubus::BlobMsgData;

fn get_obj_id<T: IO>(connection: &mut Connection<T>, name: &str) -> Option<u32> where <T as IO>::Error: Debug {
	let mut result = None;

	connection.lookup(
		|obj| {
			if name == obj.path {
				result = Some(obj.id);
			}
		},
		|_| {}
	).unwrap();

	return result;
}

fn main() {
	let socket = Path::new("/var/run/ubus.sock");

	let mut connection = match Connection::connect(&socket) {
		Ok(connection) => connection,
		Err(error) => {
			eprintln!("{}: Failed to open ubus socket. {}", socket.display(), error);
			return;
		}
	};

	match get_obj_id(&mut connection, "system") {
		Some(id) => connection.invoke(id, "info", &[], |info| {
			for item in info {
				if (item.name == Some("uptime")) {
					match item.data {
						BlobMsgData::Int32(uptime) => println!("uptime: {}", uptime),
						_ => {}
					};
				}
			}
		}).unwrap(),
		None => {
			eprintln!("Failed to get object id");
			return;
		}
	};
}
