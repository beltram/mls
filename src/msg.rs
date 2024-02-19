use crate::read_stdin;
use base64::Engine;
use bat::PagingMode;

pub fn decode_msg(draft: u8) {
    let msg = read_stdin();
    let msg = base64::prelude::BASE64_STANDARD
        .decode(msg)
        .expect("Invalid base64 message");
    let msg = decode(draft, msg);
    bat::PrettyPrinter::new()
        .input_from_bytes(msg.as_bytes())
        .colored_output(true)
        .true_color(false)
        .paging_mode(PagingMode::Always)
        .language("json")
        .print()
        .unwrap();
}

fn decode(draft: u8, msg: Vec<u8>) -> String {
    match draft {
        20 => {
            use openmls::prelude::TlsDeserializeTrait as _;

            if let Ok(msg) = openmls::prelude::MlsMessageIn::tls_deserialize(&mut msg.as_slice()) {
                format!("{msg:#?}")
            } else if let Ok(kp) =
                openmls::prelude::KeyPackageIn::tls_deserialize(&mut msg.as_slice())
            {
                println!("Not a MLS message, trying KeyPackage");
                format!("{kp:#?}")
            } else if let Ok(vgi) =
                openmls::prelude::group_info::VerifiableGroupInfo::tls_deserialize(
                    &mut msg.as_slice(),
                )
            {
                println!("Neither a MLS message nor a KeyPackage, trying GroupInfo");
                let gi: openmls::prelude::group_info::GroupInfo = vgi.into();
                format!("{gi:#?}")
            } else {
                panic!("Could not parse given input")
            }
        }
        _ => panic!("Unsupported draft version {draft}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parse_keypackage_20() {
        let msg = "AAEAASB9obtu2o2OX3DOIrxhJOpa0Ae0F5ECRec0IDFTRl8qKiDj+8G+uD6epHAkRX4/8QgKRtwty79XfPZSGQAP01PMTSC1tAaAMzL79uVcZ85ClmVtWwB2fYXfM5RdcLlQA8tpfgABQE4yNThiYjdiYy0yMTU4LTRiMDMtOWJlNS02NTE4NzEwYmEyZTc6ZDExNjE3N2NhYjNhNDUyMEBuaW5qYXMuZG9nZm9vZC53aXJlLmxpbmsCAAEIAAEAAgAHAAMCAAEMAAEAAgADAAQABQAHBAABAAIBAAAAAGR57soAAAAAZOi62gBAQJcsXjFYfJgmlIouO/HlRz4p7OdHPRc/nPgUM3MRfdp6eQXLLT5dRh82Kd+H+PHZtrnRbnPIPZRwo+S0SjKk2AYAQEBuV7hWf5J6unW6Uu3uw0qsrEfan9w4I2Fr6CZ34HgQF/rHBfj8C1b2tl8WcLmYMKolSE8EZuLWmoLer9o8tMgK";
        let msg = base64::prelude::BASE64_STANDARD.decode(msg).unwrap();
        println!("{}", decode(20, msg));
    }

    #[test]
    fn check_group_info_20() {
        let msg = "AAEAATEAAAAAAAAAAZsb6Oy/eF28nnEZ5uU3b8sAbmluamFzLmRvZ2Zvb2Qud2lyZS5saW5rAAAAAAAAAAEg6mIm/G1SP0mluA/FtHhYanZSjkMhNGLXUx/jwbfvz4sgVAXP+OAFE8vZrnWGU7fKUakup75OqhZV/tZObQVoOsQ5AAMDAAAAAAUwLyBlsjGgMGAtsMDhjdjc6nGNnLPRf0pFNEj3bMaqeM8jRwABC3dpcmUtc2VydmVyQSYAAkD+QPwBASBiPv/6XnZP+HPZu7pCGifC5MWp6p4gZWnJ80NB2kEIaiCyuLpSh3zgvsfZUvMSORlj1j+vdn5GgUMTV6/rZlDMHAABQE5lODc3NzUxYi0xZWNlLTQzZGQtODFkOS0xMDVmY2EwNmM2YzI6MTM0NjY1MDI4M2Y3ZTcxOEBuaW5qYXMuZG9nZm9vZC53aXJlLmxpbmsCAAEIAAEAAgAHAAMCAAEMAAEAAgADAAQABQAHBAABAAIDAABAQMGLlBUDvohO7ofAXT5n2D1+XNy9im2rgBcR5xfIQMczdnIY+/CEhM8peIAFPU7zu3bdD6x5+s+k9xP7Y/0j5QMABCEgw4JeKD/Th9rLzjfNqztyJ8kLDU5uJ90LBDrPUHIxoDgg6y6+CIze7tA4RdL/d1Acr2iL0yTXIurAfdX8aUpGrboAAAAAQEC1YK8/an5Jr9+z4AtKBTnrv4k0o6gWNCDAemRrVG++6KPdx/kkN9XocX1ASI4et1fBp8AJOd3njlqnAPTJFsMI";
        let msg = base64::prelude::BASE64_STANDARD.decode(msg).unwrap();
        println!("{}", decode(20, msg));
    }
}
