/// ipfs-cluster-ctl pin add --expire-in value =  Duration after which the pin should be unpinned automatically after updating
use run_script::{
    ScriptOptions, 
    run_script,
};

pub fn kumandra_start() {
    start_ipfs();
    start_ipfs_cluster();
    systemd_service();

}

fn start_ipfs() {
    let options = ScriptOptions::new();
    let ipfs_start = String::from("systemctl --user --enable --now ipfs");
    // println!("{}", export_key);
    run_script!(
        &ipfs_start,
        &vec![],
        &options
    ).unwrap();

    println!("Starting IPFS systemd service");
}

fn start_ipfs_cluster() {
    let options = ScriptOptions::new();
    // let export_key = format!("echo export CLUSTER_SECRET={} >> ~/.bashrc", secretkey);
    let ipfs_cluster_start = String::from("systemctl --user --enable --now ipfs-cluster");
    // println!("{}", export_key);
    run_script!(
        &ipfs_cluster_start,
        &vec![],
        &options
    ).unwrap();

    println!("Starting IPFS-cluster");
}

fn systemd_service() {
    use std::fs;
    use dirs::home_dir;
    let current_user = whoami::username();
    let home = home_dir().unwrap();
    let user_systemd_service_path = format!("{}/.config/systemd/user/", home.display());
    fs::create_dir_all(user_systemd_service_path).unwrap();


    let options: ScriptOptions = ScriptOptions::new();
    let ipfs_service = format!(r#"cat <<EOF >> ~/.config/systemd/user/ipfs.service
[Unit]
Description=IPFS Daemon
After=network-online.target
StartLimitIntervalSec=500
StartLimitBurst=5

[Service]
Restart=on-failure
RestartSec=5s
Type=simple
ExecStart=ipfs daemon
User={}
[Install]
WantedBy=multi-user.target
EOF"#, current_user);

let (_, output, _) = run_script!(
        &ipfs_service,
        &vec![],
        &options
    ). unwrap();

    println!("{}", ipfs_service);

    let options: ScriptOptions = ScriptOptions::new();
    let ipfs_cluster_service = format!(r#"cat <<EOF >> ~/.config/systemd/user/ipfs-cluster.service
[Unit]
Description=IPFS-Cluster Daemon
Requires=ipfs
After=syslog.target network.target remote-fs.target nss-lookup.target ipfs
StartLimitIntervalSec=500
StartLimitBurst=5
[Service]
Restart=on-failure
RestartSec=5s
Type=simple
ExecStart=ipfs-cluster-service daemon
User={}
[Install]
WantedBy=multi-user.target
EOF"#, current_user);

let (_, output, _) = run_script!(
        &ipfs_cluster_service,
        &vec![],
        &options
    ). unwrap();

    println!("{}", ipfs_cluster_service);

}