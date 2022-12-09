## Follow this guide if you want to follow a leader of a cluster pool

### Connect to the cluster pool
You need to make sure you have the cluster pool secretkey and ipfs address of the leader pool or any member in the pool fine too.
```bash
    kumandra-barrel \
    --storage-capacity 10G \
    --connect-peer "/ip4/192.168.10.1/tcp/9096/ipfs/QmZjSoXUQgJ9tutP1rXjjNYwTrRM9QPhmD9GHVjbtgWxEn" \
    --secret-key "f6e276d61ad6b382aa3072b67ab1992352f0e6e88c558363143a7cdc87e3c61b"
```