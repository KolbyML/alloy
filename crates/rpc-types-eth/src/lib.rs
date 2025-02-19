#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[macro_use]
#[allow(unused_imports)]
extern crate alloc;

/// Standardized collections across `std` and `no_std` environments.
pub mod collections {
    cfg_if::cfg_if! {
        if #[cfg(feature = "std")] {
            pub use std::collections::{hash_set, HashMap, HashSet};
            use hashbrown as _;
        } else {
            pub use hashbrown::{hash_set, HashMap, HashSet};
        }
    }
}

pub use alloy_eips::eip4895::Withdrawal;

mod account;
pub use account::*;

mod block;
pub use block::*;

#[cfg(feature = "serde")]
use alloy_serde::WithOtherFields;

/// A catch-all block type for handling blocks on multiple networks.
#[cfg(feature = "serde")]
pub type AnyNetworkBlock = WithOtherFields<Block<WithOtherFields<Transaction>>>;

pub use alloy_network_primitives::{
    BlockTransactionHashes, BlockTransactions, BlockTransactionsKind,
};

mod call;
pub use call::{Bundle, EthCallResponse, StateContext, TransactionIndex};

pub mod error;

mod fee;
pub use fee::{FeeHistory, TxGasAndReward};

mod filter;
pub use filter::*;

mod index;
pub use index::Index;

mod log;
pub use log::*;

#[cfg(feature = "serde")]
pub mod pubsub;

mod raw_log;
pub use raw_log::{logs_bloom, Log as RawLog};

pub mod state;

mod syncing;
pub use syncing::*;

pub mod transaction;
pub use transaction::*;

mod work;
pub use work::Work;

/// This module provides implementations for EIP-4337.
pub mod erc4337;
pub use erc4337::{
    PackedUserOperation, SendUserOperation, SendUserOperationResponse, UserOperation,
    UserOperationGasEstimation, UserOperationReceipt,
};

pub mod simulate;
