extern crate reqwest;

// use std::collections::HashMap;
extern crate web3;

use web3::futures::Future;
use web3::types::{TransactionRequest, U256};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let prime_factors: Vec<i64> = Vec::new(); // learn how to pass vectors properly and make them expandable...

    create_agent(
        "0x7f24339231c97f758649a3C699E0960A14194D1F".to_string(),
        "neo".to_string(),
        true,
        prime_factors
    );

    let (_eloop, transport) = web3::transports::Http::new("http://localhost:3030").unwrap();
    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().accounts().wait().unwrap();
    let balance_before = web3.eth().balance(accounts[0], None).wait().unwrap();

    let tx = TransactionRequest {
        from: accounts[0],
        to: Some(accounts[1]),
        gas: None,
        gas_price: None,
        value: Some(U256::from(99999999)),
        data: None,
        nonce: None,
        condition: None
    };

    let tx_hash = web3.eth().send_transaction(tx).wait().unwrap();

    let balance_after = web3.eth().balance(accounts[1], None).wait().unwrap();

    println!("TX Hash: {:?}", tx_hash);
    println!("Balance before: {}", balance_before);
    println!("Balance after: {}", balance_after);

    let content = "https://cloudflare-ipfs.com/ipfs/QmZtmD2qt6fJot32nabSP3CUjicnypEBz7bHVDhPQt9aAy";
    let resp = reqwest::get(content)?
        .text()?;

    println!("{:?}", resp);
    Ok(())
}

// Create Agent 

struct Agent {
    agent_address: String,
    agent_handle: String,
    is_verified: bool,
    membership: Vec<i64>
}

fn create_agent(_agent_address: String, _agent_handle: String, _is_verified: bool, _membership: Vec<i64>) {
    let a = Agent {
        agent_address: _agent_address.to_string(),
        agent_handle: _agent_handle.to_string(),
        is_verified: _is_verified,
        membership: _membership
    };
    println!("Agent: {} {} {}", a.agent_address, a.agent_handle, a.is_verified);
}

// create agent 
// add network(s) to agent -> vec


// add network to member
//    create instance of member at addr
//    create new networks data struct:
//       create Vec { network1:hash, network2:hash, network3:hash, network4:hash } 
//       reorder alphabetically (to cut down file duplication) (optimization)
//    create point: 
//       ipfs -> hash file
//       ipfs -> create link 
//    update networks in agent -> sign with local private key
//    publish new agent file -> sign with local private key
//    update agent_registry with ref to new file -> sign with local private key


// update apps available to a user within network
// create a new app point:
//    create new file:
//    ipfs -> hash file
//    ipfs -> create new link

// update link in networks
// update link in users
// update link in registry



// membership is link to :: https://gateway.ipfs.io/ipns/hash



// trait Quack {
//     fn quack(&self);
// }

// struct Duck ();

// impl Quack for Duck {
//     fn quack(&self) {
//         println!("quack!");
//     }
// }

// struct RandomBird {
//     is_a_parrot: bool
// }

// impl Quack for RandomBird {
//     fn quack(&self) {
//         if ! self.is_a_parrot {
//             println!("quack!");
//         } else {
//             println!("squawk!");
//         }
//     }
// }

// let duck1 = Duck();
// let duck2 = RandomBird{is_a_parrot: false};
// let parrot = RandomBird{is_a_parrot: true};

// let ducks: Vec<&Quack> = vec![&duck1,&duck2,&parrot];

// for d in &ducks {
//     d.quack();
// }
// // quack!
// // quack!
// // squawk!