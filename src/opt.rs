use crate::common::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "vim-profiler", about = "A vim profiling tool.")]
pub struct Opt {
  #[structopt(short, long, parse(try_from_str = Command::parse), default_value = "vim")]
  /// The command to run, e.g vim or neovim.
  command: Command,

  #[structopt(short, long)]
  /// The number of iterations.
  iter: Option<i64>,

  #[structopt(short = "x", long)]
  /// Precision in the output.
  precision: Option<usize>,

  #[structopt(short = "n", long)]
  /// The number of plugins to list in the output.
  count: Option<usize>,

  #[structopt(short, long)]
  /// Add informative messages during program execution.
  verbose: bool,

  #[structopt(short, long)]
  /// Plot the data in the terminal.
  plot: bool,

  #[structopt(short, long)]
  /// Export the results to a CSV file.
  export: bool,

  #[structopt(short, long)]
  /// Show system plugins in the output.
  sys: bool,

  #[structopt(short, long)]
  /// Display the plugin times in reverse order (fastest first).
  reverse: bool,
}

impl Opt {
  pub fn run(self) -> Result<()> {
    if self.verbose {
      env::set_var("RUST_LOG", "info");
    }

    env_logger::init();
    info!("Starting run ...");

    let worker = Worker::new(self.command, self.iter.unwrap_or(1));
    let data = worker.run()?;

    if !self.export && !self.plot {
      Printer::new(self.reverse, self.count, self.precision).summary(&data);
      return Ok(());
    }

    let stats = Stats::new(data);

    if self.export {
      info!("Writing statistics to CSV file ...");
      stats.write()?;
      return Ok(());
    }

    if self.plot {
      info!("Plotting statistics ...");
      stats.plot()?;
      return Ok(());
    }

    Ok(())
  }
}