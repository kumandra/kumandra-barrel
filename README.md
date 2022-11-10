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

**Step 2:** Build a bucket

```shell
git clone https://github.com/CESSProject/cess-bucket.git
cd kumandra-barrel/
go build -o barrel cmd/main/main.go
```

If everything going well, you will see a program call `barrel`


## License
Kumandra-Barrel is implemented from CESSProject. Licensed under [Apache 2.0](https://github.com/kumandra/kumandra-barrel/blob/main/LICENSE)
