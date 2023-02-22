use crate::read_stdin;
use base64::Engine;
use openmls::prelude::MlsMessageIn;

pub fn decode_msg(draft: u8) {
    let msg = read_stdin();
    let msg = base64::prelude::BASE64_STANDARD_NO_PAD
        .decode(msg)
        .expect("Invalid base64 message");
    match draft {
        12 => {
            let msg = MlsMessageIn::try_from_bytes(&msg).expect("Invalid MLS message");
            println!("{:#?}", msg);
        }
        _ => panic!("Unsupported draft version {draft}"),
    }
}
