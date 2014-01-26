use std::os;
use std::io::File;
use std::str;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <inputfile1> <inputfile2>", args[0]); 
    } else {
        let share1_name = args[1].clone();
        let share1_path = Path::new(share1_name.clone());
        let share1_file = File::open(&share1_path);

        let share2_name = args[2].clone();
        let share2_path = Path::new(share2_name.clone());
        let share2_file = File::open(&share2_path);

         match (share1_file, share2_file) {
            (Some(share1), Some(share2)) => {
                join(share1, share2); 
                } ,
            (_, _) => fail!("Error opening output files!"),
        }

    }
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
    ret.push(a[i] ^ b[i]);
    }
    ret
}

fn join(mut share1: File, mut share2: File) {
    let share1_bytes: ~[u8] = share1.read_to_end();
    let share2_bytes: ~[u8] = share2.read_to_end();

    let msg_bytes = xor(share1_bytes,share2_bytes);
    let s = str::from_utf8(msg_bytes);
    // try let s = str::from_utf8_slice(msg bytes);
    println!("Message: {:s}", s);

}