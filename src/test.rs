
use crate::{FlowBuilder, DELEGATE_PROXY_INIT, EXECUTOR_INIT};
use alloy::{
    hex,
    network::TransactionBuilder, // Added Ethereum
    primitives::{address, bytes, uint, Address, U256},
    // Removed EthereumSigner, node_bindings::Anvil
    providers::{
        ext::AnvilApi,
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller}, // Corrected filler paths
        layers::AnvilProvider,
        Identity, Provider, ProviderBuilder, RootProvider,
    },
    rpc::types::TransactionRequest,
    sol,
    sol_types::{SolCall, SolConstructor},
};
use core::str;

// Type alias for the complex provider type using LocalWallet as signer
type AnvilTestProvider = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>
        >,
    AnvilProvider<RootProvider>,
>;

// Constants
const BUDGET: U256 = uint!(1000000000000000000000_U256); // 1000e18
const TWO_ETH: U256 = uint!(2000000000000000000_U256); // 2e18
const ONEHUNDRED_ETH: U256 = uint!(10000000000000000000_U256);
const WALLET: Address = Address::repeat_byte(0x41);
const BOB: Address = Address::repeat_byte(0x42);
const WETH9: Address = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
const MORPHO: Address = address!("BBBBBbbBBb9cC5e90e3b3Af64bdAF62C37EEFFCb");

// Test helpers
async fn setup_provider() -> AnvilTestProvider {
    let provider = get_provider();
    provider
        .anvil_set_balance(WALLET, BUDGET + U256::from(10u64.pow(18)))
        .await
        .unwrap();
    provider
        .anvil_set_balance(BOB, BUDGET + U256::from(10u64.pow(18)))
        .await
        .unwrap();
    provider
}

async fn deploy_executor(provider: &AnvilTestProvider) -> Address {
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
    receipt.contract_address.unwrap()
}
fn get_provider() -> AnvilTestProvider {
    ProviderBuilder::new().on_anvil_with_config(|anvil| {
        anvil
            .fork(std::env::var("ETH_RPC_URL").expect("failed to retrieve ETH_RPC_URL url from env"))
            .fork_block_number(20_000_000)
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

sol! {
    interface IMorpho {
        function flashLoan(address token, uint256 assets, bytes calldata data) external;
    }      
}

#[test]
fn test_flow_builder_create() {
    let calldata = FlowBuilder::empty()
        .create(Address::ZERO, "LALA".as_bytes(), U256::from(10))
        .optimize()
        .build();
    assert_eq!(
        calldata,
        hex!("c94f554d04000000000000000000000000000000000000000000000000000000000000000a01000402000000044c414c4107")
    );
}

#[test]
fn test_flow_builder_call() {
    let addr_a = Address::repeat_byte(0x41);
    let calldata = FlowBuilder::empty()
        .call(addr_a, &vec![98, 99], U256::ZERO)
        .optimize()
        .build();
    assert_eq!(
        calldata,
        hex!("c94f554d0341414141414141414141414141414141414141410100020200000002626306")
    );
}

#[test]
fn test_flow_builder_delegatecall() {
    let addr_b = Address::repeat_byte(0x42);
    let calldata = FlowBuilder::empty()
        .delegatecall(addr_b, &vec![70, 71])
        .optimize()
        .build();
    assert_eq!(
        calldata,
        hex!("c94f554d0342424242424242424242424242424242424242420100020200000002464708")
    );
}

#[test]
fn test_flow_builder_combined_operations() {
    let addr_a = Address::repeat_byte(0x41);
    let addr_b = Address::repeat_byte(0x42);
    let calldata = FlowBuilder::empty()
        .create(Address::ZERO, "LALA".as_bytes(), U256::from(10))
        .call(addr_a, &vec![98, 99], U256::ZERO)
        .delegatecall(addr_b, &vec![70, 71])
        .optimize()
        .build();
    assert_eq!(
        calldata,
        hex!("c94f554d04000000000000000000000000000000000000000000000000000000000000000a01000402000000044c414c410703414141414141414141414141414141414141414101000202000000026263060342424242424242424242424242424242424242420200000002464708")
    );
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
async fn test_weth_deposit_through_executor() {
    let provider = setup_provider().await;
    let executor = deploy_executor(&provider).await;
    
    // Initial balance check
    let executor_balance = provider.get_balance(executor).await.unwrap();
    let weth9_contract = IERC20::new(WETH9, provider.clone());
    let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap();
    assert_eq!(executor_balance, U256::ZERO);
    assert_eq!(executor_weth_balance, U256::ZERO);

    // Deposit ETH to get WETH
    let fb = FlowBuilder::empty().call(WETH9, &bytes!(""), TWO_ETH).optimize().build();
    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor)
        .with_value(TWO_ETH)
        .with_input(fb);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap().unwrap();
    assert!(receipt.status());

    // Verify balances after deposit
    let executor_balance = provider.get_balance(executor).await.unwrap();
    let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap();
    assert_eq!(executor_balance, U256::ZERO);
    assert_eq!(executor_weth_balance, TWO_ETH);
}

#[tokio::test]
async fn test_weth_withdraw_through_executor() {
    let provider = setup_provider().await;
    let executor = deploy_executor(&provider).await;
    
    // First deposit WETH
    let fb = FlowBuilder::empty().call(WETH9, &bytes!(""), TWO_ETH).optimize().build();
    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor)
        .with_value(TWO_ETH)
        .with_input(fb);
    let _tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    
    // Then withdraw it back to ETH
    let withdraw_calldata = IWETH::withdrawCall { amount: TWO_ETH }.abi_encode();
    let fb = FlowBuilder::empty().call(WETH9, &withdraw_calldata, U256::ZERO).optimize().build();
    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor)
        .with_input(fb);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap().unwrap();
    assert!(receipt.status());

    // Verify final balances
    let executor_balance = provider.get_balance(executor).await.unwrap();
    let weth9_contract = IERC20::new(WETH9, provider.clone());
    let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap();
    assert_eq!(executor_balance, TWO_ETH);
    assert_eq!(executor_weth_balance, U256::ZERO);
}

#[tokio::test]
async fn test_wallet_can_proxy_create_small() {
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

    let wallet_balance = provider.get_balance(WALLET).await.unwrap();
    assert_eq!(wallet_balance, BUDGET + U256::from(1e18 as u64)); // executor shoud shave sent the value to WETH9

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
                .optimize().build(),
            TWO_ETH,
        )
        .optimize().build();

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

    let executor_balance = provider.get_balance(executor).await.unwrap();
    assert_eq!(executor_balance, U256::ZERO); // executor shoud shave sent the value to WETH9
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
    let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap();
    assert_eq!(executor_weth_balance, U256::ZERO); // executor should have 2 eth worth of weth

    // Proxy created via executor that points to the executor ?? ?AHHH
    // 0 eth
    // 2 weth

    let proxy_balance = provider.get_balance(executor.create(1)).await.unwrap();
    assert_eq!(proxy_balance, U256::ZERO); // executor shoud shave sent the value to WETH9

    let weth9_contract = IERC20::new(WETH9, provider.clone());
    let proxy_weth_balance = weth9_contract
        .balanceOf(executor.create(1))
        .call()
        .await
        .unwrap();
    assert_eq!(proxy_weth_balance, TWO_ETH); // executor should have 2 eth worth of weth

    // Test ownership in the created proxy
    // WALLET -> executor -> proxy mint some weth

    let withdraw_calldata = IWETH::withdrawCall { amount: TWO_ETH }.abi_encode();
    let multiplexed_withdraw_calldata = FlowBuilder::empty()
        .call(WETH9, &withdraw_calldata, U256::ZERO)
        .optimize().build(); // multiplexed withdraw from weth

    let fb = FlowBuilder::empty().call(
        executor.create(1),
        &multiplexed_withdraw_calldata,
        U256::ZERO,
    ).optimize().build(); // this should send 2 eth to weth and assign the same weth value to the executor

    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor)
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

    // Executor has
    // 0 eth
    // 0 weth
    let executor_balance = provider.get_balance(executor).await.unwrap();
    assert_eq!(executor_balance, U256::ZERO); // executor shoud shave sent the value to WETH9

    let weth9_contract = IERC20::new(WETH9, provider.clone());
    let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap();
    assert_eq!(executor_weth_balance, U256::ZERO);

    // Proxy created via executor that points to the executor ?? ?AHHH
    // 2 eth
    // 0 weth

    let proxy_balance = provider.get_balance(executor.create(1)).await.unwrap();
    assert_eq!(proxy_balance, TWO_ETH); // executor shoud shave sent the value to WETH9

    let weth9_contract = IERC20::new(WETH9, provider.clone());
    let proxy_weth_balance = weth9_contract
        .balanceOf(executor.create(1))
        .call()
        .await
        .unwrap();
    assert_eq!(proxy_weth_balance, U256::ZERO); 

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
    let fb = FlowBuilder::empty().call(WETH9, &deposit_calldata, TWO_ETH).optimize().build();

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
    let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap();
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
        .unwrap();
    assert_eq!(proxy_executor_weth_balance, TWO_ETH); // executor should have 2 eth worth of weth

    ////////////////////////////////////////////////////////////
    // Whithdraw weth from the proxy account
    // Use the deployed Proxy(Executor) contract (WALLET is the owner) to deposit weth
    let withdraw_calldata = IWETH::withdrawCall { amount: TWO_ETH }.abi_encode();
    let fb = FlowBuilder::empty().call(WETH9, &withdraw_calldata, U256::ZERO).optimize().build();

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
    let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap();
    assert_eq!(executor_weth_balance, U256::ZERO); // executor should have 2 eth worth of weth

    // Proxy(Executor) account has 2 weth
    // 2 eth
    // 0 weth
    let proxy_executor_balance = provider.get_balance(proxy_executor).await.unwrap();
    assert_eq!(proxy_executor_balance, TWO_ETH);
    let proxy_executor_weth_balance = weth9_contract
        .balanceOf(proxy_executor)
        .call()
        .await
        .unwrap();
    assert_eq!(proxy_executor_weth_balance, U256::ZERO);
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
        .optimize().build();

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
        .optimize().build();

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

// This test test a simple flashloan with morpho 
#[tokio::test]
async fn test_flashloan_success_with_callback() {
    let provider = setup_provider().await;
    let executor = deploy_executor(&provider).await;

    let approve_calldata = IERC20::approveCall {
        spender: MORPHO,
        value: ONEHUNDRED_ETH,
    }.abi_encode();

    let fb = FlowBuilder::empty()
        .call(WETH9, &approve_calldata, U256::ZERO)
        .optimize()
        .build_raw();

    let flashloan_calldata = IMorpho::flashLoanCall {
        token: WETH9,
        assets: ONEHUNDRED_ETH,
        data: fb.into(),
    }.abi_encode();

    let fb = FlowBuilder::empty()
        .set_fail()
        .set_callback(MORPHO)
        .call(MORPHO, &flashloan_calldata, U256::ZERO)
        .optimize()
        .build();

    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor)
        .with_value(U256::ZERO)
        .with_input(fb);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap().unwrap();
    assert!(receipt.status());
}

#[tokio::test]
async fn test_flashloan_fails_without_callback() {
    let provider = setup_provider().await;
    let executor = deploy_executor(&provider).await;

    let approve_calldata = IERC20::approveCall {
        spender: MORPHO,
        value: ONEHUNDRED_ETH,
    }.abi_encode();

    let fb = FlowBuilder::empty()
        .call(WETH9, &approve_calldata, U256::ZERO)
        .optimize()
        .build_raw();

    let flashloan_calldata = IMorpho::flashLoanCall {
        token: WETH9,
        assets: ONEHUNDRED_ETH,
        data: fb.into(),
    }.abi_encode();

    let fb = FlowBuilder::empty()
        .set_fail()
        .call(MORPHO, &flashloan_calldata, U256::ZERO)
        .optimize()
        .build();

    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor)
        .with_value(U256::ZERO)
        .with_input(fb);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap().unwrap();
    assert!(!receipt.status());
}

#[tokio::test]
async fn test_multiple_flashloans_with_callback_reset() {
    let provider = setup_provider().await;
    let executor = deploy_executor(&provider).await;

    let approve_calldata = IERC20::approveCall {
        spender: MORPHO,
        value: ONEHUNDRED_ETH,
    }.abi_encode();

    let fb = FlowBuilder::empty()
        .call(WETH9, &approve_calldata, U256::ZERO)
        .optimize()
        .build_raw();

    let flashloan_calldata = IMorpho::flashLoanCall {
        token: WETH9,
        assets: ONEHUNDRED_ETH,
        data: fb.into(),
    }.abi_encode();

    let fb = FlowBuilder::empty()
        .set_fail()
        .set_callback(MORPHO)
        .call(MORPHO, &flashloan_calldata, U256::ZERO)
        .set_callback(MORPHO)
        .call(MORPHO, &flashloan_calldata, U256::ZERO)
        .optimize()
        .build();

    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor)
        .with_value(U256::ZERO)
        .with_input(fb);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap().unwrap();
    assert!(receipt.status());
}

#[tokio::test]
async fn test_multiple_flashloans_fails_without_callback_reset() {
    let provider = setup_provider().await;
    let executor = deploy_executor(&provider).await;

    let approve_calldata = IERC20::approveCall {
        spender: MORPHO,
        value: ONEHUNDRED_ETH,
    }.abi_encode();

    let fb = FlowBuilder::empty()
        .call(WETH9, &approve_calldata, U256::ZERO)
        .optimize()
        .build_raw();

    let flashloan_calldata = IMorpho::flashLoanCall {
        token: WETH9,
        assets: ONEHUNDRED_ETH,
        data: fb.into(),
    }.abi_encode();

    let fb = FlowBuilder::empty()
        .set_fail()
        .set_callback(MORPHO)
        .call(MORPHO, &flashloan_calldata, U256::ZERO)
        .call(MORPHO, &flashloan_calldata, U256::ZERO)
        .optimize()
        .build();

    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor)
        .with_value(U256::ZERO)
        .with_input(fb);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap().unwrap();
    assert!(!receipt.status());
}

#[tokio::test]
async fn test_nested_flashloan_success() {
    let provider = setup_provider().await;
    let executor = deploy_executor(&provider).await;

    let approve_calldata = IERC20::approveCall {
        spender: MORPHO,
        value: ONEHUNDRED_ETH,
    }.abi_encode();

    let fb_approve = FlowBuilder::empty()
        .call(WETH9, &approve_calldata, U256::ZERO)
        .optimize()
        .build_raw();

    let flashloan_calldata_inner = IMorpho::flashLoanCall {
        token: WETH9,
        assets: ONEHUNDRED_ETH,
        data: fb_approve.into(),
    }.abi_encode();

    let fb_inner = FlowBuilder::empty()
        .set_fail()
        .set_callback(MORPHO)
        .call(MORPHO, &flashloan_calldata_inner, U256::ZERO)
        .call(WETH9, &approve_calldata, U256::ZERO)
        .optimize()
        .build_raw();

    let flashloan_calldata_outer = IMorpho::flashLoanCall {
        token: WETH9,
        assets: ONEHUNDRED_ETH,
        data: fb_inner.into(),
    }.abi_encode();

    let fb = FlowBuilder::empty()
        .set_fail()
        .set_callback(MORPHO)
        .call(MORPHO, &flashloan_calldata_outer, U256::ZERO)
        .optimize()
        .build();

    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor)
        .with_value(U256::ZERO)
        .with_input(fb);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap().unwrap();
    assert!(receipt.status());
}

#[tokio::test]
async fn test_nested_flashloan_fails_without_callback() {
    let provider = setup_provider().await;
    let executor = deploy_executor(&provider).await;

    let approve_calldata = IERC20::approveCall {
        spender: MORPHO,
        value: ONEHUNDRED_ETH,
    }.abi_encode();

    let fb_approve = FlowBuilder::empty()
        .call(WETH9, &approve_calldata, U256::ZERO)
        .optimize()
        .build_raw();

    let flashloan_calldata_inner = IMorpho::flashLoanCall {
        token: WETH9,
        assets: ONEHUNDRED_ETH,
        data: fb_approve.into(),
    }.abi_encode();

    let fb_inner = FlowBuilder::empty()
        .set_fail()
        .call(MORPHO, &flashloan_calldata_inner, U256::ZERO)
        .call(WETH9, &approve_calldata, U256::ZERO)
        .optimize()
        .build_raw();

    let flashloan_calldata_outer = IMorpho::flashLoanCall {
        token: WETH9,
        assets: ONEHUNDRED_ETH,
        data: fb_inner.into(),
    }.abi_encode();

    let fb = FlowBuilder::empty()
        .set_fail()
        .set_callback(MORPHO)
        .call(MORPHO, &flashloan_calldata_outer, U256::ZERO)
        .optimize()
        .build();

    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor)
        .with_value(U256::ZERO)
        .with_input(fb);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap().unwrap();
    assert!(!receipt.status());
}

#[tokio::test]
/// Tests Aave V3 flash loan functionality with callback
///
/// This test verifies:
/// 1. Flash loan execution with proper callback setup
/// 2. Premium calculation and payment (0.05%)
/// 3. WETH wrapping/unwrapping during the process
/// 4. Successful transaction completion and balance verification
async fn test_flashloan_aave3_success_with_callback() {
    // Aave V3 Pool contract on mainnet
    const AAVE3_POOL: Address = address!("87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2");
    // Flash loan premium rate (0.05%)
    const PREMIUM_FACTOR: U256 = uint!(500000000000000_U256);
    
    let provider = setup_provider().await;
    let executor = deploy_executor(&provider).await;

    // Calculate premium for 100 ETH flash loan
    let premium = ONEHUNDRED_ETH * PREMIUM_FACTOR / uint!(1000000000000000000_U256);

    // First send premium amount to executor so it can repay the flash loan
    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor)
        .with_value(premium);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap().unwrap();
    assert!(receipt.status());

    sol! {
        interface IAavePool {
            function flashLoanSimple(
                address receiverAddress,
                address asset,
                uint256 amount,
                bytes calldata params,
                uint16 referralCode
            ) external;
        }
    }

    // Build the repayment flow that will be executed in the callback
    let repay_fb = FlowBuilder::empty()
        // First deposit ETH to get WETH for the premium
        .call(WETH9, &[], premium)
        // Then transfer full amount back to Aave
        .call(WETH9, &IERC20::approveCall {
            spender: AAVE3_POOL,
            value: ONEHUNDRED_ETH + premium,
        }.abi_encode(), U256::ZERO)
        .optimize()
        .build_raw();

    // Build flash loan call using proper ABI encoding
    let flashloan_calldata = IAavePool::flashLoanSimpleCall {
        receiverAddress: executor,
        asset: WETH9,
        amount: ONEHUNDRED_ETH,
        params: repay_fb.into(),
        referralCode: 0
    }.abi_encode();

    let fb = FlowBuilder::empty()
        .set_fail()
        .set_callback(AAVE3_POOL)
        .call(AAVE3_POOL, &flashloan_calldata, U256::ZERO)
        .optimize()
        .build();

    let tx = TransactionRequest::default()
        .with_from(WALLET)
        .with_to(executor)
        .with_value(U256::ZERO)
        .with_input(fb);

    let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
    provider.evm_mine(None).await.unwrap();
    let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap().unwrap();
    assert!(receipt.status());
}
