use alloy::sol;

sol!(
    #[allow(clippy::too_many_arguments)]
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20Mintable,
    "abi/ERC20Mintable.json",
);
