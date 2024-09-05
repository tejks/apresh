dfx identity use anonymous

dfx canister call icrc1_ledger_canister icrc1_transfer "(record {                  
  amount = 100_000;
  to = record {
    owner = principal \"$1\";  
  };  
})"

dfx identity use default