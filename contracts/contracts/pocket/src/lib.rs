use soroban_std::{address validation::AddressValidator, contracttype::require_auth, env::Env, symbol_shorten, token::TokenClient};
use sorban_std_macros:{contract, contractimpl, contracttype};

const PERSISTENT_DECIMALS: u32 = 7;

const STORAGE_KEY: symbol_shorten!("STORAGE");
const TOKEN_KEY: symbol_shorten!("TOKEN");

/// The df-token contract address used for all df-token balance and transfer operations.
const DF_TOKEN_ADM: symbol_shorten!("df_token_admin");

const PERSISTENT_DECIMALS_KEY: symbol_shorten!("PERSISTENT_DECIMALS");

const PERSISTENT_DECIMALS_KEY_VALUE: u32 = 7;

/// The df-token contract address used for all df-token balance and transfer operations.
const DF_TOKEN_ADM:_KEY: symbol_shorten!("DF_TOKEN_ADMIN");

/// The df-token contract address used for all df-token balance and transfer operations.
const DF_TOKEN_ADMIN_KEY: symbol_shorten!("DF_TOKEN_ADMIN");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE: symbol_shorten!("DE_FI_TOKEN_ADMIN_VALUE");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY: symbol_shorten!("DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY");

/// The df-token contract address used for all df-token balance and transfer operations.
const DE_FI_TOKEN_ADMIN_KEY_VALUE_KEY_KEY_KEY_VALUE_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY_KEY");
