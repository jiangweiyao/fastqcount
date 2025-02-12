use std::env::args;
use std::fs::File;
use kseq::parse_path;
use std::collections::HashMap;


fn main(){
	let path: String = args().nth(1).unwrap();
	let mut records = parse_path(path).unwrap();
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
                } else {
                    if let Some(count) = unique_hashmap.get(&sequence){
                        unique_hashmap.insert(sequence, count + 1);
                    } else {
                        unique_hashmap.insert(sequence, 1);
                    };
                };
                nb_reads += 1;
                nb_bases += record.seq().len();
	}



        println!("Number of reads: {}", nb_reads);
        println!("Number of bases: {}", nb_bases);
        println!("Number of reads containing N: {}", nb_nrecord);
        let size = unique_hashmap.keys().len();
        println!("Number of unique reads not containing N: {}", size);
}
