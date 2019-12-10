use slog::{error, info, o, Drain, Logger};
use std::sync::Mutex;

fn main() {
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let log = Logger::root(
        Mutex::new(slog_term::FullFormat::new(plain).build()).fuse(),
        o!("build-id" => "0.1.0"),
    );

    let client = sapi::SAPI::new("http://sapi.ruidc0.joyent.us", 60, log.clone());

    let services = client.list_services().expect("list services");
    dbg!(&services);

    let svc = &services[0];
    let svc_uuid = &svc.uuid;

    let instances = client
        .list_service_instances(svc_uuid)
        .expect("list service instances");
    let zone_uuid = &instances[0].uuid;

    match client.get_zone_config(&zone_uuid) {
        Ok(resp) => {
            info!(log, "config: {:#?}", resp);
        }
        Err(e) => error!(log, "error: {:?}", e),
    }
}
