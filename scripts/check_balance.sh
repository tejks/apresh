dfx canister call icrc1_ledger_canister icrc1_balance_of "(record {                  
    owner = principal \"$1\";   
})"