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
        12 => {
            use openmls::prelude::TlsDeserializeTrait as _;

            if let Ok(msg) = openmls::prelude::MlsMessageIn::try_from_bytes(&msg[..]) {
                format!("{msg:#?}")
            } else if let Ok(welcome) =
                openmls::prelude::Welcome::tls_deserialize(&mut msg.as_slice())
            {
                println!("Not a MLS message, trying Welcome");
                format!("{welcome:#?}")
            } else {
                panic!("Could not parse given input")
            }
        }
        20 => {
            use openmls_d20::prelude::TlsDeserializeTrait as _;

            if let Ok(msg) =
                openmls_d20::prelude::MlsMessageIn::tls_deserialize(&mut msg.as_slice())
            {
                format!("{msg:#?}")
            } else if let Ok(kp) =
                openmls_d20::prelude::KeyPackageIn::tls_deserialize(&mut msg.as_slice())
            {
                println!("Not a MLS message, trying KeyPackage");
                format!("{kp:#?}")
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
    fn check_parse_welcome_12() {
        let msg = "AQABAAADvzOzJKmHRrLC+trMgmdDuyQAIA+JKd3zuFmjM6XnFQ9qXgFZ1wUiRpvb39CDH2RvoIEiAFUvT4GJEV0yCZSTfeuibgTsCB7uELXpH84M3GylrbSniT4tV6Qyty6rQ+W7PnVjrp52/OCP4uitpeCNkujAcD8n0SMLVBxKMths/QagAiTSC+E8cbbg2MY3tN/qpCpTMhnDs8HvPQAgEzuKZP1IK/mrUMc9q0PeqI1ftMI6S4AaYPLvzbmUS28AVTQ8iiHQNFvcaD0yBi+taorEHi3qAnptxXNvHf+Uj+iIt99HF+26OgLMtEzNQD9ZdZPLbJgbp0CTV+Y8UgBGBu5kIHbsDnYvqX33SfBD26sSffmpXtOP1YJ/ABHMb/iXc8XFZq7IACDQUMr/ScU3woGwSRYxCQxhwgbTQD3+BmhlbKuj8jCeAwBVoI8puiLpIpqpA3KL4IcLyxQHmWdp65tLyzVWr9EIH2rHBS5n/2H0VGDErnOAP1vh+ZaVX/AN874UrI/SPkSXtyGwvRC9u5e2K29lfAaHVFHbGygYPUV2vimUExvpZmVpbchNPY4AIP+WqxTTAHQfCCqC7jQssL6KVGL+wawB0QKbJzuERgMwAFWLyulKAfR0rlZisfZb7WtdbqMWFFH9Xv0UFCdjfJm7VRrCeEnjdKpYkLWI1LolgPyvl2hSnDNjkLuTpOILrfSSu2i0RfNG1kKhLrfh529oWqWe5dAYC5wJYMqUPek+ufMkjZBzeAAg8TSue2w9VxPYV7KP+FCcXIUMpSfcxTsEgfp2/1nJHxAAVebsG/fLQltMvBMqYmhjeZ2eYs72UkHDjT3YvTFqlC/BQUPNFJBvM1c/QF+0fwfUOLHmhFuB0sjRIkDO/KUgcr4mjJ1b9TqsQsBPiLLSfX1vsV4x5zMKODCLeg2zdipv/lvwoDYeACCtm9atE9MtHao9wj7uf3pj7VvIw5ZTtR5C0L/M+WFOTABVH5CPS5zUwFTnLljuA+U5lT9wVT+CTV2+Ib1LxF7vS2Bnukw5sHb8Rl09LtOnPOb+fLABOf1AedWlNjo7r6HLVtk3DlTMUYX6C0xQAQ9K8ryYOlwixPY6+3fH1VLoxa/rp7wtGfMAIFarkQPuDqSFArb4FmMkFX7JxinwQ9Egj+jtMh/WkUZzAFXSL8Lsh3Dgyspo9KMGPFnN0Fb33j0s935ZbZUpI5Yvir2YYWKq7DC4IbazeZPm/7j9srVBdc+0RZ/JW3RLAShgiDi4yF/vcFop1vqABzRDGFx3AuWRAAAKy/OD12RAjHsSh4ejoU/6uqbbW9FHvXl30GFrFCA1LjO469sEu3CMqCwo2OvgmTbWLKRRFLio1nukQoFa2UB0ajjKRIUwSvw+qMu9v44IXplmZKD5Q/hNDqngC25IDD0ilmQVgW6ubJtG46Dav+Fuo+b/GJHdzQJ4DYe1fy2eBtC76BBlzVIwIhhMiN3+OhvaXTulkVGTTpoEux16eX3MA+zMz7JDNNRrXL/qacEBfXSSsZvhGcpS3RBxRgdKIZdX7QH22FUf58sZzda17yzGZDO+c/LHiPCuDbDBPyfVMzNxmqmEMaPXcwh95c2Cj4xTQ6YJNcBcPj6xmrqHvu5nSbT9TA2zxderi7SNFt9r2PlZxQQydy5i+7PW7Qo+q0+5c0zaQQLXRDIb2gsJUn5gpXCAAXyfR1ybYB1+hXVVHFNEM1Oa3JUU6q8fqMIV2/DHWfm0p0jCRqM6VElh7HJrZt5USLCh+0Vxwab0Z2AhOplCvE2LVZoyO3Hs49gHkHakYOmHpAptXROGqDiCa7RV5FYT8EZEaVrRtR2ls6Zkg+qeUiXunL270IPcmQmBL/mjQPwGddCm/1kYkpr0nY16/ZMPn5NHc4+9xQ/z18cu35+sZzvTGKq4Hc0jPzTEKcoFebZqbDwngAWZ57ccPJfLXRFoHVbwWKHS+qjjn50KlIvTCgW+lYvvfIPVgCVfQmVe5YgjCQrGigSvGZErQiZU0NO+d9OVtY8EM9wsp0rDzRZCwsfodYHywzx23axnASR2V1++/rU17yRoSYE3mKQVCdd4Ui1de8r4MzIBrSNCL35rDHOUwL/IdEi5xhhnDYHzezlXCcaxOT81tefs4QoAOfGPQ+sFNDY+MraSZyGeKQImMNdyLmmE9Lus6oYHh/dgfF+sJhmNy6KaIlBL5lRAtsqCcMVymb7p7tS2qp0GH2iUz86yWmBSLWAiYXA34QPw2P1Gy0rbfxaZx7JfdtadS/H19Ku9HTu4hc4iOPV7sRvl3dyLxax04hhWdARR1sCLWHPmsSC/uS0aJ/ECUJb/D5KaQ88KMyjhITy894GPz8sNn+xKoi3/WwCv2ngSQ7Mm/pN+OMjN8RrLEVzhrPAKtxcj/oJXFURkWqZQQ4OxDxbc2AQjtuQCvmtn8BQzv+D5FN12I10bTNJW9GZ+gcxSEXdpF4PCFBANVPMVDWVnY5zUa8hd2z6nQMccF0KfE6UW3ozKvWdb1vtr/KAILwUJoUrocRZ7s+epWiPvyFUwS+NLWAdCHcj2WPOvxmqL7weq2cZBqcbi7TVHleOfRHLwv2DWoGGHtJhj3VmVRiksezQBV13ugFozNxr+w4z4KVQWEjouN8NkDHe6HEmowngsJtCdT8+9dvAB25+LetCNh6SQ8cWt+nfWvxUqKhqIB0WFfEOiXs/z9gXEQTtcfHyX6rQTwuieiWM+XD2V57JPRKQ3DKRMellaIxibJ6J4aqLl0bHC5CKuxcBanISdFoJTM+klzuwQbK95leJp5lqcQ75N/+aSOtYaj3/HBCcJwnQvC5VLyw1n6cZB1tltocCbXLtHcczvwWzUMJhHz5ExYzE2rIY/WQfslfl846uMOSoseoLSmCX29kNd7hjQWbF75FuFFwzu49+lP4oNCl1VojKG6QvHF2MKmZQky8IDYbGjbicPZCsbcdYSTfvYjzJt/UmnEGdLU7btLiLCgXR33x8xJQuTgBnc3S36/ANz7Xy1SSZxwikIKEyWBDNMwgcOLgGlmCQq3dHiLjX3qmNUEKgtTptR37Oe0n/rsODFDnyngvSZ+KEAkc67d0pZByRJKmoy9QB5/GGj3plMYAmZqLsUCWt8dxzZcZo5xseQH+HZKHefHzIopEIRd/2UPYuGm5KGF+PZBMvXZFp39Y9Y53r73z2RMI5UcXoLVLCDhLizgV7HGhHr2xP7VjF/SO0ZOSqkRy5tOnHbp801OZTrxqLnb6bRIKeIxscckf9Nes0gDtAKydMGSZ3kR7A2P1/o6SvQ+f3qfvstcjrGHa4C9RoJH5xwu+E/7zsGy5hzIeYnWeS5CLedXQSpI0Ggz/s+/IBhwcGjvGdYnChhWspQmu1ZvTZac4yl60VPFGIomZ2W6JuKmC2bskrvj79PywMmIXUQn1kgA7iuFrnzq5IVl/Y9SWBRF7eXQQBHwVv1+xec3/B46HCn5maGOvErKU2BxSSPhsLOLMdbiqF5ooJx83e7yms7tJw4ay6Enr9Lzei8ikxmPq9a/aI0gUQGaYI0T6asktcKmX86ns/ls7Ia16PUdUc+6w0DNxCGlj7fCbeJVznXbxXTpAerjQLaBL/uD0njkNgaacEtIFaYuBy3tYt0kJODdkFrBXNYNzyJzs01bQAUCRUOt683hZOjfuAXCo7YjKcf4gZfMfMTDMe+5weWqwpmvonPti+vfeqx64V1Q2hwek9X574SgmQOlYseV9MZg6xw4E4LXAvokBBPEc+fPsnAXDyHRWYHybQJ4SwE7yBdNCLkMxHCU5hsXfXkCybs5WevhEQVO3YNMmZn3j0effs8FcSjB/AQcZ0gBYeuTqdbKZHNnC3h/bCE9ety/QliV1l0neKdC3f1uLrEIXbbOHyvyxQ0kHfbW5IQ9wnSsxsyn+K1EjJEMHtbmBbRTRDoWqTwNmZQ48xlBIFV94jktvCvD7Yxe2N0Qj9u+5ZaI6LYov2LL4iOcaTMxDTyOzrIk2F8tNW7LTBcwoH3x1++7QST/j5JOFaLDlFMRXyUxoDxLZIHXl0mcNpeDawPCjrmF7omuJFlsIfnQqrUrYjbY4YxC7+LO0KV4Hhj5rY4RSzOjn6Iw2tkJQ7uxuGbkQWV1OxAk6yX8IjGhCYskv0tXKOtxjB380AuJe8E2j62qJKi0j9Ydk5ReBlv0UehrvvjUTiyKqN4R/eCQo8qcc7WIcrQ+M/d8hPAaai4UN4B3WDZcHw7Ug4/0Tc6f8eA+Um+72fG5ZnIvQat66OzTJHagKRGWcHYaNL5oekvIdq/uc49c+CdsjgSAAMhYqoboazRxh11LzPhF/Nf3Eux7fm+0Abjm175NwNXfKpvmMF5nBswMTHxst2U//1yikQ0DrC8CDVsPtfWQJM+QdhoZ1DeF/LIcMLzK+S0Gx6eiG3nJl8yOOYDbvQWt9NZaHBq+6N+NYBtne1tqa6m1GkXNd3p3zvQcoD2Qr/XkdeXdresMTaap9SvmNR4xgDY7SzketfWsrQuZWeIkNpIUcq+Bsucje31RY9Kn6+ynNTAVLl8j23R6FRYLnC6y+Zvtmpy0wE0PJR/dcJXV0p0m7CbFEzYbDVapIHonV+zJK+Hg/41TC9dBZceASkUlbJLX4IHDRndzs0pLl5rouDwb8jM0o+FzH4lcrBSqyW6okF66LmJaUtBYHrqBPju1yCZgJ3a8MEsy7uPlC6RB1DOz4KroKpT0tf1qI0iM5lt9hNvhDoA4Q8HDSlOsBtmCIr5VklsuBs2hRUb1Vnrie8depxydM0kNcP0czg4rjus0TELBsOb43dLX2XpCIpezRKIu12EjFnd1n+xZz1reTqgQXPCYYAkJwuudFTBbY0XJeM+SyVKlnVjKF7fQKMRVF+qVI9JDXJOSBoLo3f0ljKDeDbll/j9dyfqWMwGBZ7p+DfjXkgybBitfx+HCiXjDWEjAXQvSCMI7qO/zOyzP5fp2w==";
        let msg = base64::prelude::BASE64_STANDARD.decode(msg).unwrap();
        println!("{}", decode(12, msg));
    }

    #[test]
    fn check_parse_keypackage_20() {
        let msg = "AAEAASB9obtu2o2OX3DOIrxhJOpa0Ae0F5ECRec0IDFTRl8qKiDj+8G+uD6epHAkRX4/8QgKRtwty79XfPZSGQAP01PMTSC1tAaAMzL79uVcZ85ClmVtWwB2fYXfM5RdcLlQA8tpfgABQE4yNThiYjdiYy0yMTU4LTRiMDMtOWJlNS02NTE4NzEwYmEyZTc6ZDExNjE3N2NhYjNhNDUyMEBuaW5qYXMuZG9nZm9vZC53aXJlLmxpbmsCAAEIAAEAAgAHAAMCAAEMAAEAAgADAAQABQAHBAABAAIBAAAAAGR57soAAAAAZOi62gBAQJcsXjFYfJgmlIouO/HlRz4p7OdHPRc/nPgUM3MRfdp6eQXLLT5dRh82Kd+H+PHZtrnRbnPIPZRwo+S0SjKk2AYAQEBuV7hWf5J6unW6Uu3uw0qsrEfan9w4I2Fr6CZ34HgQF/rHBfj8C1b2tl8WcLmYMKolSE8EZuLWmoLer9o8tMgK";
        let msg = base64::prelude::BASE64_STANDARD.decode(msg).unwrap();
        println!("{}", decode(20, msg));
    }
}
