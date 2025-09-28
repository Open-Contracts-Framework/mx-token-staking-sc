#!/bin/bash

# Function to convert string to hex
string_to_hex() {
    echo -n "$1" | xxd -p
}

# Function to check if a string is a number
is_number() {
    [[ $1 =~ ^[0-9]+$ ]]
}

# Function to convert number to hex with proper padding
number_to_hex() {
    local num=$1
    local hex=$(printf "%x" "$num")
    
    # Add leading zero if odd length (excluding 0x prefix)
    if [ $((${#hex} % 2)) -eq 1 ]; then
        hex="0$hex"
    fi
    
    echo "$hex"
}

# Process each command line argument
for arg in "$@"; do
    # Check if it's an MultiversX address (starts with 'erd' and is 62 characters long)
    if [[ "$arg" =~ ^erd.* ]] && [ ${#arg} -eq 62 ]; then
        # Try to decode the address using mxpy
        if decoded=$(mxpy wallet bech32 --decode "$arg" 2>/dev/null) && [ -n "$decoded" ]; then
            # Successfully decoded and not empty
            result="$decoded"
        else
            # Decode failed or empty result, fall back to standard encoding
            result=$(string_to_hex "$arg")
        fi
    elif is_number "$arg"; then
        # It's a number, convert to hex
        result=$(number_to_hex "$arg")
    else
        # Standard string encoding
        result=$(string_to_hex "$arg")
    fi
    
    # Remove '0x' prefix if present and print
    echo "${result#0x}"
done