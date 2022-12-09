use run_script::{
    ScriptOptions, 
    run_script,
};

// echo "/dns4/cluster1.domain/tcp/9096/ipfs/QmcQ5XvrSQ4DouNkQyQtEoLczbMr6D9bSenGy6WQUCQUBt" >> ~/.ipfs-cluster/peerstore

pub fn insert_secret_key(secretkey: String) {
    let options = ScriptOptions::new();
    let secret_key2 = secretkey.clone();
    // let export_key = format!("echo export CLUSTER_SECRET={} >> ~/.bashrc", secretkey);
    let export_key = format!("echo export CLUSTER_SECRET={} >> ~/.zshrc && echo export CLUSTER_SECRET={} >> ~/.bashrc", secretkey, secret_key2.clone());
    // println!("{}", export_key);
    let (code, output, error) = run_script!(
        &export_key,
        &vec![],
        &options
    ). unwrap();

    println!("It is recommend to reload the shell after insert_secret_key");
}