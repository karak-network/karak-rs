use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IERC20Metadata,
    "abi/IERC20Metadata.json",
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IERC20,
    "abi/IERC20.json",
);
