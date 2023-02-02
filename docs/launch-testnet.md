
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
	cargo build --release
	cd ..
	```
- launch testnet
	```commandline
	cd substrate-parachain-PoS-template
 	cargo build --release
	cd polkadot-launch
	polkadot-launch config.json
	```


