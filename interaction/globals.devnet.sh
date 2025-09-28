PROXY="https://devnet-gateway.multiversx.com"
CHAIN_ID="D"

SC_PATH="../"
SC_NAME=$(grep -oP 'name = "\K[^"]+' $SC_PATH"Cargo.toml")
SC_BYTECODE=$SC_PATH"output/$SC_NAME.wasm"

source $SC_PATH".env.devnet"

# Validate that PEM file exists and is readable
if [ -z "$PEM" ]; then
    echo "Error: PEM variable is not set in .env.devnet file"
    exit 1
fi

if [ ! -f "$PEM" ]; then
    echo "Error: PEM file '$PEM' does not exist"
    exit 1
fi

if [ ! -r "$PEM" ]; then
    echo "Error: PEM file '$PEM' is not readable"
    exit 1
fi

# Validate that SC_ADDRESS is set and not empty
if [ -z "$SC_ADDRESS" ]; then
    echo "Error: SC_ADDRESS variable is not set in .env.devnet file"
    echo "Please deploy the smart contract first and set SC_ADDRESS in .env.devnet"
    exit 1
fi

# Validate that SC_ADDRESS has the correct format (MultiversX address)
if [[ ! "$SC_ADDRESS" =~ ^erd1[a-z0-9]{58}$ ]]; then
    echo "Error: SC_ADDRESS '$SC_ADDRESS' is not a valid MultiversX address format"
    exit 1
fi

SC_ADDRESS_HEX=$(mxpy wallet bech32 --decode $SC_ADDRESS)

OWNER_PEM=$PEM
OWNER_ADDRESS=$(mxpy wallet convert --infile $OWNER_PEM --in-format pem --out-format address-bech32 | sed -n '3p')

# Validate that owner address was successfully extracted
if [ -z "$OWNER_ADDRESS" ]; then
    echo "Error: Failed to extract owner address from PEM file '$OWNER_PEM'"
    echo "Please verify the PEM file is valid and properly formatted"
    exit 1
fi

OWNER_ADDRESS_HEX=$(mxpy wallet bech32 --decode $OWNER_ADDRESS)

EGLD="EGLD"
EGLD_HEX=$(./encode.sh $EGLD)

WEGLD="WEGLD-d7c6bb"
WEGLD_HEX=$(./encode.sh $WEGLD)

USDC="USDC-8d4068"
USDC_HEX=$(./encode.sh $USDC)