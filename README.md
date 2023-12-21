# Life Insurance Policy Project

## Introduction

This project is designed to manage life insurance policies. It includes several functions to create, update, and manage policies.

## Getting Started

1. Clone the repository: `git clone https://github.com/yourusername/life_insurance_policy.git`
2. Navigate to the project directory: `cd life_insurance_policy`
3. Install the required dependencies: `npm install` (Node.js) or `pip install -r requirements.txt` (Python)

## Usage

Here is the basic hierarchy of function calls:

1. `add_user(UserPayload)`: This function is used to add a new user. The `UserPayload` parameter should be a record containing the necessary details for the user.
2. `add_life_insurance_policy(PolicyPayload)`: This function is used to add a new life insurance policy. The `PolicyPayload` parameter should be a record containing the necessary details for the policy.
3. `get_user(nat64)`, `get_user_by_username(text)`: These functions are used to retrieve user details. The parameters are the user ID and username respectively.
4. `get_life_insurance_policy(nat64)`, `get_active_policies()`, `get_all_life_insurance_policies()`, `get_policies_by_policyholder(nat64)`: These functions are used to retrieve policy details. The parameters are the policy ID and policyholder ID respectively.
5. `update_user(nat64, UserPayload)`: This function is used to update an existing user. The parameters are the user ID and an object containing the updated user details.
6. `update_life_insurance_policy(nat64, PolicyPayload)`: This function is used to update an existing policy. The parameters are the policy ID and an object containing the updated policy details.
7. `delete_user(nat64)`: This function is used to delete a user. The parameter is the user ID.
8. `delete_life_insurance_policy(nat64)`: This function is used to delete a policy. The parameter is the policy ID.
9. `claim_policy(nat64)`: This function is used to claim a policy. The parameter is the policy ID.
