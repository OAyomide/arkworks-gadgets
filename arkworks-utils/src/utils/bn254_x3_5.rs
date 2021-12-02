use super::parse_matrix;
use crate::{
	poseidon::{sbox::PoseidonSbox, PoseidonParameters},
	utils::parse_vec,
};
use ark_ff::{vec::Vec, PrimeField};
pub const FULL_ROUNDS: u8 = 8;
pub const PARTIAL_ROUNDS: u8 = 60;
pub const WIDTH: u8 = 5;
pub const SBOX: PoseidonSbox = PoseidonSbox(3);

pub const ROUND_CONSTS: [&str; 465] = [
	"0x12f865f28b419806bd53e1d8f6389be0169e5dc5fda2a25ddf2a0e557f5055e1",
	"0x19110d104ac4f20ebcaee6bfb88886deb87ec2b6ce04846cb3366e2eacaab8f6",
	"0x09ceadb610b5d825d411a7e738a2ac70e5c3bc8a1276b4130adbeff063decdc7",
	"0x25ffc9ef6f95d3eda945246fd634c21a17fcd74ff618d5c1a57c8a460da8104f",
	"0x1fb0ede93ceb10ff9f9c58f3a7fb85569128f2772b27e48f8c850d43ceec056e",
	"0x04c8d7fc9d01ff7984fc9e46b49b5397897381d9929b7f56cce0c0655a1d750e",
	"0x114406370f76532244b017efa4db48a3a150d0111ba9c9a17f98b4e63af6ccad",
	"0x08305bf55456508de462e19acf85eea35a250409757aff8ff455e8564b1452a0",
	"0x22e75954fc0158dcf1519853c9f90eb27c4663fcaed6d0c5244cde9ce4c955ff",
	"0x1808ed91bcf33df01de6ef43d406f822d00d6ac4992caf6d72451f3f3cbc29ab",
	"0x09a094a87b0d2c9b1b9c1d9c95da10a92c7177aa4b6420e8dd735b52474e9f79",
	"0x0eda03612d167863df4d625c962935c0b20627cceb2a64073f39012e1bf43463",
	"0x2d7ef725eda3f1751cad1d6823a0d766db64faee10b744e56e4a937111306376",
	"0x295aee69261cfd8561697ac8f808215b79bf92794a8a14f874eefd89a57f51c7",
	"0x279c4b5aa60d28007cd34cc5b7a9351f505fd2ade234027816d2500992150fc9",
	"0x227907afc435ad802fd341781d39f2abd280ef2f926f85e75d6afa7b21fb03d7",
	"0x2bca43e20793972580f061131819d0b453257e0f74223bfbd097525a807486dc",
	"0x1db54d8214cb1d0b091c5d606c1e5cef6fc5dee53c01063ac1cec3b78f0a43a3",
	"0x06294691ebfad3c6ce52e875476bf59b7518d021bb92b6b260f0c91af893301d",
	"0x1433a869cceb52958b998f01c71858d3a6b4ac8c9da91152e8fadba48b725f11",
	"0x226afff7a1dbdc3cf0516cfc4d49cd82bf7f2db853bad1761d646f6dfbaf73d3",
	"0x0105070fcdc1c44db10e9259e40a61c33d7e437182f9015d6d584698344b47d7",
	"0x1545ef78ae975e1e36d965804571b86ce606e490de15b26b60e60742e839506b",
	"0x12bcd347f0abf531375ae52fd7235a22417cff79d51e08707caa76079240815e",
	"0x2371d1f54d864a3d33f7df60e4330ce341aa68f063a7d55f7fda628c56378696",
	"0x02ed81b5d4bf7d9baa42f9a0c659946897c6f703429ba2413efeadd382494795",
	"0x065418b49d27557a265b32ddee71ea6966011bb8875d42a62ddc85ecd086bfb7",
	"0x0240ebc88920315934672b4c07289286089d0c941455fd9ec3fdba737d098de6",
	"0x171110ab8290152bce6367c70b0cdc74a4d3c1914f4abc42b808d24f1a2bfed9",
	"0x15f09ef2675a909eed532230ff3bcd9f591ebd263e0acef791741d9efcdb7d2f",
	"0x17770621b482828a938c3f22e676a6ca45bbd9c6ae0d3f916030044a56a9683b",
	"0x239be83d6b3f3da3e2d2f9f739515035c52f47f66a65c27db5b3312162e416f1",
	"0x04b4d717dc16474c9c088020106fdfca0eea477f9b6d7fccf775241d51599d04",
	"0x179136aed0294e7f5e40c8b578efef51bfc024dc49ddeaf69bde7a27610288d3",
	"0x071b14565a2fd846dfd49ff9a1a4ebe6d5ddf22131d01447089a9c12f5cd70a3",
	"0x09d6ff710c2a604a53f85046d801c1d37ae26f445c9a312c85b84353422a9d6a",
	"0x0b1c62860f7c06f0eb8c4ffb9fbe855407028c9c34c9e2ca69d4f5dfbf16692c",
	"0x2e7e1226bba26567f4507ab63e6554c799f590be97e73a4957df3233be92dd73",
	"0x249a18f1bd8643064eeec318f1d1854e28cc53e8dae9628825c1243a27427610",
	"0x232ad724d35dedc017a89e239e952d357ba3af61ccdc93529316f20ea2470ac4",
	"0x28d7f5af2b5e0152a277141765e6040cd1108615af0e38bfa0fd0cfc20e7a602",
	"0x2c13689d80bd5813b68c39af7aa52d017ad14221e5e8da74268684038fd6da8a",
	"0x007707afc32053361c0708f2b4b6970c0fd5086962870f6ddf7a31fddc32a305",
	"0x168c5834e85398c349d46dfbf8093928c496195e00bf9c8dc56356064c90d6a5",
	"0x050379bad59ce726b73f37b5e3df07131a502df92a8799f1d26d6028575bda77",
	"0x235e659ccb084ae3f3f3b91be1e3279553f6dc2f88d76948fdb36bbe2b881c98",
	"0x156548bde7847ff73394ec63a5d010fa7e233335c6be416609a50963f540a088",
	"0x260c9563596498b8c2777620b3998eea6196a9e545b100027adbbef0b348f404",
	"0x1f735474f2ba0e5d27633ecfae1a1d9117b9550f61dcac30a195468292966bdc",
	"0x11a9aad61b2d641079eabd585220be1440aba9779497ae830fbc38f5079c6f0a",
	"0x0e655a90d6ddfb1e09405b821c4540304e529b4fc8dbd4d0b0805f9ee8241e86",
	"0x2455594d950673c4e47fdd5d4a46a81dc2d856722bef3efb82e79be0271910fa",
	"0x1d2c8674cef97dbd3c4a2a8a5337f2b9e07ce164a2ac6bbbc111761b0af63861",
	"0x2eded1d0511d0bfefc8c86d9e7fb3c83b5e5f908d7388184761fbe0d786e1c36",
	"0x20ce80cfdb288a4589face17c7a66fe6c7c12849c7aefdbef00fab46bb3b9db0",
	"0x1bca105d35a2e61381f9c548ad6e1be35b37c1767f869dd59c4345fb272d3854",
	"0x0270ba282632adc02a162b848294065c1d094fce1e1e47e4c3f4694a2f207d53",
	"0x0e11fc72f401fd1d97c8c67f49a54c5f9474064cc463ff37fb52e89c9c467cb5",
	"0x2bb61610deb65468ee600f5f09d728b5c61c1de64f86c95edb6ff36b4206e93f",
	"0x2f8edc9b3edcefcb294c3077ec3829d15a9aadd477788a3c8433183db29ca849",
	"0x080de824badbcd4d73044112df9f0e802f9e285a5f0f58bd81008969fe856721",
	"0x0cb99cd5203827cd219a91dd1cf4b19d7777acecb840085876ce39d1f1cb958d",
	"0x13e3ff0ef71cb846cec1ed9ca90928b33cf945d3375928f719c4f4bedcb7573a",
	"0x05ba2d15c9988e11e91697ef1c339217270dc8baf0f625db50e024d58bd26016",
	"0x2445df79e8178f96bb2178baee7ba5a05a444afb119cb7ce6ca91c687ac03b70",
	"0x27556504be526b31e554afafdfbb58df7ebbabb6fb81547eb648d90b866daf68",
	"0x06c41a9d3f36d205d13a377242fc775ede0c525909f26e82e38894b423fb33f0",
	"0x1d4270541fd09ee5fd6674c10352558e9853c5579244346d2cfdb11b2c31707e",
	"0x1be10fd1e925ecb64425d95db0cf35dc99632f97da716053665554036b24e5fc",
	"0x1c7b9090c39ad0b51b9c5c7cf221ba4b8b12e8ca957c162d597815e7ff32b841",
	"0x0331af2546100c794b7241d49c9f94446fa6a8f35a0d58a009d6e09112dafb95",
	"0x1156c59a14ccfcc72df1f9eb13c8b5fd7377983e17cb23ee8e5b20b5983d13a4",
	"0x0330124f8942d7839c58cdb7e984848cb1f426c46095e2fceab582cb202406c3",
	"0x164a6f4f08f7e1500d012e1c530e4b2e1bab04f28f7bf3de9d2813e4ae83e424",
	"0x141e47c9ccbe9c06c80a738803be4733d05379fcb35ad3ef4b612102d37f3385",
	"0x1f8392fc20f1ec026e075101814cb1192e8bdcdc1a1a62126e537b65f48cdda7",
	"0x1874e3d249c8bf23b7359bca3ef76f77601b1731b5abfe3f338612225f0eae69",
	"0x106be7c6d1eddfb727ffce78d3d1ad6bb1795107191f3344840ba1f2952c94b8",
	"0x299289000dfb8fe950c8ba3062b432d9b47c379886a5ba21d5cc0d8e37342e35",
	"0x160950bd6d1ad4597b07c7aa6445be9569558222c304395fd9e84fbd01f54f77",
	"0x29ed8422160518b4070eb3c320b53f16aa5ecf5a3dc397c1585fb9427f5f0f0e",
	"0x06d836461b54b99e195f95167cd72a9b0269cee71937966341fbfe61a2beb6f1",
	"0x16a5b6ca9dafdaff5cd0902c530f64910c44530aca3926f1f8344f283da94db6",
	"0x074713cd47b2193686f62b4c5b4e9bf0b2080528e321c01a3aa0b03247a55db2",
	"0x03c981654ae70dfff74927d2722516e480e27bcdf331e3a08e6b731e39c4fe1f",
	"0x2e4c255039b77cd7e9b009f94c80ba69affbe1ccdab7fdd41da95e9ffdb28e78",
	"0x24ad239d4847ed1433ffb4badabecfc7be26b405180a518ffa6f95ca2352eade",
	"0x02e0d2a60010318edd9739bb50d9316c9aa9846e96c9bcaafed17961ace21ebb",
	"0x267028b5f34b949f1da9dc2faa1b39e48434268df4a438b5869ff2cfd75cac50",
	"0x1e362cdbb5ea70b2c374cc0a070c14d06977be7df3535e127fd1bc0969ad3b0a",
	"0x25abdf118fc520f00ed560e5c30aface1a0d407324b51b5c64a9ff92b28d2159",
	"0x04b34cda04788ce04075f9c4af7f0c1c74598f798e429de8887ca1dbe63632a2",
	"0x1e6d9c0e91e63a754536de4323367c08853a8a0ff924f1f3599d61d930739aa5",
	"0x2dd4e4f1f86df4c91455202bc31e8b9d08856f400321072ce0151b7d916aa189",
	"0x061934bce6f909d40dfa46ecbec2aac90a802b4470558001d4fe2787129fd108",
	"0x2bb859624e28180c08199664c07de3cbe8c45cfecacd750aa967201d2fd2fd6b",
	"0x0899fd814b55f538537151799a6b7ca7b1a6c0c72a6aa383975160696384c512",
	"0x2b35e462a1c0bb028345da2069b27fbe1e212be51f917083c8f58cd46c9a5196",
	"0x0faf326350e6ca25fa8153a640469c3af1133997209fe55fdb367b0af708cc0e",
	"0x1dca6bca56b02624d047be5bff90a09ea2ac57e53f144a7214123ce3b8193f29",
	"0x1104c027da7e49c274d1cf5f134adba7a37cc2b66460f70f0f7618e6f722cb52",
	"0x2e85f5c67d90addbe560948a74b55df6be997534f76ecc1a949f92f415a28335",
	"0x10b8443c23b30ab22d70ddc6f90cb5db8892d91c265931d00071caa90b1d3d75",
	"0x2f73b3b3ad2e213881ee0da9a38a751bcb4aa85d32a2576abd8592b2ce8c7a4f",
	"0x0c741815d21f7b26fb51618080792ce2758c66b9ddc0bdaaa7c91935e1d7232c",
	"0x25d9f98c4ef9e1bed52ab67ce8bae7fc5b769cb5ba8ce620eb68d1325f8083ff",
	"0x04cc3b4283df81a16d794817fc8ba5dc29faf9ddce230d16ee37016cb7b5cc70",
	"0x15c64a064098b841bf9207381214a362d555935dd80a323da2fd6114846ab504",
	"0x2c0ef8bc3545f40f412192e3e9cfd8722a7b49d712335f863d0cff9c48c39b9d",
	"0x251aff7aa0d03af7cf43be10d089fe11b65beca3094b8a57cea04905881b3d19",
	"0x0658412806ef3b53a3c66df2650974326ba5168c85c63f2835a12e3d81825c21",
	"0x1480ff7a16e0e90fb00a40eb5cbc4ec426cebf0cd6f5ecd57eb7d7d4991630af",
	"0x0fecb3df017a1f323dc0666110a2b77434ecb430ba06d6620514e36a685800a0",
	"0x2f29bc013889e5bb49106e26b1cabbd9bf17321a65c5723eea4fac69dea5d152",
	"0x0589dcd473793ad003f39943848d58fc5ccc420875d6f4f46685073a23f4dca8",
	"0x0ed6aa8de103871fb5a4187a19c3d61bb23d174638a3a26bb462928eabf044a0",
	"0x2c548dfc14cab8acd8f0076f7c78abd248bc73811024e6e1eca498dd113b2b18",
	"0x29f39532f56b213f4e2eabd5de2122c88b50e742871c70b756fcbbee02da6968",
	"0x24ba27b613bcce6ed01497f2d8cb3c09dd2dd2dabd3d8db23915d5f41e68198f",
	"0x03471fc410ab1221b6413c830054daf93f6199504f2440e9c25d8bf74340658a",
	"0x093ec7e97b462ddad37aa6bee81a49a0223bfa95ec4e9d87294a0cf61f336dff",
	"0x016a33b1df538e8dccb0535b7de8eb5584586d72652d6acd37420f2314997def",
	"0x2f6094a1165d9be696412cbbc37bd783e7fc8afd0c7bba588395a9d7ba45ed18",
	"0x2ca4d612030aede2eb4c8bf5eb8eaa32598b2ad842209eed8094b67ab95e0384",
	"0x0b7861aef942649a9f779cd3bc9caa71254d24fb1588e95ad2cc8b4318da3ea1",
	"0x2755b6a630a664eb5ad30cc226e73373b8fa5a031614e12ae518fc12f6afe137",
	"0x0df436ac99ee11bd2dce021f8ee2c9599f85f2a0c67c6810ae445a0b0f917368",
	"0x29c0b59b330ba3142a1355fa261ee7a75e019b488f347ae494cac927d8520efd",
	"0x18afad6f77fadd2139ff8e1cd96647a0910bf7bcb36fc407c8993ed4959931a1",
	"0x2461c899523b438c41b7ff22b7d50f49fc9d81b13219a4621d2b3e1d6b44404d",
	"0x2f4e26789884287e427004b1d39a7d09dd8dfc465e3d95724a5eb8a98c2dee0d",
	"0x0dd52ecd02f4fbf14b11bde6dabd5a48d370504fddf04faa65d41f4ecadd11b5",
	"0x246ddd3ae78ce40fe883b7421bea0d8e17372c8f1074b0e77fa209e9fb4d8346",
	"0x160cf11af535f9d2645821ac298a109ffdf7530d235a6393d6e8773f57dc91b7",
	"0x229ec37ab07a36a57df60e63573b8ba3ffc4405d0f5f15bf4da824ae47b473bb",
	"0x2353e3350dde6cff614305dd37150007733d7d7b385d8563e43a83a80f744040",
	"0x0b02245d2b87417e31771f2a1ed3d537afd981e2a847ecbb39d19c4d00389a61",
	"0x21784ec3328dd2923d9a1b41450206f97181cecf2b26d8347c0a9f0639449391",
	"0x2a73f82632ca1f147f5c5b02effcbba8369bdf6113c6b9f9d49a50d682556fa4",
	"0x301ac47cdd1748bd8b1f96bac150c95af8f8e95922ef2043549b9339b7902711",
	"0x00399a984db671f431632e25567a276fde3b59d335d653b766894fd1a309143c",
	"0x1771135e6df25024d04c24dec881460e96d0085869bd4a16c0573b4ab5041c81",
	"0x21ebebd4e9b10fb53d69657d03a2dfb48934558cf9d18b22351a3e10d0601830",
	"0x2b248b1c67bbe2061fc3c90f77bac3f700448a169ebd83481054949e15e77ee2",
	"0x07a5d6224f1ebe39d3284f2c90d3792b67161c33e68ea21ba0cdb970b856f062",
	"0x078ee8acd3bb7b008d49fac848ed147490a7459700bf032f0b515639f480a916",
	"0x27345d3c800004536e7519a4c649a1f5543eb8db1f53c02ffa3426976e814a22",
	"0x1b68bc20a356acf6fd26261e7f6b5d73f2ce198e6aa70b098e0cfb0d8021be5a",
	"0x13792adeca9e5345a760371783fda292907a22d11518c38871489394be84c164",
	"0x0042ec51030aa3cfb201830bc2320cfb48a46d5b789f8abed0262d30c39e5eab",
	"0x0f43e0aef5bfd3b09c4a84b5f2a6ea261789e147060c70139bf0d25bbc7e3f1c",
	"0x2d78258bba14512eb83d20be8660715a3db1a553773d028393b70d19a0621e16",
	"0x2f17dc718f036f2e803c4bbc10feb796754d4361ec745f739bd7b8ce154df776",
	"0x20940c460db970704cb2dbe4dd5f17087dfdda0059a55e477f0add9e8446e046",
	"0x1ebbdc5aa02ac7dbfd98cb29c6f8bef74f3cc235514bd211a630b94fb077aa40",
	"0x18336838f7ec680461f60efeda3b571e14a8fea4545ea2fbacc487aebc35a5c8",
	"0x0a49d02ac11ec0eb9cc8734741e75a31177e349b7619dbe513f5fdf72a73b52d",
	"0x107c6e359c6b14169c49ec5e752e668c0fdff89d6830dc5bb1758ae8087ae321",
	"0x2f14f728a647154db5bd8589f648ab4007fd96cb363d0ae41c97631b8f50caf5",
	"0x25ac078bffa94ebe085421f9338e7987b7b8cc187ecee352ec5d2ab2350d6e7e",
	"0x18d18c2ac26b496f7d483cde942e8ca40c8650d683fc04405db6a932b5aec5e8",
	"0x12b77fb4910c8c25618c2360624b00e915b1b50d2c924541b7da62640df11491",
	"0x2fbf5349fed116f3a8586c0dd216e377fb073be992a27be86fa688f9b31c08a4",
	"0x0b42a3b36e9977c3e1c1b06d6b25bc2f2a2e8fb1f2e13f01f46d2a2613fb41e1",
	"0x1ea6911c18c0d1d840bac07e3a27cfba0839f865a0e81bd56700ecd6cb2ac453",
	"0x005cc3c7e47905e111f11b5f992e7392afe02c116e4f84f45017abb1bb11998a",
	"0x181b337ba3100f6f6d7bb93e7f2a06fad0aebdf4c038646c50d0919537d73e9c",
	"0x0c3fa4f65a195fdccb59d7be91d4b149922d14c9b8ad6afba56a03363b976c42",
	"0x2d2ea06391eb2a180726e2121c3ec7b7ac7c203d60127f578594c4bd5d941782",
	"0x1cbccdc2ed98e30dcc7c2c06dae81ee277dfcea4c361e9eb801b2aa1c7b0421c",
	"0x2e7f1b4b780164a245024bd825227d8cda21434e74108320efecc209426671f1",
	"0x1205ab20564be8129b38c5bdacbd7a9057675e21322ccdc849e6553e9d0295b8",
	"0x0f86c33dbc8fa2baa4e02c9ee6b944f55eab706ec19f7ee06ab8146bb3a465a1",
	"0x23fd753498d20da6b209e9f3ab33835b1b1121a99ca547cfbace2d22ca874544",
	"0x275f1b168a65bf8d062cbe07eb7ae1e34a707f43f82283db191690d09c88d738",
	"0x10c2696961f6916084463826c3fbb10c5713dbe5c6e11cf3e469b81be9426d9a",
	"0x189127c8d37cad0e505759ab8736c3f256f38b61f6f9ac346f33e3421505c76a",
	"0x1a417d3b893f7a482d9ddbe501b12beafe17228a49ab24958bbda8988ff2e4a3",
	"0x05724947021b1e8260d50890f84e4782c359768e1172a3c70eec68ca0618fe7a",
	"0x1cba9aa634e6d188bf609bacb4a6cc3038793487f75f66c106ccdbb6c780b79b",
	"0x212483de2d901728ef5463ebb3644af275e62ffee3f0c256a6370e1f57136a20",
	"0x07a9d2bd34158843ef4081aac26c3e5325f2c003f40815dc542c7659fb8b52c8",
	"0x025045cefd1da8207468049f563d52ade5aee281f6cd4656ffebc221ab1e404e",
	"0x2aa478132c24efb16487386e804040f23863b57ff25ef9fe6021bda57318ad71",
	"0x1f7f13ec0bfd0906b91401571d7449229cf292a553cb073fe677944bd07c25fa",
	"0x064cd65e86d43fdd248256b3912ba095bdb67fb8741ec9134d60651e10b926d3",
	"0x05e593e0270d2f2930d4488d81b6f13efabb5f886da44d78ec97e5036b17fffa",
	"0x29a6acaf60431224bc4e832382ebbf1024ee64f2b602b8f221a328c7a6fd21eb",
	"0x0e612df9857c159e6f94d4f0a0c2321c83466e65f7915d5e99ca65dec357078c",
	"0x2d0eb21d37a4b83879d81927ba8384c935768a2caf138158804d92f59b251c11",
	"0x1700e90cfb7ce024edf9ab2770d7e8ce02cfb4968e3de13d872d1dc329504151",
	"0x17e9910485caf7592f0ba5ec9c182ed7464ce4c5146f21a94c989c44c886ff96",
	"0x0b3b99bc8d914a2ec332f5bf5ca8050e0622b0c56dbcec02cf9e40f7c92553cd",
	"0x1194d3f64c97559a1ce52db1be8c902c9942b0e086f3494fa67b8b006598b273",
	"0x26276b3aee22ab947fc8e97e871c9f4b8ca89e092102652d9aa7bfc8a0169014",
	"0x132b02bb2079490d9d06a3e4914ff72655cd3e80c38905754b6b171a23f876e7",
	"0x2bb9337ac48f30abfa95c1cd993222085b288efa08fc03182d6be38a4ab0e1a5",
	"0x1bb8d3515a1f8cdf35ce332f25a9cb0042afde6ad5754e560aa1a228534dfe5b",
	"0x0c5372308e14cbedaa30e815311a9c0491220cde779d28b6cf64949387502784",
	"0x0264323dda5551dd44ce2c3880a2672a48b20bddbdbb650a1b32f1381ff626db",
	"0x05454f489afa720eafb8c71b31a55071317ef8ba0e9d0de8dcdaf3f3e8ed7be5",
	"0x10c599bad05bf72388c05467d4b64296744bdeb951202c7ad022103ab777306b",
	"0x10777ce051d3eda99b196fe0aceb2557d2140fac4e950ae34e2644ebd0c6aa44",
	"0x22cd6bd028c1a6b05ff85dd02d755784b2f637fc6d14105ea9aced57dc8aaf0e",
	"0x253983bfeb7d6546286c88ab9d5ba03090d136ce492aa767949b4baa12f43e29",
	"0x11cc39f9106b4b1ce3b4202e676dee6784fcbce5219f9f1920465d8c56758050",
	"0x1e94905c3b3bd8b889ebb8805d083bd8b3e12ecbb76a43b9960b378a47f0d208",
	"0x12d4e82a14bc973c86539d7eff1bfea36ac2f997bc7d7c786e241de12d12e6c2",
	"0x109a2aa7f1e3d5480bd98f2910b9468583e5ef41de7acc835e77ac20ef5f492d",
	"0x208c6396eeb33f6958960d8743e8f6aed9df00a020ca30b40a7529e219230df4",
	"0x192017af59476d6634c5aac851641ffd5c366627982a1939aadebb710fe9940a",
	"0x10b32cb4c4f1cbfd12b3b3bad280ade0a12ccf554b2b575d7e143c4d8f4176be",
	"0x2ff2ebcac43f2325a2cd77067cfb98deda4a6f2039601150a1561af350cf22ad",
	"0x137ce0d1fc739a7dc03d6c8c2ba63fcb145d578b6c3106c05559349e50f8c78c",
	"0x25dbbd3534b807883949b5830091802dd238b6194763f0c8e1c6fc43204f2f80",
	"0x14eaaa0dd2e2b263b26d8352203afe3658a37f46879b9f33c693e9f60a84ee12",
	"0x1fe9d89cdfd5f45f32244c3c8a0a7b8a5e85cfd2e558c2cdee037764aa031f1b",
	"0x0da1d80f4c3cd8a8ff23697773fcf42bfee7eaa02efb8267213357cd245b337b",
	"0x065d4ee2b2a50f8980c2293f3b2854e32b1cd3f00828bcf6a6f6659e43cc9099",
	"0x14d3aa6704936322ec1cfa06349b0681a1da32de4ac7b501300aa4af2dbd7432",
	"0x2bd44340761c89bde6eae117f5ab2e4d7bee995b1fcc2880316f90bdbd1369e5",
	"0x07e2ed387f2325f9d4b82851959cdb1032f68d1380b03c25ad7a1a55b9510a5d",
	"0x1303fc0135c2534cbf12aea813afb4e3b46a3f476fb46fb32cf0e976917493e1",
	"0x26ec51ede11a9014ff5f2469c729e0e89174f76bc2118d1e21937d9064695abe",
	"0x2b5d97d131db3700a92690bb9aa8de619244bc52053c56e5a7a363dd15e93c23",
	"0x172664b0119a02b83f39a5a59681f31af182275cb27d2a34ee3a80b85ce58b01",
	"0x2fb9abfb63108a30221c2d1f50757026ddbd02b941bc6f57171220c5be0e1986",
	"0x09175e2e067f6e7a375e7e52891339fa05bf50a79a290ec0036dc176ed127f9e",
	"0x08d8d681178b43f051b0d378eb272b4afb72f7e1ddad38d6a3d65ad3962ecfa1",
	"0x24d24b8b0ac70fe4dd9b8f7565e84157ff88006e1340c7ec5e663380f86d7e02",
	"0x18168013324b4961c273c3a246a4f84e7e9854a195faea53e5429fc1d6d9ed38",
	"0x26c304df33531d5bdf2c4371946d09b450021b8667ec6481e3b3deae64506bf0",
	"0x04ee1f3728484b2390d4d013419a78ae61f2e4a5b19a8e7a544d281b5877b954",
	"0x06cce44bdd423f836ff162df3b21887ca17b582107ca0e4841d4991e4d746699",
	"0x2ad8365ed53a0141b59b022acbdb1578b48e835e8246cacd153ccf10531a6fd2",
	"0x1079baa4e36f8f120e9c6b43b7cce57dc3d34a8d68eb24567cf328ff2e169996",
	"0x29f5836d091c0ba5b075ceb190f26bad9da0043b668e5bc6af232c9c49408209",
	"0x0072d25217c6eb5443c1568ca80798d891e54fa8bd99008fa162617d03c687ce",
	"0x0c29fcd8d3bf208774950a357da3e2440d5355452845927877aee9586c663ef0",
	"0x149f4cbd5c2e9ab02d7bca34758066150e7055e2949422fa72f4a61832ec3c5c",
	"0x20d734d44d50ef35779f97eef17c1c4b44e3b8b8fd99ea4c5a22dc9fecaed023",
	"0x098c403c8f5190203acffef6a55e33ff1e4975521357f89d8c350952fea6c200",
	"0x1517ac814d359e379378e6197f9c2327719f4dd507607827978780c88e2daddc",
	"0x1d15a572bfab286c97a463120ab1b677c757c7c4d8d534f9479abac13a2cfc75",
	"0x1ff4e96d0aa62e9f2fc5167e959d188bf4cea781fb2629c1271c4f5bf2e596ee",
	"0x13538cd8f139a3df0099a835b82b7a1adde4da732f9c00b936a0341ba47903f4",
	"0x0588b7f6e54a844808e36d8d7e4fb6ca692de2e0208e0dc6945f8be0ec0a9992",
	"0x1661dbf023d64cf034fe6d0eb721408dbbbdec3a17e3dfb5b18a39b64c707180",
	"0x137d848cf760842e4a7e54302d44045621abf6b583cd885e08e8cc93efaa3ef3",
	"0x1c968d060d6b80b236383bd6597ffbdb099dd4f4720ad911166f503deef25571",
	"0x0e7bbb9d55142f0508c0f3bea9c368d375c49358f1856307fefca858cc6e3aa3",
	"0x030c89236c6c4d711829f9829a32e704683c522b3b482151e0b48e0ec744f678",
	"0x1a2e9310173b671b81ebd84362d248c1fca5a0c67325517aa8b9109084e75db4",
	"0x2b184778ba8109a7d7ef107668607899ccfca839a00ad366482eecd8ace6d73e",
	"0x0887cc5e059704b32b0ca6d5a027b6ae2162845a51afc59cbb4ce8bf4d483b72",
	"0x2d8944e1937efe5c29ce5b09654c5cd96d878a3e73e8861bf2626e63aa89a147",
	"0x1942012c725d4b7e77ba67b34f9a6bf3f37e06391c6f0a932907675bc3259409",
	"0x0bf0923eadd085cb5c60b7a466411b0a7c1b7bc169bde495f101181197b88224",
	"0x00d9a3de5beac45a8dac37b4ade4b4d5acc86dda59edc5bdfea5553b2772da12",
	"0x1f6c6215c92fd369b8c7b84ca24c1e5f7db7112630fbeafa9d89c3be93be32b2",
	"0x021ea684e600c322ef3ffb815a3b8d7987cca1b352f72ce295f8dd7c5cb4b941",
	"0x0d142c3fce47cbdd984b983bcb909bb067798b9de15fb2034446bfb0c582aac0",
	"0x18201e34e5d677ae483ad6331fbbf8135581b098283ea32140c904ea65d22914",
	"0x1f14ae90f8f75570dbcf0ade0739b426a0d3cae8c6632492cc8cef5c04265a7c",
	"0x00a0275c80e8070f760ff4874ea4cc66104e249d9f52b4d21b564423e21c9c16",
	"0x28460031e1a74d3550e3f6f727ddb9a9fe6923b7504ac4ef58e1db97c676a330",
	"0x1d2aec68cd7ebc7c9c538c5ed56cf874bf9b0ca862dcf33ee561f8bbbec2301f",
	"0x0823a337fea6ece8a09a3087c354d01ea3a98f0dda1b575f0775475e3d1548da",
	"0x27ae0a8d5cd7822b47205cd6016763becdb9112a11ba39f7621a8f33d7bac53c",
	"0x0ac025f2f0490902eb1c4acffd1db5161a5ce2c53cd3e7cc9d368f8ba64ee0ca",
	"0x262a169b9798becf7edb9eafd1b0e40b19f8d9631361ad3ce23dbd6c7174d3e7",
	"0x0a78e13ffacd896820ff6f498c177acc04c5447e4936ae5f75a72441e0b89652",
	"0x2db60eacf421716f6446fb8cf48b726c77354a051c5161b76bd033801d141ee2",
	"0x2a57c44d264bf024d548a3e60d0a1022bce2c838431707b8828f24e4cea223f0",
	"0x05fa66d7687117bd5bbbd64eb68bcc046a4588050e08ff98ad9f1399efe6047c",
	"0x064e37d6c7a23968d0554c1bd0a134be6013bfc0c0e66e3ec6991f59e18304b0",
	"0x234a82160b99f4d216a1eb321b653c9adf553cbb22f17d11f67da4b02f999a23",
	"0x216d6ebce9359d9d2caabe69aed8f3c8d1c04067e18e3be7d202359d15e311f9",
	"0x1081c467c51566a679ce539ac12849bdd3a2c364b8f9ced1646dd1b8af2fecbe",
	"0x248bcc5f27a31ae9d20753dd587a51ef2d9b94c23a9a05fa77a38fa87fe133cc",
	"0x2364f214e220b903ee979708e6019df0fef1a3e3547183ec90eb8f9782a6cd22",
	"0x043e6458e4756a4f49c4faa52816e050c4846268f24be27060295c9a771b9c5d",
	"0x0094db2df344c5c09b9978632bf3cfdad90331aede8466a731d9bddce89be18e",
	"0x0c6d2edc1733852ba3add72a26752fca93f1a217b04728cd649111d973c0bf74",
	"0x16a6098ea17152bff6c8a75ec34ce2485c0e48e43764d4b0edcf2f107c9c822d",
	"0x118117722fe2d88f2934105759ebbfe24d3ce059deb8a650e369bd3cf6abbbd9",
	"0x2ea0c2712ad80b856b10bfc458561134b738ee060a6e47af530b3ce898e3d2b1",
	"0x2e0058a1035ba4601a5513824c3d8b5f6027e41420489d9ae48309ad9e5b141b",
	"0x151ee5f5ade36b9a33cb014fec55821574dae02844ff8951ce6983810c728bf2",
	"0x2a128060ad3339f6e1a475646754582896ac8e350f7995b03112e46339e77514",
	"0x0f77e79682a11c8068409634161a47a7c40aa69461a52bd039eae9648a55d7ce",
	"0x28bc7af99d8f049a7c3ab9ab5aeca0545436c976cf39230ae387a75e32b81070",
	"0x07a45aea0715ba2db9f7a740ac4f9773e191b0aa700bacd37e66f5eda1703221",
	"0x1040a307df900dc9eeeeb13e9d179b6acd0d5bb610613a525a8fac18a502c679",
	"0x27f167b042d6e438bcac5d1b905bd5d7097e4203f1803f9562b359a17ed41b94",
	"0x06b8aa727541f813d6d077fde225cacef534951a57cf2b363674822b69a520a0",
	"0x2e21f7386079e1738490072c7654e0cdd035a96b2a9fd9f14aff1c7eeafd5e86",
	"0x1295d2863d742cba6d7ef86673c8c414d9642c3dad23bb3d4085f1f7ca792cb0",
	"0x0087dac54159013d757ed28fcd5cb60ab4b2efd416ddc1e9c99ae8d41b110ce6",
	"0x1674607610758fe6a8d783bbe930aecc2ef82138a0fa7197f29e5126b45cff40",
	"0x11e197aff531c06ac58e0666e5252cb3049e89c674de2e8007749b9c181c8ab3",
	"0x1d4a81dfdb2236958193be9c1d821ec987512df15135070b717b0fcc37be2ec6",
	"0x298395cd61f543423b057a22c27efe4a3be20b59e01a31cfb2d79577e69df57d",
	"0x129f54671bce53f0d91b1468c1c2e0c7fb423e4df309605edf7704b0b93459af",
	"0x077880f8a19629b3cc1fe9d6e66b8f93bf92e8461016e56d0b7732b7ade1390f",
	"0x26a1bb930107b22c3e7a0c29dea8a93273538fff4ec4a4d039c13f3d0e2435b3",
	"0x00753c7c895c2194425bcf934cda35a22084b505140d9437349e4b8bf7816f41",
	"0x2ba2689e82aa8edcec7aa242022f3c166eaff38cb66395a64e060f4d1a86c740",
	"0x214a423c20bc7b58f5674654e24e90363fb6529c5a1eea2e9941fe543b56d0fc",
	"0x1ff4ff392f03b97c70a2e1fcb88a5fc1db4e34b543042db333b84afa25df5083",
	"0x15b34fa2581c45082cca16177dd5068b345c6c7efdff3accce1b9f7dfbd3eb41",
	"0x11bc462199036da340f3c0d41a6bdf839bc1ee0cb9ebadb6bb38427e361611b3",
	"0x28161d79066d711aac31be72a36fcc7df61d67bfa4456b7f914b39a7c1342858",
	"0x2ae16e32d491cbd72835f71e7e86617e2c7b495f7b861f0b8c4159602755d3a8",
	"0x01d96e5487af6de5fc1eb22b7299bdaf642fa9e6dbb9404e01518a760dc7224d",
	"0x074532519035231538dc8bc5167d4680c85ceba9b3b8e20290de09b98f30387d",
	"0x2ea4c255ef1e0c5cfb01c1e8c1e45dc05edd83947d61d05c70eb0a3c3df66ac0",
	"0x1ae8baafbbe48e1107e8aeba0fecc74ba5c8cd52fc83c80ed82e5c9388180d21",
	"0x0cd3f59be85c9322de0a5075b39610f4a00a40dd630d4624b65033c2fd5e79b2",
	"0x1a3f08bc979b4d6eadb182ea2f866735dc08b71e127e8a1244f9cdc355cb97c3",
	"0x20db22962066e62742f06fa99b7213b466954e1764fa27769e59cf648c7fde3e",
	"0x0350fb4047e3536cdf4d26945a067f18bceeb588375ad9ac34622d416c2abe68",
	"0x2257581b7dae080a1a206371246db391c0f8b623d678d2c3ca4579859a055314",
	"0x23c7c5ff1a69ed05d3f369017dbf6b094f8449c34ecceed93e7e4b134d7c383b",
	"0x194aa80909a1a4824b11eb06183a3c96b0189ca6f374befd03ee174ed12e19a9",
	"0x05c3f3a4c83f4b1b2a1c4fab2c9c6b59986d8ba2915b04936cd6429096cf4983",
	"0x20a5052477e55329307d8563b918f070d676cc3d9fda1c8fc490a3ca96869ea0",
	"0x152542255b09409555d12c7e8e138e5c87806d92298a31b498dd51346921fdfc",
	"0x1d86689b0740bb87db126563f0af369a6e9a3c07e9fd05ae7aee248b65dbeeae",
	"0x007106ab2934b0507a0dfe1b1ebf39c5f16b80e5f8b7fb5ad27d24fc213a0774",
	"0x057a73ced1d87b2712dce78de77b2741af9ea3287e547e7caf46095e6e7d43f3",
	"0x158ae6167c5adb50143d0ec37c64dce1029d1699114071d52d9006b85e70fc65",
	"0x1fe448b3420cc22a07a836b87b33eb26db7bba3993dd80466790acc72789072e",
	"0x16a1c7f567984b3b345627ff95310f37b157a7d624e170955c8b5fefcd02c052",
	"0x117c27091dc5c53bca9bbb3406ccf22c022870a39e7a140abddac2a022b3fb2f",
	"0x046cafa3e203bd7e47657c0a6fd63dd52f1394d9fa81eeae1aafdd787d93344c",
	"0x0356c1d0dfb86fe5e298b2f9c3e162f832e0d8b7026d589488ceec826d4d8863",
	"0x2e2ff3f00c7e8378e30d59bee1e487f51942318e99933a720528d2e20de763f8",
	"0x2d14d3872e1f5439373520493878195003b0d1f6f5cc4d98dfe90ea17e2b5cbb",
	"0x250f449319112438705990d79f16510940316bc23574c47604a9d9c4ff5c19e9",
	"0x19734869e4b612c077b19a39ed2cfbb76d48aea3449143c69698268345e55718",
	"0x2528513d4161158c3668ba2a2e0a2dbd03347905e1148ece1b832edb1cd034d2",
	"0x089a9313be58454fc39abe163bccb73e760b282b49b6f9ab27572edcb83a00ce",
	"0x15eb8789989ef7eb35b2fc888bbcd9af8023cb318b64c0aced1cf2b1b9bf0586",
	"0x1ae7d9fc7e40737e2082b5cde96cada877b8b2e70d7053ce2af16247377f65b7",
	"0x259d1a4038ea066761f3bb50a79ccd9a1b6b4dd28eb48cf253e7d43cd107739e",
	"0x0cb7fbe7e94f3f26106406e70ff834aa4f675e72b668acf56fc28f5ff1749fce",
	"0x247f38a84de7e142d99ba06ffbbdb5795e100922f0470e160a62035c079351b6",
	"0x1de970b548bdbe6aeea6ba1ab509138ee1b3f8513a6d8b435c7d01f2f2bfefa5",
	"0x24eb907a4281ea460d703bb49488a9c9c849ae77f0af94cbb9586644607b7e65",
	"0x00446d59d24f80f778b2b3ac4986cfacfc0ad671f373118cb51a986fa2d88eca",
	"0x17074e2d8c687842c894fb5dbd58a805aea2d3bd39dc1888968a90dbd659d444",
	"0x1030cd0e0220d45aea30acb045917caab18cb2149a5eb795bf7f47749b17b858",
	"0x06ef9dde018edcfcb5074efbfa02d84fefbb8a0996db20fe83c45a0fe1dc0e25",
	"0x0d4114759b4c754875a2790867d24583774d678478995ce080b747625e5b74c6",
	"0x21c1f6ca2fda43baa1d8cb6fbd867b3fa6a8608e10d21ed0025fca278ac545de",
	"0x0a2747be7013e3d0422fd4fc434634fa7e9245e69301039f905df00ad3bfb20f",
	"0x2eb56281737b890c29bccfd0e3f9723ba3b578d55d4faa75249e52bc4a381964",
	"0x04b256bc91c536044b2485e73c068d0dcee4b368211cb7acbef5315c1da8c37e",
	"0x03a2e38ae0785d00dc5db546f38f68f33591621345cd3290a661291749aaf6b3",
	"0x2537227e4f6f61d4b46426e68290232bf5aa2cf643e714f045ce247a2cfe8df2",
	"0x212fc1a7b151e40059f288b897739383e0d379ebb3106573ffc7f526e5784241",
	"0x2fff933c425707ebdb526ac6b656af9d8c8ce509c975b78758fb31997f3f4b1c",
	"0x1cbbc328136f125aa236c3734add32f8b4ad183194779379e08f73295456290a",
	"0x1f483cf056e84c45c473b639e5028701419932a6f7c335053a8e45196f533836",
	"0x0aa01f67d5a4359cf4cd37c2e93738d5c1f20a63951cf40e0407106ba7d36813",
	"0x206f6d1f48c5c03985b98e1599a381bd7fadce1d8963e1f05aabf1c56902c8ea",
	"0x22be88ab8a4f5379d117d3e85be4966d74cee4265d4c0d166a962e944527a7a4",
	"0x165107df1752a3bd24bd3560712e1843481e0b4a5b3008afd29f98dff494efbd",
	"0x1b46f7fec07da7b115f28afb13d8058419ec91afd3326aad742250e5159986a0",
	"0x2a99c7e64065aab37cdf24bbf3d5fb0f0637ad20091a51fa528e8cf309164161",
	"0x1ee88f03930b80d31e4c5fe072eabf7a86e48c4d7f983327a39abf0ad3057a33",
	"0x0fb305cbd11b156aa6762215c22287ea941619d85dc2410214cde116bbc0443c",
	"0x1797ad79a9d724da6c2ab4597be09ade55120383354592ae9c8d2e4ec07c84a7",
	"0x059e98c3acff40749903a86a1fd3b018684e77502cbab1d6b521ab14a367f906",
	"0x2827a15f27d2a5f7a6ec18c156e0bbef7746468807803acf5d6172b6c5c5155d",
	"0x0b028de9c9cb108265d1b9a4884d4eb002b61c06d70f6923c5582d2b181b97d0",
	"0x187dae466c822aa7c690f4b892cfd7a1965b2a7f1f2db4e2904ed479f255584b",
	"0x09ad6b0135efa9f60b55562f0893ab9026ec6697a3db8b3c5504b5fe301397fb",
	"0x2e8517f56603d4dbefed956c6e39e795f2308efa3c79aa31186e93e96824b429",
	"0x2dc0dd15f5448cba36371a75fa9231191ad7ec0da26dfab569f12c2ba179156a",
	"0x0af3aaeadcb07d7d8119e86aa8a8aeeb3576c5dfe6b5e83e9ea2758d06427350",
	"0x0b680e85d63a81fd0f210fca379cff967c4797b2fa606a990bd1c16c833b7e0d",
	"0x22da35f5ff4568d2c4a5e51e7703d8a8ffdbed29d075eb3d4d3a512b19487fc6",
	"0x08c890ad1e8e4446f04a38da51b98dbf43b49563c41a6425715b1cd2ceaa08db",
	"0x082668b36ac1582e89deaf6400a4d7f966352ed5e366035664b273ff750da5aa",
	"0x2e9e2ab06d12706142232ca895a3ae79253fde4b6676f5076cd2bf0746bf0d90",
	"0x093cd1f6dfafb4f62b6b2c400f870cdeab22e1e311d71340ebb31ad7e9433b79",
	"0x0149bfc947d77c2afa12dc2927fd03613969ea17ac0f0a9b29fb49962d18fae7",
	"0x253fd1e4ea7110b50fa0200c98f8b296933b1a231048bd6ce091dc04ad99c1f0",
	"0x1c10ec888868e964e3bd2012d426182dba62dcd718a4b3aaf66a97cda68acdc6",
	"0x010e2356118699b37a60bba59f5a6ee3132f7a18996b332fbff3eb264fd89b73",
	"0x1f9b7bbd96c4d9f9a54ac5355760cf463c0bdaab0ea3d045e4ec5da5c4832e94",
	"0x1491c0a7112863c2be71ffd534367994d84004463021185cf0c35874ed2b1590",
	"0x0959d453bdf0abcc8f715d36c6492aea411ae30adac999ff917169b8caea277f",
	"0x1c702e3ebc57ceea6564038c13ec1ece7e5896b3c9086ed7d4fa90fbbbe5c6ff",
	"0x1ad9668fd77b506a8ef16cd18909974ab667e12c6c265451d23aa02b0f67a0a3",
	"0x28741fd4900d3594a89b745ef99a3a6e55bc48b6f3a26808a6b5697d7015d969",
	"0x089209285a64e7e4f6335dad32c7800b3bbd001d8093fef931d7999c07787f1c",
	"0x23fb32b97ef7d64b0be2901f490c7e5c9e519d39d47e7128674c62f6e9cf5b9c",
	"0x21bf2774ffcadee48ceb32c2df18c138323d40c97ccefea84fbe700d5ca21fef",
	"0x240a33f4b2cf1f69644df229e69f833510e942306d9c2b9c52bcf02a2cb8f678",
	"0x242268039b321caa8433d0d2ac21b7898a8143f13ae9506b469f6bc12c509a67",
	"0x287b77185a2c6dfd946facff45806b26e613fbdd78b703f030780fd66f0f6c5e",
	"0x12dc35ddaf09fe7235704dddb9d5c38623eff27378c0b65210d89a8b81d1eefa",
	"0x1ada1ca21d8b80383f6d58fbddf38508144fb8052ae9f7b928304f36545e2b3d",
	"0x2babfd2a256b88c905054ed999b515956871db159a18fe115e93180c44f37c46",
	"0x1871c482064e022ed622de11fafd1fa3139ed8f3fc0bc4dfc5909c232e77427c",
	"0x2233740cc192366f9a9cd43f35b1c32140c7efafc239a2e45d6d6f8e41e6a491",
	"0x2d5bc4bb53068d31e2ee07e3243faf53d4e24cd32631b8776ebf0166df0ee204",
	"0x26d150046df16c3bf386753d617d807e06fa24515320f393120c56bdd307355d",
	"0x225053710fb843019d8770bffb36d1daceaf52805aa5da0c7b0d8e5f648d2e59",
	"0x11532827bc5a08abdc637072e39e6789f881ce9093b67b12b21fc9b1a4e82fde",
	"0x1c7e76731ec56598629809cb3b8a98a08b406d70a53b5fe07a18f5e2d29fd710",
	"0x2d20032406ab91cf178aa72fc438a9832503b77de20c20325263421ee3ef5a49",
	"0x172fea32532ade5764f59663af133086f2d273b949fa7e48a875e4ae1372f111",
	"0x297be38439c4f893354bf1fac9cf60200ad1c9ca6514e3bfc8b7d14a8b5e92cc",
	"0x0fba7c3f1f63461ab05678a189c6dd6f78c37a2d2229acb0af3a74bce6963ca1",
	"0x2c737a9d679dfc75a851e59516648d5fe434ebf034ff2d11f5469a9fa79c5150",
	"0x0558102437790cf69fe8fc0887201c8ec64b7724250313df8eec9ae2c85b2ed2",
	"0x20e5a6b670df17dba01825dc706bf6348727258df3b7767a09a2b02191b93620",
	"0x0397dec50c6ad272802b1dcfc96a982c6ae3c79faf1311c64c1efeb6e44e3bb3",
	"0x13c8e709a7286f1bba4f57ea31ae54e69566f9d95e21aaf51cbeb20a0727422b",
	"0x1836b19f5f1b74f7ce0847c5cc84dfe49e854b24088644a3f106e21263ccfedf",
	"0x29578c68ed2259d51a0493e1d8241f6292f22454b7446d3c9980888ad5ccedc3",
	"0x0ffe73fdf5d2b55acced9253778cddf961778c1686c303913c9a1a7ae4e927e4",
	"0x01bc12e7b7b3d5032274199b3d08891db84627b97b771e0fb6c85d372eb0c4c6",
	"0x0c102a40968ab44039258460ed597c8a4e44aab4337be6bccdba0663f978a61d",
	"0x223204d7331c8f804fa561f0cc921ce289451ed9875a320edf0bc2fac7bb2b76",
	"0x2b790afbb55f6b9ed025b441e5b1a58c9d120f98ed3f522383100128123a6be7",
	"0x0a3ff4aa95c6d1cd4cc8ca7c863bfc6cb80bdff43f3c25a9ec5c135392d63db6",
	"0x1ff8432b762327efb282cde798bf85fc4872750d996414b58d5e00d422cabfd2",
	"0x2e3f40008f1c38a4f26946a7de27844fb945b931f90b809a4675148de71286f8",
	"0x1bbc62d1ed35958a0d6a200b5c4aed44140fc67c3ac6e4dd3fb58caaf7ebacce",
	"0x199f477dc031719ec4887c42673bb613c5adedc364e8f1a82f073f32ac557367",
	"0x169ac35166e5d5940ec64966b054b1b308a66e84efd6db86c99b4f250dedb226",
	"0x2c41095444f05d3b856191c96f8c7af7966800a0a25a8662b99d034fac463aca",
	"0x142ee7817af8c723effd0ed4aba66fbcc2cf413d023daacaf4b29521754299d7",
	"0x1a2d45ef338277f5b5ad10ea1c303a2d7c293c94e36e107188688918cebf1343",
	"0x06373b65f9e8658ab23b29cca058d38fb85d78c172b2de3b90b3866996b6cc04",
	"0x096bb52f1df377c6ca866b5fade0cddd3c89f94b5f59c15313ef22f8fd33303d",
	"0x1c3a9b8dff2a897be7c322eae74a82dbbe93477dbadd5e3a5d6e0fc39d239b17",
	"0x235c65be6417f827fc7d1761dd9cb6b133d553b8688ae048a9dff47b9e3dd2fa",
	"0x26b53c08d440b9d3ad076d1535d0e95ee2cd7580e484c34b8c03b969f3d593f3",
	"0x2184682a8cb42da60baa6273bc42a022d74ff08ffa910dc6a636fdc86473d3ba",
	"0x0a75229dd3c1088575014a67cf1c97dfeae9713b65cfe92fe474e34a99af7731",
	"0x1ccbeba43038e3fd0b828f023fffb5c67c8b1030ef955c97bc0f865f83494638",
	"0x2d6d18c43fb508141b3b61c6b058a465f7dc60d503f22ec1892e39bc91f1b946",
	"0x03c30479b58b546a0708da59a351ba8e2cb345429335cd0ce4136cb4e7e1cf89",
	"0x10ef2cce78c0fe730bb924186f8f435759dca183289e55d4e9111a3d8b28ff47",
	"0x0e2b7bec69288ef539d7410bb8d745f1c769f2e591ad6cdcc4c98c60ef89119b",
	"0x00ae09352dbea59b99167dc9fed2acb7d667190a721e98ec9deda6b9b6316afd",
	"0x0ec2c2931833e777af2409c36e8c7e4cb73c954f13211b4bdf12f3f5b5e2e64a",
	"0x1eb6b70aa74ce4b3371c80c4b59ee72a91a2dfea2a8b94cae0d38fdbc7a7b7bc",
	"0x2b6504c7d071d8c8126aad705a5738012b63a4f71558be735123273119d7c342",
	"0x1ba086f838f3611e4b65cf44c7a8c88078bb448fdce0329e883966eef56b41ab",
	"0x076a1379185c408d0d3ac6e839585876ac75e93f1d4502bba5e65fa540031880",
	"0x111640c85e2215c998c8cc61683a808ea131d6a42cebf68e951eca71d43c7cf6",
	"0x2c9b548aaa7704c429dca768df5a92132768c9f20f504dee6921c081c982e7a1",
	"0x2cdda963f6a1eea779a3f4fa155e75b1ce5e10e372554e1a8900fec5b94c40c8",
	"0x1fd0b0ec6152dfd4732bd15432f087b8cefa2a348d9677f115e41f50d397af88",
	"0x1de41229f4a96c0addf67391eadd20872e0cf59a386fab0df12f99693263d25e",
	"0x2a6059bf23409f0b335fe5cb71e3800a5cf02ba12714d8fc535953381075f031",
	"0x2ae7470b8aebb2a75477de36955ee86dd081800ad758116c5336e63d636faaad",
	"0x07a274bf6c009e61a156fa06515bb93d38058d75647ea80aab981b14fd85a35a",
];
pub const MDS_ENTRIES: [[&str; 5]; 5] = [
	[
		"0x0f754ed4286eddc119da595757352ee4e8d08794fd42d5b4a0e9eb7c15a4be7f",
		"0x0dfb14b9bc39250dcd9cfeb17fd1aadbbbcf83098dba5af872dfdb31c7383c0a",
		"0x0bf9c31074794ed5ba3f153378af6e9d251bc7b7b3187939c2f5e0a71471d2cc",
		"0x1b0e46f0de792b82ab0581d9117c2ff0a27d795d77ceaa93e374966e4d2b0a7b",
		"0x29d3fe5323f74bb43006ecacc5b6b014897de19d7c1642d4a13a68715f56f505",
	],
	[
		"0x1006dbcb453799b3e3ccdd4434b5b9d6df1b152f37aa39bfe122dbe7ba90779b",
		"0x062dde697032bdccdb235105996f0096ddb8f6a4155c7b68ca1e89de6578b098",
		"0x0e75d9f202ee7e78cd97c0babc6d5299a4e4f418e3fda05d011652ecc3c0ddbe",
		"0x27c5c227352a389eb9cf95b40f7d0accb80d3269fe3e75c028ac0581ecd6b669",
		"0x05485ca8b0a9aeb812c1658ce5c7fac88088ea53244a5b1c89138a2350b1eaa3",
	],
	[
		"0x22c42b2b4213502f74dc84eeafbf04a4cfbf8737815171a1e9aa60635140f612",
		"0x19494eafdfac930888a74229e643d6186b3ead8c7888c29b241ad4db45e409f6",
		"0x05af2be87681dd56ff875b0040f2bea03107d6a5fe482b6bc4f35501ee4aaeef",
		"0x0bcf71bc07881e3102e041d59981e24abe05ac3836a3a5145fddcb588bd9e50e",
		"0x21137ecc29e97f36b3952eaa7eca7a0868e071a21a1b01abd0e1755eda8cdc8f",
	],
	[
		"0x0d2df2f0631f3e8b6ca5814e24712563c3db9e2833d995e4a7de7eeb2f17d7b3",
		"0x0fa1b37f4438c10a4d7e0122fe8f7cbc19ec5f2aa8f3531cae13a2c3f45c0607",
		"0x012f7124314119a80e97335f5582a5ed720062466f7d02a8ad18f941e139ee26",
		"0x0c949c2193e445b5e35a3249666ee59c22c8e4b9b83f2e94b33d06c7433a8f25",
		"0x2f44338416bc000de0358c02ff17f82b51cd09f5a3d694104fe3ae96ef0a43a1",
	],
	[
		"0x1dc1c7e53d480961b07594be9acb76170ade382097ae0194cd685deb8360f1e9",
		"0x2ee218b3da5f5df6986e69713d3dc58dbe1bef87ac91027d4630c4f5cc3faff1",
		"0x2d03a0c76689d3404bdb0f140240c33c489bed08b5493cd17a64d0aad91a4fd5",
		"0x128365feba2143eab58b42221bdad7db2bad6d89f8be64853e08e718b07a6c90",
		"0x1d5b3f45251cbe0793b42619735b0f07cd5bb335533a2d418cd477abe9388ae3",
	],
];

pub fn get_rounds_poseidon_bn254_x3_5<F: PrimeField>() -> Vec<F> {
	parse_vec(ROUND_CONSTS.to_vec())
}
pub fn get_mds_poseidon_bn254_x3_5<F: PrimeField>() -> Vec<Vec<F>> {
	parse_matrix(MDS_ENTRIES.iter().map(|x| x.to_vec()).collect::<Vec<_>>())
}

pub fn get_poseidon_bn254_x3_5<F: PrimeField>() -> PoseidonParameters<F> {
	let rounds = get_rounds_poseidon_bn254_x3_5();
	let mds = get_mds_poseidon_bn254_x3_5();
	PoseidonParameters::<F>::new(rounds, mds, FULL_ROUNDS, PARTIAL_ROUNDS, WIDTH, SBOX)
}
