//! Probe the VM-dialog chain for one NPC: event DAT → strings → block → VM.
//! Usage: FFXI_DAT_PATH=... cargo run -p ffxi-event --example event-probe -- <zone> <actor_id> <event_id>

use ffxi_dat::dmsg::StringDat;
use ffxi_dat::event_dat::EventDat;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let zone: u16 = args.get(1).map(|s| s.parse().unwrap()).unwrap_or(231);
    let actor: u32 = args.get(2).map(|s| s.parse().unwrap()).unwrap_or(17723509);
    let event_id: u16 = args.get(3).map(|s| s.parse().unwrap()).unwrap_or(660);

    let root = ffxi_dat::DatRoot::from_env_or_default().expect("DAT root");
    println!("root: {}", root.root().display());

    let loc = ffxi_dat::event_locate::zone_id_to_event_location(zone);
    println!("event DAT location for zone {zone}: {loc:?}");
    let Some(loc) = loc else { return };
    let path = loc.path_under(root.root());
    println!("event DAT path: {} (exists: {})", path.display(), path.exists());
    let bytes = std::fs::read(&path).expect("read event DAT");
    let dat = match EventDat::parse(&bytes) {
        Ok(d) => d,
        Err(e) => {
            println!("event DAT parse FAILED: {e:?}");
            return;
        }
    };

    let sid = ffxi_dat::zone_dat::zone_id_to_string_file_id(zone);
    println!("string file id: {sid:?}");
    let strings = sid
        .and_then(|id| root.resolve(id).ok())
        .map(|l| l.path_under(root.root()))
        .and_then(|p| {
            println!("string DAT path: {} (exists: {})", p.display(), p.exists());
            std::fs::read(p).ok()
        })
        .and_then(|b| match StringDat::parse(&b) {
            Ok(s) => Some(s),
            Err(e) => {
                println!("string DAT parse FAILED: {e:?}");
                None
            }
        });
    let Some(strings) = strings else {
        println!("no strings — begin() would return None here");
        return;
    };

    let block = dat.block_for_actor(actor);
    println!("block_for_actor({actor}): {}", if block.is_some() { "FOUND" } else { "MISSING — begin() returns None here" });
    let Some(block) = block else {
        return;
    };

    let runner = ffxi_event::DialogRunner::start(block, event_id, 117);
    println!("DialogRunner::start(event_id={event_id}): {}", if runner.is_some() { "OK" } else { "None — no such event in block" });
    let Some(mut runner) = runner else { return };

    match runner.advance(None, &strings) {
        ffxi_event::DialogStep::Frame(f) => {
            println!("FIRST FRAME: text={:?} choices={:?}", f.text, f.choices);
        }
        ffxi_event::DialogStep::Ended { end_para } => {
            println!("VM ENDED immediately (end_para={end_para}) — begin() returns None");
        }
        ffxi_event::DialogStep::Stopped(r) => {
            println!("VM STOPPED: {r:?} — begin() returns None");
        }
    }
}
