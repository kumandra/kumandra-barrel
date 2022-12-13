use std::process::Command;
use dirs::home_dir;
use run_script::{
    ScriptOptions, 
    run_script,
};

pub fn create_pool() {
    let home = home_dir().unwrap();
    let ipfs_cluster_path = format!("{}/.ipfs-cluster", home.display());
    
    // Remove ~/.ipfs-cluster folder first
    Command::new("rm")
        .args(["-rf", &ipfs_cluster_path])
        .spawn()
        .expect("failed to remove ipfs-cluster directory");

    println!("Initialize IPFS-Cluster");
    Command::new("ipfs-cluster-service")
        .args(["init", "--consensus", "raft"])
        .output()
        .expect("failed to initialize");

    // jq '.' ~/.ipfs-cluster/identity.json | grep id | awk -F: '{ print $2 }' | tr -d '",'
    let options = ScriptOptions::new();
    let peer = String::from("jq '.' ~/.ipfs-cluster/identity.json | grep id | awk -F: '{ print $2 }' | tr -d '\", '");
    let (_, peer_id, _) = run_script!(
        &peer,
        &vec![],
        &options
    ). unwrap();


    // /ip4/192.168.1.2/tcp/9096/p2p/QmPSoSaPXpyunaBwHs1rZBKYSqRV4bLRk32VGYLuvdrypL

    let options = ScriptOptions::new();
    let ip = String::from("curl ip.me | tr -d ' '");
    let (_, ip, _) = run_script!(
        &ip,
        &vec![],
        &options
    ). unwrap();
        
    // println!("public ip: {}", ip);

    let ipfs_cluster_peer_address = format!("/ip4/{}/tcp/9096/p2p/{}", ip.trim(), peer_id);

    // println!("{}", ipfs_cluster_peer_address);

    let options = ScriptOptions::new();
    let secret_key = String::from("jq '.' ~/.ipfs-cluster/service.json | grep secret | awk -F: '{ print $2 }' | tr -d '\", '");
    let (_, secret_key, _) = run_script!(
        &secret_key,
        &vec![],
        &options
    ). unwrap();

    // Pretty Print Message
    println!("
        Your peer address: {}
        Your public ip: {}
        Your secretkey: {}
        share this with other member: {}
    ", peer_id, ip, secret_key, ipfs_cluster_peer_address);

    

}