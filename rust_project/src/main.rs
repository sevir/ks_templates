use structopt::StructOpt;
use exitfailure::ExitFailure;

#[derive(StructOpt)]
struct Options{
    #[structopt(short = "d", long = "debug")]
    debugmode: bool,
}

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();

    println!("Param debugmode: {}", options.debugmode);
    Ok(())
}
