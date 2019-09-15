mod test_log;
mod test_tree;
mod test_pagecache;
mod tree;

pub fn setup_logger() {
    use std::io::Write;

    fn tn() -> String {
        std::thread::current()
            .name()
            .unwrap_or("unknown")
            .to_owned()
    }

    #[cfg(feature = "pretty_backtrace")]
    color_backtrace::install();

    let mut builder = env_logger::Builder::new();
    builder
        .format(|buf, record| {
            writeln!(
                buf,
                "{:05} {:20} {:10} {}",
                record.level(),
                tn(),
                record.module_path().unwrap().split("::").last().unwrap(),
                record.args()
            )
        })
        .filter(None, log::LevelFilter::Info);

    if std::env::var("RUST_LOG").is_ok() {
        builder.parse_filters(&std::env::var("RUST_LOG").unwrap());
    }

    let _r = builder.try_init();
}