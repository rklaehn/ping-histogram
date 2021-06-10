use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::time::{Duration, Instant, SystemTime};

fn main() -> anyhow::Result<()> {
    let t_start = SystemTime::now();
    loop {
        let at = SystemTime::now();
        let t0 = Instant::now();
        ping::ping(
            IpAddr::V4(Ipv4Addr::new(193, 99, 144, 85)),
            Some(Duration::from_secs(30)),
            None,
            None,
            None,
            None,
        )
        .map_err(|x| anyhow::anyhow!("{}", x))?;
        let dt = t0.elapsed();
        println!(
            "{}\t{}",
            at.duration_since(t_start)?.as_millis(),
            dt.as_millis()
        );
        std::thread::sleep(Duration::from_secs(1));
    }
}
