use beerus_api::gen::{
    Address, BlockId, BlockNumber, BlockTag, BroadcastedInvokeTxn, BroadcastedTxn, Felt, FunctionCall, GetBlockWithTxHashesResult, GetBlockWithTxsResult, GetTransactionByBlockIdAndIndexIndex, InvokeTxn, InvokeTxnV1, InvokeTxnV1Version, PriceUnit, Rpc, StorageKey, SyncingResult, Txn, TxnExecutionStatus, TxnHash, TxnStatus
};

mod common;

#[tokio::test]
#[allow(non_snake_case)]
async fn test_specVersion() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let ret = ctx.client.specVersion().await?;
    assert_eq!(ret, "0.6.0");
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_chainId() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let ret = ctx.client.chainId().await?;
    assert_eq!(ret.as_ref(), "0x534e5f4d41494e");
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_blockHashAndNumber() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let ret = ctx.client.blockHashAndNumber().await?;
    assert!(*ret.block_number.as_ref() > 600612);
    assert!(!ret.block_hash.0.as_ref().is_empty());
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_blockNumber() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let ret = ctx.client.blockNumber().await?;
    assert!(*ret.as_ref() > 600612);
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_call() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let request = FunctionCall {
        calldata: Vec::default(),
        contract_address: Address(Felt::try_new(
            "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        )?),
        entry_point_selector: Felt::try_new(
            "0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60",
        )?,
    };

    let block_id =
        BlockId::BlockNumber { block_number: BlockNumber::try_new(33482)? };

    let ret = ctx.client.call(request, block_id).await?;
    assert_eq!(ret.len(), 1);
    assert_eq!(ret[0].as_ref(), "0x4574686572");
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_estimateFee() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let calldata = vec![
        "0x2",
        "0x57c4b510d66eb1188a7173f31cccee47b9736d40185da8144377b896d5ff3",
        "0x2f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354",
        "0x0",
        "0x1",
        "0x57c4b510d66eb1188a7173f31cccee47b9736d40185da8144377b896d5ff3",
        "0x2f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354",
        "0x1",
        "0x1",
        "0x2",
        "0x0",
        "0x1",
    ];
    let calldata: Result<Vec<Felt>, _> =
        calldata.into_iter().map(|felt| Felt::try_new(felt)).collect();

    let signature = vec![
        "0x42527ffe9912b338983cbed67e139cfcc26a4d8cf1d1c2a85e4125fdf5f59ed",
        "0x636147d06fefd02ed37984b752556d4b9aefdac1a50b3df0528ec7c201ad84b",
    ];
    let signature: Result<Vec<Felt>, _> =
        signature.into_iter().map(|felt| Felt::try_new(felt)).collect();

    let request = vec![
        BroadcastedTxn::BroadcastedInvokeTxn(
            BroadcastedInvokeTxn(
                InvokeTxn::InvokeTxnV1(
                    InvokeTxnV1 {
                        calldata: calldata?,
                        signature: signature?,
                        sender_address: Address(Felt::try_new("0x13e3ca9a377084c37dc7eacbd1d9f8c3e3733935bcbad887c32a0e213cd6fe0")?), 
                        max_fee: Felt::try_new("0x28ed6103d0000")?, 
                        nonce: Felt::try_new("0x1")?,
                        version: InvokeTxnV1Version::V0x1,
                        r#type: beerus_api::gen::InvokeTxnV1Type::Invoke, 
                    }
                )
            )
        )
    ];

    let simulation_flags = vec![];

    let block_id =
        BlockId::BlockNumber { block_number: BlockNumber::try_new(59999)? };

    let ret =
        ctx.client.estimateFee(request, simulation_flags, block_id).await?;
    assert_eq!(ret.len(), 1);
    assert_eq!(ret[0].overall_fee.as_ref(), "0x1abd7b153e472");
    assert_eq!(ret[0].gas_price.as_ref(), "0x67edb4f57");
    assert_eq!(ret[0].gas_consumed.as_ref(), "0x41de");
    assert!(matches!(ret[0].unit, PriceUnit::Wei));
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getBlockTransactionCount() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let block_id = BlockId::BlockTag(BlockTag::Latest);

    let ret = ctx.client.getBlockTransactionCount(block_id).await?;
    assert!(*ret.as_ref() > 0);
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getBlockWithTxHashes() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let block_id = BlockId::BlockTag(BlockTag::Latest);

    let ret = ctx.client.getBlockWithTxHashes(block_id).await?;
    assert!(matches!(ret, GetBlockWithTxHashesResult::BlockWithTxHashes(_)));
    let GetBlockWithTxHashesResult::BlockWithTxHashes(ret) = ret else {
        panic!("unexpected pending block");
    };
    assert!(ret.block_body_with_tx_hashes.transactions.len() > 0);
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getBlockWithTxs() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let block_id = BlockId::BlockTag(BlockTag::Latest);

    let ret = ctx.client.getBlockWithTxs(block_id).await?;
    assert!(matches!(ret, GetBlockWithTxsResult::BlockWithTxs(_)));
    let GetBlockWithTxsResult::BlockWithTxs(ret) = ret else {
        panic!("unexpected pending block");
    };
    assert!(ret.block_body_with_txs.transactions.len() > 0);
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_syncing() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let ret = ctx.client.syncing().await?;
    assert!(matches!(ret, SyncingResult::SyncStatus(_)));
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getNonce() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let block_id = BlockId::BlockTag(BlockTag::Latest);

    let address = Address(Felt::try_new(
        "0x10b6c96d364cf182964fbd4a3438a5ae84cab990770c07994f9cb99fd26f6dc",
    )?);

    let ret = ctx.client.getNonce(block_id, address).await?;
    assert!(!ret.as_ref().is_empty());
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getTransactionByHash() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let hash =
        "0x2e2a98c1731ece2691edfbb4ed9b057182cec569735bd89825f17e3b342583a";

    let transaction_hash = TxnHash(Felt::try_new(hash)?);

    let ret = ctx.client.getTransactionByHash(transaction_hash).await?;
    assert!(matches!(ret.txn, Txn::InvokeTxn(InvokeTxn::InvokeTxnV1(_))));
    assert_eq!(ret.transaction_hash.0.as_ref(), hash);
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getTransactionByBlockIdAndIndex() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let block_id = BlockId::BlockTag(BlockTag::Latest);

    let index = GetTransactionByBlockIdAndIndexIndex::try_new(0)?;

    let ret =
        ctx.client.getTransactionByBlockIdAndIndex(block_id, index).await?;
    assert!(!ret.transaction_hash.0.as_ref().is_empty());
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getStorageAt() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let contract_address = Address(Felt::try_new(
        "0x6a05844a03bb9e744479e3298f54705a35966ab04140d3d8dd797c1f6dc49d0",
    )?);

    let key = StorageKey::try_new(
        "0x0341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1",
    )?;

    let block_id =
        BlockId::BlockNumber { block_number: BlockNumber::try_new(600612)? };

    let ret = ctx.client.getStorageAt(contract_address, key, block_id).await?;
    assert_eq!(ret.as_ref(), "0x47616d65206f66204c69666520546f6b656e");
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getProof() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let contract_address = Address(Felt::try_new(
        "0x49D36570D4e46f48e99674bd3fcc84644DdD6b96F7C741B1562B82f9e004dC7",
    )?);

    let key = StorageKey::try_new(
        "0x02c401056f9582175d3219f1ac8f974b7960f2edfc8bc03197718dc8967ba1ab",
    )?;

    let block_id =
        BlockId::BlockNumber { block_number: BlockNumber::try_new(354824)? };

    let ret = ctx.client.getProof(block_id, contract_address, vec![key]).await?;
    assert_eq!(ret.class_commitment.unwrap().as_ref(), "0x4570dad16b85ea5076806bfb74c85bbb2b38485e6f3bd1bf163ab5f9ce1de53");
    assert_eq!(ret.state_commitment.unwrap().as_ref(), "0xd9b8e8d51f3f284e62eb8c1fd7278c20bd4c0cd3033c4cce32c513e93ed663");
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getTransactionStatus() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let transaction_hash = TxnHash(Felt::try_new("0x2e2a98c1731ece2691edfbb4ed9b057182cec569735bd89825f17e3b342583a")?);

    let ret = ctx.client.getTransactionStatus(transaction_hash).await?;
    assert!(matches!(ret.execution_status, Some(TxnExecutionStatus::Succeeded)));
    assert!(matches!(ret.finality_status, TxnStatus::AcceptedOnL1));
    Ok(())
}

/*
#[tokio::test]
#[allow(non_snake_case)]
async fn test_?() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let ret = ctx.client.?().await?;
    println!("{ret:#?}");

    assert_eq!(ret, ?);
    Ok(())
}
*/

// TODO: getClass
// TODO: getClassAt
// TODO: getClassHashAt
// TODO: getTransactionReceipt
