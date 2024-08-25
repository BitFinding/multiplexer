use alloy::{
    hex,
    primitives::{Address, U256},
};

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
    pub offset: u16,     // Offset to copy code to
    pub size: u16,       // Size of the code to copy
}

impl ExtCodeCopy {
    pub fn new(source: Address, offset: u16, size: u16) -> Self {
        ExtCodeCopy {
            source,
            offset,
            size,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.push(OP_EXTCODECOPY); // Opcode
        encoded.extend(&self.source); // Source address
        encoded.extend(&self.offset.to_be_bytes()); // Offset
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
    fn new() -> Self {
        Self::default()
    }

    fn empty() -> Self {
        Self::default()
    }

    fn set_addr_op(&mut self, addr: Address) {
        self.actions.push(Action::SetAddr(SetAddr { addr }));
    }

    fn set_value_op(&mut self, value: U256) {
        self.actions.push(Action::SetValue(SetValue { value }));
    }

    fn set_data_op(&mut self, offset: u16, data: &[u8]) {
        self.actions.push(Action::SetData(SetData {
            offset,
            data: data.to_owned(),
        }));
    }

    fn set_cleardata_op(&mut self, size: u16) {
        self.actions.push(Action::ClearData(ClearData { size }));
    }

    fn call_op(&mut self) {
        self.actions.push(Action::Call(Call::new()));
    }

    fn create_op(&mut self, created_address: Address) {
        self.actions
            .push(Action::Create(Create { created_address }));
    }

    fn delegatecall_op(&mut self) {
        self.actions.push(Action::DelegateCall(DelegateCall::new()));
    }

    pub fn call(mut self, target: Address, data: &[u8], value: U256) -> Self {
        assert!(data.len() < u16::MAX as usize, "datalen exceeds 0xffff");

        self.set_addr_op(target);
        self.set_value_op(value);
        self.set_cleardata_op(data.len() as u16);
        self.set_data_op(0, data);
        self.call_op();
        self
    }

    pub fn delegatecall(mut self, target: Address, data: &[u8]) -> Self {
        self.set_addr_op(target);
        self.set_cleardata_op(data.len() as u16);
        self.set_data_op(0, data);
        self.delegatecall_op();
        self
    }

    pub fn create(mut self, created_address: Address, data: &[u8], value: U256) -> Self {
        self.set_value_op(value);
        self.set_cleardata_op(data.len() as u16);
        self.set_data_op(0, data);
        self.create_op(created_address);
        self
    }

    pub fn build(self) -> Vec<u8> {
        let mut res = Vec::new();
        for action in self.actions {
            res.extend(&action.encode());
        }
        println!("{}", hex::encode(&res));
        res
    }
}

#[cfg(test)]
mod test {
    use core::str;

    use crate::FlowBuilder;
    use alloy::{
        hex,
        network::TransactionBuilder,
        primitives::{address, bytes, Address, ChainId, U256},
        providers::{
            self, ext::AnvilApi, layers::AnvilProvider, Provider, ProviderBuilder, RootProvider,
        },
        rpc::types::TransactionRequest,
        sol,
        transports::http::{Client, Http},
    };


    const EXECUTOR_INIT: &[u8] = include_bytes!("../contracts/executor.bin");
    const DELEGATE_PROXY_INIT: &[u8] = include_bytes!("../contracts/proxy.bin");

    fn get_provider() -> AnvilProvider<RootProvider<Http<Client>>, Http<Client>> {
        // Create a provider.
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

    #[test]
    fn test() {
        // Basic smoke test for the FlowBuilder
        let addr_a = Address::repeat_byte(0x41);
        let addr_b = Address::repeat_byte(0x42);
        let calldata = FlowBuilder::empty()
            .create(Address::ZERO, "LALA".as_bytes(), U256::from(10))
            .call(addr_a, &vec![98, 99], U256::ZERO)
            .delegatecall(addr_b, &vec![70, 71])
            .build();
        assert_eq!(calldata, hex!("03000000000000000000000000000000000000000000000000000000000000000a00000401000000044c414c410602414141414141414141414141414141414141414103000000000000000000000000000000000000000000000000000000000000000000000201000000026263050242424242424242424242424242424242424242420000020100000002464707"));
    }

    #[tokio::test]
    async fn test_bob_can_not_interact() {
        // A random account can not interact with multiplexer
        let provider = ProviderBuilder::new().on_anvil();
        let budget = U256::from(1000e18 as u64);
        let wallet = Address::repeat_byte(0x41);
        let bob = Address::repeat_byte(0x42);

        provider
            .anvil_set_balance(wallet, budget + U256::from(10u64.pow(18)))
            .await
            .unwrap();
        provider
            .anvil_set_balance(bob, budget + U256::from(10u64.pow(18)))
            .await
            .unwrap();

        let tx = TransactionRequest::default()
            .with_from(wallet)
            .with_deploy_code(hex::decode(EXECUTOR_INIT).unwrap())
            .with_nonce(0);

        let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

        provider.evm_mine(None).await.unwrap();

        let res = provider
            .get_transaction_receipt(tx_hash)
            .await
            .unwrap()
            .unwrap();
        let executor_wallet = res.contract_address.unwrap();

        // Executor address is deterministic because we use always same wallet and nonce.
        assert_eq!(
            address!("c088f75b5733d097f266010c1502399a53bdfdbd"),
            executor_wallet
        );

        let tx = TransactionRequest::default()
            .with_from(bob)
            .with_to(executor_wallet)
            .with_nonce(1)
            .with_value(budget);

        let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

        provider.evm_mine(None).await.unwrap();

        let res = provider.get_transaction_receipt(tx_hash).await.unwrap();

        assert!(res.is_none()); // Tx can not be send from bob
    }

    #[tokio::test]
    async fn test_wallet_can_interact() {
        let provider = ProviderBuilder::new().on_anvil();
        let budget = U256::from(1000e18 as u64);
        let wallet = Address::repeat_byte(0x41);
        let bob = Address::repeat_byte(0x42);

        provider
            .anvil_set_balance(wallet, budget + U256::from(10u64.pow(18)))
            .await
            .unwrap();
        provider
            .anvil_set_balance(bob, budget + U256::from(10u64.pow(18)))
            .await
            .unwrap();

        let tx = TransactionRequest::default()
            .with_from(wallet)
            .with_deploy_code(hex::decode(EXECUTOR_INIT).unwrap())
            .with_nonce(0);

        let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
        provider.evm_mine(None).await.unwrap();

        let res = provider
            .get_transaction_receipt(tx_hash)
            .await
            .unwrap()
            .unwrap();
        let executor_wallet = res.contract_address.unwrap();
        // Executor address is deterministic because we use always same wallet and nonce.
        assert_eq!(
            address!("c088f75b5733d097f266010c1502399a53bdfdbd"),
            executor_wallet
        );

        let tx = TransactionRequest::default()
            .with_from(wallet)
            .with_to(executor_wallet)
            .with_nonce(1)
            .with_value(budget);

        let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

        provider.evm_mine(None).await.unwrap();

        let res = provider.get_transaction_receipt(tx_hash).await.unwrap();

        assert!(res.is_some()); // Tx succeed from wallet

        let account_balance = provider.get_balance(executor_wallet).await.unwrap();
        assert_eq!(account_balance, budget); // executor has the money sent in empty tx
    }

    #[tokio::test]
    async fn test_wallet_can_proxycall() {
        let provider = get_provider();

        // reality check
        let two_eth = U256::from(2e18 as u64);
        let weth9 = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
        let weth_balance = provider.get_balance(weth9).await.unwrap();
        assert_eq!(format!("{}", weth_balance), "2933633723194923479377016");

        // test wallets
        // 0x4141414141..4141414141  with 1001 eth
        // 0x4242424242..4242424242  with 1001 eth
        let budget = U256::from(1000e18 as u64);
        let wallet = Address::repeat_byte(0x41);
        let bob = Address::repeat_byte(0x42);

        provider
            .anvil_set_balance(wallet, budget + U256::from(1e18 as u64))
            .await
            .unwrap();
        provider
            .anvil_set_balance(bob, budget + U256::from(1e18 as u64))
            .await
            .unwrap();

        // Make the Executor contract (wallet is the owner)
        let tx = TransactionRequest::default()
            .with_from(wallet)
            .with_deploy_code(hex::decode(EXECUTOR_INIT).unwrap())
            .with_nonce(0);

        let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
        provider.evm_mine(None).await.unwrap();
        let res = provider
            .get_transaction_receipt(tx_hash)
            .await
            .unwrap()
            .unwrap();
        let executor = res.contract_address.unwrap();


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
            .with_from(wallet)
            .with_to(executor)
            .with_nonce(1)
            .with_value(two_eth)
            .with_input(fb.build());

        let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

        provider.evm_mine(None).await.unwrap();

        let receipt = provider
            .get_transaction_receipt(tx_hash)
            .await
            .unwrap()
            .unwrap();

        //assert!(receipt.status());
        for log in receipt.inner.as_receipt().unwrap().logs.iter() {
            //println!("TX receipt {}", hex::decode(hex::encode(&log.data().data[64..])).unwrap());
            if &log.data().data.len() > &64 {
                println!(
                    "TX receipt {}",
                    str::from_utf8(&log.data().data[64..]).unwrap()
                );
            } else {
                println!("TX receipt {}", &log.data().data);
            }
        }

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
            .with_from(wallet)
            .with_to(executor)
            .with_value(U256::ZERO)
            .with_input(fb.build());

        println!("TX: {:?}", tx);
        let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

        provider.evm_mine(None).await.unwrap();

        let receipt = provider
            .get_transaction_receipt(tx_hash)
            .await
            .unwrap()
            .unwrap();

        println!("R: {:?}",receipt);
        
        //assert!(receipt.status());
        for log in receipt.inner.as_receipt().unwrap().logs.iter() {
            //println!("TX receipt {}", hex::decode(hex::encode(&log.data().data[64..])).unwrap());
            if &log.data().data.len() > &64 {
                println!(
                    "TX receipt {}",
                    str::from_utf8(&log.data().data[64..]).unwrap_or("default")
                );
            } else {
                println!("TX receipt {}", &log.data().data);
            }
        }

        // 2 eth
        // 0 weth
        let executor_balance = provider.get_balance(executor).await.unwrap();
        assert_eq!(executor_balance, two_eth); // executor shoud shave sent the value to weth9

        let weth9_contract = IERC20::new(weth9, provider.clone());
        let executor_weth_balance = weth9_contract.balanceOf(executor).call().await.unwrap()._0;
        assert_eq!(executor_weth_balance, U256::ZERO); // executor should have 2 eth worth of weth

    }

    #[tokio::test]
    async fn test_wallet_can_proxycreate() {
        let provider = get_provider();

        // reality check
        let weth9 = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
        let weth_balance = provider.get_balance(weth9).await.unwrap();
        assert_eq!(format!("{}", weth_balance), "2933633723194923479377016");

        // test wallets
        // 0x4141414141..4141414141  with 1001 eth
        // 0x4242424242..4242424242  with 1001 eth
        let budget = U256::from(1000e18 as u64);
        let wallet = Address::repeat_byte(0x41);
        let bob = Address::repeat_byte(0x42);

        provider
            .anvil_set_balance(wallet, budget + U256::from(1e18 as u64))
            .await
            .unwrap();
        provider
            .anvil_set_balance(bob, budget + U256::from(1e18 as u64))
            .await
            .unwrap();
        // Make the Executor contract (wallet is the owner)
        let tx = TransactionRequest::default()
            .with_from(wallet)
            .with_deploy_code(hex::decode(EXECUTOR_INIT).unwrap())
            .with_nonce(0);

        let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
        provider.evm_mine(None).await.unwrap();
        let res = provider
            .get_transaction_receipt(tx_hash)
            .await
            .unwrap()
            .unwrap();
        let executor = res.contract_address.unwrap();

        // Create dellegate proxy
        let fb = FlowBuilder::empty().create(Address::ZERO, &hex::decode(DELEGATE_PROXY_INIT).unwrap(), U256::ZERO);

        let tx = TransactionRequest::default()
            .with_from(wallet)
            .with_to(executor)
            .with_value(U256::ZERO)
            .with_input(fb.build());

        let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

        provider.evm_mine(None).await.unwrap();

        let receipt = provider
            .get_transaction_receipt(tx_hash)
            .await
            .unwrap()
            .unwrap();

        for log in receipt.inner.as_receipt().unwrap().logs.iter() {
            //println!("TX receipt {}", hex::decode(hex::encode(&log.data().data[64..])).unwrap());
            if &log.data().data.len() > &64 {
                println!(
                    "TX receipt {}",
                    str::from_utf8(&log.data().data[64..]).unwrap()
                );
            } else {
                println!("TX receipt {}", &log.data().data);
            }
        }

        let account_balance = provider.get_balance(executor).await.unwrap();
        assert_eq!(account_balance, U256::ZERO); // executor shoud shave sent the value to weth9


        // Test ownership in the created proxy
        // wallet -> executor -> proxy :check:
        // bob -> executor -> ?? :fail:
        // bob -> proxy  :fail:
         
    }
}
