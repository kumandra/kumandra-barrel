## Follow this guide if you want to become a leader of a cluster pool

### Connect to Kumandra Gateway
```bash
    kumandra-barrel --bootstrap
```
this will connect you with the Kumandra Gateway peers.

### Generate a pool
```bash
    kumandra-barrel --create-pool --storage-capacity 10G
```
> This program will generate a pool for you and secret-key, share this key and the ipfs peer address with other who want to be in the cluster pool with you.

### Start Kumandra-barrel
```bash
    kumandra-barrel start
```
You should only let other follower, pledge the same amount of you, will explain more details in the future.

### Provide API to the network
TODO: need to have domain name and public id, as a leader of the pool need to provide upload API