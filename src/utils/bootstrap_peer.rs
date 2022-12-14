use std::process::Command;
use std::{thread, time};

pub fn bootstrap_peer() {
    ipfs_init();
    // This function need to run after IPFS already installed
    // TODO: Check if IPFS is installed or not
    let list_peers =
    [
    "/dns/bootstrap0.kumandra.org/tcp/4001/p2p/12D3KooWCKPb44ngT6jAn83ZYk3QUL1Wy34Ds6haHLdVc3biikY3",
    "/dns/bootstrap1.kumandra.org/tcp/4001/p2p/12D3KooWCKPb44ngT6jAn83ZYk3QUL1Wy34Ds6haHLdVc3biikY3",
    "/dns/bootstrap2.kumandra.org/tcp/4001/p2p/12D3KooWCKPb44ngT6jAn83ZYk3QUL1Wy34Ds6haHLdVc3biikY3",
    ];

    // Delete the entire bootstrap list at once
    ipfs_bootstrap_rm();

    // run ipfs bootstrap add
    for peer in list_peers.iter() {
        ipfs_bootstrap_add(peer);
        let ten_millis = time::Duration::from_millis(10);
        // let now = time::Instant::now();
        thread::sleep(ten_millis);

    }
}

fn ipfs_bootstrap_rm() {
    Command::new("ipfs")
        .args(["bootstrap", "rm", "--all"])
        .spawn()
        .expect("failed to add Kumandra peers");
        let ten_millis = time::Duration::from_millis(20);
        // let now = time::Instant::now();
        thread::sleep(ten_millis);
}

fn ipfs_bootstrap_add(peer: &&str) {
        Command::new("ipfs")
        .args(["bootstrap", "add"])
        .arg(peer)
        .spawn()
        .expect("failed to add Kumandra peers");
        let ten_millis = time::Duration::from_millis(10);
        // let now = time::Instant::now();
        thread::sleep(ten_millis);
        
}

fn ipfs_init() {
    use std::path::Path;
    use dirs::home_dir;
    // Check if ~/.ipfs not exist then run ipfs init
    let home_dir = home_dir().unwrap();
    let ipfs_folder = format!("{}/.ipfs", home_dir.display());
    if !Path::new(&ipfs_folder).exists() {
        println!("IPFS directory not exist, start ipfs initializing now...");
        Command::new("ipfs")
        .arg("init")
        .spawn()
        .expect("failed to init ipfs");
        let ten_millis = time::Duration::from_millis(90);
        thread::sleep(ten_millis);
    }

}


