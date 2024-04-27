// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import {Halo2VerifyingKey} from "../src/VerifyingKey.sol";
import "../src/InclusionVerifier.sol";
import { console2 } from "forge-std/src/console2.sol";
import {Test} from "forge-std/src/Test.sol";

contract InclusionVerifierTest is Test  {
    Halo2VerifyingKey internal key;
    InclusionVerifier internal inclusionVerifier;

    function setUp() public {
        key = new Halo2VerifyingKey();
        inclusionVerifier = new InclusionVerifier();
    }

    function test_verify_inclusion_proof() public {
        // @audit copied from inclusion_proof_solidity_calldata.json
        string memory inclusionProofString = "0x0faaa8aa92bf36e2601bdd9d5be92bb5c7aa6f982661cff0611d0bd8ccccb9bd171586516226f99958cdb181eecdfb1ce9bd48e5ae3c19aea353c163c7acf1db0938c672375247acf7a110224e1f3d0e118b289a02c8b4a0acd33a8d209dbebc01895e90be59da66f1de8e8a144047936e7393e312fbbcfe11960be9de661277150f79ec188c492fc71556342c001c2cdf89aca2df24b2f68514e1742185c2021f0479d9e8e1a3e1c14e2bac6af1c5ebd3d6c5b35c230c0cc92c554cc245a9d5";
        string memory snarkProofString     = "0x1b200581ce1ccff3f92e35dddd962cbda546c1c03834ca56296e567de16461a4075d4eb1bcbb18879d8c84f3804c11e920660ae1c62e0acb20e476c903ae0a3a261b66db77d711fd90671890321022121affeaf8353d696bf4597a6d9fba617c1560b42a43ed84a13785aa23e8d95a604d7aed90b8a4dd41c15284ac81189b812d536e6e7be13957f2eb82d0ced26c44b76e3dee0b896af61f57575935a346871fb31cc9f50e9d5084bb0d2856c98aa13718b511415ecad0e00bbf66c8f6d900";//09b068aa48cacfb89f98f17d513735930d7a9ca7c72f5c47ca044d5f145a16721a3cd8678d4ff1328601f5246e8fdfa240975ca59d6553add481f1647dcef2a71f250fb136e25a6ac0884fc15417d2cc436a3b9fc7c57f1f4bd0353ea1249f411f7c76d365074a7c695d59edd2a7c5b9e1d9d2f1f851a3495ebbbcdf88a7e74d2cc291aad02f0ce6e69aac34cb88d35bb2f35f56f873060f47b840745d64ef950cde7ea63d577bbc926c6ebb9b10f3ba1082e6d0c17977c3dda4a9f4212f8a22000f603776df9e725b18eac4a405bd18a56cfe291e84c9b3ef90d7192d1d8b78027de76238369cc7e8759784685b5c0fb08a0a52058dcec38e4d67b64072636414b5494d7838057eb66259bb5c984e864a8b604d96d35790a7e079cc152679241cf6d45a7166853135d1ca333727e8b2353eb7e90ae8391783731d392a79063e28d451e54b553700b3a5d8f6b4fb2c069440b4e26d748d6a425b770433a47c6b03181db50c65bfcb472b070c9353228023f3766f536bc953b8c0b2ed54a879242874e7d166e7573897e65ffb943ee855df1a131f86c18da6126f551dc7e1ae192a693fae10d79f0d9d119b145871e6a6fc1143cd5b53f90a48f849727b79fab93048e2f14f00c297ade3227b32a5f2d19145d0c5c9f7ace1bef44481525a227e1f041a4fb26e18d77036ace80d4d29959a5654893185d6c6e9c1a66dcb0b874b007eeef5091b519b8e16e5877b0746d4bff497fa3022d30675d981ce513bed8628e0f67d71b6eea2c0047b1ef4b85a3ac0bf0462abc518a82ecc900f3f32ce6a21f30533df9e36688b94d1ff7c51f5530a36962556c9ec526fda390a6dec8f0502819520ddcc9722ef50731fdd0698e91d53764a0929bed7d7aab2246d287b612baf952b31fae1fd4270034332ed893d7d0ab181f7d9a68205570b60f99cfd19147ba2d8a835fae9f72a43c9db4c9a5dd3e0060fc69af8a1ec14da5c170eb12316c4f940985d6d3133f36b14e6da6ea88bacf4f26641cddf3a0a540bfe4198e2275b31cb08b7e45baadde23fb64b9bbc95e713fc1b55396a150e75b0038837c40d69af90e6144f5e7c0f2b12349f8a0acf52e21a8219aafe765d9d2f7724425a28e8c9e850b2b52a59a87ede18618b6aa8a10eac03a536e1b996767e0583839b11608307bfc8a7048e7069fe123e06ee9f72c2dcdb3c94d135f8a8f95bca6efb29a5ea36536bf72bb3039e04baddac9f49b8b46cabf9902f828b5a70416a4017005f6933903f821274efad4634fcc2ed4da97e43e6ca1eb07197920066f7d7bb115909765a3e1ea151a31f01c63e7930647424be324d3dc45458237194c38e1e1ffb93985978172ee13e425b627f9800c3eeb1a230540a67ad8d699d02227636029c959c26b504774a84ec7d2e64d7ad3f892f94c4fae8abb26935f2f2006b832af8d3a66febdcc3e5659fd28ff59ada6c7ae44f565e10e6f0a074d86402c6b4176c4853110edae31ee186904261ca3fb3794ac42f8a229422b0099486dc6e5f04314e7ec83fd1e051b33299b14c75ac2b2f5996b4ab0467c40d7978ea2c718d005dfc20f5a84b7b669ae47a3f8dc359417c62745a598ea53308377552579dea12f44b1f03372e42eb71af3e464148d4446b1c6768fc1eea18a20a7ac48d41b12cd41e17891dccaf592009218490a38c9a7b3596aa6ab9ecf4bfffce31eb125e2f339daf6e57f4093b6494784831c8846e0fe84536802805e273cbe14d8531dc12ef4ab51059fc94d959899a25ae31bcd868fca678ab710be698014ff31eee271a23055aa10c35cee8c5b9f6dc3e6277dc8742ab9db7049827e1dfa16be2d0f410e037716dd68a33493c79211c18755892db6d6a279405af7198af9e645d618724d3a287384bcd7bda940d5bfa11abde37d1e0adde6ab114aed31a05d9f2ef3924003f4918f100ddf49def61a1fc3362ddc863ef0c47edfa88444eb7e9912d9c01651316d0b3a50ae36c01955898e0533f18da2bca239959cf058a230c5ec8ad215a0df35d515b250ce2dcc58e738698d8f4ed6b4119fa04b5f224b581b2caa008a111bc65c68ad917ee74c11c57bd93414133c4f856d38d0196f26a686052b92bc59d15f547cab99b15904b5333d5d36a34bce08c8f1d0c88624d294a77d2b707e27dff709dcaa80aa6355b832e6fe4ea8492ca325551707b10f9e5bc5aab64092cd1c0fef6f4165a26c20f5c5d014432b3163fce63b2edfa62a260a40172182a928b6b21e05186bc2191cfb9862ebd6f1352ad66f1074c8f7daf06a4e3d777236e6efe6a555655996d7c1126607f9745d2c0d5466a2a5c83b7cee5ab324b0923e67df7d3799436d4435112ddd5c0abaf53d9fc2fc47734e0fd47b77e1f94ca29a14c3cc78915b9e0628c61b7d677faf49be039084d6db04afb75d5eea151cd2d21d59bc86e914bd9b3c98aff084c09fcac9b1290c5cca92f9771ca41ebbed40709fc41153455c8d86753e9752e34c29060148f8704607a6c3bd3dfddf6ceb72435f3ef8ca862b6eaae98b2fe29b51475cca9179ee06f542a044aa1e3e9d1ed1ff1f31fd1450f557ef56cb8dfe6620fe0aa7b9562e9faf479964d17c877834604ad3fe40204e4fe18ff27388a35200e07faaa9487b50d2b0afe02ffcf0ed6522f9d1d256ce6c89af49420fb379fd793c5e87a01c6541412ed0f348681c03c630b90a0e829e5c873ce97b70fe4f33aab122796c833a9e8aa15f81cc0f9d524e11ff88113a2e0325ab73455dd950276848d9f85aba0006a362c2f963cb473ba401af97c4e4d1bf6c9f70a194b41f27a145c8b69fc045953ae04296ec52bcd3b6e260d3a53f50d1f20c07f1200e2637fa545d1390ebd812771b9bebd6bb189e3990257ccaf91e84f211af01b415fb8d7d493a304cfb03824ff680b22f8ae609d621164ef9d70d88f64c9c48ef3265fdc81cd72940aa59a63c06fa63e79c74e5f2d00ce48d62a3536299dde64df718796e7425826cc487fdb1f179c61edeaf040280aa100f302519c2042d64cca4138eec8b33729f5dfc0a03e795d7cf570a8045e1eff836baf5f06c6c61cca266e976ea0eb122b5e6bd29f3c240645c5c2404e2513eb2b331f3802856bc83bbdc40a64ba6065d4a82663ac87dc08a6bc93e39e2f2767f750e2eab6e88032e0199cb35733e6ac0d17534cd413bab641f8a75534cf049faf9192f6b83cb439f5c5572aea5ce5d581371d4116f1c43ad60a1e3f163f2e15f81ab9597985951f0f063f86976b1077bbcefdf1e4d7fa477fdcc1e7d1390e260f969e8e2c7376b72fa5401ef62d30c409a2d7d579a8eb39c763c2fd84d121771983521ae3de3ad954507938271080cbf83231efb45e28c82d0cb47f1e88162e663c0d8a75e673f4b0e9d9aea6d2f82aa8c6204a695c7bf1b0f7a5bf46772faa8fb8a6e36ed22763eebf1427dd5a8c1da1bdd8fd14eca9d9e65601454ba4082a17f58d273c30f931af7885a25e8e4f2baa9f85dcc970f3cc97c946df591f09b78b7da9db60a8c54e6e1f6a030f1be20bef6e22d747c25e0d268ed3d9f93715b2a618cce6b02161dc00858ce2233068f3c83053eddb805596bd4c58466e96209f7299fdffeca7ff573e2fa41d5b8655626421145f467f05451ac19334b4260a0f443cd9704c549d8d4d96b2a1219ea4e66daec875bfd74f4bb0712e6ac5b22f6ff8564c9bb0907283b79e1dc87b48641d166c40c4e5448d4e8a84c20f120e1da1487aa706d72e4e205c36815c47675754b52b41e51fb237ae36274d12de2121ed49ec2a3cbc8ac40166687bf18acdf75e062b826b25050ebc62f402cbb21820b774f663dea6af4868efa2181eed9dc9321365a104dbe16d01ae67bcb80fe10428abe1a98b038c25a0e9fc54fec9c120a1eff63de904e6306bbfa99b6ee9ba18881a5354469eeb4feef89fa682dcdee62a33757a61d8efdde248a005442cc6247f38fe393a44f0812814b7be1f7d5ee0975060aa82a9e78602419eb5a8f6610723b52608263605d5dff29dbc80d514a99f3d0a9d4dacb07817995102047f1d29347b1317934162c2fd55c39831119eb40fa67e6f7e326fa1fa61122ceeca05144db4355935cb1d1762842b1f1e40423813cfd82113914c83966ad8486ddfcd1fe2390b2d01794e522a4c95d6483661bc24af467c5915ab1a5bf315cc7a2abb08627f170ee896b9015162a8a9d741847ed80e2848af9d632682dee79b61de361c0ba9d2513c8aa794495b3123989c10343db49ad6786f1716dde66d811974890806eea7d70f5154dba13a45816639d2c2dc2b17b6c0b3710c7deeded162de6e0c2f4b9f600209591d33208fd26e8d8c525b6f03931fc714c451b8c40d38f69d04694df2f4a14cbd76f97351c41d940c12dbc04ed654d8cf62f44afdf43dca752191c47a3cf7d2f3932c94a294538bbd2262dd65e25720de90e8f0db604192d1100da03880dbb08234529a77695c7fa3ca1b7cbbd1c347dbb0558dae5ef439ba1917cfa948c1c509f9a7e2ef3ba9d78503975d769c1934b78c741c74fc36c7a106c6942af654bbae1ddfeee473492411ea6d23f7094d280c83869a00ecda054c1992416c6808c8941b3407c22fceb6800eb8b1da9e78b4af5ed9b80af86e2351049341e779893782438dd60920e6ea219aa20cb9189855f6e410548ff582511c1a7e25ef38919457e8e23d8e90f594c68816968ffd8d4fb39aa99467fb149ed529bbe7879d39b284bf9f009b71720f426a566c81848f7bbcbcbf37061e06357c2ee53ecdab1453c7cce77e2c06972e04825d2e4336b95a5609cb08fb693638910269a4f72095076b7d46dc24743fe951485d751282d0418cabeb16ac50444d861a27552c4d6b349fe034be3fc2e35087013ef76f871845ede39ba92386b08a6a0bf6b447d95e05ad65ba15a0b1a636b1a41d6b94fddede61528842ada317b4711bcc372e891ed3cc68634ee0ddad9bbd5c2fcf0ff74cbea665414f482faaeeae09383bfb526a0c9c18a80e93f2a9440a0c398ca9548d39f1ed352d464f0809ca0dba2e2521de785c977c7979c43269fd71ec56b9afae58cf35cc465fc88e168b15e2460234bf5b91183032f4dd8c3aa15d582fa1d17bb55e446292e68565a10a024a7bb44dc8ab203969ff6f5d2b1e2a7e208de5106350fa21ec5246ac5d20e40df1f8439af5f86a6002199b9082f878e7286c1071c6588341de511fac571f5513f7c058dfc614b4f570de2a62614351ea3e664e8971a7e5d63004c9026dd400052a97f9b59d3de75552b7185b5912be32f9a49cbb5cb8f3b716c4137911bdb006bc52868e283baca4692ba389032bfe9d0a700e2f47c14a104a4ea731cabc9e23a3dff6c06ceac32421283e96970dae4f22dc3848d3834f675bbda5ba3a436618b97e5277db6e1cf10d7eb5a49f0540dcf50fc9e469957df24d69a9e790eed20f18d2e8f8d535a388019d6ea8197fe373f663819fb47255677e5585892fc4d20859f6834baf37ac4d9989960790fdf08e0989859b76196fffcff9ab80e3df3905fabb664473c9476f71bd5ef2621ed47f76d076a3ff0fe27777073243b68eeb2a14ab9c57dd4fcf758b03a35f49da50238a7c0a464dd73823abe97ea44a97bc2bbec4ca8ae8bed8200bb2da0e2fca94839b3a7cd06c3f8a7fe341e2b1194ddf23b92500bc858bcf7365a1b4ea53bec3beef3ee71f41c148cd4c382b5e48cd9c16fac06e418ec1af458c8f5b658cc117893d685826985759c1500b28137461c00fc713188ce3264701d7376f588240430bc2a3a441cd811d77218385336781e01958b3b3e68c741d9a15e5ec4d4f591d0341f6270b7ebc262e0e4507c33a73442f8279c5d8f1a3fa68b892e74931532df4d030c30e728d32e9d3e364f578c89710d858432377e59e820ccd54fcef1fe1558a09d96c6758d425b20d61020dbcdf12fb55d12da9804179e37b5eb23de9077b73f10a4de909183782a36b65ec92442930f98bd428540c05622b2a545465c000bbd9b4e70f6f9faaf5af85730e7dd203ab945c07893c6096a8b2f457d092153b54369f007997883cc4430775a70d1718241c395063816dce07ea10d4baba9884b568382b7c0efd47d95d21f8d815441a978863e211e1611841e43049ff989aa3595e64052b448f633be2b2f5c1614227d9a49cf7c6f1b53c0aad1ac041d5c734fc3f2ca2830bd6886250ccb3031747251504c8850d9b3845a76b7b77e0cedd57694098b25b30aa62d26be5f435c7ae2404c804e3805870acea4b76fc9e08a2805549b478b9e9cc3379225fb40e755d10acb7167d639d02fc954d7dfb940d8f23320026967a5eaee125318b5d53046b27d81d57642b283e33b631c8d3f8d36ba43daa3ed16a793d3f5d78afefa136d02d76c7efef3836f37ef3318b0ef2b03a6e76151ba5bf4ceef026d6fae19b369a0cb3cc3debd9e28c13999fb73d6e6913064ce706f066b3735eb9b33c7ff59752164f50f5a2274c3f2abf4e499ec60a0c365c8d00d28909b9b032deeae5fb4b0904cdd16690c9631a25828559ab5d2ff25de31fc6cd68c5f474dcf1f99a8c6c770b20baf5752cc4937046638161fafe364b8d3e2ff83024540a0c1794346e27221b4d0fb61d725325f48465ddc9250922e8784b6740fe9e3e594ff29d84a6e18b0f5c9f0895d79912e990bfbf032e29717962423ef29e5d301e7f6df98b302ef811fe513641dc7844134461fe40bd7122588ee3b5ed9bfe78ea04e4ea585eef2d248d9a64528b632f0a5e570c242dc3188c8b3cc536c7e6248fbf74e28be9baf810f6fec952208765498c1fad4de6bb42a2a60d4eb5f84c76357fb527777f320805a00fe719cad11599aa48c6ea066a8818cfe25ca6041ce3de19e319b48ef7bc30395f7ca65c24d6c6a898742cd1ed8858bf4a3f1e8d0ff948351b860768f8b52440e6f08ce61f0c72d813d09eafb2e702ee1b5a70acbd1c5dd6d06ea2d125af13e9c56d652434e16633d851b80969521d4e32e209a0caedc6b8422cf4f0c57f1a809e252787f9f6d88f9f60273268286b3d5e4ddfb534fae863dd381ef919152b9a220f12b3890b1e1a312919e9776a292c3b4bac2ff35b180d5d83232321b60d9dc75397d11aaa6d151a96028edd8060d23c3b3bd15d179e113a10b14eff8a1f27bfae91aa88a9fa6d7243244e1c1c77bbe9b28bdfd0ebcf0f0eb79b99771b0c02446010ee5bdff5d215283109f54fab077adf59a5961d48105b5c95402f3129d7b10968df0858e798858e96acacc3ba49e02dd8afaafbc45ebd0a89363365";
        bytes memory inclusionProof = vm.parseBytes(inclusionProofString);

        // snarkProofString = snarkProofString[0:inclusionProof.length];
        bytes memory snarkProof = vm.parseBytes(snarkProofString);
        bytes memory proof = abi.encodePacked(inclusionProof, snarkProof);

        uint256[] memory challenges = new uint256[](4);
        challenges[0] = 0xf79a0045992596e3278606b5317aaf4f6bb65071219b1c89d542509fe6dddd3;
        challenges[1] = 0x2299faaf0e21893e99005dc9165fba869b5aa88bcac5af4395071fd569686fde;
        challenges[2] = 0x2480ccf8a3834d03badc6b0c35773d758908019658fe9ea3bf85dc202257cfda;
        challenges[3] = 0x2bd763bd08804bb437e62575eff584497632e3d3034a20f2a868b1136e4adb6e;

        uint256[] memory values = new uint256[](3);
        values[0] = 0x4d426c6662424749;
        values[1] = 0x108ef;
        values[2] = 0x48db;
        inclusionVerifier.verifyProof(address(key), proof, challenges, values);
    }
}
