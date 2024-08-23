use alloy::{
    hex,
    network::{EthereumWallet, TransactionBuilder},
    node_bindings::Anvil,
    primitives::{keccak256, Address},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::LocalSigner,
    sol,
    sol_types::SolValue,
    transports::Transport,
};
use bytecode::Bytecode;
use eyre::Result;
use karak_bls::keypair_signer::KeypairSigner;
use karak_bls::registration::BlsRegistration;
use karak_kms::keypair::{
    bn254::{self},
    traits::Keypair,
};
use karak_kms::signer::traits::Signer;
use BlsSdk::BlsSdkInstance;
use Verify::VerifyInstance;

mod bytecode;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    BlsSdk,
    "tests/artifacts/BlsSdk.json",
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    BN254,
    "tests/artifacts/BN254.json",
);

// import {BN254} from "./utils/BN254.sol";
// import {BlsSdk} from "./BlsSdk.sol";

// contract Verify {
//     function verifySignature(bytes memory input) external view returns (bool) {
//         (BN254.G1Point memory g1Key, BN254.G2Point memory g2Key, BN254.G1Point memory sign, bytes32 msgHash) =
//             abi.decode(input, (BN254.G1Point, BN254.G2Point, BN254.G1Point, bytes32));
//         BlsSdk.verifySignature(g1Key, g2Key, sign, msgHash);
//         return true;
//     }
// }
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Verify,
    "tests/artifacts/Verify/abi.json",
);

// TODO: Deploy only once
async fn deploy_bls_sdk<T: Transport + Clone, P: Provider<T>>(
    provider: P,
) -> Result<BlsSdkInstance<T, P>> {
    let bls_sdk = BlsSdk::deploy(provider).await?;
    println!("BlsSdk deployed at: {}", bls_sdk.address());
    Ok(bls_sdk)
}

// TODO: Deploy only once
async fn deploy_verify<T: Transport + Clone, P: Provider<T> + Clone>(
    provider: P,
    bls_sdk_address: &Address,
) -> Result<VerifyInstance<T, P>> {
    let bytecode = serde_json::from_str::<Bytecode>(
        std::fs::read_to_string("tests/artifacts/Verify/bytecode.json")?.as_str(),
    )?;

    // TODO: Refactor this out into a helper function
    let offset = bytecode.link_references["src/BlsSdk.sol"]["BlsSdk"][0].clone();
    let mut buffer = bytecode.object.into_bytes();
    // 2 for 0x, 2 for each byte
    buffer[2 + 2 * offset.start..2 + 2 * (offset.start + offset.length)]
        .copy_from_slice(bls_sdk_address.to_string()[2..].as_bytes());
    let bytecode = hex::decode(&buffer[2..])?;

    let tx = TransactionRequest::default().with_deploy_code(bytecode);
    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;
    let address = receipt
        .contract_address
        .ok_or(eyre::eyre!("Contract deployment failed"))?;

    println!("Verify deployed at: {}", address);
    Ok(Verify::new(address, provider))
}

#[tokio::test]
async fn test_registration() -> Result<()> {
    let anvil = Anvil::new().try_spawn()?;
    let wallet = EthereumWallet::from(LocalSigner::from(anvil.keys()[0].clone()));
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(anvil.endpoint_url());

    let bls_sdk = deploy_bls_sdk(provider.clone()).await?;
    let verify = deploy_verify(provider.clone(), bls_sdk.address()).await?;

    let keypair = bn254::Keypair::generate();

    let signer = KeypairSigner::from(keypair.clone());
    let message = keccak256(b"hello world");
    let signature = signer.sign_message(message)?;
    let registration = BlsRegistration {
        g1_pubkey: keypair.public_key().g1,
        g2_pubkey: keypair.public_key().g2,
        signature,
        msg_hash: message,
    };

    let verified = verify
        .verifySignature(registration.abi_encode().into())
        .call()
        .await?;
    assert!(verified._0);

    Ok(())
}
