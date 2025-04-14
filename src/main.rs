use clap::Parser;
use kseq::parse_path;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Parser)]
#[command(name = "FastqCount")]
#[command(version = "0.1")]
#[command(about = "Record and counts the unique sequences in a FASTQ file", long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
    #[arg(short, long)]
    name: String,
}

fn main() {
    let cli = Cli::parse();

    //let inpath: String = args().nth(1).unwrap();
    //let outpath: String = args().nth(2).unwrap();

    println!("input: {:?}", cli.input);
    println!("output: {:?}", cli.output);
    println!("name: {:?}", cli.name);

    let mut records = parse_path(cli.input).unwrap();
    let mut unique_hashmap = HashMap::new();

    let mut nb_reads = 0;
    let mut nb_bases = 0;
    let mut nb_nrecord = 0;
    // let mut records = parse_reader(File::open(path).unwrap()).unwrap();
    while let Some(record) = records.iter_record().unwrap() {
        // println!("head:{} des:{} seq:{} qual:{} len:{}",
        //	record.head(), record.des(), record.seq(),
        //	record.qual(), record.len());
        // let record_sequence = record.seq();

        let sequence = String::from(record.seq());
        if record.seq().contains("N") {
            nb_nrecord += 1;
        } else if let Some(count) = unique_hashmap.get(&sequence) {
            unique_hashmap.insert(sequence, count + 1);
        } else {
            unique_hashmap.insert(sequence, 1);
        };
        nb_reads += 1;
        nb_bases += record.seq().len();
    }

    println!("Number of reads: {}", nb_reads);
    println!("Number of bases: {}", nb_bases);
    println!("Number of reads containing N: {}", nb_nrecord);
    let size = unique_hashmap.keys().len();
    println!("Number of unique reads not containing N: {}", size);
    let countoutput = cli.output.clone() + &cli.name.clone() + "_count.txt";
    let _ = fs::create_dir_all(&cli.output);
    let f = File::create(countoutput).expect("unable to create file");
    let mut f = BufWriter::new(f);

    writeln!(f, "NReads,{}", nb_nrecord).expect("unable to write");
    for (key, value) in unique_hashmap {
        // println!("{},{}", key, value);
        writeln!(f, "{},{}", key, value).expect("unable to write");
    }

    let statoutput = cli.output.clone() + &cli.name.clone() + "_stat.txt";
    let mut e = File::create(statoutput).expect("unable to create file");
    writeln!(e, "Number of reads containing N: {}", nb_nrecord).expect("unable to write");
    writeln!(e, "Number of bases: {}", nb_bases).expect("unable to write");
    writeln!(e, "Number of reads containing N: {}", nb_nrecord).expect("unable to write");
}
