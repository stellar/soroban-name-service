#![no_std]

use soroban_sdk::{contractimpl, contracttype, contracterror, Env, map, Map, BytesN, Address, Bytes};

pub struct Contract;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    // Resolution errors
    InvalidHashInput = 1,
    NotFound = 2,
    // Registration errors
    ParentNotFound = 3,
    NotAuthorized = 4,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    RMap
}

#[contracttype]
pub struct Node {
    pub owner: Address,
    pub p_hash: BytesN<32>,
    pub res_addr: Address
}

#[contractimpl]
impl Contract {
    pub fn init(env: Env) {
        if env.storage().has(DataKey::RMap) {
            panic!("Contract already initialized")
        }

        let mut map: Map<BytesN<32>, Node> = map![&env];

        // Root node is empty hash, owned by contract initializer
        map.set(BytesN::from_array(&env, &[0; 32]), Node {
            owner: env.invoker(),
            p_hash: BytesN::from_array(&env, &[0; 32]),
            res_addr: env.invoker() // This should be empty but I don't know how to default init Address
        });

        env.storage().set(DataKey::RMap, map)
    }

    // Given a nameHash, returns the associated address
    pub fn resolve(env: Env, hash: BytesN<32>) -> Result<Address, Error> {
        // Should not support empty queries, even if "technically" possible with initial root node
        if hash.is_empty() {
            return Err(Error::InvalidHashInput);
        }

        let map = Self::get_map(&env);
        match map.get(hash) {
            Some(node) => Ok(node.unwrap().res_addr),
            None => Err(Error::NotFound)
        }
    }

    // Registers subdomain under parent node
    pub fn register(env: Env, parent_hash: BytesN<32>, leaf_hash: BytesN<32>, owner: Address, res_addr: Address) -> Result<BytesN<32>, Error> {
        let mut map = Self::get_map(&env);

        // Check if parent hash exists
        let parent_node = match map.get(parent_hash.clone()) {
            Some(node) => node.unwrap(),
            None => return Err(Error::ParentNotFound)
        };

        // Check if invoker is authorized to create subdomain
        if !Self::auth_check(&env, &parent_node) {
            return Err(Error::NotAuthorized)
        }

        // Insert new node
        let key = Self::append_hash(&env, &parent_hash, &leaf_hash);
        map.set(key.clone(), Node {
            owner: owner,
            p_hash: parent_hash,
            res_addr
        });

        Ok(key)
    }

    // Checks if caller owns node or any of node's parents
    fn auth_check(env: &Env, node: &Node) -> bool {
        if node.owner == env.invoker() {
            return true
        }

        // If parent hash is empty, current node mut be root node
        let parent_hash = node.p_hash.clone();
        if parent_hash.is_empty() {
            return false
        }

        let map = Self::get_map(env);
        let parent_node = map.get(parent_hash).unwrap().unwrap();
        Self::auth_check(env, &parent_node)
    }

    fn append_hash(env: &Env, parent_hash: &BytesN<32>, leaf_hash: &BytesN<32>) -> BytesN<32> {
        let mut bytes = Bytes::new(env);
        bytes.append(&leaf_hash.clone().into());
        bytes.append(&parent_hash.clone().into());
        env.crypto().sha256(&bytes)
    }

    fn get_map(env: &Env) -> Map<BytesN<32>, Node> {
        return env.storage().get_unchecked(DataKey::RMap).unwrap()
    }
}

mod test;