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
    WrongComponentCount,
    EmptyComponent,
    InvalidComponent,
    OutOfRangeComponent,
}

impl TryFrom<EndpointText> for Endpoint {
    type Error = ParseEndpointError;

    fn try_from(value: EndpointText) -> Result<Self, Self::Error> {
        let address_component: Vec<&str> = value.address.split('.').collect();
        if address_component.len() != 4 {
            return Err(Self::Error::WrongComponentCount);
        }

        let mut ep: Endpoint = Endpoint::default();
        ep.port = value.port;
        
        for i in 0..4 {
            if address_component[i].is_empty() {
                return Err(Self::Error::EmptyComponent);
            }
            if !address_component[i].bytes().all(|x| x.is_ascii_digit()) {
                return Err(Self::Error::InvalidComponent);
            }
            match address_component[i].parse::<u8>() {
                Ok(v) => {
                    ep.address[i] = v;
                },
                _ => {
                    return Err(Self::Error::OutOfRangeComponent);
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

        let et_min_into: EndpointText = Endpoint{
            address: [0,0,0,0],
            port: 0,
        }.into();
        assert_eq!(et_min_into.address, "0.0.0.0".to_string());
        assert_eq!(et_min_into.port, 0);

        let et_max = EndpointText::from(Endpoint{
            address: [255,255,255,255],
            port: 65535,
        });
        assert_eq!(et_max.address, "255.255.255.255".to_string());
        assert_eq!(et_max.port, 65535);
        
        let et_max_into: EndpointText = Endpoint{
            address: [255,255,255,255],
            port: 65335,
        }.into();
        assert_eq!(et_max_into.address, "255.255.255.255".to_string());
        assert_eq!(et_max_into.port, 65335);

    }

    #[test]
    fn test_tryfrom_valid() {
        let t0 =
            Endpoint::try_from(
                EndpointText{address: "192.168.1.10".to_string(), port:0},
            );
        assert_eq!(t0, Ok(Endpoint{ address: [192,168,1,10], port:0}));

        let t1 =
            Endpoint::try_from(
                EndpointText{address: "001.002.030.004".to_string(), port:0},
            );
        assert_eq!(t1, Ok(Endpoint{ address: [1,2,30,4], port:0}));
    }
    
    #[test]
    fn test_tryfrom_invalid_dotted_quad() {
        for sample in ["", "1.2.3", "1.2.3.4.5"] {
            let t =
                Endpoint::try_from(
                    EndpointText{address:sample.to_string(), port:0},
                );
            assert_eq!(t, Err(ParseEndpointError::WrongComponentCount));
        }
    }

    #[test]
    fn test_tryfrom_empty_component() {
        for sample in ["0.0.0.", ".0.0.0", "1..xyz.4"] {
            let t =
                Endpoint::try_from(
                    EndpointText{address:sample.to_string(), port:0},
                );
            assert_eq!(t, Err(ParseEndpointError::EmptyComponent));
        }
    }

    #[test]
    fn test_tryfrom_invalid_component() {
        for sample in ["0.0.0.-1", "x.0.0.0", "0.0.x.0",
            "1.a.3.4", "1.-2.3.4"] {
            let t =
                Endpoint::try_from(
                    EndpointText{address:sample.to_string(), port:0},
                );
            assert_eq!(t, Err(ParseEndpointError::InvalidComponent));
        }
    }

    #[test]
    fn test_tryfrom_out_of_range_component() {
        for sample in ["1.256.3.4", "1.2.99999999999.4"] {
            let t =
                Endpoint::try_from(
                    EndpointText{address:sample.to_string(), port:0},
                );
            assert_eq!(t, Err(ParseEndpointError::OutOfRangeComponent));
        }
    }
}
