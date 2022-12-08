use std::process::Command;
use std::{thread, time};

pub fn bootstrap_peer() {
    // This function need to run after IPFS already installed
    // TODO: Check if IPFS is installed or not
    let list_peers =
    [
    "/dns/bootstrap0.kumandra.org/tcp/4001/p2p/12D3KooWCKPb44ngT6jAn83ZYk3QUL1Wy34Ds6haHLdVc3biikY3",
    "/dns/bootstrap1.kumandra.org/tcp/4001/p2p/12D3KooWCKPb44ngT6jAn83ZYk3QUL1Wy34Ds6haHLdVc3biikY3",
    "/dns/bootstrap2.kumandra.org/tcp/4001/p2p/12D3KooWCKPb44ngT6jAn83ZYk3QUL1Wy34Ds6haHLdVc3biikY3",
    ];

    // Delete the entire bootstrap list at once
    Command::new("ipfs")
        .args(["bootstrap", "rm", "--all"])
        .spawn()
        .expect("failed to add Kumandra peers");
        let ten_millis = time::Duration::from_millis(20);
        // let now = time::Instant::now();
        thread::sleep(ten_millis);

    for peer in list_peers.iter() {
        ipfs_bootstrap_add(peer);
        let ten_millis = time::Duration::from_millis(10);
        // let now = time::Instant::now();
        thread::sleep(ten_millis);

    }

   
        


  


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