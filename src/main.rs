extern crate rstox;
extern crate markov;

use rstox::core::*;
use markov::*;
use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::BufWriter;

static BOOTSTRAP_IP: &'static str = "192.254.75.98";
static BOOTSTRAP_PORT: u16 = 33445;
static BOOTSTRAP_KEY: &'static str =
                    "951C88B7E75C867418ACDB5D273821372BB5BD652740BCDF623A4FA293E75D2F";
static BOT_NAME: &'static str = "Rusty";

fn main() {
	let mut chain = Chain::for_strings();
    let mut tox = Tox::new(ToxOptions::new(), None).unwrap();
    tox.set_name(BOT_NAME).unwrap();
    let bootstrap_key = BOOTSTRAP_KEY.parse().unwrap();
    tox.bootstrap(BOOTSTRAP_IP, BOOTSTRAP_PORT, bootstrap_key).unwrap();

    println!("{}", tox.get_address());
	
	let path = Path::new("brain");
	chain.feed_file(&path);

    loop {
        for ev in tox.iter() {
            match ev {
                FriendRequest(cid, _) => {
                    tox.add_friend_norequest(&cid).unwrap();
                },
                FriendMessage(fid, kind, msg) => {
					let path = Path::new("brain");
					let file = OpenOptions::new()
						.read(true)
						.write(false)
						.create(false)
						.append(true)
						.open(&path);
					let mut writer = BufWriter::new(&file);
					writer.write_all(b"asdasd");
					
					let msg = chain.generate_str();
					tox.send_friend_message(fid, kind, &msg).unwrap();
                },
                GroupInvite(fid, kind, data) => {
                    match kind {
                        GroupchatType::Text => { tox.join_groupchat(fid, &data).unwrap(); },
                        _ => {},
                    }
                },
                ev => { println!("Tox event: {:?}", ev); },
            }
        }

        tox.wait();
    }
}
