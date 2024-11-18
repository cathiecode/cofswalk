use std::{env::args, ffi::OsString, path::PathBuf};

use cofswalk::DirWalker;

fn main() {
    let mut a = PathBuf::new().join(".").join(".");

    println!("{:?}", a);

    let args: Vec<String> = args().collect();

    let walker = DirWalker::new(OsString::from(args[1].clone()).as_os_str()).unwrap();

    let mut serialized_walker = serde_json::to_string(&walker).unwrap();

    loop {
        let mut walker: DirWalker = serde_json::from_str(&serialized_walker).unwrap();

        println!("{:?}", walker);

        let mut done = true;

        {
            let mut i = 0;

            for entry in &mut walker {
                if i > 10 {
                    done = false;
                    break;
                }

                println!("{:?}", entry);

                i += 1;
            }
        }

        if done {
            break;
        }

        serialized_walker = serde_json::to_string(&walker).unwrap();

        println!("{}", serialized_walker);
    }
}
