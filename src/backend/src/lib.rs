use candid::{CandidType, Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// ... (existing imports and types)

#[derive(CandidType, Clone, Serialize, Deserialize, Default)]
struct User {
    id: u64,
    username: String,
    email: String,
    role: UserRole,
    created_at: u64,
    updated_at: Option<u64>,
}

// Define the UserRole enum
#[derive(CandidType, Clone, Serialize, Deserialize)]
enum UserRole {
    Policyholder,
    Agent,
    Administrator,
}

// Manually implementing Default for UserRole
impl Default for UserRole {
    fn default() -> Self {
        UserRole::Policyholder
    }
}

// Implementing Storable and BoundedStorable traits for User
impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(CandidType, Serialize, Deserialize, Default)]
struct UserPayload {
    username: String,
    email: String,
    role: UserRole,
}

// ... (existing thread-local variables and payload structure)

// New thread-local variables for our Life Insurance Policy Management app

thread_local! {
    static USER_MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static USER_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(USER_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter for users")
    );

    static USER_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> =
        RefCell::new(StableBTreeMap::init(
            USER_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

// Helper method to perform insert for User
fn do_insert_user(user: &User) {
    USER_STORAGE.with(|service| service.borrow_mut().insert(user.id, user.clone()));
}

#[derive(CandidType, Clone, Serialize, Deserialize, Default)]
struct LifeInsurancePolicy {
    id: u64,
    policyholder_id: u64,
    policy_amount: Option<u64>,
    is_active: bool,
    is_claimed: bool,
    tenure_years: u32,
    created_at: u64,
    updated_at: Option<u64>,
}

// Implementing Storable and BoundedStorable traits for LifeInsurancePolicy
impl Storable for LifeInsurancePolicy {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for LifeInsurancePolicy {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(CandidType, Serialize, Deserialize, Default)]
struct PolicyPayload {
    policyholder_id: u64,
    policy_amount: Option<u64>,
    tenure_years: u32,
}

// ... (existing thread-local variables and payload structure)

// New thread-local variables for our Life Insurance Policy Management app

thread_local! {
    static POLICY_MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static POLICY_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(POLICY_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter for life insurance policies")
    );

    static POLICY_STORAGE: RefCell<StableBTreeMap<u64, LifeInsurancePolicy, Memory>> =
        RefCell::new(StableBTreeMap::init(
            POLICY_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

// Helper method to perform insert for LifeInsurancePolicy
fn do_insert_life_insurance_policy(policy: &LifeInsurancePolicy) {
    POLICY_STORAGE.with(|service| service.borrow_mut().insert(policy.id, policy.clone()));
}

// get_user Function:
#[ic_cdk::query]
fn get_user(id: u64) -> Result<User, Error> {
    match _get_user(&id) {
        Some(user) => Ok(user),
        None => Err(Error::NotFound {
            msg: format!("a user with id={} not found", id),
        }),
    }
}

// 3.4.2 _get_user Function:
fn _get_user(id: &u64) -> Option<User> {
    USER_STORAGE.with(|s| s.borrow().get(id))
}

// 3.4.3 add_user Function:
#[ic_cdk::update]
fn add_user(user_payload: UserPayload) -> Option<User> {
    let id = USER_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter for users");
    let timestamp = time();
    let user = User {
        id,
        username: user_payload.username,
        email: user_payload.email,
        role: user_payload.role,
        created_at: timestamp,
        updated_at: None,
    };
    do_insert_user(&user);
    Some(user)
}

// 3.4.4 update_user Function:
#[ic_cdk::update]
fn update_user(id: u64, payload: UserPayload) -> Result<User, Error> {
    match USER_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut user) => {
            user.username = payload.username;
            user.email = payload.email;
            user.role = payload.role;
            user.updated_at = Some(time());
            do_insert_user(&user);
            Ok(user)
        }
        None => Err(Error::NotFound {
            msg: format!("couldn't update a user with id={}. user not found", id),
        }),
    }
}

// 3.4.5 delete_user Function:
#[ic_cdk::update]
fn delete_user(id: u64) -> Result<User, Error> {
    match USER_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(user) => Ok(user),
        None => Err(Error::NotFound {
            msg: format!("couldn't delete a user with id={}. user not found.", id),
        }),
    }
}

// 3.5.3 add_life_insurance_policy Function:
#[ic_cdk::update]
fn add_life_insurance_policy(policy_payload: PolicyPayload) -> Option<LifeInsurancePolicy> {
    let id = POLICY_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter for life insurance policies");
    let timestamp = time();
    let policy = LifeInsurancePolicy {
        id,
        policyholder_id: policy_payload.policyholder_id,
        policy_amount: policy_payload.policy_amount,
        is_active: true,   // By default, a new policy is active
        is_claimed: false, // By default, a new policy has not been claimed
        tenure_years: policy_payload.tenure_years,
        created_at: timestamp,
        updated_at: None,
    };
    do_insert_life_insurance_policy(&policy);
    Some(policy)
}

// 3.5.1 get_life_insurance_policy Function:
#[ic_cdk::query]
fn get_life_insurance_policy(id: u64) -> Result<LifeInsurancePolicy, Error> {
    match _get_life_insurance_policy(&id) {
        Some(policy) => Ok(policy),
        None => Err(Error::NotFound {
            msg: format!("a life insurance policy with id={} not found", id),
        }),
    }
}

// 3.5.2 _get_life_insurance_policy Function:
fn _get_life_insurance_policy(id: &u64) -> Option<LifeInsurancePolicy> {
    POLICY_STORAGE.with(|s| s.borrow().get(id))
}

// 3.5.4 update_life_insurance_policy Function:
#[ic_cdk::update]
fn update_life_insurance_policy(
    id: u64,
    payload: PolicyPayload,
) -> Result<LifeInsurancePolicy, Error> {
    match POLICY_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut policy) => {
            policy.policy_amount = payload.policy_amount;
            policy.tenure_years = payload.tenure_years;
            policy.updated_at = Some(time());
            do_insert_life_insurance_policy(&policy);
            Ok(policy)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a life insurance policy with id={}. policy not found",
                id
            ),
        }),
    }
}

// 3.5.5 delete_life_insurance_policy Function:
#[ic_cdk::update]
fn delete_life_insurance_policy(id: u64) -> Result<LifeInsurancePolicy, Error> {
    match POLICY_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(policy) => Ok(policy),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a life insurance policy with id={}. policy not found.",
                id
            ),
        }),
    }
}

#[ic_cdk::update]
fn claim_policy(policy_id: u64) -> Result<LifeInsurancePolicy, Error> {
    match _get_life_insurance_policy(&policy_id) {
        Some(policy) => {
            if policy.is_active && !policy.is_claimed {
                // Perform the update within _get_life_insurance_policy
                _update_life_insurance_policy(&policy_id, |mut stored_policy| {
                    stored_policy.is_claimed = true;
                    stored_policy.updated_at = Some(time());
                    stored_policy
                });
                Ok(policy.clone())
            } else {
                Err(Error::InvalidOperation {
                    msg: format!("cannot claim the policy with id={}", policy_id),
                })
            }
        }
        None => Err(Error::NotFound {
            msg: format!("policy with id={} not found", policy_id),
        }),
    }
}

// Helper method to update LifeInsurancePolicy within POLICY_STORAGE
fn _update_life_insurance_policy<F>(id: &u64, update_fn: F)
where
    F: FnOnce(LifeInsurancePolicy) -> LifeInsurancePolicy,
{
    // Get the policy using the _get_life_insurance_policy function
    if let Some(policy) = _get_life_insurance_policy(id) {
        // Apply the update function
        let updated_policy = update_fn(policy.clone());

        // Update the policy in the storage
        POLICY_STORAGE.with(|service| {
            let mut storage = service.borrow_mut();
            storage.insert(id.clone(), updated_policy);
        });
    }
}

// Define the Error enum
#[derive(CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidOperation { msg: String },
}

// To generate the Candid interface definitions for our canister
ic_cdk::export_candid!();
