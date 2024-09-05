use anyhow::anyhow;
use candid::Principal;
use icrc_ledger_types::{
    icrc1::{
        account::Account,
        transfer::{BlockIndex, Memo, NumTokens, TransferArg, TransferError},
    },
    icrc2::transfer_from::{TransferFromArgs, TransferFromError},
};

pub const LEDGER_CANISTER_ID: &str = "mxzaz-hqaaa-aaaar-qaada-cai";

pub struct TransferInParams {
    pub amount: NumTokens,
    pub from: Account,
    pub memo: Option<Memo>,
}

pub struct TransferOutParams {
    pub amount: NumTokens,
    pub to: Account,
    pub memo: Option<Memo>,
}

fn canister_default_account() -> Account {
    Account::from(ic_cdk::api::id())
}

fn ledger_canister_principal() -> Principal {
    Principal::from_text(LEDGER_CANISTER_ID).expect("Could not decode the principal.")
}

pub async fn transfer_in(args: TransferInParams) -> anyhow::Result<()> {
    ic_cdk::println!(
        "Transferring {} tokens from account {}",
        &args.amount,
        &args.from,
    );

    let transfer_from_args = TransferFromArgs {
        from: Account::from(ic_cdk::caller()),
        // maybe add something valuable here
        memo: args.memo,
        amount: args.amount,
        spender_subaccount: None,
        // TODO: inspect what id exactly do
        fee: None,
        to: canister_default_account(),
        created_at_time: None,
    };

    let block_index =
        ic_cdk::call::<(TransferFromArgs,), (Result<BlockIndex, TransferFromError>,)>(
            ledger_canister_principal(),
            "icrc2_transfer_from",
            (transfer_from_args,),
        )
        .await
        .map_err(|e| anyhow!("failed to call ledger: {:?}", e))?
        .0
        .map_err(|e| anyhow!("ledger transfer error {:?}", e))?;

    ic_cdk::println!("Transfer successful. Block index: {:?}", block_index);

    Ok(())
}

pub async fn transfer_out(args: TransferOutParams) -> anyhow::Result<()> {
    ic_cdk::println!(
        "Transferring {} tokens to account {}",
        &args.amount,
        &args.to,
    );

    let transfer_args = TransferArg {
        from_subaccount: None,
        memo: args.memo,
        amount: args.amount,
        to: args.to,
        fee: None,
        created_at_time: None,
    };

    let block_index = ic_cdk::call::<(TransferArg,), (Result<BlockIndex, TransferError>,)>(
        ledger_canister_principal(),
        "icrc1_transfer",
        (transfer_args,),
    )
    .await
    .map_err(|e| anyhow!("failed to call ledger: {:?}", e))?
    .0
    .map_err(|e| anyhow!("ledger transfer error {:?}", e))?;

    ic_cdk::println!("Transfer successful. Block index: {:?}", block_index);

    Ok(())
}
