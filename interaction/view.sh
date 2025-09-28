#!/bin/bash

mkdir -p reports

use_devnet() {
    source globals.devnet.sh
}

use_mainnet() {
    source globals.mainnet.sh
}

## Select env
case $1 in
    "D")
        echo "Use Devnet"
        use_devnet
        ;;
    "1")
        echo "Use Mainnet"
        use_mainnet
        ;;
    *)
        echo "Require MultiversX chain id (D, 1). Ex $0 D" && exit
        ;;
esac

source snippets.sh

# Add your custom smart contract queries below this line
# Available function: view
# Example: view getUserBalance "$(./encode.sh erd1...)"

######################## START ########################
