use clap::Clap;
use pacaptr::dispatch::Opt;
use pacaptr::print::{print_err, PROMPT_ERROR};

fn main() {
    smol::block_on(async_main());
}

async fn async_main() {
    let opt = Opt::parse();
    match opt.dispatch().await {
        Ok(n) => std::process::exit(n),
        Err(e) => {
            print_err(e, PROMPT_ERROR);
            std::process::exit(1);
        }
    }
}
