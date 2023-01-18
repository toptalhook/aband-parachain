
# 1. start a collator node
```commandline
./target/release/aband --chain local --collator --ws-port 9922 --base-path ./target/release/db -- --chain polkadot-launch/rococo-local-raw.json
```

# 2. Create stash and controller Account
click [https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9922#/accounts](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9922#/accounts),
create `C` 和 `S`.
And transfer 5000 tokens to `C` and `S` respectively.
![C_S](./images/C_S.png)
# 3. generate session-keys
click [https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9922#/rpc](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9922#/rpc) `rotateKeys`,
and copy the keys.
![session-keys](./images/session-keys.png)
# 4. `+ Validator`
 On [https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9922#/staking/actions](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9922#/staking/actions), found `+ Validator` and click it,
follow the steps and paste the keys in 3.
![validator](./images/validator.png)

After successful operation, we will find that `S` has been added in ***Waiting***,
![Waiting](./images/waiting.png)

# 5. Set validators count.

What we find is that the validators has reached `2/2` (100%),
![2_2](./images/2_2.png)
Therefore, in order for the validator added above to be able to enter the validators set, we have increased the maximum number of validators.
On [https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9922#/sudo](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9922#/sudo),
chose `staking`module,  `setValidatorCount`
![set_validator_count](./images/set_validator_count.png)
validators count update successfully.
![set_validator_count_success](./images/set_validator_count_success.png)
Waiting for the end of the era, we will find that 'S' has successfully entered the validators set and the collators set.
![validator_success](./images/validators.png)
![collator_success](./images/collator_success.png)



