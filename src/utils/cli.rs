use std::env;
use crate::utils::bootstrap_peer::bootstrap_peer;
use crate::utils::start::kumandra_start;
use crate::utils::create_pool::create_pool;
use crate::utils::leader_peer::insert_leader_peer;
use crate::utils::secret_key::insert_secret_key;

pub fn kumandra_cli() {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        help();
    } else {
        match args[0].as_ref() {
            "create-pool" | "--create-pool" | "-cp" | "cp" => {
                create_pool();
            }
            "--secret-key" | "secret-key" | "-sk" | "sk" | "--key" | "key" => {
                // println!("This is the secret key: {}", args[1]);
                insert_secret_key(args[1].to_string());
            }
            "--leader-addr" | "leader-addr" | "-la" | "la" => {
                println!("This is the leader peer of the cluster pool: {}", args[1]);
                insert_leader_peer(args[1].to_string());
            }
            "--start" | "start" | "-s" | "s"  => {
                println!("Starting Pledging Storage to the cluster");
                kumandra_start();
            }
            "--stop" | "stop" => {
                println!("Stopping Pledging Storage to the cluster");
            }
            "--update" | "update" | "-u" | "u" => {
                println!("Updating the Software");
            }
            "--bootstrap" | "bootstrap" | "-b" | "b" => {
                println!("Bootstraping Kumandra IPFS Peer");
                bootstrap_peer();
            }
            _ => {
                help()
            }
        }
    }
}

fn help() {
    print!(
        r#"
USAGE:
kumandra-barrel version 0.1.0
OPTIONS:
    --bootstrap   | bootstrap    -> add Kumandra Bootstrap
    --create-pool | create-pool  -> create IPFS cluster pool
    --secret-key  | secret-key   -> import secret-key
    --leader-addr | leader-addr  -> add bootstrap leader address to ipfs-cluster
    --start       | start        -> start pledging storage resource
    --stop        | stop         -> stop pledging storage resource
"#
    );
}




