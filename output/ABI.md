<sub>*This file has been auto-generated using the [abi-markdowner](https://github.com/0xk0stas/abi-markdowner).*</sub>

# Smart Contract: Template

<details>
<summary>Documentation</summary>

A Smart Contract that allows users to stake tokens and earn rewards over time.

- Users can stake a specific token and receive share tokens in return that represent their stake.

- Users can claim rewards based on the time they have staked their tokens and the fixed reward rate.

- Users can unstake their tokens by burning their share tokens.

- During staking or unstaking, any pending rewards are automatically claimed and sent to the user.

- Also, if multiple share tokens are sent, they are merged into a single share token to reduce NFT clutter.

- Farm Owner is responsible to fund the rewards reserve and set the farm parameters.
</details>

<details>
<summary>Build info</summary>

- **Rustc Version**: 1.86.0
- **Commit Hash**: 05f9846f893b09a1be1fc8560e33fc3c815cfecb
- **Commit Date**: 2025-03-31
- **Channel**: Stable

- **Framework**: multiversx-sc
- **Version**: 0.62.0
</details>

<details>
<summary>Links</summary>

- **Mainnet Deployments**:
  - **[SC](https://explorer.elrond.com/address/)**: 
- **Devnet Deployments**:
  - **[SC](https://devnet-explorer.elrond.com/address/)**: 
</details>

## Table of Contents

- [Types](#types)
- [Endpoints](#endpoints)
- [Views](#views)
- [Events](#events)

## Types

<details>
<summary>Farm</summary>

#### Struct Fields:
| Name | Type |
| - | - |
| staked_token | EgldOrEsdtTokenIdentifier |
| share_token | TokenIdentifier |
| reward_token | EgldOrEsdtTokenIdentifier |
| rewards_reserve | BigUint |
| start_ts_ms | u64 |
| end_ts_ms | u64 |
| reward_per_sec | BigUint |
| reward_per_share | BigUint |

</details>

## Endpoints

### Deploy

<details>
<summary>init</summary>


</details>

### Upgrade

<details>
<summary>upgrade</summary>


</details>

### Owner Only

<details>
<summary>addAdmins</summary>

#### Inputs:
| Name | Type | MultiValue |
| - | - | - |
| addresses | Address | ✔ |


</details>

<details>
<summary>removeAdmins</summary>

#### Inputs:
| Name | Type | MultiValue |
| - | - | - |
| addresses | Address | ✔ |


</details>

<details>
<summary>createFarm</summary>

#### Note: This endpoint is payable by any token.

#### Inputs:
| Name | Type |
| - | - |
| staked_token | EgldOrEsdtTokenIdentifier |
| reward_token | EgldOrEsdtTokenIdentifier |
| start_ts_ms | u64 |
| end_ts_ms | u64 |
| reward_per_sec | BigUint |
| reward_per_share | BigUint |
| share_token_display_name | bytes |
| share_token_ticker | bytes |


</details>

### Other

<details>
<summary>pause</summary>


</details>

<details>
<summary>unpause</summary>


</details>

<details>
<summary>modifyStartTs</summary>

#### Inputs:
| Name | Type |
| - | - |
| new_start_ts_ms | u64 |


</details>

<details>
<summary>modifyEndTs</summary>

#### Inputs:
| Name | Type |
| - | - |
| new_end_ts_ms | u64 |


</details>

<details>
<summary>modifyRewards</summary>

#### Inputs:
| Name | Type |
| - | - |
| new_reward_per_sec | BigUint |
| new_reward_per_share | BigUint |


</details>

<details>
<summary>depositRewards</summary>

#### Note: This endpoint is payable by any token.


</details>

<details>
<summary>withdrawRewards</summary>

#### Inputs:
| Name | Type |
| - | - |
| amount | BigUint |


</details>

<details>
<summary>stake</summary>

#### Note: This endpoint is payable by any token.


</details>

<details>
<summary>unstake</summary>

#### Note: This endpoint is payable by any token.

#### Inputs:
| Name | Type | Optional |
| - | - | - |
| opt_unstake_amount | BigUint | ✔ |


</details>

<details>
<summary>claimRewards</summary>

#### Note: This endpoint is payable by any token.


</details>

## Views

<details>
<summary>isAdmin</summary>

#### Inputs:
| Name | Type |
| - | - |
| address | Address |

#### Outputs:
| Type |
| - |
| bool |


</details>

<details>
<summary>getAdmins</summary>

#### Outputs:
| Type | MultiValue |
| - | - |
| Address | ✔ |


</details>

<details>
<summary>isPaused</summary>

#### Outputs:
| Type |
| - |
| bool |


</details>

<details>
<summary>getFarmInfo</summary>

#### Outputs:
| Type |
| - |
| Farm |


</details>

<details>
<summary>getClaimableRewards</summary>

#### Inputs:
| Name | Type | MultiValue |
| - | - | - |
| address | Address |  |
| share_tokens | multi&lt;u64,BigUint | ✔ |

#### Outputs:
| Type |
| - |
| BigUint |


</details>

## Events

<details>
<summary>adminsAdded</summary>

#### Inputs:
| Name | Type | MultiValue |
| - | - | - |
| admins | Address | ✔ |

</details>

<details>
<summary>adminsRemoved</summary>

#### Inputs:
| Name | Type | MultiValue |
| - | - | - |
| admins | Address | ✔ |

</details>

<details>
<summary>paused</summary>

</details>

<details>
<summary>unpaused</summary>

</details>

<details>
<summary>farmCreated</summary>

#### Inputs:
| Name | Type |
| - | - |
| staked_token | EgldOrEsdtTokenIdentifier |
| reward_token | EgldOrEsdtTokenIdentifier |
| start_ts_ms | u64 |
| end_ts_ms | u64 |
| reward_per_sec | BigUint |
| reward_per_share | BigUint |
| share_token | TokenIdentifier |

</details>

<details>
<summary>startTsModified</summary>

#### Inputs:
| Name | Type |
| - | - |
| new_start_ts_ms | u64 |

</details>

<details>
<summary>endTsModified</summary>

#### Inputs:
| Name | Type |
| - | - |
| new_end_ts_ms | u64 |

</details>

<details>
<summary>rewardsModified</summary>

#### Inputs:
| Name | Type |
| - | - |
| new_reward_per_sec | BigUint |
| new_reward_per_share | BigUint |

</details>

<details>
<summary>rewardsDeposited</summary>

#### Inputs:
| Name | Type |
| - | - |
| amount | BigUint |

</details>

<details>
<summary>rewardsWithdrawn</summary>

#### Inputs:
| Name | Type |
| - | - |
| amount | BigUint |

</details>

<details>
<summary>staked</summary>

#### Inputs:
| Name | Type |
| - | - |
| address | Address |
| staked_amount | BigUint |
| share_token_supply | BigUint |
| reward_amount | BigUint |

</details>

<details>
<summary>unstaked</summary>

#### Inputs:
| Name | Type |
| - | - |
| address | Address |
| unstaked_amount | BigUint |
| share_token_supply | BigUint |
| reward_amount | BigUint |

</details>

<details>
<summary>rewardsClaimed</summary>

#### Inputs:
| Name | Type |
| - | - |
| address | Address |
| amount | BigUint |

</details>

