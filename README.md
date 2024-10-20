# Karak Core - Operator Registration

## Repository
[Karak-rs GitHub Repository](https://github.com/karak-network/karak-rs)

## Steps to Register as Operator on Karak

### Step 1: Create Vault Command

Run the following command to create a vault for your operator:

```bash
cargo run -- operator create-vault \
  --assets 0xaf88d065e77c8cC2239327C5EDb3A432268e5831,0x82aF49447D8a07e3bd95BD0d56f35241523fBab1,0x35751007a407ca6FEFfE80b3cB397736D2cf4dbe,0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9 \
  --vault-impl 0x6dAB3085943Adb6C6785f51bab7EDc1f9e9B1077 \
  --core-address 0xb3E2dA61df98E44457190383e1FF13e1ea13280b \
  --secp256k1-keystore-type [aws,local] \
  --secp256k1-keystore-path /Users/Document/keystore \
  --secp256k1-passphrase keystorepassword \
  --aws-region ap-south-1 \
  --aws-access-key-id AVXXXXXX \
  --aws-secret-access-key AVXXXX \
  --aws-operator-key-id {AWS_KMS_KEY_ID}
  ```

### Command Breakdown:

- **--assets**  
  Comma-separated list of asset addresses [optional, if you do not pass this it will ask for asset in prompt]:  
  `0xaf88d065e77c8cC2239327C5EDb3A432268e5831(USDC), 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1(WETH), 0x35751007a407ca6FEFfE80b3cB397736D2cf4dbe(weETH), 0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9(USDT)`

- **--vault-impl**  
  Vault implementation address: `0x6dAB3085943Adb6C6785f51bab7EDc1f9e9B1077`

- **--core-address**  
  Core address: `0xb3E2dA61df98E44457190383e1FF13e1ea13280b`

- **--secp256k1-keystore-type**  
  Choose keystore type: `[aws, local]`

- **--secp256k1-keystore-path**  
  Path to your keystore(when keystore type is 'local'): `/Users/Document/keystore`

- **--secp256k1-passphrase**  
  Keystore passphrase(when keystore type is 'local'): `keystorepassword`

- **--aws-region**  
  AWS region(when keystore type is 'aws'): `ap-south-1`

- **--aws-access-key-id**  
  AWS Access Key ID(when keystore type is 'aws'): `AVXXXXXX`

- **--aws-secret-access-key**  
  AWS Secret Access Ke(when keystore type is 'aws')y: `AVXXXX`

- **--aws-operator-key-id**  
  AWS Operator Key ID(when keystore type is 'aws'): `{AWS_KMS_KEY_ID}`

### Step 2: Reach Out for Final Registration

After completing the vault creation, contact the Karak Team for final registration on V2 with your operator address and KNS name (e.g., `v1.arbitrum.mainnet.operator.yourname`).