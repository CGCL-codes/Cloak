#[macro_use]
extern crate log;

use anyhow::{Result};
use structopt::StructOpt;
use std::path::{Path, PathBuf};
use chain_demo::*;
use chain_demo_simchain::SimChain;

#[derive(StructOpt, Debug)]
#[structopt(name="simchain-build")]
struct Opts {
    //input data_path
    #[structopt(long, short)]
    input_data_path: PathBuf,

    //output db_path
    #[structopt(short="db", long)]
    db_path: PathBuf,


    //use inter-index default == false
    #[structopt(long)]
    inter_index: bool,

    //use intra-index
    #[structopt(long)]
    intra_index: bool,

    //error_bounds
    #[structopt(long, default_value = "5.0")]
    error_bounds: FloatType,
}

fn build_chian(data_path: &Path, out_db_path: &Path, param: &mut Parameter) -> Result<()> {
    info!("build chain using data from {:?}", data_path);
    info!("output db path: {:?}",out_db_path);
    info!("param: {:?}",param);
    let raw_txs = load_raw_tx_from_file(data_path)?;
    let mut chain = SimChain::create(out_db_path, param.clone())?;
    
    let mut block_count: IdType = 0;
    let start_block_id: IdType =  raw_txs.keys().min().unwrap().to_owned();
    let mut block_headers: Vec<BlockHeader> = Vec::new();
    
    let mut pre_hash = Digest::default();
    for (id,tx) in raw_txs.iter(){
        info!("build block {}", id);
        let block_header = build_block(*id, pre_hash, tx.iter(), &mut chain)?;
        block_headers.push(block_header.clone());
        pre_hash = block_header.to_digest();
        block_count += 1;
    }
    param.block_count = block_count;
    param.start_block_id = start_block_id;
    chain.set_parameter(param.clone())?;
    Ok(())
}



fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", "info"));

    let opts = Opts::from_args();
    let mut param = Parameter {
        error_bounds: opts.error_bounds,
        inter_index: opts.inter_index,
        intra_index: opts.intra_index,
        start_block_id: 0,
        block_count: 0,
        inter_index_timestamps: Vec::new(),
    };
    let timer_total = howlong::HighResolutionTimer::new();
    build_chian(&opts.input_data_path, &opts.db_path, &mut param)?;
    info!("build total time {:#?}", timer_total.elapsed());
    Ok(())
}