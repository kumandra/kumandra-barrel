use std::process::Command;
use std::env;
use crate::utils::bootstrap_peer::bootstrap_peer;
use crate::utils::start::kumandra_start;

pub fn kumandra_cli() {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        help();
    } else {
        match args[0].as_ref() {
            "create-pool" | "--create-pool" | "-cp" | "cp" => {
                println!("Creating IPFS Cluster Pool");
                let secretkey = Command::new("bash")
                    .args(["-c", "kumandra-swarm-key-gen"])
                    // .args(["|", "head"])
                    .output()
                    .expect("kumandra-swarm-key-gen command failed to start");

                let stdout = String::from_utf8(secretkey.stdout).unwrap();

                let s: Box<str> = stdout.into_boxed_str();

                println!("save the secret key to somewhere safe: {:?}", &s[31..]);
            }
            "--secret-key" | "secret-key" | "-sk" | "sk" | "--key" | "key" => {
                println!("This is the secret key: {}", args[1]);
                // TODO: Write the secret key to the environment
            }
            "--start" | "start" | "-s" | "s"  => {
                println!("Starting Pledging Storage to the cluster");
                kumandra_start();
                // TODO: 
            }
            "--stop" | "stop" => {
                println!("Stopping Pledging Storage to the cluster");
                // TODO: 
            }
            "--update" | "update" | "-u" | "u" => {
                println!("Updating the Software");
                // TODO: 
            }
            "--bootstrap" | "bootstrap" | "-b" | "b" => {
                println!("Bootstraping Kumandra IPFS Peer");
                bootstrap_peer();
                // TODO: 
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
--create-pool | create-pool  => create IPFS cluster pool
--secret-key  | secret-key   => import secret-key
--start       | start        => start pleding storage resource
--stop        | stop         => stop pleding storage resource
"#
    );
}


    // let secret_key = String::new();
    // let matches = Command::new("pacman")
    //     .about("Storage Worker Tool")
    //     .version("0.1.0")
    //     .subcommand_required(true)
    //     .arg_required_else_help(true)
    //     .author("KOOMPI Teams")
    //     // create-pool subcommand
    //     //
    //     // Only a few of its arguments are implemented below.
    //     .subcommand(
    //         Command::new("create-pool")
    //             .short_flag('p')
    //             .long_flag("create-pool")
    //             .about("Create IPFS Cluster Pool."),
    //     )
    //     // Secret-key subcommand
    //     //
    //     // Only a few of its arguments are implemented below.
    //     .subcommand(
    //         Command::new("secret-key")
    //             .short_flag('k')
    //             .long_flag("secret-key")
    //             .about("Add Cluster Secret Key")
    //             .arg(arg!(-c --config <CONFIG> "Optionally sets a config file to use"))
    //             .action(ArgAction::Set)
    //             .num_args(1..),
    //     )
    //     .get_matches();

    // match matches.subcommand() {
    //     Some(("secret-key", key)) => {
    //         println!("this is the secret key: {:?}", key.contains_id("config"));
    //         // println!("{:?}", key);
    //         if key.contains_id("secret-key") {
    //             let key: Vec<_> = key
    //                 .get_many::<String>("secret-key")
    //                 .expect("contains_id")
    //                 .map(|s| s.as_str())
    //                 .collect();
    //             // let values = key.join(", ");
    //             println!("Secret key {:?}...", key);
    //             return;
    //         }

    //         println!("{}", secret_key);
    //         // if sync_matches.contains_id("search") {
    //         //     let packages: Vec<_> = sync_matches
    //         //         .get_many::<String>("search")
    //         //         .expect("contains_id")
    //         //         .map(|s| s.as_str())
    //         //         .collect();
    //         //     let values = packages.join(", ");
    //         //     println!("Searching for {}...", values);
    //         //     return;
    //         // }

    //         // let packages: Vec<_> = sync_matches
    //         //     .get_many::<String>("package")
    //         //     .expect("is present")
    //         //     .map(|s| s.as_str())
    //         //     .collect();
    //         // let values = packages.join(", ");

    //         // if sync_matches.get_flag("info") {
    //         //     println!("Retrieving info for {}...", values);
    //         // } else {
    //         //     println!("Installing {}...", values);
    //         // }
    //     }
    //     Some(("create-pool", _)) => {
    //         let secretkey = RunCommand::new("bash")
    //             .args(["-c", "kumandra-swarm-key-gen"])
    //             // .args(["|", "head"])
    //             .output()
    //             .expect("kumandra-swarm-key-gen command failed to start");

    //         let stdout = String::from_utf8(secretkey.stdout).unwrap();

    //         let s: Box<str> = stdout.into_boxed_str();

    //         println!("copy the secret key to somewhere safe: {:?}", &s[31..]);
    //     }
    //     _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    // }

