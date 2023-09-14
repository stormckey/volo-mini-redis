use mini_redis::parse::parse;
use std::fs::File;
use std::io::Write;

fn main() {
    let (proxy, vec) = parse();
    let mut file = File::create("begin.sh").expect("Cant create begin.sh");
    file.write_all("#! /bin/bash\n".as_bytes()).unwrap();
    for v in vec {
        let master = v.iter().next().unwrap();
        file.write_all((format!("echo \"server -p {} master &\"\n", master)).as_bytes())
            .unwrap();
        file.write_all((format!("server -p {} master &\n", master)).as_bytes())
            .unwrap();
        for slave in v.iter().skip(1) {
            file.write_all(
                (format!("echo \"server -p {} slave {} &\"\n", slave, master)).as_bytes(),
            )
            .unwrap();
            file.write_all((format!("server -p {} slave {} &\n", slave, master)).as_bytes())
                .unwrap();
        }
    }
    file.write_all(format!("echo \"server -p {} proxy &\"\n", proxy).as_bytes())
        .unwrap();
    file.write_all(format!("server -p {} proxy &\n", proxy).as_bytes())
        .unwrap();
    println!("source begin.sh to start all the master-slave servers.");
}
