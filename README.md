# Karak Core - Operator Registration

## Prerequisites
- Karak CLI
- Keystore set up (if using keystore)
- AWS KMS Key (if using AWS KMS)

## Add Karak cli

``` bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/karak-network/karak-rs/releases/download/karak-cli-v0.2.1/karak-cli-installer.sh | sh
```

## Steps to Register as Operator on Karak

### Step 1: Create Vault Command

Run the following command to create a vault for your operator:

```bash
karak operator create-vault \
  --assets 0xaf88d065e77c8cC2239327C5EDb3A432268e5831,0x82aF49447D8a07e3bd95BD0d56f35241523fBab1,0x35751007a407ca6FEFfE80b3cB397736D2cf4dbe,0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9 \
  --vault-impl 0x0000000000000000000000000000000000000000 \
  --core-address {CORE_ADDRESS} \
  --rpc-url {RPC_URL} \
  --secp256k1-keystore-type [aws,local] \
  --secp256k1-keystore-path {KEYSTORE_PATH} \
  --secp256k1-passphrase {KEYSTORE_PASSWORD} \
  --aws-region {AWS_REGION} \
  --aws-access-key-id AVXXXXXX \
  --aws-secret-access-key AVXXXX \
  --aws-operator-key-id {AWS_KMS_KEY_ID}
  ```

## Note

The `secp256k1-keystore-type` parameter can be set to either `"aws"` or `"local"`:

- **For local keystore**: You need to provide `secp256k1-keystore-path` and `secp256k1-passphrase`(optional if not specified can be entered via prompt) .
- **For AWS KMS**: You must specify `aws-region`, `aws-access-key-id`, `aws-secret-access-key`, and `aws-operator-key-id`.

### Command Breakdown:

| Parameter                     | Description                                                                                                                                                                                                                                                                                                                                                             |
|-------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `--assets`                     | Comma-separated list of asset addresses (optional, if not provided, you will be prompted):<br>- `0xaf88d065e77c8cC2239327C5EDb3A432268e5831` (USDC)<br>- `0x82aF49447D8a07e3bd95BD0d56f35241523fBab1` (WETH)<br>- `0x35751007a407ca6FEFfE80b3cB397736D2cf4dbe` (weETH)<br>- `0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9` (USDT) [All addresss are of arbitrum mainnnet] |
| `--vault-impl`                 | Vault implementation address: `0x0000000000000000000000000000000000000000`                                                                                                                                                                                                                                                                                              |
| `--core-address`               | Core address as per network                                                                                                                                                                                                                                                                                                                                             |
| `--secp256k1-keystore-type`    | Keystore type: `aws` or `local`                                                                                                                                                                                                                                                                                                                                         |
| `--secp256k1-keystore-path`    | Path to your keystore (when keystore type is 'local'): `/Path/to/keystore`                                                                                                                                                                                                                                                                                              |
| `--secp256k1-passphrase`       | Keystore passphrase (when keystore type is 'local', if not provided, it can be entered via prompt): `yourkeystorepassword`                                                                                                                                                                                                                                              |
| `--aws-region`                 | AWS region (when keystore type is 'aws'): `{AWS_REGION}`                                                                                                                                                                                                                                                                                                                |
| `--aws-access-key-id`          | AWS Access Key ID (when keystore type is 'aws'): `AVXXXXXX`                                                                                                                                                                                                                                                                                                             |
| `--aws-secret-access-key`      | AWS Secret Access Key (when keystore type is 'aws'): `AVXXXX`                                                                                                                                                                                                                                                                                                           |
| `--aws-operator-key-id`        | AWS Operator Key ID (when keystore type is 'aws'): `{AWS_KMS_KEY_ID}`                                                                                                                                                                                                                                                                                                   |

---


### Step 2: Reach Out for Final Registration

After completing the vault creation, contact the Karak Team for final registration on V2 with your operator address and KNS name (e.g., `v2.arbitrum.mainnet.operator.yourname`).

## Contract Address


| Contract                    | Arbitrum                                                                                                                                                                                    |
|-----------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `Core Address`              |0xc4B3D494c166eBbFF9C716Da4cec39B579795A0d |
