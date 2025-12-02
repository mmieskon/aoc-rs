pub struct Args {
    pub year: u32,
    pub day: u32,
    pub path: String,
}

impl Args {
    pub fn parse() -> Self {
        fn try_parse() -> Result<Args, ()> {
            let mut args = std::env::args().skip(1);

            // TODO: Add year parsing
            let year = 2025;
            let day_str = args.next().ok_or(())?;
            let day = day_str.parse().map_err(|_| ())?;
            let filename = args.next().ok_or(())?;

            Ok(Args {
                year,
                day,
                path: filename,
            })
        }

        match try_parse() {
            Ok(val) => val,
            Err(()) => print_usage_and_exit(),
        }
    }
}

pub fn print_usage_and_exit() -> ! {
    eprintln!("USAGE: {} <DAY> <PATH>", std::env::args().next().unwrap());
    std::process::exit(1);
}
