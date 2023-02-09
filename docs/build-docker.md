

#  clone polkadot

Run the following command in the parent directory of `substrate-parachain-PoS-tempalate`.

```commandline
git clone https://github.com/paritytech/polkadot.git
```

# build

Run the following command in the `substrate-parachain-PoS-tempalate` directory
```commandline
sudo ./scripts/build.sh
sudo ./scripts/start_testnet.sh
```
# Run your mainnet node
```commandline
sudo docker run --name aband-local-node -it aband:latest 
```
> No blocks will be generated, because it is connected to kusama and needs to bid for slots.
# Run your testnet 
```commandline
sudo docker run --name testnet -it testnet:latest 
```
> Run the testnet locally. Docker contains relay chain and parachain nodes.

