use ethers::contract::ContractFactory;
use ethers::prelude::*;
use ethers::types::Address;

// Define the ERC-20 token contract ABI
const ERC20_ABI: &[u8] = include_bytes!("erc20_abi.json");

// Main function to deploy the contract
async fn deploy_contract(provider: &Provider<Http>, signer: &Signer) -> ContractResult<Address> {
    // Create a factory for deploying contracts
    let factory = ContractFactory::new(ERC20_ABI, "Erc20")
        .provider(provider.clone())
        .signer(signer.clone());

    // Deploy the contract with initial parameters
    let contract = factory
        .deploy(("MyToken", "MTK", U256::from(1_000_000)))
        .await?;
    
    // Get the address of the deployed contract
    let contract_address = contract.address();
    println!("Contract deployed at: {:?}", contract_address);

    // Return the contract address
    Ok(contract_address)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a local Ethereum node (replace with your own endpoint)
    let provider = Provider::<Http>::new("http://localhost:8545");
    let provider = provider.timeout(std::time::Duration::from_secs(20));

    // Use a local key for deploying (replace with your own private key)
    let wallet = "YOUR_PRIVATE_KEY".parse()?;
    let wallet = Wallet::new(wallet, provider.clone());

    // Deploy the contract
    let contract_address = deploy_contract(&provider, &wallet).await?;

    // Print the deployed contract address
    println!("Contract deployed at: {:?}", contract_address);

    Ok(())
}
