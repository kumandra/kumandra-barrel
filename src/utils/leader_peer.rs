use run_script::{
    ScriptOptions, 
    run_script,
};

// echo "/dns4/cluster1.domain/tcp/9096/ipfs/QmcQ5XvrSQ4DouNkQyQtEoLczbMr6D9bSenGy6WQUCQUBt" >> ~/.ipfs-cluster/peerstore

pub fn insert_leader_peer(leader_peer_address: String) {
    let options = ScriptOptions::new();
    let leader_addr_peer = format!("echo {} >> ~/.ipfs-cluster/peerstore", leader_peer_address);
    run_script!(
        &leader_addr_peer,
        &vec![],
        &options
    ). unwrap();
}
