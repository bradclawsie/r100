#[derive(Debug, PartialEq, Eq)]
struct Endpoint {
    address: [u8; 4],
    port: u16,
}

impl Default for Endpoint {
    fn default() -> Self {
        Self {
            address: [0; 4],
            port: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct EndpointText {
    address: String,
    port: u16,
}

impl From<Endpoint> for EndpointText {
    fn from(endpoint: Endpoint) -> Self {
        Self {
            address: format!("{}.{}.{}.{}",
                endpoint.address[0],
                endpoint.address[1],
                endpoint.address[2],
                endpoint.address[3],
            ),
            port: endpoint.port,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParseEndpointError {
    InvalidAddress(String),
}

impl TryFrom<EndpointText> for Endpoint {
    type Error = ParseEndpointError;

    fn try_from(value: EndpointText) -> Result<Self, Self::Error> {
        let addr_elts: Vec<&str> = value.address.split('.').collect();
        if addr_elts.len() != 4 {
            return Err(Self::Error::InvalidAddress(
                format!(
                    "{} does not seem to be dotted quad",
                    value.address,
                ),
            ));
        }

        let mut ep: Endpoint = Endpoint::default();
        ep.port = value.port;
        
        for i in 0..4 {
            match addr_elts[i].parse::<u8>() {
                Ok(v) => {
                    ep.address[i] = v;
                },
                _ => {
                    return Err(Self::Error::InvalidAddress(
                        format!(
                            "{} elt {} ({}) is not u8",
                            value.address, i, addr_elts[i], 
                        ),
                    ));
                },
            } 
        }        
        Ok(ep)
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_default() {
        let et_min = EndpointText::from(Endpoint{
            address: [0,0,0,0],
            port: 0,
        });
        assert_eq!(et_min.address, "0.0.0.0".to_string());
        assert_eq!(et_min.port, 0);

        let et_max = EndpointText::from(Endpoint{
            address: [255,255,255,255],
            port: 65535,
        });
        assert_eq!(et_max.address, "255.255.255.255".to_string());
        assert_eq!(et_max.port, 65535);
    }

    #[test]
    fn test_tryfrom() {
        let t0 =
            Endpoint::try_from(
                EndpointText{address:"".to_string(), port:0},
            );

        match t0 {
            Err(ParseEndpointError::InvalidAddress(msg)) => {
                assert!(msg.contains("dotted quad"));
            }
            other => panic!("expected InvalidAddress error, got {other:?}"),
        }
    }
}
