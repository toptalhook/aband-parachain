# ***Collators***

## Overview
The role of the Collators pallet is to provide a collators set for consensus.
The validator can come from the staking module, or can also be set by `AuthorityOrigin` in this module.
It means that with this template, you can also use the Staking function in the case of PoA,
which is very useful if you just only want to reward collators.

***
## All Calls
Only `AuthorityOrigin` can execute, ordinary users cannot execute all calls of this module.

- `close_pos` Close PoS, and use PoA instead.
- `open_pos` Reopen PoS.
- `set_collators` Set collators set.
- `add_collator` Add collator.
- `remove_collator` Remove collator.
