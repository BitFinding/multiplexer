use alloy::{hex, primitives::{Address, U256}};

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
    fn new() -> Self { Self::default() }

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

    pub fn build(mut self) -> Vec<u8> {
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
        hex, network::TransactionBuilder, node_bindings::Anvil, primitives::{address, bytes, Address, ChainId, U256}, providers::{ext::AnvilApi, Provider, ProviderBuilder}, rpc::types::TransactionRequest
    };

    const EXECUTOR_INIT: [u8; 1918] = hex!("608060408190525f80546001600160a01b0319163317905561075a90816100248239f3fe60806040526004361015610522575b361561051c575f80516020610705833981519152606060405160208152600860208201526743414c4c4241434b60c01b6040820152a15f546001600160a01b0316330361051e5761006661006136610595565b61056a565b368152365f60208301375f602036830101525f80516020610705833981519152604051806100b38160609060208152600a602082015269455845414354494f4e5360b01b60408201520190565b0390a15f808060605b845182101561051c5760016100f36100ee6100e86100da868a6105b1565b516001600160f81b03191690565b60f81c90565b6105f4565b9201916100ff816105d6565b806101635750509061015261015993925f805160206107058339815191526040518061014981606090602081526009602082015268434c4541524441544160b81b60408201520190565b0390a1856106aa565b9390610652565b915b9290916100bc565b61016f819492946105d6565b600181036101fd5750906101c16101c9915f80516020610705833981519152604051806101b8816060906020815260076020820152665345544441544160c81b60408201520190565b0390a1866106aa565b9050856106aa565b93905f905b8082106101dc57505061015b565b90946101ea600191886106eb565b969060208260051b8801015201906101ce565b610206816105d6565b6002810361025b575050610255905f805160206107058339815191526040518061024c8160609060208152600760208201526629a2aa20a2222960c91b60408201520190565b0390a18461067a565b9261015b565b610267819592956105d6565b600381036102be5750506102b7905f80516020610705833981519152604051806102ae8160609060208152600860208201526753455456414c554560c01b60408201520190565b0390a1846106eb565b929061015b565b6102ca819593956105d6565b6004810361035057506020610342610330610327610339975f805160206107058339815191526040518061031e8160609060208152600b60208201526a455854434f4445434f505960a81b60408201520190565b0390a18961067a565b899291926106aa565b899891986106aa565b899391936106aa565b93909397870101903c61015b565b610359816105d6565b60058103610458575061044e5f807fe11c90dd1ef1a936510a4a968128ab3a2930bdb0f190feda812ac9c83f4af8c9935f80516020610705833981519152604051806103be8160609060208152600460208201526310d0531360e21b60408201520190565b0390a16040516001600160a01b03871681527fb84ae18be1d2e5a3a025b0234713048b3f07219071b2a53347ba59e44c1d40bf90602090a17f61193bd2fe35a1a699938a95fcbde5c2c4f24bb10c90c42fcfa754d42c063eaa604051806104258a82610628565b0390a18651906020880190875af161043b610603565b5060405190151581529081906020820190565b0390a15f5b61015b565b610461816105d6565b600681036104b5575090505f80516020610705833981519152604051806104a38160609060208152600660208201526543524541544560d01b60408201520190565b0390a18151906020830190f05f61015b565b806104c16007926105d6565b03610453575f80516020610705833981519152604051806104ff8160609060208152600860208201526744454c454741544560c01b60408201520190565b0390a15f80845160208601855af450610516610603565b5061015b565b005b5f80fd5b5f3560e01c638da5cb5b0361000e573461051e575f36600319011261051e575f546001600160a01b03166080908152602090f35b634e487b7160e01b5f52604160045260245ffd5b6040519190601f01601f1916820167ffffffffffffffff81118382101761059057604052565b610556565b67ffffffffffffffff811161059057601f01601f191660200190565b9081518110156105c2570160200190565b634e487b7160e01b5f52603260045260245ffd5b600811156105e057565b634e487b7160e01b5f52602160045260245ffd5b60ff1660088110156105e05790565b3d15610623573d9061061761006183610595565b9182523d5f602084013e565b606090565b602060409281835280519182918282860152018484015e5f828201840152601f01601f1916010190565b9061065f61006183610595565b8281528092610670601f1991610595565b0190602036910137565b8160209193929301015160601c60148301809311610696579190565b634e487b7160e01b5f52601160045260245ffd5b91909161ff00806106bb85846105b1565b5160f01c16169060018401808511610696576106d6916105b1565b5160f81c906002840180941161069657179190565b816020919392930101516020830180931161069657919056fed2f6c0020d30a86146de6300741f2bd90869bddf3818f8d3294ae782f6216176a26469706673582212206d459b9af770207931cbdd48fc216252120bd96c26360398f6d1bcb5cce1727164736f6c634300081a0033");


    #[test]
    fn test() {
        let addr_a = Address::repeat_byte(0x41);
        let addr_b = Address::repeat_byte(0x42);
        let calldata = FlowBuilder::empty()
            .create(Address::ZERO, "LALA".as_bytes(), U256::from(10))
            .call(addr_a, &vec![98, 99], U256::ZERO)
            .delegatecall(addr_b, &vec![70, 71])
            .build();
        println!("Encoded calldata {:?}", calldata);
    }

    #[tokio::test]
    async fn test_bob_can_not_interact() {
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
            .with_deploy_code(EXECUTOR_INIT)
            .with_nonce(0);

        let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

        provider.evm_mine(None).await.unwrap();

        let res = provider
            .get_transaction_receipt(tx_hash)
            .await
            .unwrap()
            .unwrap();
        let executor_wallet = res.contract_address.unwrap();
        println!("TX receipt {:?}", executor_wallet);

        let tx = TransactionRequest::default()
            .with_from(bob)
            .with_to(executor_wallet)
            .with_nonce(1)
            .with_value(budget);

        let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

        provider.evm_mine(None).await.unwrap();

        let res = provider.get_transaction_receipt(tx_hash).await.unwrap();

        assert!(res.is_none());     // Tx can not be send from bob

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
            .with_deploy_code(EXECUTOR_INIT)
            .with_nonce(0);

        let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
        provider.evm_mine(None).await.unwrap();


        let res = provider
            .get_transaction_receipt(tx_hash)
            .await
            .unwrap()
            .unwrap();
        let executor_wallet = res.contract_address.unwrap();

        let tx = TransactionRequest::default()
            .with_from(wallet)
            .with_to(executor_wallet)
            .with_nonce(1)
            .with_value(budget);

        let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

        provider.evm_mine(None).await.unwrap();

        let res = provider.get_transaction_receipt(tx_hash).await.unwrap();

        assert!(res.is_some()); // Tx can not be send from bob
        println!("TX receipt {:?}", res);

        let account_balance = provider.get_balance(executor_wallet).await.unwrap();
        assert_eq!( account_balance, budget);  // executor has the money sent in empty tx

    }


    #[tokio::test]
    async fn test_wallet_can_proxycall() {
        // Create a provider.
        let provider = ProviderBuilder::new().on_anvil_with_config(|anvil| {
            anvil
            .fork(std::env::var("ETH_RPC_URL").expect("failed to retrieve RPC url from env"))
            .fork_block_number(20000000)
        });


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
            .with_deploy_code(EXECUTOR_INIT)
            .with_nonce(0);

        let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();
        provider.evm_mine(None).await.unwrap();
        let res = provider
            .get_transaction_receipt(tx_hash)
            .await
            .unwrap()
            .unwrap();
        let executor = res.contract_address.unwrap();


        let two_eth = U256::from(2e18 as u64);
        let fb = FlowBuilder::empty()
                .call(weth9, &bytes!(""), two_eth);  // this should send 2 eth to weth and assign the same weth value to the executor

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

        let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap().unwrap();
        for log in receipt.inner.as_receipt().unwrap().logs.iter(){
            println!("TX receipt {:?}", &log.data().data);
            //println!("TX receipt {}", str::from_utf8(&log.data().data[64..]).unwrap());
        }
 
        let account_balance = provider.get_balance(executor).await.unwrap();
        assert_eq!( account_balance, U256::ZERO);  // executor shoud shave sent the value to weth9


        
        // WETH withdraw!!

        let two_eth = U256::from(2e18 as u64);    
        let mut withdraw_calldata = hex::decode("2e1a7d4d").unwrap();
        withdraw_calldata.extend(two_eth.to_be_bytes::<32>().iter());
        let fb = FlowBuilder::empty()
                .call(weth9, &withdraw_calldata, U256::ZERO);  // this should send 2 eth to weth and assign the same weth value to the executor

        let tx = TransactionRequest::default()
            .with_from(wallet)
            .with_to(executor)
            .with_value(two_eth)
            .with_input(fb.build());

        let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

        provider.evm_mine(None).await.unwrap();

        let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap().unwrap();
        for log in receipt.inner.as_receipt().unwrap().logs.iter(){
            println!("TX receipt {:?}", &log.data().data);
            //println!("TX receipt {}", str::from_utf8(&log.data().data[64..]).unwrap());
        }
 
        let account_balance = provider.get_balance(executor).await.unwrap();
        assert_eq!( account_balance, two_eth);  // executor shoud shave sent the value to weth9




    }

}
