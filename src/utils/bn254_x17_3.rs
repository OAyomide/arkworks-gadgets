use crate::poseidon::sbox::PoseidonSbox;

pub const FULL_ROUNDS: u8 = 8;
pub const PARTIAL_ROUNDS: u8 = 57;
pub const WIDTH: u8 = 3;
pub const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(17);

pub const ROUND_CONSTS: [&str; 123] = [
	"0x1b3076f41cbcf5e9fa4b78634027ea500524cba9bbbad22fa1c4b7cc19aa66c3",
	"0x2bf1bc122783fa0c18aded20b5f1bd9aaefd0d87b6a8ea8fad2bddf367cc083d",
	"0x2f6f368166d3e953002ccc2e0c0e61663fdbb0034bb97b51ea95a935f0a9d49b",
	"0x25686dd23dc2d13b1f5495c9766ff0b529958dfa7568e1dbf9ba1a72d5e8207f",
	"0x10bda68ef40806e06c7a5a337200f7bb0780e78726fb3e656b40cb3eeadd2234",
	"0x28d2c5710e7bf5f003c277f1c6dd77e9be9368e01312c6d8c0c5b44194729db6",
	"0x2d595d8bfea5799caacdccf6a679d758999ce723fb5031e5632535adedf9fcb0",
	"0x298b70c75b803c70d32fbf16b6270fe96fca63b71f2458fe3752631e4b3283a7",
	"0x215f35ff3c6092ccd587a1c7e0f76cc058d4c1215c0709145c0df5fddbf610d1",
	"0x1f95bb518cee738e8ba1c5bef5ee4245c335e80d8066529f483d842bb8724108",
	"0x014e086a0e26de2206c607c19c294d9a4f1271642ec3e6fa25d75b34d3edd896",
	"0x1bc43ff9bc4dada561a29a272522b5ce78a607b27823b36702fa49b2caf0737d",
	"0x0feb0ad8ec3390116eb8ec87c2f8db1b8de77de5f0ceaa048af31846737c704b",
	"0x297861c63fc1006e29cb4453c0f52816daba75b697f7506a505b741e3c1012d9",
	"0x1a77213275c2433f16588a5802e35e34c66e36f5a55599fe7f306e05bf6031c8",
	"0x1db5a0c8a1fad3fca0edf3ced0caf927725633d2a7e004d3c2e0d911631bb597",
	"0x2ed8c931f260729c27badbfac10ffac282a932de62374bd3ac9678496764e5a4",
	"0x0ec01e6492f3d1016e906dd15a671c24d00a962e32c915065ca42533a36a8eb1",
	"0x1d5d981cc72a322e7684d9799600c4b938ae611b85ccc0359db9d50a14443edf",
	"0x1b4bf8681db9becdcb407038189624cbe13e3a1c9e730d5973bd01bc2e0b0b30",
	"0x2d4e57bda01ba34988e59232a99997e61d9079d0f7e93579a19432e90b527366",
	"0x242f43b7b5c2a7efe2457d534a82266de37f7d07b8bb558eab4596edb712e804",
	"0x15f4cf135455a6b6d08be9090458b92b6906fb109309679e10da9b895f76b0a1",
	"0x1c87f41b43d145318d4691e949b5251e608775a94f5955a8bc9bbf28087b72fd",
	"0x2ff9a5cde190b4e0e9c118daf823d0296b5b733974a42479d4f1208a52fe0814",
	"0x2462bf4ca426f9321111c199599054179995a6b73ce156035c59a16bd70bf2f7",
	"0x2398a82cba8e0cea9bbf22967159ad1144505be8799bb7a9b598fd8b1d2eb63e",
	"0x0ae80bd7e2cc3a57f9cedfc234e544c8098e2a22eab3b41befaf9ae1f45fabca",
	"0x015506a250a1fb6f5fc65a3386f33a93dc53b26aa4f495fe907bb37d802c7b5f",
	"0x16e741399442b2d58927d2594bb7e0d5a983a1eeea8dadb2fd8b93aeeec26655",
	"0x1bd3b94b87302d11a3d4d04f767a73790d68b73248c2e571c4d6347e39ca6917",
	"0x1a010f2e472e644cf4e0eeb7c18ecce574ec7820fbef669c326d52e1b097e272",
	"0x0797cb1c81f29d9d3f0884f643aa4233165186535cba0c2197afc75fe25ceef2",
	"0x17a8be97d599f43ffefc64559d3f4e14fab35ddd80b5b60c88cddccc9bb80e4d",
	"0x0dc0c75325ff35a54ca0979107cfbd05008b1811e5dbd13dda372576377c37cb",
	"0x07933bbb92bc777e0857ed9f4571a9af14cbb751d98b434c8eb41f837af0471c",
	"0x0abfdd844e379184e9941e370de6992f0dffd70b9fb6ba868972791c47390a38",
	"0x0e188bc2aac54a48e42e94ac535af5c68df6fa8ce64bf43dfd6b20b3f5a1f989",
	"0x242052180b3a9a1107743b627a15edc956c211680c004ef8812df8bc65d37da6",
	"0x0e9bd8d7e31cfc09d362c27017555ee4cb6536b8f664663afc5be308aaa55aa2",
	"0x0907a05e96848e94ba98195f0834606aea8df71ae86be5261959dc41779bb0cc",
	"0x27137bccb6552c33fafdfe103a856564a412b50af49d6f512ccc3cbd79da7716",
	"0x0864f9061ebb55440d5c877aaf4d36d3952d897e8f4581e01e1a4b75a5fb7b57",
	"0x1951ec61a3c2085511849a92fe479185356cb5921b2807dc5c822fa58e0e179a",
	"0x0e5207ab6856d389c917134b647a8d4a85731614c5e319155ae2b3ab91fdf5af",
	"0x15349491fa137b868541bcf10c3fe6f6ba2d3e238ca27bfcd044047811dfe388",
	"0x096b26a94ce9af3e87871606f27e3b767f8f46bd98ff4d7e454c1ba5fb49c9f3",
	"0x10047b0649fba4f0cab2c289fb2eb17fd4dc63f2ffbaf9a3cd441fe753157815",
	"0x0a516d7e68946f915be287d8d7000923e27a09d49198108a5636fc6ecd82d416",
	"0x2ead013caba384351de47091b8aaad12764debc85c368ff98de89f75cf558482",
	"0x1a258e974fc498e766987463fbd111eea24cb8ea8a232de8079ee49ab534264e",
	"0x1f392db78f8adfd70fea8d88c3e7388d062999b0d8f19c0f2b4c85d2196a7ea9",
	"0x0ff2cdd0237410ff183ceccaff4f48446d41a712b7c4babdd4a09c79bce40322",
	"0x1954994cd2ab37da04bb80325c4e16652ed79e9c84879ba1204eba55f66bc4b1",
	"0x091dace3590396c4bbc0bc75c15dbb9e379f8cb450f340673efc0353b12cc42d",
	"0x2a1d4c5e20d6567cfdb428365faa28177d900b268d71eeffa0e852a32e500de5",
	"0x08f7df5ae38ec0a864203d5ce74aca86cd6273e5d045cb98e2908004ed2183e5",
	"0x15ee3fc251e3afdb1d87fbdf5297088064b89959e47c96b451644082129a0a63",
	"0x17e8b55b6b221947a0e89a2df49533190cf71c906fed16f1d85aaa65fbcb5345",
	"0x045a0001f5ca1008c1297c8c845f438819a3e1de12a4db6601d364fefce08466",
	"0x21ffc6a3475e5a77759c45f9e15f5a140cb6d23d103951b951467acefdcbe48a",
	"0x1cb90d3f9df7e3838e40c7b8776f692b10ddc169e7d29d5485f1c478ff3e9b74",
	"0x1274b0e74b22eedbd0b3edf57270525c14c4dc13b3098b0bbf089953b8f2b4b8",
	"0x0f919a05a6d6762f3ca0b3042148f0b4501751ec146cda108bc6a4937dcd7a20",
	"0x278d8a78a647db6b698ba4740533fffe59412591f8f9eb295ea50e08ce38bf08",
	"0x211320e94a851bf7c2660648a3bba16f822aef5118561805427651995f8db3f4",
	"0x1950aa5510490993c0a5470f4d300223c26e0abf8ec3bab547a6fb1668319afe",
	"0x12612b4f58eb767dab7223de0b53106e5f78f60ae5854c862b14f834c7e34e2d",
	"0x0ccf8cdab25538f28cf9e2ab2aa54043b40365a5dfce0dc56940118a175a4936",
	"0x225d91a14448a93785457903a1444e2c0bca0ae3bb3490b4a1f448bbde88321a",
	"0x17f866a83abbfb3f5b20a0fb40861b669fdeaac1673ccf7233bff2726ef07c12",
	"0x274ff4db91a135b7ba61d39ddd2734fac38587d980992415c29dda232958e785",
	"0x2c7003822597c33cdb0b1c6fab8661f76ca8ed03be676a582572d773a3433dfc",
	"0x001cd295eabaac4a64c57e07364caf03279e25b58805d1590e493e79b2dd1ae4",
	"0x09d6d81f21776bbef325f9bae6f9f417059bdaeef40c3d3de755203370493efc",
	"0x09136a76223adbe2bac5105bdbe30e0a4aad7f84b30226712572adac387199d9",
	"0x1cfcd0dee899234b8829088c6a46c2892050a436fe261a9fe4180d0e8719f94b",
	"0x1f45484b7e5d4bd51a2ddef8d7bbfb1d3f4aa19728233025eb422837af8d9072",
	"0x1c4abe3b58f679db34b38c58a4df389ed7e981bce43df9e53099bec757963a57",
	"0x06982e0f4b537c75a436b321de06f9ab347519500a38a5b22ea7554a8f56e0f8",
	"0x07046365d522cc5022a72f52bf0dcacca143e90f54cd1b0caee3a5b19d2246f4",
	"0x01efcc089f07fe8a25fde5c328678633c9ec23cdaf554d21b17a7f11ffd13cb7",
	"0x14bc004b83a71020e8b683b6e845db1073bcf25b0d7203f43d8a7fff2b5fae77",
	"0x2ddd92756d1ca22599283cb365f50d429f4737d258a4241f80d9babae4730b4c",
	"0x17ece7004ca56d33c9c5968ef9e150da329b8882987c323d81093d6ac0824b2e",
	"0x281bbf1c45bff63378c0457dc44fb069d8c237e27995dc61846d46cebbb18238",
	"0x1a00b62874d4619e211be98505ef9ef764b21f9e0c63d95ae5baa5bd9956e1af",
	"0x20ad921df9e684a135efa3f1391f81c3dbbde67b7ba74892540eabed4b8a18e5",
	"0x06bbbd8057fe0e147cf1cbbd40251f0bf5987e7cd67f69a512b66e3070173ffc",
	"0x1e1c12cb34e808e17fb41da0a333d721a674864cc579b7ea0b38cadcadcaa680",
	"0x2b883aff71e7de12f8cbd77e365315fce4cef16f39e0ba936dd486fc93143d82",
	"0x2b0c1f712a5d20afc47b55229943b6e18624294f5ecd61c9a63fa5460a7ca60b",
	"0x20318affc0894f26029dc515a9481d42cae1bdceae78ba8fd3148868dedcf5fb",
	"0x0c6e1b2ac252339f0df200a40e58be6a84ad16eed290688a93ccbd89f4218170",
	"0x2006de7a6d053e39653625c6b32429a189794bf217b16ff2157ea40ee9795f4e",
	"0x13eb3541962c2a5408302a4a7ef4ae5b42e1aeb65b663f5c2b62bd13c7cc214e",
	"0x180631350852e5a0c04efd2898d963bc5b3ad352ec98116840e7898f259a9713",
	"0x1e93c8feacc032df27f42338fc2764e60cf84ff4e24ed1adf6509b2989eb411e",
	"0x07911273cff73e90c9c63fe23d05a7fde3fd086ccff068c4fb42bb2a8ce4304d",
	"0x24133be73ee303f6ded5cec002ff07c22534ec770055b03dff7ce601691f49b1",
	"0x120a1a812ebcd499ff86344575d580c545f2a87d6abe23f0576e66ea3daee174",
	"0x0b91597db31d928492d432dfc29817b5bbb8255cc43f3ddb27dd8bcd4bfb50b8",
	"0x140715365175ebd2328718b979ac6d994e304dc26a0a57484abf61a72f4b856f",
	"0x2a1fc89fe556f8675356da275941577f76c3743f45ea6381c437bfd2c072d1c1",
	"0x04f981ebe367ab4076112d77fbafb9e1467738b9300bbb7526de0a526c81288c",
	"0x090294318e3ce0b78e5afa9a0dce6b35e83169e07c4416239a3b45f89e672711",
	"0x2c242a8b017fe4ca2b5eba61e1da12b5739e01681409e697331c0692c0906031",
	"0x2dbe33685c5ac43222a9d6ba08183a3866938c8935a8f2e848441a6665f8c9a0",
	"0x076a8f44648ee1b4cf58ca4aa24a9a81c86f729a1fbae07effc0b7301fbcbe84",
	"0x0923e6801cdbb68c1762040ab0745b7e0cc0a0775d6c9df01b90e299174fa03c",
	"0x28187baae9864f27813aeb073775f9ed0752167e598612a2e21652da9e1e9cd7",
	"0x0041d72e6ecaf8313949f8ad7b5f370ffa403abff55cfd285be93083a810d90a",
	"0x0453618b764c28db39028e848007d73ec0262a5a2b38033238b1e27d6468cdb0",
	"0x142da4aab19cf148479876d07ef12d2105e504f2980b8fa21a0869344e6b167a",
	"0x157ac3ce8fff96af9216dd67cc31d93c10fc38437398a57c22997aad11851d46",
	"0x076d9c8634610797282a670a965cf9c72cc3200c7da2f1fcdbd375fa07e2fab4",
	"0x2002548afde4ddb43cdfc34b7ac0e0c6e35863bc649b161f52244eba8b792973",
	"0x04728521ee32f7da39fdd4fae3160234e81d682160a89a34a76273bff47136f6",
	"0x09d2d5ad962d3faaf1735bcef4878aca2cb3e9d667ea366a3c97f1e4e38fd07b",
	"0x069e6ab1f581cfa30207b88ae57c3b14e36119ffcd81db94ea295fab5e81b174",
	"0x2a0343cb81ef3a8b39a96185d2289c9cf36cda665bb76a241282b35564e51add",
	"0x0336c68f9dab0d529e632f6bc7a20702e81d7c2285b73541ad6536e5fa1ee91f",
	"0x196f84b4b83f43f417a720c73b1afc4351085da4426fe6ca79b44e0beb6a065b",
];
pub const MDS_ENTRIES: [[&str; 3]; 3] = [
	[
		"0x11092c76ff96d6a5a24f9bb3b960f3d860d35e0b95a0f94fd6ffa3784ece5ce4",
		"0x19e8e51059fbe675b15135eb37fb1bd7cfe8ecb5720cfcf215d72274a16b9eae",
		"0x2850794e401ab8618711e58af8befe1a486515e401a6df5d89c306f820c91f20",
	],
	[
		"0x2b188f45b21764e43e4ace9ebf0cddb859e0e2a00c2b51b1c4471e73c6648d35",
		"0x01f574af43d21ea5ed52561538aa36cdeb8436589314368262f42a988d83ad5f",
		"0x1190ec00645944c68d1a3134f5f0fdedb38130950f55b996a2b2b70d84809b60",
	],
	[
		"0x27be16a50d71f7009fd08e84936e1156ac558a467fb595281bb27530edbd4416",
		"0x0a2dbdac4f4e49c36bfc08c9b3f6b3f2fc3d1aa3601a2ae1dbd64c46ae5491d2",
		"0x160f3229d7a28e97795fb81af3858237d7992950dac77754b465d5bc40ad17cc",
	],
];
