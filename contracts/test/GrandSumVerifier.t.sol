// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import {Halo2VerifyingKey} from "../src/VerifyingKey.sol";
import "../src/GrandSumVerifier.sol";
import { console2 } from "forge-std/src/console2.sol";
import {Test} from "forge-std/src/Test.sol";

contract GrandSumVerifierTest is Test  {
    Halo2VerifyingKey internal key;
    GrandSumVerifier internal grandSumVerifier;

    function setUp() public {
        key = new Halo2VerifyingKey();
        grandSumVerifier = new GrandSumVerifier();
    }

    function test_codesize() public {
        uint256 size;
        bytes memory x;
        assembly {
            extcodecopy(key.slot, 0, 0, 0x20)
            size := extcodesize(key.slot)
        // x := mload(x, 0)
        }
        console2.log("size: %s", size); // 1376
        console2.logBytes(x); // 0x00
    }

    function test_verify_grandsum_proof() public {
        // @audit copied from commitment_solidity_calldata.json
        string memory proofString = "0x17e2032176f6575e95aa4d9d97293edf675fd8aad89e76d99883b4a830564e7d2ca14616b46c35c4573a4e5806a7fde693b0da39ca285023e93c2e3ee781b78b18c815403ccb3ac8188e4a1b761df4504068402c880e4a687311455818ed4ca32367d0768c54895acb9875b2b2f60d85102d455cc28f0d9d2af67ecaa4ac662f261b66db77d711fd90671890321022121affeaf8353d696bf4597a6d9fba617c1560b42a43ed84a13785aa23e8d95a604d7aed90b8a4dd41c15284ac81189b812d536e6e7be13957f2eb82d0ced26c44b76e3dee0b896af61f57575935a346871fb31cc9f50e9d5084bb0d2856c98aa13718b511415ecad0e00bbf66c8f6d900";
        bytes memory proof = vm.parseBytes(proofString);
        uint256[] memory values = new uint256[](2);
        values[0] = 0x87f3e;
        values[1] = 0x87f3e;
        grandSumVerifier.verifyProof(address(key), proof, values);
    }
}
