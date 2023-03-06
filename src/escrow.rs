use near_sdk::{env};
use near_sdk::collections::UnorderedMap;
use ethers::prelude::*;
use rust_web3::contract::tokens::Tokenize;
use rust_web3::contract::{Contract, Options};
use rust_web3::types::U256;


near_sdk::setup_alloc!();

pub payable struct MyPayableStruct {
    owner: Option<web3::types::Address>,
    commissionWallet: Option<web3::types::Address>,
    buyer:Option<web3::types::Address>,
    seller: Option<web3::types::Address>,
}

pub struct NumberStruct {
    minimumEscrowAmount: U256,
    commissionRate: U256,
    depositTime: U256,
}

enum State {
    INIT,
    FUNDED,
    ACCEPTED,
    RELEASED,
    REFUNDED,
    WITHDRAWED_BY_OWNER,
}

// modifiers

macro_rules! is_address_valid {
    ($addr:expr) => {
        {
            assert!($addr.len() == 20 && $addr != [0u8; 20], "Invalid address!");
            _
        }
    };
}

macro_rules! buyeronly {
    ($_buyer:expr) => {
        {
            assert!(_buyer == buyer, "Only accessible by buyer!");
            _
        }
    }
}

macro_rules! selleronly {
    ($_seller:expr) => {
        {
            assert!(_seller == seller, "Only accessible by seller!");
            _
        }
    }
}

macro_rules! owneronly {
    ($_addr:expr) => {
        {
            assert!(_addr == addr, "Only accessible by owner!");
            _
        }
    }
}

macro_rules! buyer_or_seller_only {
    ($_buyer:expr, $_seller: expr) => {
        {
            assert!(_buyer == buyer && _seller == seller);
            _
        }
    }
}


macro_rules! init_by_owner {
    ($_owner:expr) => {
        assert!(_owner != owner, "Deal not initialized yet!");
        _
    }
}

macro_rules! init_check {
    ($_owner:expr) => {
        assert!(_owner == owner, "Can't initialize a deal twice!");
        _
    }
}

macro_rules! state_init {
    () => {
        assert!(currentState == State.INIT, "State's not INIT!");
        _
    }
}

macro_rules! state_funded {
    () => {
        assert!(currentState == State.FUNDED, "Deal not funded yet!");
        _
    }
}

macro_rules! state_accepted {
    () => {
        assert!(currentState == State.ACCEPTED, "Deal not accepted yet!");
        _
    }
}