use ipnetwork::IpNetwork;
use rtnetlink::{new_connection, Error, Handle};
use std::net::IpAddr;

pub struct Bridge {
    pub name: String,
    pub gateway: IpNetwork,
}

#[allow(dead_code)]
impl Bridge {
    pub fn new() -> Bridge {
        Bridge {
            name: "vas-quoad-container-bridge".to_string(),
            ip: "172.0.0.1".parse().unwrap(),
        }
    }

    pub fn add_bridge(&self) -> Result<(), ()> {
        let (connection, handle, _) = new_connection().unwrap();
        handle
            .link()
            .add()
            .bridge(self.name.into())
            .execute()
            .await
            .map_err(|e| format!("{}", e))
    }

    pub fn add_gateway_address() -> Result<(), Error> {
        let mut links = handle
            .link()
            .get()
            .set_name_filter(self.name.to_string())
            .execute();
        if let Some(link) = links.try_next().await? {
            handle
                .address()
                .add(link.header.index, self.ip.ip(), self.ip.prefix())
                .execute()
                .await?
        }
        Ok(())
    }
}
