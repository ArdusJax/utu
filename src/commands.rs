use gumdrop::Options;
use std::{env, thread::sleep};

// Define options for the program.
#[derive(Debug, Options)]
pub struct UtuOptions {
    // Options here can be accepted with any command (or none at all),
    // but they must come before the command name.
    #[options(help = "print help message")]
    pub help: bool,
    #[options(help = "be verbose")]
    pub verbose: bool,

    #[options(command)]
    pub command: Option<Command>,
}

#[derive(Debug, Options)]
pub enum Command {
    #[options(help = "make stuff")]
    Watch(WatchOpts),
    #[options(help = "install stuff")]
    Sign(SignOpts),
}

// Options accepted for the `make` command
#[derive(Debug, Options)]
pub struct WatchOpts {
    #[options(help = "print help message")]
    pub help: bool,
    #[options(free)]
    pub free: Vec<String>,
    #[options(help = "number of jobs", meta = "N")]
    pub jobs: Option<u32>,
}
#[derive(Debug)]
struct Attestation {
    kind: String,
    public_key_id: String,
    content: String,
}
impl WatchOpts {
    fn print_attestation(kind: String) {
        match kind.as_ref() {
            "BUILD" => { 
                println!("Hashing metadata...");
                sleep(std::time::Duration::from_secs(2));
                println!("Signing metadata using the {} Authority...", kind); 
                println!("{:#?}", Attestation{kind: String::from(&kind), public_key_id: String::from("builder.pub"), content: String::from("aXMgYSB0d28gbGluZSBzdHJpbmcsIHRoZSBzYW1lIGFzICJsaW5lIG9uZVxubGluZSB0d28iIChvZiBjb3Vyc2Ugb25lIGNhbiB1c2UgdGhlIFxuIG5ld2xpbmUgZXNjYXBlIGRpcmVjdGx5IHRvbykuIElmIHlvdSB3aXNoIHRvIGp1c3QgYnJlYWsgYSBzdHJpbmcgYWNyb3NzIG11bHRpcGxlIGxpbmVzIGZvciBmb3JtYXR0aW5nIHJlYXNvbnMgeW91IGNhbiBlc2NhcGUgdGhlIG5ld2xpbmUgYW5kIGxlYWRpbmcgd2hpdGVzcGFjZSB3aXRoIGEgXDsgZm9yIGV4YW1wbGU6")});
                sleep(std::time::Duration::from_secs(2));
                println!("Signing metadata using the Build Authority..."); 
                sleep(std::time::Duration::from_secs(2));
            },
            "PUBLISH" => println!("{:#?}", Attestation{kind: String::from(&kind), public_key_id: String::from("publish.pub"), content: String::from("aXMgYSB0d28gbGluZSBzdHJpbmcsIHRoZSBzYW1lIGFzICJsaW5lIG9uZVxubGluZSB0d28iIChvZiBjb3Vyc2Ugb25lIGNhbiB1c2UgdGhlIFxuIG5ld2xpbmUgZXNjYXBlIGRpcmVjdGx5IHRvbykuIElmIHlvdSB3aXNoIHRvIGp1c3QgYnJlYWsgYSBhY3Jvc3MgbXVsdGlwbGUgbGluZXMgZm9yIGZvcm1hdHRpbmcgcmVhc29ucyB5b3UgY2FuIGVzY2FwZSB0aGUgbmV3bGluZSBhbmQgbGVhZGluZyB3aGl0ZXNwYWNlIHdpdGggYSBcOyBmb3IgZXhhbXBsZTo=")}),
            "SCAN" => println!("{:#?}", Attestation{kind: String::from(&kind), public_key_id: String::from("scan.pub"), content: String::from("IGEgdHdvIGxpbmUgc3RyaW5nLCB0aGUgc2FtZSBhcyAibGluZSBvbmVcbmxpbmUgdHdvIiAob2YgY291cnNlIG9uZSBjYW4gdXNlIHRoZSBcbiBuZXdsaW5lIGVzY2FwZSBkaXJlY3RseSB0b28pLiBJZiB5b3Ugd2lzaCB0byBqdXN0IGJyZWFrIGEgYWNyb3NzIG11bHRpcGxlIGxpbmVzIGZvciBmb3JtYXR0aW5nIHJlYXNvbnMgeW91IGNhbiBlc2NhcGUgdGhlIG5ld2xpbmUgYW5kIGxlYWRpbmcgd2hpdGVzcGFjZSB3aXRoIGEgXDsgZm9yIGV4cGxlOg==")}),
            "DEPLOY" => println!("{:#?}", Attestation{kind: String::from(&kind), public_key_id: String::from("deploy.pub"), content: String::from("S20gb2RrIGRrZCBra2FrZCAgdHdvIGxpbmUgc3RyaW5nLCB0aGUgc2FtZSBhcyAibGluZSBvbmVcbmxpbmUgdHdvIiAob2YgY291cnNlIG9uZSBjYW4gdXNlIHRoZSBcbiBuZXdsaW5lIGVzY2FwZSBkaXJlY3RseSB0b28pLiBJZiB5b3Ugd2lzaCB0byBqdXN0IGJyZWFrIGEgYWNyb3NzIG11bHRpcGxlIGxpbmVzIGZvciBmb3JtYXR0aW5nIHJlYXNvbnMgeW91IGNhbiBlc2NhcGUgdGhlIG5ld2xpbmUgYW5kIGxlYWRpbmcgd2hpdGVzcGFjZSB3aXRoIGEgXDsgZm9yIGV4cGxlOiBpZGllaGEgZCA4OSBka3NhbGQga2Q5MCkqOGQgZmpkaiAgIGpqc2tkIGRrZHM=")}),
            _ => panic!("unsupported attestation"),
        }
    }
    fn print_meta_data_step(kind: &str) {
        println!("Gathering {} metadata...", &kind);
        sleep(std::time::Duration::from_secs(2));
        for (key, value) in env::vars_os() {
            sleep(std::time::Duration::from_secs(1));
            if key.to_string_lossy().starts_with("CARGO") {
                println!("{:#?} ::: {:#?}", key, value);
            }
        }
        WatchOpts::print_attestation(kind.to_string());
    }
}
// Options accepted for the `install` command
#[derive(Debug, Options)]
pub struct SignOpts {
    #[options(help = "print help message")]
    pub help: bool,
    #[options(help = "target directory")]
    pub dir: Option<String>,
}
