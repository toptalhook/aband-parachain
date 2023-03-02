

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
sudo docker run -it \
--name aband-mainnet \
--net host \
-p 30333:30333 \
-p 9933:9933 \
-p 9944:9944 \
-p 9615:9615 \
aband-mainnet:latest
```
> No blocks will be generated, because it is connected to kusama and needs to bid for slots.
# Run your testnet
```commandline
sudo docker run -it \
--name abandtesnet \
--net host \
-p 9966:9966 \
-p 30666:30666 \
-p 9955:9955 \
-p 30555:30555 \
-p 9944:9944 \
-p 30333:30333 \
-p 9999:9999 \
-p 31300:31300 \
aband-tesnet:latest
```
> Run the testnet locally. Docker contains relay chain and parachain nodes.

