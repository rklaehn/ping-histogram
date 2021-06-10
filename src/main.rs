use std::net::Ipv4Addr;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime};
use std::net::IpAddr;

fn main() -> anyhow::Result<()> {
  let t_start = SystemTime::now();
  let measurements: Arc<Mutex<Vec<(SystemTime, Duration)>>> = Arc::new(Mutex::new(Vec::new()));
  let measurements2 = measurements.clone();
  ctrlc::set_handler(move || {
    println!("----");
    for (t, dt) in measurements2.lock().unwrap().iter() {
      println!("{}\t{}", t.duration_since(t_start).unwrap().as_millis(), dt.as_millis());
    }
    std::process::exit(0);
  }).expect("Error setting Ctrl-C handler");
  loop {
    let at = SystemTime::now();
      let t0 = Instant::now();
      ping::ping(IpAddr::V4(Ipv4Addr::new(193,99,144,85)), Some(Duration::from_secs(30)), None, None, None, None).map_err(|x| anyhow::anyhow!("{}", x))?;
      let dt = t0.elapsed();
      println!("{}", dt.as_millis());
      measurements.lock().unwrap().push((at, dt));
      std::thread::sleep(Duration::from_secs(1));
    }
}
