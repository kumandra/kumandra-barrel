# Kumandra-Barrel
Storage Mining tool for Kumandra Network

## Build
## Build from source

**Step 1:** Install go locale

Kumandra-Barrel requires [Go 1.19](https://golang.org/dl/) or higher.

> See the [official Golang installation instructions](https://golang.org/doc/install) If you get stuck in the following process.

- Download go1.19 compress the package and extract it to the /use/local directory:

```shell
sudo wget -c https://golang.org/dl/go1.19.linux-amd64.tar.gz -O - | sudo tar -xz -C /usr/local
```

- You'll need to add `/usr/local/go/bin` to your path. For most Linux distributions you can run something like:

```shell
echo "export PATH=$PATH:/usr/local/go/bin" >> ~/.bashrc && source ~/.bashrc
```

- View your go version:

```shell
go version
```

**Step 2:** Build a barrel

```shell
git clone https://github.com/kumandra/kumandra-barrel.git
cd kumandra-barrel/
go build -o barrel cmd/main/main.go
```

If everything going well, you will see a program call `barrel`

# Start Mining with Barrel

**Step 1:** Create two wallet account from [Kumandra Explorer](https://testnet.kumandra.org)

For wallet one, it is called an  `income account`, which is used to receive rewards from mining, and you should keep the private key carefully.

For wallet two, it is called a `signature account`, which is used to sign on-chain transactions. You need to recharge the account with a small tokens and provide the private key to the miner's configuration file. The kumandra system will not record and destroy the account.

**Step 2:** Recharge your signature account

If you are using the test network, we have [faucet](https://faucet.kumandra.org) to get it for free.

**Step 3:** Prepare configuration file

Use `barrel` to generate configuration file templates directly in the current directory:

```shell
sudo chmod +x barrel
./barrel default
```
The content of the configuration file template is as follows. You need to fill in your own information into the file. By default, the `barrel` uses `conf.toml` in the current directory as the runtime configuration file. You can use `-c` or `--config` to specify the configuration file Location.
```
# The rpc address of the chain node
RpcAddr      = ""
# Path to the mounted disk where the data is saved
MountedPath  = ""
# Total space used to store files, the unit is GB
StorageSpace = 0
# The IP of the machine running the mining service
ServiceIP    = ""
# Port number monitored by the mining service
ServicePort  = 0
# The address of income account
IncomeAcc    = ""
# phrase of the signature account
SignatureAcc = ""
# If 'ServiceIP' is not public IP, You can set up a domain name
DomainName   = ""
```
*Our testnet rpc address is as follows:*<br>
`wss://testnet-rpc0.kumandra.org`<br>
`wss://testnet-rpc1.kumandra.org`

**Step 4:** View barrel features

The `barrel` has many functions, you can use `-h` or `--help` to view, as follows:

- flag

| Flag        | Description                             |
| ----------- | --------------------------------------- |
| -c,--config | Custom profile |
| -h,--help   | Print help information                  |

- command

| Command  | Description                                    |
| -------- | ---------------------------------------------- |
| version  | Print version number                           |
| default  | Generate configuration file template           |
| register | Register mining miner information to the chain |
| state    | Query mining miner information                 |
| run      | Register and run the mining program            |
| exit     | Exit the mining platform                       |
| increase | Increase the deposit of mining miner           |
| withdraw | Redemption deposit of mining miner             |
| update_address | Update the miner's access address             |
| update_income  | Update the miner's income account             |

**Step 5:** Use barrel

*All `barrel` commands (except default and version) need to be registered before they can be used.*

```shell
sudo ./barrel register
```

- Query miner status

```shell
sudo ./barrel state
```

- Increase the miner's deposit by 1000

```shell
sudo ./barrel increase 1000
```

- Exit the mining platform

```shell
sudo ./barrel exit
```

- Redeem the miner's deposit

```shell
sudo ./barrel withdraw
```

- Update the miner's access address
```shell
sudo ./barrel update_address <ipv4>:<port>
```

- Update the miner's income account
```shell
sudo ./barrel update_income cXic3Whct......vV5Sbq4f
```

- Start mining

```shell
sudo nohup ./barrel run 2>&1 &
```

## License
Kumandra-Barrel is implemented from CESSProject. Licensed under [Apache 2.0](https://github.com/kumandra/kumandra-barrel/blob/main/LICENSE)
