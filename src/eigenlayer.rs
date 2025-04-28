use alloy_primitives::{address, Address, BlockHash, Bytes, U256};
use alloy_sol_types::{sol, SolCall};

use cartesi_coprocessor_evm::{
    evm::{EVMError, EVM},
    gio_client::GIOClient,
};

pub async fn query_operator_token_balance(
    gio_client: GIOClient,
    block_hash: BlockHash,
    operator: Address,
    erc20: Address,
) -> Result<U256, EVMError> {
    sol! {
        function balanceOf(address) external view returns (uint256);
    }
    let encoded = balanceOfCall::new((operator,)).abi_encode();

    let mut evm = EVM::new(gio_client, block_hash);
    let resutl = evm.call(
        address!("0000000000000000000000000000000000000000"),
        erc20,
        0,
        U256::ZERO,
        Bytes::from(encoded),
    )?;

    let balance = balanceOfCall::abi_decode_returns(&resutl)
        .expect("failed to decode return value of balanceOf call");

    Ok(balance)
}
