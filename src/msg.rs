use crate::read_stdin;
use base64::Engine;
use bat::PagingMode;
use openmls::prelude::MlsMessageIn;

pub fn decode_msg(draft: u8) {
    let msg = read_stdin();
    let msg = base64::prelude::BASE64_STANDARD_NO_PAD
        .decode(msg)
        .expect("Invalid base64 message");
    match draft {
        12 => {
            let msg = MlsMessageIn::try_from_bytes(&msg).expect("Invalid MLS message");
            let msg = format!("{msg:#?}");
            bat::PrettyPrinter::new()
                .input_from_bytes(msg.as_bytes())
                .colored_output(true)
                .true_color(false)
                .paging_mode(PagingMode::Always)
                .language("json")
                .print()
                .unwrap();
        }
        16 => panic!("Draft-16 is not implemented yet"),
        _ => panic!("Unsupported draft version {draft}"),
    }
}
