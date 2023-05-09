# Producer-Consumer-Rust
## Program Usage
While in src folder, in terminal, run:

"cargo run -- (number of threads) (ledger file)"
#
## main.rs
- Takes in number of threads and filename from command line and passes it to the initBank function (which is in the ledger.rs file)

## ledger.rs
* Imports "Bank" Module
- Defines emum: Mode (Transfer, Withdraw, Deposit)
- Defines struct: Ledger
* function: initBank()
    * Defines list of Ledger objects wrapped in a Lock, wrapped in a Smart Pointer
    * Defines Bank object wrapped in a Smart Pointer
    1. Load ledger file contents into the ledger
    2. Spawn worker threads, calling "worker" function for each and passing the worker id, bank, and ledger list as copied Smart Pointers
    3. Join worker theads
##
* function: loadLedger()
    1. Read in contents of ledger file into the passed in ledger list
##
* function: worker()
    1. Grab the lock on the ledger list
    2. While ledger list is not empty, remove the top item and save it, if not, exit
    3. Give up lock on ledger list
    4. Call deposit(), withdraw(), or transfer() on the bank object
    5. Regain lock on ledger list
    6. Go back to step 2

## bank.rs
- Defines struct: Account (stores account ID and balance)
- Defines struct: Bank (number of accounts, number of successes, number of fails, list of accounts)
### All below functions are part of the Bank struct
* function: new()
    1. Creates a new instance of the Bank struct with a list of empty accounts. It also has a num_fail and num_succ field that are both protected by a lock wrapped in a Smart Pointer.
##
* function: printAccounts()
    1. Print balances for all accounts and total success and fail count
##
* function: recordFail()
    1. Increment the fail count
##
* function: recordSucc()
    1. Increment the success count
##
* function: deposit()
    1. Increase balance in the account by the given amount
    2. Record a success
##
* function: withdraw()
    1. Decrease balance in the account by the given amount
    2. If there isn't enough in the account to take out, record a fail, otherwise record a success
##
* function: transfer()
    * If transfering from one account to the same account, exit and record a fail
    1. Lock both accounts, locking the one with the lower account ID first
    2. Take money from the designated account and put it in the other
    3. If there isn't enough to take out from the designated account, record a fail, otherwise record a success