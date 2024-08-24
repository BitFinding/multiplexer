pub use action_executor::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod action_executor {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("LOG"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LOG"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LOGADDR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LOGADDR"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LOGBOOL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LOGBOOL"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LOGBYTES"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LOGBYTES"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ACTIONEXECUTOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90Ua\x08\xA4\x80a\0\"_9_\xF3\xFE`\x80`@R`\x046\x10a\0!W_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x14a\0\xBCWa\0(V[6a\0(W\0[_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\0_\x90` \x80\x82R`\x08\x90\x82\x01RgCALLBACK`\xC0\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\0|W_\x80\xFD[a\0\xBA_6\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\0\xF6\x92PPPV[\0[4\x80\x15a\0\xC7W_\x80\xFD[P_Ta\0\xDA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\x01/\x90` \x80\x82R`\n\x90\x82\x01RiEXEACTIONS`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1_\x80\x80``[\x84Q\x84\x10\x15a\x06\xDAW_\x85\x85\x81Q\x81\x10a\x01YWa\x01Ya\x07~V[\x01` \x01Q`\xF8\x1C`\x07\x81\x11\x15a\x01rWa\x01ra\x07\x92V[`\x01\x95\x90\x95\x01\x94\x90P_\x81`\x07\x81\x11\x15a\x01\x8EWa\x01\x8Ea\x07\x92V[\x03a\x02.W_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\x01\xCB\x90` \x80\x82R`\t\x90\x82\x01RhCLEARDATA`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1_a\x01\xDE\x87\x87a\x06\xE1V[\x96P\x90P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xFBWa\x01\xFBa\x07\xA6V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x02%W` \x82\x01\x81\x806\x837\x01\x90P[P\x92PPa\x06\xD4V[`\x01\x81`\x07\x81\x11\x15a\x02BWa\x02Ba\x07\x92V[\x03a\x02\xD7W_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\x02}\x90` \x80\x82R`\x07\x90\x82\x01RfSETDATA`\xC8\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1_\x80a\x02\x91\x88\x88a\x06\xE1V[\x97P\x91Pa\x02\x9F\x88\x88a\x06\xE1V[\x97P\x90P_[\x81\x81\x10\x15a\x02\xCFW_a\x02\xB8\x8A\x8Aa\x07JV[` \x84\x81\x02\x89\x01\x01\x91\x90\x91R\x98PP`\x01\x01a\x02\xA5V[PPPa\x06\xD4V[`\x02\x81`\x07\x81\x11\x15a\x02\xEBWa\x02\xEBa\x07\x92V[\x03a\x03AW_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\x03&\x90` \x80\x82R`\x07\x90\x82\x01Rf)\xA2\xAA \xA2\")`\xC9\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1a\x038\x86\x86a\x07dV[\x95P\x93Pa\x06\xD4V[`\x03\x81`\x07\x81\x11\x15a\x03UWa\x03Ua\x07\x92V[\x03a\x03\xACW_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\x03\x91\x90` \x80\x82R`\x08\x90\x82\x01RgSETVALUE`\xC0\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1a\x03\xA3\x86\x86a\x07JV[\x95P\x92Pa\x06\xD4V[`\x04\x81`\x07\x81\x11\x15a\x03\xC0Wa\x03\xC0a\x07\x92V[\x03a\x04VW_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\x03\xFF\x90` \x80\x82R`\x0B\x90\x82\x01RjEXTCODECOPY`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1_\x80_\x80a\x04\x15\x8A\x8Aa\x07dV[\x99P\x93Pa\x04#\x8A\x8Aa\x06\xE1V[\x99P\x92Pa\x041\x8A\x8Aa\x06\xE1V[\x99P\x91Pa\x04?\x8A\x8Aa\x06\xE1V[\x99P\x90P\x80\x82\x87\x85\x01` \x01\x86<PPPPa\x06\xD4V[`\x05\x81`\x07\x81\x11\x15a\x04jWa\x04ja\x07\x92V[\x03a\x05\xB9W_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\x04\xA2\x90` \x80\x82R`\x04\x90\x82\x01Rc\x10\xD0S\x13`\xE2\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@Q`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R\x7F\xB8J\xE1\x8B\xE1\xD2\xE5\xA3\xA0%\xB0#G\x13\x04\x8B?\x07!\x90q\xB2\xA53G\xBAY\xE4L\x1D@\xBF\x90` \x01`@Q\x80\x91\x03\x90\xA1\x7Fa\x19;\xD2\xFE5\xA1\xA6\x99\x93\x8A\x95\xFC\xBD\xE5\xC2\xC4\xF2K\xB1\x0C\x90\xC4/\xCF\xA7T\xD4,\x06>\xAA\x82`@Qa\x05\x15\x91\x90a\x07\xDCV[`@Q\x80\x91\x03\x90\xA1_\x84`\x01`\x01`\xA0\x1B\x03\x16\x84\x84`@Qa\x057\x91\x90a\x08\x0EV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x05qW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x05vV[``\x91P[PP`@Q\x81\x15\x15\x81R\x90\x91P\x7F\xE1\x1C\x90\xDD\x1E\xF1\xA96Q\nJ\x96\x81(\xAB:)0\xBD\xB0\xF1\x90\xFE\xDA\x81*\xC9\xC8?J\xF8\xC9\x90` \x01`@Q\x80\x91\x03\x90\xA1_\x93PPa\x06\xD4V[`\x06\x81`\x07\x81\x11\x15a\x05\xCDWa\x05\xCDa\x07\x92V[\x03a\x06!W_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\x06\x07\x90` \x80\x82R`\x06\x90\x82\x01ReCREATE`\xD0\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x81Q` \x83\x01\x84\xF0\x93P_\x92Pa\x06\xD4V[`\x07\x81`\x07\x81\x11\x15a\x065Wa\x065a\x07\x92V[\x03a\x06\xD4W_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\x06q\x90` \x80\x82R`\x08\x90\x82\x01RgDELEGATE`\xC0\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1_\x84`\x01`\x01`\xA0\x1B\x03\x16\x83`@Qa\x06\x92\x91\x90a\x08\x0EV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x06\xCAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x06\xCFV[``\x91P[PPPP[Pa\x01=V[PPPPPV[_\x80\x80\x84a\x06\xF0\x85`\x01a\x08)V[\x81Q\x81\x10a\x07\0Wa\x07\0a\x07~V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16`\x08\x86\x86\x81Q\x81\x10a\x07%Wa\x07%a\x07~V[\x01` \x01Q`\xF8\x1C\x90\x1B\x17\x90P\x80a\x07>\x85`\x02a\x08)V[\x92P\x92PP\x92P\x92\x90PV[_\x80_` \x84\x86\x01\x01Q\x90P\x80\x84` a\x07>\x91\x90a\x08)V[\x81\x81\x01` \x01Q_\x90\x81\x90``\x1C\x80a\x07>\x85`\x14a\x08)V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_[\x83\x81\x10\x15a\x07\xD4W\x81\x81\x01Q\x83\x82\x01R` \x01a\x07\xBCV[PP_\x91\x01RV[` \x81R_\x82Q\x80` \x84\x01Ra\x07\xFA\x81`@\x85\x01` \x87\x01a\x07\xBAV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[_\x82Qa\x08\x1F\x81\x84` \x87\x01a\x07\xBAV[\x91\x90\x91\x01\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x08HWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x92\x91PPV\xFE\xD2\xF6\xC0\x02\r0\xA8aF\xDEc\0t\x1F+\xD9\x08i\xBD\xDF8\x18\xF8\xD3)J\xE7\x82\xF6!av\xA2dipfsX\"\x12 ~\x0B\xB4\xEF\x8F\xC4\x0C\x11\xF7X\x14\x9E\t\xC0\xBCt\xE7\xE4\xF1\x9D2\xE8\x8E\x0F\xE5\xD5\xF5C\x19\xDD\xFB\xE6dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static ACTIONEXECUTOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0!W_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x14a\0\xBCWa\0(V[6a\0(W\0[_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\0_\x90` \x80\x82R`\x08\x90\x82\x01RgCALLBACK`\xC0\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\0|W_\x80\xFD[a\0\xBA_6\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\0\xF6\x92PPPV[\0[4\x80\x15a\0\xC7W_\x80\xFD[P_Ta\0\xDA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\x01/\x90` \x80\x82R`\n\x90\x82\x01RiEXEACTIONS`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1_\x80\x80``[\x84Q\x84\x10\x15a\x06\xDAW_\x85\x85\x81Q\x81\x10a\x01YWa\x01Ya\x07~V[\x01` \x01Q`\xF8\x1C`\x07\x81\x11\x15a\x01rWa\x01ra\x07\x92V[`\x01\x95\x90\x95\x01\x94\x90P_\x81`\x07\x81\x11\x15a\x01\x8EWa\x01\x8Ea\x07\x92V[\x03a\x02.W_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\x01\xCB\x90` \x80\x82R`\t\x90\x82\x01RhCLEARDATA`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1_a\x01\xDE\x87\x87a\x06\xE1V[\x96P\x90P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xFBWa\x01\xFBa\x07\xA6V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x02%W` \x82\x01\x81\x806\x837\x01\x90P[P\x92PPa\x06\xD4V[`\x01\x81`\x07\x81\x11\x15a\x02BWa\x02Ba\x07\x92V[\x03a\x02\xD7W_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\x02}\x90` \x80\x82R`\x07\x90\x82\x01RfSETDATA`\xC8\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1_\x80a\x02\x91\x88\x88a\x06\xE1V[\x97P\x91Pa\x02\x9F\x88\x88a\x06\xE1V[\x97P\x90P_[\x81\x81\x10\x15a\x02\xCFW_a\x02\xB8\x8A\x8Aa\x07JV[` \x84\x81\x02\x89\x01\x01\x91\x90\x91R\x98PP`\x01\x01a\x02\xA5V[PPPa\x06\xD4V[`\x02\x81`\x07\x81\x11\x15a\x02\xEBWa\x02\xEBa\x07\x92V[\x03a\x03AW_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\x03&\x90` \x80\x82R`\x07\x90\x82\x01Rf)\xA2\xAA \xA2\")`\xC9\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1a\x038\x86\x86a\x07dV[\x95P\x93Pa\x06\xD4V[`\x03\x81`\x07\x81\x11\x15a\x03UWa\x03Ua\x07\x92V[\x03a\x03\xACW_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\x03\x91\x90` \x80\x82R`\x08\x90\x82\x01RgSETVALUE`\xC0\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1a\x03\xA3\x86\x86a\x07JV[\x95P\x92Pa\x06\xD4V[`\x04\x81`\x07\x81\x11\x15a\x03\xC0Wa\x03\xC0a\x07\x92V[\x03a\x04VW_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\x03\xFF\x90` \x80\x82R`\x0B\x90\x82\x01RjEXTCODECOPY`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1_\x80_\x80a\x04\x15\x8A\x8Aa\x07dV[\x99P\x93Pa\x04#\x8A\x8Aa\x06\xE1V[\x99P\x92Pa\x041\x8A\x8Aa\x06\xE1V[\x99P\x91Pa\x04?\x8A\x8Aa\x06\xE1V[\x99P\x90P\x80\x82\x87\x85\x01` \x01\x86<PPPPa\x06\xD4V[`\x05\x81`\x07\x81\x11\x15a\x04jWa\x04ja\x07\x92V[\x03a\x05\xB9W_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\x04\xA2\x90` \x80\x82R`\x04\x90\x82\x01Rc\x10\xD0S\x13`\xE2\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@Q`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R\x7F\xB8J\xE1\x8B\xE1\xD2\xE5\xA3\xA0%\xB0#G\x13\x04\x8B?\x07!\x90q\xB2\xA53G\xBAY\xE4L\x1D@\xBF\x90` \x01`@Q\x80\x91\x03\x90\xA1\x7Fa\x19;\xD2\xFE5\xA1\xA6\x99\x93\x8A\x95\xFC\xBD\xE5\xC2\xC4\xF2K\xB1\x0C\x90\xC4/\xCF\xA7T\xD4,\x06>\xAA\x82`@Qa\x05\x15\x91\x90a\x07\xDCV[`@Q\x80\x91\x03\x90\xA1_\x84`\x01`\x01`\xA0\x1B\x03\x16\x84\x84`@Qa\x057\x91\x90a\x08\x0EV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x05qW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x05vV[``\x91P[PP`@Q\x81\x15\x15\x81R\x90\x91P\x7F\xE1\x1C\x90\xDD\x1E\xF1\xA96Q\nJ\x96\x81(\xAB:)0\xBD\xB0\xF1\x90\xFE\xDA\x81*\xC9\xC8?J\xF8\xC9\x90` \x01`@Q\x80\x91\x03\x90\xA1_\x93PPa\x06\xD4V[`\x06\x81`\x07\x81\x11\x15a\x05\xCDWa\x05\xCDa\x07\x92V[\x03a\x06!W_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\x06\x07\x90` \x80\x82R`\x06\x90\x82\x01ReCREATE`\xD0\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x81Q` \x83\x01\x84\xF0\x93P_\x92Pa\x06\xD4V[`\x07\x81`\x07\x81\x11\x15a\x065Wa\x065a\x07\x92V[\x03a\x06\xD4W_\x80Q` a\x08O\x839\x81Q\x91R`@Qa\x06q\x90` \x80\x82R`\x08\x90\x82\x01RgDELEGATE`\xC0\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1_\x84`\x01`\x01`\xA0\x1B\x03\x16\x83`@Qa\x06\x92\x91\x90a\x08\x0EV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x06\xCAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x06\xCFV[``\x91P[PPPP[Pa\x01=V[PPPPPV[_\x80\x80\x84a\x06\xF0\x85`\x01a\x08)V[\x81Q\x81\x10a\x07\0Wa\x07\0a\x07~V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16`\x08\x86\x86\x81Q\x81\x10a\x07%Wa\x07%a\x07~V[\x01` \x01Q`\xF8\x1C\x90\x1B\x17\x90P\x80a\x07>\x85`\x02a\x08)V[\x92P\x92PP\x92P\x92\x90PV[_\x80_` \x84\x86\x01\x01Q\x90P\x80\x84` a\x07>\x91\x90a\x08)V[\x81\x81\x01` \x01Q_\x90\x81\x90``\x1C\x80a\x07>\x85`\x14a\x08)V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_[\x83\x81\x10\x15a\x07\xD4W\x81\x81\x01Q\x83\x82\x01R` \x01a\x07\xBCV[PP_\x91\x01RV[` \x81R_\x82Q\x80` \x84\x01Ra\x07\xFA\x81`@\x85\x01` \x87\x01a\x07\xBAV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[_\x82Qa\x08\x1F\x81\x84` \x87\x01a\x07\xBAV[\x91\x90\x91\x01\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x08HWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x92\x91PPV\xFE\xD2\xF6\xC0\x02\r0\xA8aF\xDEc\0t\x1F+\xD9\x08i\xBD\xDF8\x18\xF8\xD3)J\xE7\x82\xF6!av\xA2dipfsX\"\x12 ~\x0B\xB4\xEF\x8F\xC4\x0C\x11\xF7X\x14\x9E\t\xC0\xBCt\xE7\xE4\xF1\x9D2\xE8\x8E\x0F\xE5\xD5\xF5C\x19\xDD\xFB\xE6dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static ACTIONEXECUTOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ActionExecutor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ActionExecutor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ActionExecutor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ActionExecutor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ActionExecutor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ActionExecutor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ActionExecutor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ACTIONEXECUTOR_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                ACTIONEXECUTOR_ABI.clone(),
                ACTIONEXECUTOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `LOG` event
        pub fn log_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogFilter> {
            self.0.event()
        }
        ///Gets the contract's `LOGADDR` event
        pub fn logaddr_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogaddrFilter> {
            self.0.event()
        }
        ///Gets the contract's `LOGBOOL` event
        pub fn logbool_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogboolFilter> {
            self.0.event()
        }
        ///Gets the contract's `LOGBYTES` event
        pub fn logbytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogbytesFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ActionExecutorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ActionExecutor<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "LOG", abi = "LOG(string)")]
    pub struct LogFilter(pub ::std::string::String);
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "LOGADDR", abi = "LOGADDR(address)")]
    pub struct LogaddrFilter(pub ::ethers::core::types::Address);
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "LOGBOOL", abi = "LOGBOOL(bool)")]
    pub struct LogboolFilter(pub bool);
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "LOGBYTES", abi = "LOGBYTES(bytes)")]
    pub struct LogbytesFilter(pub ::ethers::core::types::Bytes);
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum ActionExecutorEvents {
        LogFilter(LogFilter),
        LogaddrFilter(LogaddrFilter),
        LogboolFilter(LogboolFilter),
        LogbytesFilter(LogbytesFilter),
    }
    impl ::ethers::contract::EthLogDecode for ActionExecutorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(ActionExecutorEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogaddrFilter::decode_log(log) {
                return Ok(ActionExecutorEvents::LogaddrFilter(decoded));
            }
            if let Ok(decoded) = LogboolFilter::decode_log(log) {
                return Ok(ActionExecutorEvents::LogboolFilter(decoded));
            }
            if let Ok(decoded) = LogbytesFilter::decode_log(log) {
                return Ok(ActionExecutorEvents::LogbytesFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ActionExecutorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LogFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogaddrFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogboolFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogbytesFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LogFilter> for ActionExecutorEvents {
        fn from(value: LogFilter) -> Self {
            Self::LogFilter(value)
        }
    }
    impl ::core::convert::From<LogaddrFilter> for ActionExecutorEvents {
        fn from(value: LogaddrFilter) -> Self {
            Self::LogaddrFilter(value)
        }
    }
    impl ::core::convert::From<LogboolFilter> for ActionExecutorEvents {
        fn from(value: LogboolFilter) -> Self {
            Self::LogboolFilter(value)
        }
    }
    impl ::core::convert::From<LogbytesFilter> for ActionExecutorEvents {
        fn from(value: LogbytesFilter) -> Self {
            Self::LogbytesFilter(value)
        }
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
}
