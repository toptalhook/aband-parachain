0. install rust
[https://docs.substrate.io/install/linux/](https://docs.substrate.io/install/linux/)
1. install `node` >= 16
	```commandline
	wget https://github.com/nodejs/node/archive/refs/tags/v16.18.1.tar.gz
	make
	make install
	```
2. install yarn and polkadot-launch by npm
	```commandline
	npm install -g yarn
	npm i polkadot-launch -g
	```
3. launch your testnet

- build polkadot
	```commandline
	git clone https://github.com/paritytech/polkadot.git
	cd polkadot
 	git fetch origin release-v0.9.37
	git checkout release-v0.9.37
	cargo build --release
	cd ..
	```
- launch testnet
	```commandline
	cd substrate-parachain-PoS-template
	git fetch origin aband-polkadot-v0.9.37
	git checkout aband-polkadot-v0.9.37
 	cargo build --release
	cd polkadot-launch
	polkadot-launch config.json
	```


