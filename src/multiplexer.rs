use alloy_primitives::{Address, U256};

// Contract bytecode
pub const EXECUTOR_INIT: &[u8] = include_bytes!("../contracts_output/executor.bin");
pub const DELEGATE_PROXY_INIT: &[u8] = include_bytes!("../contracts_output/proxy.bin");

pub const EXECUTOR_RUNTIME: &[u8] = include_bytes!("../contracts_output/executor_runtime.bin");
pub const DELEGATE_PROXY_RUNTIME: &[u8] = include_bytes!("../contracts_output/proxy_runtime.bin");

// Operation opcodes as constants
pub const OP_CLEARDATA: u8 = 0x00;
pub const OP_SETDATA: u8 = 0x01;
pub const OP_SETADDR: u8 = 0x02;
pub const OP_SETVALUE: u8 = 0x03;
pub const OP_EXTCODECOPY: u8 = 0x04;
pub const OP_CALL: u8 = 0x05;
pub const OP_CREATE: u8 = 0x06;
pub const OP_DELEGATECALL: u8 = 0x07;

// Struct for the CLEARDATA operation
pub struct ClearData {
    pub size: u16,
}

impl ClearData {
    pub fn new(size: u16) -> Self {
        ClearData { size }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.push(OP_CLEARDATA); // Opcode
        encoded.extend(&self.size.to_be_bytes()); // Size
        encoded
    }
}

// Struct for the SETDATA operation
pub struct SetData {
    pub offset: u16,
    pub data: Vec<u8>,
}

impl SetData {
    pub fn new(offset: u16, data: Vec<u8>) -> Self {
        SetData { offset, data }
    }

    pub fn encode(&self) -> Vec<u8> {
        let data_size = self.data.len() as u16;
        let mut encoded = Vec::new();
        encoded.push(OP_SETDATA); // Opcode
        encoded.extend(&self.offset.to_be_bytes()); // Offset
        encoded.extend(&data_size.to_be_bytes()); // Data Size
        encoded.extend(&self.data); // Data

        encoded
    }
}

// Struct for the SETADDR operation
pub struct SetAddr {
    pub addr: Address, // 20-byte address
}

impl SetAddr {
    pub fn new(addr: Address) -> Self {
        SetAddr { addr }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.push(OP_SETADDR); // Opcode
        encoded.extend(&self.addr); // Address
        encoded
    }
}

// Struct for the SETVALUE operation
#[derive(Clone, Debug)]
pub struct SetValue {
    pub value: U256,
}

impl SetValue {
    pub fn new(value: U256) -> Self {
        SetValue { value }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.push(OP_SETVALUE); // Opcode
        encoded.extend(&self.value.to_be_bytes::<32>()); // Value
        encoded
    }
}

// Struct for the EXTCODECOPY operation
pub struct ExtCodeCopy {
    pub source: Address, // Address of contract to copy code from
    pub data_offset: u16,     // Offset in the data to copy the code to
    pub code_offset: u16,     // Offset in the code to copy from 
    pub size: u16,       // Size of the code to copy
}

impl ExtCodeCopy {
    pub fn new(source: Address, data_offset: u16, code_offset: u16, size: u16) -> Self {
        ExtCodeCopy {
            source,
            data_offset,
            code_offset,            
            size,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.push(OP_EXTCODECOPY); // Opcode
        encoded.extend(&self.source); // Source address
        encoded.extend(&self.data_offset.to_be_bytes()); // Offset
        encoded.extend(&self.code_offset.to_be_bytes()); // Offset
        encoded.extend(&self.size.to_be_bytes()); // Size
        encoded
    }
}

// Struct for the CALL operation
#[derive(Default)]
pub struct Call {}

impl Call {
    pub fn new() -> Self {
        Call {}
    }

    pub fn encode(&self) -> Vec<u8> {
        vec![OP_CALL] // Opcode
    }
}

// Struct for the CREATE operation
#[derive(Default)]
pub struct Create {
    pub created_address: Address,
}

impl Create {
    pub fn new(created_address: Address) -> Self {
        Self { created_address }
    }

    pub fn encode(&self) -> Vec<u8> {
        vec![OP_CREATE] // Opcode
    }
}

// Struct for the DELEGATECALL operation
#[derive(Default)]
pub struct DelegateCall {}

impl DelegateCall {
    pub fn new() -> Self {
        DelegateCall {}
    }

    pub fn encode(&self) -> Vec<u8> {
        vec![OP_DELEGATECALL] // Opcode
    }
}

// Enum for all actions
pub enum Action {
    ClearData(ClearData),
    SetData(SetData),
    SetAddr(SetAddr),
    SetValue(SetValue),
    ExtCodeCopy(ExtCodeCopy),
    Call(Call),
    Create(Create),
    DelegateCall(DelegateCall),
}

impl Action {
    fn encode(&self) -> Vec<u8> {
        match self {
            Action::ClearData(cd) => cd.encode(),
            Action::SetData(sd) => sd.encode(),
            Action::SetAddr(sa) => sa.encode(),
            Action::SetValue(sv) => sv.encode(),
            Action::ExtCodeCopy(ecc) => ecc.encode(),
            Action::Call(c) => c.encode(),
            Action::Create(c) => c.encode(),
            Action::DelegateCall(dc) => dc.encode(),
        }
    }
}

// FlowBuilder to manage the actions
#[derive(Default)]
pub struct FlowBuilder {
    actions: Vec<Action>,
}

impl FlowBuilder {
    pub fn empty() -> Self {
        Self::default()
    }

    /// A simple optimizer that will remove redundant sets
    fn peephole_opt(&mut self) {
        let mut ops_to_remove = Vec::new();
        let mut last_value = U256::ZERO;
        let mut last_target = Address::ZERO;
        let mut last_data: Vec<u8> = Vec::new();

        for (idx, action) in self.actions.iter().enumerate() {
            let to_remove = match action {
                Action::Call(_) => {
                    last_value = U256::ZERO;
                    false
                },
                Action::Create(Create { created_address }) => {
                    last_target = *created_address;
                    last_value = U256::ZERO;
                    false
                },
                Action::SetAddr(SetAddr { addr }) => {
                    let res = last_target == *addr;
                    last_target = *addr;
                    res
                },
                Action::SetValue(SetValue { value }) => {
                    let res = last_value == *value;
                    last_value = *value;
                    res
                },
                Action::ClearData(ClearData { size }) => {
                    let res = last_data.len() == *size as usize;
                    last_data = vec![0; *size as usize];
                    res
                },
                Action::SetData(SetData { offset, data }) => {
                    let offset_uz = *offset as usize;
                    let mut new_data = last_data.clone();
                    new_data.splice(offset_uz  .. offset_uz + data.len(), data.to_owned());
                    let res = last_data == new_data;
                    last_data = new_data;
                    res
                },
                _ => false,
            };
            if to_remove {
                ops_to_remove.push(idx);
            }
        }

        for idx in ops_to_remove.into_iter().rev() {
            self.actions.remove(idx);
        }
    }

    pub fn set_extcodecopy_op(mut self, source: Address, data_offset: u16, code_offset: u16, size: u16) -> Self {
        self.actions.push(Action::ExtCodeCopy(ExtCodeCopy{ source, data_offset, code_offset, size }));
        self
    }

    pub fn set_addr_op(mut self, addr: Address) -> Self {
        self.actions.push(Action::SetAddr(SetAddr { addr }));
        self
    }

    pub fn set_value_op(mut self, value: U256) -> Self {
        self.actions.push(Action::SetValue(SetValue { value }));
        self
    }

    pub fn set_data_op(mut self, offset: u16, data: &[u8]) -> Self {
        self.actions.push(Action::SetData(SetData {
            offset,
            data: data.to_owned(),
        }));
        self
    }

    pub fn set_cleardata_op(mut self, size: u16) -> Self {
        self.actions.push(Action::ClearData(ClearData { size }));
        self
    }

    pub fn call_op(mut self) -> Self {
        self.actions.push(Action::Call(Call::new()));
        self
    }

    pub fn create_op(mut self, created_address: Address) -> Self {
        self.actions
            .push(Action::Create(Create { created_address }));
        self
    }

    pub fn delegatecall_op(mut self) -> Self {
        self.actions.push(Action::DelegateCall(DelegateCall::new()));
        self
    }

    pub fn call(self, target: Address, data: &[u8], value: U256) -> Self {
        assert!(data.len() < u16::MAX as usize, "datalen exceeds 0xffff");

        self.set_addr_op(target)
            .set_value_op(value)
            .set_cleardata_op(data.len() as u16)
            .set_data_op(0, data)
            .call_op()
    }

    pub fn delegatecall(self, target: Address, data: &[u8]) -> Self {
        self.set_addr_op(target)
            .set_cleardata_op(data.len() as u16)
            .set_data_op(0, data)
            .delegatecall_op()
    }

    pub fn create(self, created_address: Address, data: &[u8], value: U256) -> Self {
        self.set_value_op(value)
            .set_cleardata_op(data.len() as u16)
            .set_data_op(0, data)
            .create_op(created_address)
    }

    pub fn build(mut self, enable_opt: bool) -> Vec<u8> {
        let mut res = Vec::new();
        if enable_opt {
            self.peephole_opt();
        }
        for action in self.actions {
            res.extend(&action.encode());
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::{FlowBuilder, DELEGATE_PROXY_INIT, EXECUTOR_INIT};
    use alloy::{
        hex,
        network::TransactionBuilder,
        primitives::{uint, address, bytes, Address, ChainId, U256},
        providers::{
            self, ext::AnvilApi, layers::AnvilProvider, Provider, ProviderBuilder, RootProvider,
        },
        rpc::types::TransactionRequest,
        sol,
        sol_types::SolConstructor,
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
                .fork(std::env::var("ETH_RPC_URL").expect("failed to retrieve RPC url from env"))
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
        assert_eq!(calldata, hex!("03000000000000000000000000000000000000000000000000000000000000000a00000401000000044c414c410602414141414141414141414141414141414141414100000201000000026263050242424242424242424242424242424242424242420100000002464707"));
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

        // Executor address is deterministic because we use always same wallet and nonce.
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
        // Executor address is deterministic because we use always same wallet and nonce.
        assert_eq!(
            address!("c088f75b5733d097f266010c1502399a53bdfdbd"),
            executor_wallet
        );

        let tx = TransactionRequest::default()
            .with_from(WALLET)
            .with_to(executor_wallet)
            .with_nonce(1)
            .with_value(budget);

        let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

        provider.evm_mine(None).await.unwrap();

        let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap();

        assert!(receipt.is_some()); // Tx succeed from wallet

        let account_balance = provider.get_balance(executor_wallet).await.unwrap();
        assert_eq!(account_balance, budget); // executor has the money sent in empty tx
    }

    #[tokio::test]
    async fn test_wallet_can_proxy_call() {
        let provider = get_provider();

        // reality check
        let weth9_balance = provider.get_balance(WETH9).await.unwrap();
        assert_eq!(format!("{}", weth9_balance), "2933633723194923479377016");

        // test wallets
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

        // Make the Executor contract (wallet is the owner)
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
        assert_eq!(executor_balance, U256::ZERO); // executor shoud shave sent the value to weth9
        let weth9_contract = IERC20::new(weth9, provider.clone());
        let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap()._0;
        assert_eq!(executor_weth_balance, U256::ZERO); // executor should have 2 eth worth of weth

        let fb = FlowBuilder::empty().call(weth9, &bytes!(""), two_eth); // this should send 2 eth to weth and assign the same weth value to the executor
                                                                         // SETADDR 02 c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2
                                                                         // SETVALUE 03 0000000000000000000000000000000000000000000000001bc16d674ec80000
                                                                         // CLRDATA 00 0000
                                                                         // SETDATA 01 0000 0000
                                                                         // 05
        let tx = TransactionRequest::default()
            .with_from(WALLET)
            .with_to(executor)
            .with_nonce(1)
            .with_value(TWO_ETH)
            .with_input(fb.build(true));

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
        assert_eq!(executor_balance, U256::ZERO); // executor shoud shave sent the value to weth9

        let weth9_contract = IERC20::new(weth9, provider.clone());
        let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap()._0;
        assert_eq!(executor_weth_balance, two_eth); // executor should have 2 eth worth of weth

        // WETH withdraw!!
        // TODO try to USE sol!() like the balanceOf IERC20 example instead to encode the withdraw(...) funcid
        let mut withdraw_calldata = hex::decode("2e1a7d4d").unwrap();
        withdraw_calldata.extend(two_eth.to_be_bytes::<32>().iter());

        let fb = FlowBuilder::empty().call(weth9, &withdraw_calldata, U256::ZERO); // this should send 2 eth to weth and assign the same weth value to the executor

        let tx = TransactionRequest::default()
            .with_from(WALLET)
            .with_to(executor)
            .with_value(U256::ZERO)
            .with_input(fb.build(true));

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
        assert_eq!(executor_balance, two_eth); // executor shoud shave sent the value to weth9

        let weth9_contract = IERC20::new(weth9, provider.clone());
        let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap()._0;
        assert_eq!(executor_weth_balance, U256::ZERO); // executor should have 2 eth worth of weth
    }

    #[tokio::test]
    async fn test_wallet_can_proxy_create() {
        let provider = get_provider();

        // reality check
        let weth9 = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
        let weth_balance = provider.get_balance(weth9).await.unwrap();
        assert_eq!(format!("{}", weth_balance), "2933633723194923479377016");

        // test wallets
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
        // Make the Executor contract (wallet is the owner)
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
                &FlowBuilder::empty().call(weth9, &vec![], two_eth).build(true),
                two_eth,
            );

        let tx = TransactionRequest::default()
            .with_from(WALLET)
            .with_to(executor)
            .with_value(TWO_ETH)
            .with_input(fb.build(true));

        let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

        provider.evm_mine(None).await.unwrap();

        let receipt = provider
            .get_transaction_receipt(tx_hash)
            .await
            .unwrap()
            .unwrap();

        assert!(receipt.status());

        let account_balance = provider.get_balance(executor).await.unwrap();
        assert_eq!(account_balance, U256::ZERO); // executor shoud shave sent the value to weth9
        assert_eq!(
            address!("c84f9705070281e8c800c57d92dbab053a80a2d0"),
            executor.create(1)
        );

        // Executor has
        // 0 eth
        // 0 weth
        let executor_balance = provider.get_balance(executor).await.unwrap();
        assert_eq!(executor_balance, U256::ZERO); // executor shoud shave sent the value to weth9

        let weth9_contract = IERC20::new(weth9, provider.clone());
        let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap()._0;
        assert_eq!(executor_weth_balance, U256::ZERO); // executor should have 2 eth worth of weth

        // Proxy crested via executor that points to the executor ?? ?AHHH
        // 0 eth
        // 2 weth

        let executor_balance = provider.get_balance(executor.create(1)).await.unwrap();
        assert_eq!(executor_balance, U256::ZERO); // executor shoud shave sent the value to weth9

        let weth9_contract = IERC20::new(weth9, provider.clone());
        let executor_weth_balance = weth9_contract
            .balanceOf(executor.create(1))
            .call()
            .await
            .unwrap()
            ._0;
        assert_eq!(executor_weth_balance, two_eth); // executor should have 2 eth worth of weth

        // Test ownership in the created proxy
        // wallet -> executor -> proxy mint some weth

        // WETH withdraw!!
        // TODO try to USE sol!() like the balanceOf IERC20 example instead to encode the withdraw(...) funcid
        let mut withdraw_calldata = hex::decode("2e1a7d4d").unwrap();
        withdraw_calldata.extend(two_eth.to_be_bytes::<32>().iter());

        let multiplexed_withdraw_calldata = FlowBuilder::empty()
            .call(weth9, &withdraw_calldata, U256::ZERO)
            .build(true); // multiplexed withdraw from weth

        let fb = FlowBuilder::empty().call(
            executor.create(1),
            &multiplexed_withdraw_calldata,
            U256::ZERO,
        ); // this should send 2 eth to weth and assign the same weth value to the executor

        let tx = TransactionRequest::default()
            .with_from(WALLET)
            .with_to(executor)
            .with_value(two_eth)
            .with_input(fb.build(true));

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
        assert_eq!(executor_balance, two_eth); // executor shoud shave sent the value to weth9

        let weth9_contract = IERC20::new(weth9, provider.clone());
        let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap()._0;
        assert_eq!(executor_weth_balance, U256::ZERO); // executor should have 2 eth worth of weth

        // Proxy created via executor that points to the executor ?? ?AHHH
        // 0 eth
        // 0 weth

        let executor_balance = provider.get_balance(executor.create(1)).await.unwrap();
        assert_eq!(executor_balance, U256::ZERO); // executor shoud shave sent the value to weth9

        let weth9_contract = IERC20::new(weth9, provider.clone());
        let executor_weth_balance = weth9_contract
            .balanceOf(executor.create(1))
            .call()
            .await
            .unwrap()
            ._0;
        assert_eq!(executor_weth_balance, two_eth); // executor should have 2 eth worth of weth

        // bob -> executor -> ?? :fail:
        // bob -> proxy  :fail:
    }

    #[tokio::test]
    async fn test_wallet_can_proxy_create_ultimate() {
        let provider = get_provider();

        // reality check
        let weth9 = WETH9;
        let weth9_contract = IERC20::new(WETH9, provider.clone());
        let weth_balance = provider.get_balance(WETH9).await.unwrap();
        assert_eq!(format!("{}", weth_balance), "2933633723194923479377016");

        // test wallets
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
        // Make the Executor contract (wallet is the owner)
        let tx = TransactionRequest::default()
            .with_from(wallet)
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
        // Make the Proxy(Executor) contract (wallet is the owner)
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
            .with_from(wallet)
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
        assert_eq!(proxy_executor, wallet.create(1));

        ////////////////////////////////////////////////////////////
        // Deposit weth in the proxy account
        // Use the deployed Proxy(Executor) contract (wallet is the owner) to deposit weth
        let deposit_calldata = [];
        let fb = FlowBuilder::empty().call(weth9, &deposit_calldata, two_eth);

        let tx = TransactionRequest::default()
            .with_from(WALLET)
            .with_to(proxy_executor)
            .with_value(TWO_ETH)
            .with_input(fb.build(true));

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
        assert_eq!(executor_balance, U256::ZERO); // executor shoud shave sent the value to weth9
        let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap()._0;
        assert_eq!(executor_weth_balance, U256::ZERO); // executor should have 2 eth worth of weth

        // Proxy(Executor) account has 2 weth
        // 0 eth
        // 2 weth
        let proxy_executor_balance = provider.get_balance(proxy_executor).await.unwrap();
        assert_eq!(proxy_executor_balance, U256::ZERO); // executor shoud shave sent the value to weth9
        let proxy_executor_weth_balance = weth9_contract
            .balanceOf(proxy_executor)
            .call()
            .await
            .unwrap()
            ._0;
        assert_eq!(proxy_executor_weth_balance, two_eth); // executor should have 2 eth worth of weth

        ////////////////////////////////////////////////////////////
        // Whithdraw weth from the proxy account
        // Use the deployed Proxy(Executor) contract (wallet is the owner) to deposit weth
        let mut withdraw_calldata = hex::decode("2e1a7d4d").unwrap();
        withdraw_calldata.extend(two_eth.to_be_bytes::<32>().iter());

        let fb = FlowBuilder::empty().call(weth9, &withdraw_calldata, U256::ZERO);

        let tx = TransactionRequest::default()
            .with_from(WALLET)
            .with_to(proxy_executor)
            .with_value(U256::ZERO)
            .with_input(fb.build(true));

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
        assert_eq!(executor_balance, U256::ZERO); // executor shoud shave sent the value to weth9
        let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap()._0;
        assert_eq!(executor_weth_balance, U256::ZERO); // executor should have 2 eth worth of weth

        // Proxy(Executor) account has 2 weth
        // 0 eth
        // 2 weth
        let proxy_executor_balance = provider.get_balance(proxy_executor).await.unwrap();
        assert_eq!(proxy_executor_balance, U256::ZERO); // executor shoud shave sent the value to weth9
        let proxy_executor_weth_balance = weth9_contract
            .balanceOf(proxy_executor)
            .call()
            .await
            .unwrap()
            ._0;
        assert_eq!(proxy_executor_weth_balance, two_eth); // executor should have 2 eth worth of weth
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
        let weth9 = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
        let weth_balance = provider.get_balance(weth9).await.unwrap();
        assert_eq!(format!("{}", weth_balance), "2933633723194923479377016");

        // test wallets
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
        // Make the Executor contract (wallet is the owner)
        let tx = TransactionRequest::default()
            .with_from(wallet)
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
            .with_from(wallet)
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
            .create_op(flipper1);

        // create normal flipper account. Using data ops
        let tx = TransactionRequest::default()
            .with_from(wallet)
            .with_to(executor)
            .with_input(fb.build(true));

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

            .set_extcodecopy_op(flipper1, flipper_prolog.len() as u16, 0, created_flipper_runtime.len() as u16)
            .create_op(flipper2);


        let tx = TransactionRequest::default()
            .with_from(wallet)
            .with_to(executor)
            .with_input(fb.build(true));

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
}
