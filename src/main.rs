use clap::{Parser, Subcommand};
use std::error::Error;

mod stats;
mod subset;
mod io_utils;

#[derive(Parser)]
#[command(name = "sparx")]
#[command(about = "Disk-based sparse matrix statistics and subsetting.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compute row and column statistics of the MTX file
    Stats {
        /// Input MTX file
        #[arg(short, long)]
        input: String,

        /// Output prefix for the statistics files
        #[arg(short, long, default_value = "stats")]
        output_prefix: String,
    },
    /// Subset the MTX file based on specified rows and columns
    Subset {
        /// Input MTX file
        #[arg(short, long)]
        input: String,

        /// Output MTX file
        #[arg(short, long)]
        output: String,

        /// File containing row indices to retain (one per line)
        #[arg(long)]
        rows: Option<String>,

        /// File containing column indices to retain (one per line)
        #[arg(long)]
        cols: Option<String>,

        /// Do not reindex the output matrix (keep original indices)
        #[arg(long)]
        no_reindex: bool,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Stats { input, output_prefix } => {
            stats::compute_stats(&input, &output_prefix)?;
        }
        Commands::Subset {
            input,
            output,
            rows,
            cols,
            no_reindex,
        } => {
            subset::subset_matrix(&input, &output, rows, cols, no_reindex)?;
        }
    }

    Ok(())
}