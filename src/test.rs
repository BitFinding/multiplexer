
use crate::{FlowBuilder, DELEGATE_PROXY_INIT, EXECUTOR_INIT};
use alloy::{
    hex,
    network::TransactionBuilder,
    primitives::{address, bytes, uint, Address, U256},
    providers::{ext::AnvilApi, layers::AnvilProvider, Provider, ProviderBuilder, RootProvider},
    rpc::types::TransactionRequest,
    sol,
    sol_types::{SolCall, SolConstructor},
    transports::http::{Client, Http},
};
use core::str;

// 1000e18
const BUDGET: U256 = uint!(1000000000000000000000_U256);
// 2e18
const TWO_ETH: U256 = uint!(2000000000000000000_U256);
const WALLET: Address = Address::repeat_byte(0x41);
const BOB: Address = Address::repeat_byte(0x42);
const WETH9: Address = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
fn get_provider() -> AnvilProvider<RootProvider<Http<Client>>, Http<Client>> {
    ProviderBuilder::new().on_anvil_with_config(|anvil| {
        anvil
            .fork(std::env::var("ETH_RPC_URL").expect("failed to retrieve ETH_RPC_URL url from env"))
            .fork_block_number(20000000)
    })
}

sol! {
    #[sol(rpc)]
    interface IERC20 {
        event Transfer(address indexed from, address indexed to, uint256 value);
        event Approval(address indexed owner, address indexed spender, uint256 value);
        function totalSupply() external view returns (uint256);
        function balanceOf(address account) external view returns (uint256);
        function transfer(address to, uint256 value) external returns (bool);
        function allowance(address owner, address spender) external view returns (uint256);
        function approve(address spender, uint256 value) external returns (bool);
        function transferFrom(address from, address to, uint256 value) external returns (bool);
    }
}

sol! {
    #[sol(rpc)]
    contract IProxy {
        constructor(address _target, bytes memory constructorData) payable;
    }
}

sol! {
    #[sol(rpc)]
    interface IWETH {
        function deposit() external payable;
        function transfer(address to, uint value) external returns (bool);
        function withdraw(uint amount) external;
    }
}

#[test]
fn test() {
    // Basic smoke test for the FlowBuilder
    let addr_a = Address::repeat_byte(0x41);
    let addr_b = Address::repeat_byte(0x42);
    let calldata = FlowBuilder::empty()
        .create(Address::ZERO, "LALA".as_bytes(), U256::from(10))
        .call(addr_a, &vec![98, 99], U256::ZERO)
        .delegatecall(addr_b, &vec![70, 71])
        .build(true);
    assert_eq!(calldata, hex!("c94f554d03000000000000000000000000000000000000000000000000000000000000000a00000401000000044c414c410602414141414141414141414141414141414141414100000201000000026263050242424242424242424242424242424242424242420100000002464707"));
}

#[tokio::test]
async fn test_bob_cannot_interact() {
    // A random account can not interact with multiplexer
    let provider = ProviderBuilder::new().on_anvil();
    provider
        .anvil_set_balance(WALLET, BUDGET + U256::from(10u64.pow(18)))
        .await
        .unwrap();
    provider
        .anvil_set_balance(BOB, BUDGET + U256::from(10u64.pow(18)))
        .await
        .unwrap();

    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_deploy_code(EXECUTOR_INIT)
        .with_nonce(0);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

    provider.evm_mine(None).await.unwrap();

    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();
    assert!(receipt.status());
    let executor_wallet = receipt.contract_address.unwrap();

    // Executor address is deterministic because we use always same WALLET and nonce.
    assert_eq!(
        address!("c088f75b5733d097f266010c1502399a53bdfdbd"),
        executor_wallet
    );

    let tx = TransactionRequest::default()
        .with_from(BOB)
        .with_to(executor_wallet)
        .with_nonce(1)
        .with_value(BUDGET);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

    provider.evm_mine(None).await.unwrap();

    let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap();

    assert!(receipt.is_none()); // Tx can not be send from bob
}

#[tokio::test]
async fn test_wallet_can_interact() {
    let provider = ProviderBuilder::new().on_anvil();
    provider
        .anvil_set_balance(WALLET, BUDGET + U256::from(10u64.pow(18)))
        .await
        .unwrap();
    provider
        .anvil_set_balance(BOB, BUDGET + U256::from(10u64.pow(18)))
        .await
        .unwrap();

    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_deploy_code(EXECUTOR_INIT)
        .with_nonce(0);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();

    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();
    let executor_wallet = receipt.contract_address.unwrap();
    // Executor address is deterministic because we use always same WALLET and nonce.
    assert_eq!(
        address!("c088f75b5733d097f266010c1502399a53bdfdbd"),
        executor_wallet
    );

    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor_wallet)
        .with_nonce(1)
        .with_value(BUDGET);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

    provider.evm_mine(None).await.unwrap();

    let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap();

    assert!(receipt.is_some()); // Tx succeed from WALLET

    let account_balance = provider.get_balance(executor_wallet).await.unwrap();
    assert_eq!(account_balance, BUDGET); // executor has the money sent in empty tx
}

#[tokio::test]
async fn test_wallet_can_proxy_call() {
    let provider = get_provider();

    // reality check
    let weth9_balance = provider.get_balance(WETH9).await.unwrap();
    assert_eq!(format!("{}", weth9_balance), "2933633723194923479377016");

    // test WALLETs
    // 0x4141414141..4141414141  with 1001 eth
    // 0x4242424242..4242424242  with 1001 eth
    provider
        .anvil_set_balance(WALLET, BUDGET + U256::from(1e18 as u64))
        .await
        .unwrap();
    provider
        .anvil_set_balance(BOB, BUDGET + U256::from(1e18 as u64))
        .await
        .unwrap();

    // Make the Executor contract (WALLET is the owner)
    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_deploy_code(EXECUTOR_INIT)
        .with_nonce(0);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();
    assert!(receipt.status());
    let executor = receipt.contract_address.unwrap();

    // 0 eth
    // 0 weth
    let executor_balance = provider.get_balance(executor).await.unwrap();
    assert_eq!(executor_balance, U256::ZERO); // executor shoud shave sent the value to WETH9
    let weth9_contract = IERC20::new(WETH9, provider.clone());
    let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap()._0;
    assert_eq!(executor_weth_balance, U256::ZERO); // executor should have 2 eth worth of weth

    // this should send 2 eth to weth and assign the same weth value to the executor
    // SETADDR 02 c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2
    // SETVALUE 03 0000000000000000000000000000000000000000000000001bc16d674ec80000
    // CLRDATA 00 0000
    // SETDATA 01 0000 0000
    // 05
    let fb = FlowBuilder::empty().call(WETH9, &bytes!(""), TWO_ETH).build(true);
    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor)
        .with_nonce(1)
        .with_value(TWO_ETH)
        .with_input(fb);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

    provider.evm_mine(None).await.unwrap();

    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();

    assert!(receipt.status());

    // 0 eth
    // 2 weth
    let executor_balance = provider.get_balance(executor).await.unwrap();
    assert_eq!(executor_balance, U256::ZERO); // executor shoud shave sent the value to WETH9

    let weth9_contract = IERC20::new(WETH9, provider.clone());
    let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap()._0;
    assert_eq!(executor_weth_balance, TWO_ETH); // executor should have 2 eth worth of weth

    let withdraw_calldata = IWETH::withdrawCall { amount: TWO_ETH }.abi_encode();
    let fb = FlowBuilder::empty().call(WETH9, &withdraw_calldata, U256::ZERO).build(true); // this should send 2 eth to weth and assign the same weth value to the executor

    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor)
        .with_value(U256::ZERO)
        .with_input(fb);

    println!("TX: {:?}", tx);
    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

    provider.evm_mine(None).await.unwrap();

    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();

    assert!(receipt.status());

    // 2 eth
    // 0 weth
    let executor_balance = provider.get_balance(executor).await.unwrap();
    assert_eq!(executor_balance, TWO_ETH); // executor shoud shave sent the value to WETH9

    let weth9_contract = IERC20::new(WETH9, provider.clone());
    let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap()._0;
    assert_eq!(executor_weth_balance, U256::ZERO); // executor should have 2 eth worth of weth
}

#[tokio::test]
async fn test_wallet_can_proxy_create() {
    let provider = get_provider();

    // reality check
    let weth_balance = provider.get_balance(WETH9).await.unwrap();
    assert_eq!(format!("{}", weth_balance), "2933633723194923479377016");

    // test WALLETs
    // 0x4141414141..4141414141  with 1001 eth
    // 0x4242424242..4242424242  with 1001 eth
    provider
        .anvil_set_balance(WALLET, BUDGET + U256::from(1e18 as u64))
        .await
        .unwrap();
    provider
        .anvil_set_balance(BOB, BUDGET + U256::from(1e18 as u64))
        .await
        .unwrap();
    // Make the Executor contract (WALLET is the owner)
    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_deploy_code(EXECUTOR_INIT)
        .with_nonce(0);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();
    assert!(receipt.status());
    let executor = receipt.contract_address.unwrap();

    // Create dellegate proxy
    // let mut calldata = DELEGATE_PROXY_INIT;
    // calldata.extend(hex!("00").repeat(12));
    // calldata.extend(executor.as_slice());
    let mut calldata = DELEGATE_PROXY_INIT.to_vec();
    calldata.extend(
        IProxy::constructorCall {
            _target: executor,
            constructorData: "".into(),
        }
        .abi_encode(),
    );

    let fb = FlowBuilder::empty()
        .create(executor.create(1), &calldata, U256::ZERO)
        .call(
            executor.create(1),
            &FlowBuilder::empty()
                .call(WETH9, &vec![], TWO_ETH)
                .build(true),
            TWO_ETH,
        )
        .build(true);

    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor)
        .with_value(TWO_ETH)
        .with_input(fb);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

    provider.evm_mine(None).await.unwrap();

    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();

    assert!(receipt.status());

    let account_balance = provider.get_balance(executor).await.unwrap();
    assert_eq!(account_balance, U256::ZERO); // executor shoud shave sent the value to WETH9
    assert_eq!(
        address!("c84f9705070281e8c800c57d92dbab053a80a2d0"),
        executor.create(1)
    );

    // Executor has
    // 0 eth
    // 0 weth
    let executor_balance = provider.get_balance(executor).await.unwrap();
    assert_eq!(executor_balance, U256::ZERO); // executor shoud shave sent the value to WETH9

    let weth9_contract = IERC20::new(WETH9, provider.clone());
    let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap()._0;
    assert_eq!(executor_weth_balance, U256::ZERO); // executor should have 2 eth worth of weth

    // Proxy crested via executor that points to the executor ?? ?AHHH
    // 0 eth
    // 2 weth

    let executor_balance = provider.get_balance(executor.create(1)).await.unwrap();
    assert_eq!(executor_balance, U256::ZERO); // executor shoud shave sent the value to WETH9

    let weth9_contract = IERC20::new(WETH9, provider.clone());
    let executor_weth_balance = weth9_contract
        .balanceOf(executor.create(1))
        .call()
        .await
        .unwrap()
        ._0;
    assert_eq!(executor_weth_balance, TWO_ETH); // executor should have 2 eth worth of weth

    // Test ownership in the created proxy
    // WALLET -> executor -> proxy mint some weth

    let withdraw_calldata = IWETH::withdrawCall { amount: TWO_ETH }.abi_encode();
    let multiplexed_withdraw_calldata = FlowBuilder::empty()
        .call(WETH9, &withdraw_calldata, U256::ZERO)
        .build(true); // multiplexed withdraw from weth

    let fb = FlowBuilder::empty().call(
        executor.create(1),
        &multiplexed_withdraw_calldata,
        U256::ZERO,
    ).build(true); // this should send 2 eth to weth and assign the same weth value to the executor

    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor)
        .with_value(TWO_ETH)
        .with_input(fb);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

    provider.evm_mine(None).await.unwrap();

    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();

    assert!(receipt.status());

    // Executor has
    // 2 eth
    // 0 weth
    let executor_balance = provider.get_balance(executor).await.unwrap();
    assert_eq!(executor_balance, TWO_ETH); // executor shoud shave sent the value to WETH9

    let weth9_contract = IERC20::new(WETH9, provider.clone());
    let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap()._0;
    assert_eq!(executor_weth_balance, U256::ZERO); // executor should have 2 eth worth of weth

    // Proxy created via executor that points to the executor ?? ?AHHH
    // 0 eth
    // 0 weth

    let executor_balance = provider.get_balance(executor.create(1)).await.unwrap();
    assert_eq!(executor_balance, U256::ZERO); // executor shoud shave sent the value to WETH9

    let weth9_contract = IERC20::new(WETH9, provider.clone());
    let executor_weth_balance = weth9_contract
        .balanceOf(executor.create(1))
        .call()
        .await
        .unwrap()
        ._0;
    assert_eq!(executor_weth_balance, TWO_ETH); // executor should have 2 eth worth of weth

    // bob -> executor -> ?? :fail:
    // bob -> proxy  :fail:
}

#[tokio::test]
async fn test_wallet_can_proxy_create_ultimate() {
    let provider = get_provider();

    // reality check
    let weth9_contract = IERC20::new(WETH9, provider.clone());
    let weth_balance = provider.get_balance(WETH9).await.unwrap();
    assert_eq!(format!("{}", weth_balance), "2933633723194923479377016");

    // test WALLETs
    // 0x4141414141..4141414141  with 1001 eth
    // 0x4242424242..4242424242  with 1001 eth
    provider
        .anvil_set_balance(WALLET, BUDGET + U256::from(1e18 as u64))
        .await
        .unwrap();
    provider
        .anvil_set_balance(BOB, BUDGET + U256::from(1e18 as u64))
        .await
        .unwrap();

    ////////////////////////////////////////////////////////////
    // Make the Executor contract (WALLET is the owner)
    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_deploy_code(EXECUTOR_INIT)
        .with_nonce(0);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();

    assert!(receipt.status());
    let executor = receipt.contract_address.unwrap();

    ////////////////////////////////////////////////////////////
    // Make the Proxy(Executor) contract (WALLET is the owner)
    // Link the proxy to the executor but do not use the delegatecall in the constructor

    let mut deploy_proxy_executor = DELEGATE_PROXY_INIT.to_vec();
    deploy_proxy_executor.extend(
        IProxy::constructorCall {
            _target: executor,
            constructorData: "".into(),
        }
        .abi_encode(),
    );

    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_deploy_code(deploy_proxy_executor)
        .with_nonce(1);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();
    assert!(receipt.status());
    let proxy_executor = receipt.contract_address.unwrap();
    assert_eq!(proxy_executor, WALLET.create(1));

    ////////////////////////////////////////////////////////////
    // Deposit weth in the proxy account
    // Use the deployed Proxy(Executor) contract (WALLET is the owner) to deposit weth
    let deposit_calldata = [];
    let fb = FlowBuilder::empty().call(WETH9, &deposit_calldata, TWO_ETH).build(true);

    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(proxy_executor)
        .with_value(TWO_ETH)
        .with_input(fb);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();

    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();
    assert!(receipt.status());

    // Executor account has no assets
    // 0 eth
    // 0 weth
    let executor_balance = provider.get_balance(executor).await.unwrap();
    assert_eq!(executor_balance, U256::ZERO); // executor shoud shave sent the value to WETH9
    let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap()._0;
    assert_eq!(executor_weth_balance, U256::ZERO); // executor should have 2 eth worth of weth

    // Proxy(Executor) account has 2 weth
    // 0 eth
    // 2 weth
    let proxy_executor_balance = provider.get_balance(proxy_executor).await.unwrap();
    assert_eq!(proxy_executor_balance, U256::ZERO); // executor shoud shave sent the value to WETH9
    let proxy_executor_weth_balance = weth9_contract
        .balanceOf(proxy_executor)
        .call()
        .await
        .unwrap()
        ._0;
    assert_eq!(proxy_executor_weth_balance, TWO_ETH); // executor should have 2 eth worth of weth

    ////////////////////////////////////////////////////////////
    // Whithdraw weth from the proxy account
    // Use the deployed Proxy(Executor) contract (WALLET is the owner) to deposit weth
    let withdraw_calldata = IWETH::withdrawCall { amount: TWO_ETH }.abi_encode();
    let fb = FlowBuilder::empty().call(WETH9, &withdraw_calldata, U256::ZERO).build(true);

    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(proxy_executor)
        .with_value(U256::ZERO)
        .with_input(fb);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();

    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();
    assert!(receipt.status());

    // Executor account has no assets
    // 0 eth
    // 0 weth
    let executor_balance = provider.get_balance(executor).await.unwrap();
    assert_eq!(executor_balance, U256::ZERO); // executor shoud shave sent the value to WETH9
    let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap()._0;
    assert_eq!(executor_weth_balance, U256::ZERO); // executor should have 2 eth worth of weth

    // Proxy(Executor) account has 2 weth
    // 0 eth
    // 2 weth
    let proxy_executor_balance = provider.get_balance(proxy_executor).await.unwrap();
    assert_eq!(proxy_executor_balance, U256::ZERO); // executor shoud shave sent the value to WETH9
    let proxy_executor_weth_balance = weth9_contract
        .balanceOf(proxy_executor)
        .call()
        .await
        .unwrap()
        ._0;
    assert_eq!(proxy_executor_weth_balance, TWO_ETH); // executor should have 2 eth worth of weth
}

#[tokio::test]
async fn test_extcodecopy() {
    // Flipper.sol::
    // contract proxy {
    //     bool flag;
    //     function flip() external {
    //         flag = !flag;
    //     }
    // }

    let flipper_init = hex!("6080604052348015600e575f80fd5b50608f80601a5f395ff3fe6080604052348015600e575f80fd5b50600436106026575f3560e01c8063cde4efa914602a575b5f80fd5b60306032565b005b5f8054906101000a900460ff16155f806101000a81548160ff02191690831515021790555056fea264697066735822122054836815366ebd9b068e7694d59a986fb0267bc2cc7c9ec20ffdccea97c00a3b64736f6c634300081a0033");
    let flipper_runtime = hex!("6080604052348015600e575f80fd5b50600436106026575f3560e01c8063cde4efa914602a575b5f80fd5b60306032565b005b5f8054906101000a900460ff16155f806101000a81548160ff02191690831515021790555056fea264697066735822122054836815366ebd9b068e7694d59a986fb0267bc2cc7c9ec20ffdccea97c00a3b64736f6c634300081a0033");
    let flipper_prolog = hex!("6080604052348015600e575f80fd5b50608f80601a5f395ff3fe");
    let provider = get_provider();

    // reality check
    let weth_balance = provider.get_balance(WETH9).await.unwrap();
    assert_eq!(format!("{}", weth_balance), "2933633723194923479377016");

    // test WALLETs
    // 0x4141414141..4141414141  with 1001 eth
    // 0x4242424242..4242424242  with 1001 eth
    provider
        .anvil_set_balance(WALLET, BUDGET + U256::from(1e18 as u64))
        .await
        .unwrap();
    provider
        .anvil_set_balance(BOB, BUDGET + U256::from(1e18 as u64))
        .await
        .unwrap();
    // Make the Executor contract (WALLET is the owner)
    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_deploy_code(EXECUTOR_INIT)
        .with_nonce(0);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();
    assert!(receipt.status());
    let executor = receipt.contract_address.unwrap();

    // create normal flipper account.
    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_deploy_code(flipper_init);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();
    assert!(receipt.status());
    let flipper = receipt.contract_address.unwrap();

    let created_flipper_runtime = provider.get_code_at(flipper).await.unwrap();

    // The created flipper has the expected runtime bytecode. duh
    assert_eq!(created_flipper_runtime.to_vec(), flipper_runtime.to_vec());

    let flipper1 = executor.create(1);
    let fb = FlowBuilder::empty()
        .set_cleardata_op(flipper_init.len() as u16)
        .set_data_op(0, &flipper_init)
        .create_op(flipper1)
        .build(true);

    // create normal flipper account. Using data ops
    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor)
        .with_input(fb);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();
    assert!(receipt.status());
    println!("NO excodecopy gas used {:?}", receipt.gas_used);

    assert_eq!(
        address!("c84f9705070281e8c800c57d92dbab053a80a2d0"),
        flipper1
    );

    let created_flipper1_runtime = provider.get_code_at(flipper1).await.unwrap();
    assert_eq!(created_flipper1_runtime, created_flipper_runtime);

    // create normal flipper account. Using data Extcodecopy
    let flipper2 = executor.create(2);
    let fb = FlowBuilder::empty()
        .set_cleardata_op(flipper_init.len() as u16)
        .set_data_op(0, &flipper_prolog)
        //  .set_data_op(flipper_prolog.len() as u16, &flipper_runtime)
        //.set_data_op(0, &flipper_init)
        .set_extcodecopy_op(
            flipper1,
            flipper_prolog.len() as u16,
            0,
            created_flipper_runtime.len() as u16,
        )
        .create_op(flipper2)
        .build(true);

    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor)
        .with_input(fb);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();
    assert!(receipt.status());
    println!("excodecopy gas used {:?}", receipt.gas_used);
    assert_eq!(
        address!("6266c8947cb0834202f2a3be9e0b5f97e0089fda"),
        flipper2
    );

    let created_flipper2_runtime = provider.get_code_at(flipper2).await.unwrap();
    assert_eq!(created_flipper2_runtime, created_flipper_runtime);
}
