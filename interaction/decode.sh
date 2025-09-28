#!/bin/bash

# Function to convert hex to string
hex_to_string() {
    local hex="$1"
    # Remove 0x prefix if present
    hex="${hex#0x}"
    # Try to convert hex to string
    if decoded=$(echo "$hex" | xxd -r -p 2>/dev/null); then
        # Check if the decoded string contains only digits
        if [[ "$decoded" =~ ^[0-9]+$ ]]; then
            echo "\"$decoded\""
        else
            echo "$decoded"
        fi
        return 0
    else
        return 1
    fi
}

# Function to convert hex to integer
hex_to_int() {
    local hex="$1"
    # Add 0x prefix if not present
    if [[ "$hex" != 0x* ]]; then
        hex="0x$hex"
    fi
    # Convert to decimal
    echo $((hex))
}

# Process each command line argument
for arg in "$@"; do
    # Check if it's a 64-character hex string (potential address)
    if [ ${#arg} -eq 64 ]; then
        # Try to encode as MultiversX address using mxpy
        if encoded=$(mxpy wallet bech32 --encode "$arg" 2>/dev/null) && [ -n "$encoded" ]; then
            # Successfully encoded and not empty
            result="$encoded"
        else
            # Encode failed or empty result, fall back to standard decoding
            if hex_to_string_result=$(hex_to_string "$arg"); then
                result="$hex_to_string_result"
            else
                result=$(hex_to_int "$arg")
            fi
        fi
    else
        # Not a 64-character string, try standard hex decoding
        if hex_to_string_result=$(hex_to_string "$arg"); then
            result="$hex_to_string_result"
        else
            result=$(hex_to_int "$arg")
        fi
    fi
    
    echo "$result"
done