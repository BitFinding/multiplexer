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

    const EXECUTOR_INIT: [u8; 2914] = hex!("608060408190525f80546001600160a01b03191633179055610b3e90816100248239f3fe608060405260043610156108e6575b36156108e0575f80516020610ac9833981519152606060405160208152600860208201526743414c4c4241434b60c01b6040820152a15f546001600160a01b031633036108e25761006661006136610959565b61092e565b368152365f60208301375f602036830101525f80516020610ac9833981519152604051806100b38160609060208152600a602082015269455845414354494f4e5360b01b60408201520190565b0390a15f808060605b84518410156108e05760016100f36100ee6100e86100da888a610975565b516001600160f81b03191690565b60f81c90565b6109b8565b9401906100ff8561099a565b8461019d5750610195935061016a905f80516020610ac98339815191526040518061014881606090602081526009602082015268434c4541524441544160b81b60408201520190565b0390a16040518181525f80516020610ae983398151915290602090a185610a6e565b93905f80516020610ae98339815191526040518061018d84829190602083019252565b0390a1610a16565b915b916100bc565b9291936101a98161099a565b6001810361053d57509361025b6102649394955f80516020610ac9833981519152604051806101f4816060906020815260076020820152665345544441544160c81b60408201520190565b0390a15f80516020610ac98339815191526040518061023981606090602081526011602082015270031b0b6363230ba30afb7b33339b2ba1d1607d1b60408201520190565b0390a16040518181525f80516020610ae983398151915290602090a182610a6e565b82949194610a6e565b959095905f80516020610ac9833981519152604051806102a68160609060208152600d60208201526c03230ba30afb7b33339b2ba1d1609d1b60408201520190565b0390a16040518581525f80516020610ae983398151915290602090a15f80516020610ac9833981519152604051806102fe8160609060208152600b60208201526a03230ba30afb9b4bd329d160ad1b60408201520190565b0390a16040518781525f80516020610ae983398151915290602090a15f915b6103278860051c90565b8310156103d85761033a60019185610aaf565b93905f80516020610ae98339815191526103a9865f80516020610ac98339815191526040518061038a8160609060208152600b60208201526a024aa22a920aa24a7a71d160ad1b60408201520190565b0390a16040518581528390602090a16040519081529081906020820190565b0390a16040518181525f80516020610ae983398151915290602090a160208260051b898b01010152019161031d565b95949093929691506103ea8260051c90565b60051b9182915b888285106104635750505050505f80516020610ae98339815191526040518061041f87829190602083019252565b0390a15f80516020610ac98339815191526040518061045b8160609060208152600860208201526729a2aa2220aa209960c11b60408201520190565b0390a1610197565b885f80516020610ae9833981519152610516878b6105056104f6600198999a9b9f6100da908a995f80516020610ac9833981519152604051806104c68160609060208152600b60208201526a024aa22a920aa24a7a71d160ad1b60408201520190565b0390a16040518781528990602090a16040518281528990602090a160405163ffffffff81528990602090a1610975565b928b8a010180935f1a92610975565b536040519081529081906020820190565b0390a16040518181525f80516020610ae983398151915290602090a10197019291906103f1565b61054a819592939561099a565b600281036105b85750506105b2905f80516020610ac9833981519152604051806105908160609060208152600760208201526629a2aa20a2222960c91b60408201520190565b0390a16040518181525f80516020610ae983398151915290602090a184610a3e565b92610197565b6105c48195929561099a565b6003810361063457505061062d905f80516020610ac98339815191526040518061060b8160609060208152600860208201526753455456414c554560c01b60408201520190565b0390a16040518181525f80516020610ae983398151915290602090a184610aaf565b9290610197565b6106408195939561099a565b600481036106c6575060206106b86106a661069d6106af975f80516020610ac9833981519152604051806106948160609060208152600b60208201526a455854434f4445434f505960a81b60408201520190565b0390a189610a3e565b89929192610a6e565b89989198610a6e565b89939193610a6e565b93909397870101903c610197565b6106cf8161099a565b600581036107e757506107dd5f807fe11c90dd1ef1a936510a4a968128ab3a2930bdb0f190feda812ac9c83f4af8c9935f80516020610ac9833981519152604051806107348160609060208152600460208201526310d0531360e21b60408201520190565b0390a16040518881525f80516020610ae983398151915290602090a16040516001600160a01b03871681527fb84ae18be1d2e5a3a025b0234713048b3f07219071b2a53347ba59e44c1d40bf90602090a17f61193bd2fe35a1a699938a95fcbde5c2c4f24bb10c90c42fcfa754d42c063eaa604051806107b48a826109ec565b0390a18651906020880190875af16107ca6109c7565b5060405190151581529081906020820190565b0390a15f5b610197565b6107f08161099a565b60068103610879575090505f80516020610ac9833981519152604051806108328160609060208152600660208201526543524541544560d01b60408201520190565b0390a18151906020830190f06040516001600160a01b03821681527fb84ae18be1d2e5a3a025b0234713048b3f07219071b2a53347ba59e44c1d40bf9080602081016107dd565b8061088560079261099a565b036107e2575f80516020610ac9833981519152604051806108c38160609060208152600860208201526744454c454741544560c01b60408201520190565b0390a15f80845160208601855af4506108da6109c7565b50610197565b005b5f80fd5b5f3560e01c638da5cb5b0361000e57346108e2575f3660031901126108e2575f546001600160a01b03166080908152602090f35b634e487b7160e01b5f52604160045260245ffd5b6040519190601f01601f1916820167ffffffffffffffff81118382101761095457604052565b61091a565b67ffffffffffffffff811161095457601f01601f191660200190565b908151811015610986570160200190565b634e487b7160e01b5f52603260045260245ffd5b600811156109a457565b634e487b7160e01b5f52602160045260245ffd5b60ff1660088110156109a45790565b3d156109e7573d906109db61006183610959565b9182523d5f602084013e565b606090565b602060409281835280519182918282860152018484015e5f828201840152601f01601f1916010190565b90610a2361006183610959565b8281528092610a34601f1991610959565b0190602036910137565b8160209193929301015160601c60148301809311610a5a579190565b634e487b7160e01b5f52601160045260245ffd5b91909161ff0080610a7f8584610975565b5160f01c16169060018401808511610a5a57610a9a91610975565b5160f81c9060028401809411610a5a57179190565b8160209193929301015160208301809311610a5a57919056fed2f6c0020d30a86146de6300741f2bd90869bddf3818f8d3294ae782f6216176416a008d195f46a118bc00a1b9556fcd514fef4e1796c4990e6a8195dff122e6a2646970667358221220c6257ef08d6a27c1c1a8af48730a58ce44a182cfa7ebfc5e64d712eb9fee791e64736f6c634300081a0033");
    const DELEGATE_PROXY_INIT: [u8; 747] = hex!("60a06040526102eb8038038061001481610130565b928339810160408282031261012c578151916001600160a01b0383169081840361012c576020810151906001600160401b03821161012c57019180601f8401121561012c5782519361006d61006886610169565b610130565b9085825260208201926020878701011161012c575f602087829882849901875e8401015284546001600160a01b0319163317855560805251915af43d15610127573d6100bb61006882610169565b9081525f60203d92013e5b156100e257604051610166908161018582396080518160350152f35b60405162461bcd60e51b815260206004820152601360248201527f44454c454741544543414c4c5f4641494c4544000000000000000000000000006044820152606490fd5b6100c6565b5f80fd5b6040519190601f01601f191682016001600160401b0381118382101761015557604052565b634e487b7160e01b5f52604160045260245ffd5b6001600160401b03811161015557601f01601f19166020019056fe60808060405260043610156100ff575b505f546001600160a01b031633036100fb575f80604051368282378036810183815203907f00000000000000000000000000000000000000000000000000000000000000005af43d156100f6573d67ffffffffffffffff81116100e25760405190601f8101601f19908116603f0116820167ffffffffffffffff8111838210176100e25760405281525f60203d92013e5b156100a757005b60405162461bcd60e51b81526020600482015260136024820152721111531151d0551150d0531317d19052531151606a1b6044820152606490fd5b634e487b7160e01b5f52604160045260245ffd5b6100a0565b5f80fd5b5f3560e01c638da5cb5b0361000f57346100fb575f3660031901126100fb575f546001600160a01b03168152602090f3fea2646970667358221220920a7362c01a0c29d5e824bcaa30d06c06afce0bd1f9a254c921db71e3365f3464736f6c634300081a0033");

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


    #[tokio::test]
    async fn test_wallet_can_proxycreate() {
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


        // Create dellegate proxy
        let fb = FlowBuilder::empty()
        .create(Address::ZERO, &DELEGATE_PROXY_INIT, U256::ZERO);

        let tx = TransactionRequest::default()
            .with_from(wallet)
            .with_to(executor)
            .with_value(U256::ZERO)
            .with_input(fb.build());
        println!("X {:?}", tx);

        let tx_hash = provider.eth_send_unsigned_transaction(tx).await.unwrap();

        provider.evm_mine(None).await.unwrap();

        let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap().unwrap();
        println!("RECO {:?}", receipt);
        for log in receipt.inner.as_receipt().unwrap().logs.iter(){
            //println!("TX receipt {}", hex::decode(hex::encode(&log.data().data[64..])).unwrap());
            if &log.data().data.len() > &64 {
                println!("TX receipt {}", str::from_utf8(&log.data().data[64..]).unwrap());
            }else{
                println!("TX receipt {}", &log.data().data);

            }
        }
        
        let account_balance = provider.get_balance(executor).await.unwrap();
        assert_eq!( account_balance, U256::ZERO);  // executor shoud shave sent the value to weth9

    }


}
