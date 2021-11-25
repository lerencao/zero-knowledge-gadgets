// https://github.com/webb-tools/bulletproof-gadgets/tree/main/src/crypto_constants/data/poseidon

// Parameter for:
// exponentiation = 3
// width = 3
// full rounds = 8
// partial rounds = 84
// prime field =
// 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001

// Sage script command:
// sage generate_parameters_grain.sage 1 0 255 3 8 84
// 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
use crate::poseidon::sbox::PoseidonSbox;

pub const FULL_ROUNDS: u8 = 8;
pub const PARTIAL_ROUNDS: u8 = 57;
pub const WIDTH: u8 = 3;
pub const EXPONENTIATION: u8 = 3;
pub const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(3);
pub const PRIME_FIELD: &str = "0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001";

use crate::utils::parse_vec;
use ark_ff::PrimeField;

pub fn get_rounds_poseidon_bls381_x3_3<F: PrimeField>() -> Vec<F> {
	parse_vec(ROUND_CONSTS.to_vec())
}
pub fn get_mds_poseidon_bls381_x3_3<F: PrimeField>() -> Vec<Vec<F>> {
	parse_matrix(MDS_ENTRIES.iter().map(|x| x.to_vec()).collect::<Vec<_>>())
}

use super::{parse_matrix, PoseidonParameters};
pub fn get_poseidon_bls381_x3_3<F: PrimeField>() -> PoseidonParameters<F> {
	let rounds = get_rounds_poseidon_bls381_x3_3();
	let mds = get_mds_poseidon_bls381_x3_3();
	PoseidonParameters::<F>::new(rounds, mds, FULL_ROUNDS, PARTIAL_ROUNDS, WIDTH,EXPONENTIATION, SBOX)
}

pub const ROUND_CONSTS: [&str; 276] = [
	"0x5988cfda06a07f0df8194e3d7d091a856092e94678aa0829c65776147ed67243",
	"0x53f64ebe4d4c21940e5d3b349a6de478747f56cadccd5e52cf480a4fbb3cab17",
	"0x0bd5f0adb65c01aff48bfd03a756509bcf990057ce33328d0a00e533ebc07303",
	"0x3ec0404b965e7894da0d71d96661a7a7d70392f30769a0eb11d64876d1952859",
	"0x2eb359f33d2e93a0f01b2574a87a1b9c9a5e2e0d38a3b031a45356593d6bb957",
	"0x1499c96a2bac923f1bf9e9bbe8817a57bbb12c43bb63ab4f026c912e834e3964",
	"0x24618a4ae619589014f9a355cda00cab1997d7db5a696e36c76cbd0c5c52ecfb",
	"0x30abf1255a1df805bb90350cc4823c1aa07d6717bc5182be340ededd21ac01e2",
	"0x12979471dc79443a8ac220cd73b97987f387fa69f0f5a8db63e1236673f02927",
	"0x5dbe1d8622a3bdfbdcb746ae775e5c887aa80b3724ec23c7e712352b2d28f968",
	"0x51349863b0a83019165490d9fbffda72c597bd4ce691162ef75756d53cc730a9",
	"0x448204fec14d20153d1e1d27beb6e57ac17e0c8a5cb292945c4462fd8df7dbff",
	"0x72f6dab8b2d52dcea64186a5148b139c4f52dedc5c3dbdb5e401cf653114340d",
	"0x49a7730f45ce58bdb0bcf1dc772f542aaba6b8dfc0835464a202435846895e90",
	"0x10e786dbc12944978219554eae5e8e7cf4d386a1c21e2f1fa0cf3c30f8dc329b",
	"0x664ab5c73cb4df46a02a3d0071d032e7ca86df3c71eba51545d481f9029e3a92",
	"0x3264351609a1756a00d06e0760f08f6c241f2b1137c759cbf4dbe345a3fe6697",
	"0x24565f96236be5674370497757b28f75a04ab7b04ffc26a175798d01fa790979",
	"0x266bcdb5d65f750dd7e1c89ae1722c98f96f07df8cd16c9a31718a6b4b3746c2",
	"0x694f3c304bad276f70e73ef818430f40f9980e549d27d69b6e5e1ae53a2bbf03",
	"0x6486c6509ade9d432cda6d50a5f9dc16a829d998e92298953c42d63d1fce141b",
	"0x58a8d6e806766f1e3a45a7b13323336bbdf0b0ace17ab09bbdda13efc4c442a4",
	"0x52025c39ebfe992f0222c3b018e6df85b92d427738cf6da964a65cd759fbb8fa",
	"0x11e41c7490654d2248a14f6bb216b9504d731796c45d771ac032370ec4f9d865",
	"0x62fd0bc210e947113c57ef457db3b467fdb3bc5c385618d5add33f406ef8c7bc",
	"0x2f57c5abe672aaefe89905af533503c261cd820e32573037cd0a1960dc9de83d",
	"0x31fcfbeaaeb988c7930ff8aefb3edac9b5da79f7c78504ddf935a5a661ea7175",
	"0x684b079356e72dd702efba0331ac8199dfed54df6cb8b6b906787d62bb603540",
	"0x10bb9ea277da8237ec78e695ebc28355c62118107048fb292ca3d84e8c2c259f",
	"0x35f6fe54f986ffc1a60dca4d086c42f403390424899a4d1b9636ce3b3745d1ab",
	"0x4cc860dc65aae1289b83293c445f0a0afd61d00b7ae83922ae78fa147eb7b5b7",
	"0x3501153a1999c9b484ed86029cab260009154c932134a42ade4a9a4817419393",
	"0x0404fc37f6218328363d557f8f27417eaf2d7b106f99fb9c7592f2f83f5d28c2",
	"0x403afc13d4d68bbda7d553165387a6b41f379e46a06c63f0a242174e2d3e53cf",
	"0x06c906498d66bd1356d2f733fb1e1e5ffeaff935dde0b756452109c6b1300eca",
	"0x61146d3515b7f5e44a92950eb1f51ac5ab680c17a65e89515dae573080197134",
	"0x1a60779b93ad4c55b3481a828cd4d70574bdc3de172d6249d8e910e49c2a363d",
	"0x6937c4631916a7627c5d6ac922c7337cd1f951f676d5eeae1cf3cee0dc3c000e",
	"0x5b6992155a29bb42363e00cd6840cb5bf2b0faed1c686fff68651cba0eee8b5d",
	"0x0594c35129003e48a0ab7e970e217ee9e2f41c62e8c6c0e723d2e9b36648ebe3",
	"0x351721f2f53444cb10e9490293b83f4c8990c352138010f1cc0c1d0d18366697",
	"0x5115373cc4bc37619fa52406518994992a295ac9135d8e61212a8a28598b3f38",
	"0x2480e887efdd33b091854afaac20c734adce45854e74fbe7d289709c988e37ec",
	"0x377ae2c87c8936adacc250ac425f587aa46b89766d12dfcdc844c7093e89dd16",
	"0x18ffb24b776bc28e1c9627da8dd4bcd394126aaa08e24ab41c5ae09f2077c62e",
	"0x418ed2afaabd7a6cb220cd2476e1bbfd9d01ddfcd431d7e53c86b77cddec07cb",
	"0x184c319f969de4643b492c57151a2ba5acd7a96d441487084022b2464433d738",
	"0x10eaa428c829626504dc9d013c730b13ac03092c1a8e19558e74eb1779562a40",
	"0x4be6c56669cd4a11e5b0802e98ac77a57d556ec2e4a3fc7b723fb0a762cfc323",
	"0x047099c4837d695accf740878ed00f705b19afbb106c01ee1e4d6d87844a87b8",
	"0x5d4fd36e9270fddc37b9bd5d58dc4d86ddf4187150e3b873e1166b5e3c410173",
	"0x0ff9597646beaece1e981985d49cae34a56840b46be7cc58970f2a6c67d4380d",
	"0x565fd832e9ee47132307b6966d7e293ce7f117d057a33636175e20811fbff034",
	"0x64ecfc53fe6a8f5a3bbf5ba68fb829a52b42e8905ff36c2a0ffaa8843881a267",
	"0x040e084e4e964d62eb2971746c22a34531a2d30715b9ea134ec95af098e25e8a",
	"0x0baa6c02713bf12dd4445da8ea48f5870119c86dbe094a0b4178a06c725b9c08",
	"0x4856f4bf875efa4389417698bfe82a5cbbd825a125518c0bdcff671f4e67dd41",
	"0x403f771fe22c282a1c8b9f3294743d8a13d8b96da4a32daf820203a4ef849eb6",
	"0x2822dc0c5bf4d6beabcaa9accff55347e7c02ea5236323e39c228a732d6bc251",
	"0x617c185152fea1f2e777b91ae7175fe51d7ac8ed40f8e30796bead106aa2597e",
	"0x1aef92993d086983d8af883d8e6f6966f39aefb4fecf7d82ef85dbbf6ff82763",
	"0x18defe54d86d1f87ba94f81512dcf6a157644c0bf1c82c16ad4bcb2a71409b88",
	"0x1a8189182f7b29ebf1062c26e0e403dfb1fed3ab5e58ad2ecbdd2a91be19d488",
	"0x2bf67a0f076c657bd09f0acd3221b956f35f7f04d21fee182088971845f93dce",
	"0x3e9d6c6f18feb07d796c3f6206b4331c6a01bb0a9ad9d44a5f84747969742e43",
	"0x2edb035fc529ca54ae10ccc4fd097caeb6d9d209d32cc6978cbdc85e86bbc2d2",
	"0x596fc7a226b17b0f4e2d0c71de6299bbb3fdc2380091ebdd354c474e55da9ca6",
	"0x2bd67721c38f4ed256a38e4bc751ca020c62be5aec5425754b5ed9644a3e3455",
	"0x0a02a55672b747d75f5b3cf2c2b25a2363f1fbf4f537b1da7331364fa2d7a8af",
	"0x60d0c2f98765d91bf342297398e9e69fc0001be58a8f0f3d470d78c3f4f9f4b1",
	"0x1be07aaf15d0eb4b1faa82b9d3becc3c973be1769ff9eacb8a0a761b3424c26d",
	"0x067eab6d6a96a682b893c9c8b7add3c01667327d3c31ab60b2cb287c90816795",
	"0x3af7ab0dd00237281f73d410b3e97712dfe7ecb2746934c1ee573f47e0d23755",
	"0x35f5935e99599183e936000c62528fa4e2f3b2c9d3dd2c786f589be1f88af8cb",
	"0x1b4e5aa733e1fd4d08837bf6b35d942f16f0aa43223b01f883be73361f00ba53",
	"0x5bdc74ab7d583475977cce45e1d65159faddb0c001fc5326078784c68cbe53d2",
	"0x3fdedce56325846753df68e47a01f59328f4ff7c97ed435442aa8cdddbb10b39",
	"0x537f5a5b9a837fa5b7038c60643985665a329b11a7ec50d899f6738f0bc0cbd5",
	"0x1fe7efb91bcee148c0016658abade1b6bd55240945ccd81a3b0b1a48ae5e2dcc",
	"0x069a4c6c9e94b8f4d52c4c811329e32ced5f7039488ae25a7ae8e87d3404866e",
	"0x054c740c379383a458b24a9535091f14485978337064a60dafa0e980b4655236",
	"0x194787a013a794dede4fe45489e5a6ca88637be1e15c59069a56682bb7ddb0bc",
	"0x601464d8a6ff5a72bb6902468552c7536fa5fd074543f46c62f62d511814a7bd",
	"0x0c8358612cc52568b6569fb01964ff8e56f729d9f680dd3ecd6d40adf87af56b",
	"0x1a7d4fe8df060a605b4d527a0a9cf741f99edddfb46f40e17e01ed4d5a8bc92b",
	"0x3984399d8a3202fdf41a332526b37b790de9cbbcf16705a62b5b6eaa93fe2bfe",
	"0x59e0e7f9f43397d874ea920afa65e2748671f24dc499e3b8eab4997983bef27a",
	"0x4020f749926cab8c24cd930c6912b71823e5e1a1072ef850df3b0c0cd153a4a8",
	"0x3fbf7afb19179a96959bccf173c9cfe3b91ef4aa3794615ce00ffe814a7b5208",
	"0x4c532ed5b8539aad7bdd4e426d66d370bf475750cfb8314d3a9584b5e7a98262",
	"0x4887fb024e68355d345cc07a8efe75407318e2cd7144556c13a66dae56f46472",
	"0x671d60bc81de0efc4fa0ef3db7dbfb25deb325f49930626555ad7e42e62c3cf4",
	"0x0aff3a9165fe5740403069ee00f287e74d33b864a8b3efead20abe2bca187439",
	"0x15bd9fb05fdbd0f388930c8afbb77a7e8b88d3c1cc07b676f9d36ab964c586e3",
	"0x1cba898c3b906d20cc6b2c02c3f92de8caf49208a678358b7a93f0779f869de7",
	"0x0d959e41d1820323b1d4ec9c102ef1a0820129a6603b1cfda37e18673aa10fe8",
	"0x56a2c193486a97ceec07c3f0fded6980ddd88baf9df97fd4cf802a722d16ae47",
	"0x2de0d95bfdaeba84599b61d40df92e0748cf9e60b1b7f97fc157841066ee509b",
	"0x5d2224651e4785bf39dbbf009d5232588c417f9c6ea17dd8761dbc57a91cd0ec",
	"0x6d8a32cb01dd24063625c75bae69a7f401d10d136041f4d0dddc7ca4674819a0",
	"0x114c2bfe741052133362fdf60c50b03bfa809211b5687d6d244780215abece1f",
	"0x6f444d87ec937a8448863f65b7e225bcc448a1b2a34124ff937a9a6124b91d4e",
	"0x03b735b1689b01cbb41c1cc5619a85b13c5f3700026853c98eac443dfa9a29bb",
	"0x3a8dd7fd48227c0601a85c268efe9793b23cbfd741b6c599b9aee5b3cac0920f",
	"0x470a0265cae5703a28b03e754d54718408e01f2d52013bf065e9625d49d84ee3",
	"0x351f70de76472a9658a070b9818e47946472538242ba01bfc4ccf03566d9f551",
	"0x47dcbe798565f716c17c1c447851b373252637d3363569521d82f31f49373d6d",
	"0x6377cf2fbb2f1e7b51107913d4d12aa29349bbe62ab12325b4011e328d4c8dfa",
	"0x4f21c91aa65cb16902a2961b556d29acfa955661f3b982c2249a9428c80d766d",
	"0x01cf2ea74a8905e81c5cfb54bd5aca9bbe6d92646117eff23405b6a4cd05d70b",
	"0x09e540e959d1b2f649b05d4760b3f227a1cb92622b3b61f5e968585c684ec847",
	"0x5deaf955f3e034a96382108a4fc9c58afde4f67104fe3a769d4524522c381412",
	"0x3854955cd2a7d0baf0509ac7e81916653324d2b2ef974ed21453967cb7054b5d",
	"0x3bc643ce00efa7a830a17124993b1a21d45c636bd7628c3bc3e9c15a5bbbe6a5",
	"0x477acf6b4e8520571b5e55f6e2d9d7b30986a010a328520083dc4ad8b59470b0",
	"0x4c65d322897137c2ab74f5a0a0dce45280eea75dad74319f6a1358416f3323a3",
	"0x3b31ab64a837280fafeb69d4f445001b0bd3df014f802100ebc12853cfc72e97",
	"0x720ce7e9cc1a0771e5385e59db78fcd0f8b1dbf55f8e6f38a7263633ff037687",
	"0x3af63996b89a177dbb85ebcbab6f4752e9c3a944920d27b4385d265a08ae84da",
	"0x3d1d23cbf279c23f96bf24ca15900e351c7d5c87ac3d0c055d473cf7d3f4662e",
	"0x110f90711168f8e2d9dd4b2f7c2cc12d5abd1b34f873c5feb2b61b9f6e453ff2",
	"0x1a19365650c4ad01bcd0d5f742c5b771f1b48bcccda741c7eebcfe806ea1b8b9",
	"0x2f6cb2c3585c1609069093e226dfbd072bc9d774a5d205ae57da728fe8e79dfe",
	"0x5668b3214d355727858268d2a5f030908c36b799012b504cbd40c0680c812b7d",
	"0x28245b77870ad8c9aa0d5d3b3fb21eca657b6556eb3bc6f0629f462edf44bf66",
	"0x226b53c8cba33bc586bd894eec8eb6c92d41520d25970a3c60808deee5e7a969",
	"0x155cf08e8b1adc03bd14557018de93d768437b5afb0a232542ed43f9052465c0",
	"0x6230034dec1ceffe11587054058b81ce936ecfd8143d2340ff6b9e9393cde8e2",
	"0x0faf92c181f9c928798381d84b2609b1b2de36b136a1c0fb480d396b79a32689",
	"0x47e9dbb74d53190b09fed18fde048a9275e902b85c3df509728df452ba61a5ed",
	"0x2bddb2a7495f5d9b7a21ed67680110a88ea53b6e1cb59265d736c7a19d071782",
	"0x5084064db8d032066721c58ba009ef0214d0e3925081ed558b75d3d0c900c725",
	"0x3cc675425ec468b5b3b1bfc971098728eecff0c47afa2fd1bca0600341668ce7",
	"0x7257c54c1a73482934336eed0db48dce0101786583ecf212a6bc44ee663d610c",
	"0x44a8eae14d4f8ed89aa7cfd0465feca269b16902b39180c8236080a1643ad2e1",
	"0x096d21be543258a0de364555e80cb75ba7cb26c5793d2c418e04ca5adee0ee15",
	"0x721fc4f896c06630048ad3726ed7e30d285778b585d5b15c1c7961a253afe95a",
	"0x45fde7e6f53517614f39d55e68861db9e2bd4aa6d02ba3aa17c8bb3aabda38cb",
	"0x5d288aaa2e6329c3b2f8688ad62e0472890b9df56a3bc1344da43a8a7ca5e624",
	"0x664b3cf94b7b1ddeed624b2199029d35d758529680241d8f49a93df7dd8d42d1",
	"0x2b450b5911a1ed81493cba7f71e31ec4af32822843260419b60a837c547567ff",
	"0x68a138009de34c62daa7b5043ac37c4bd85617537be1b533db8e044dec6d1388",
	"0x4ccd208512e56f28b4212596abf91e36cc42a35ec72e86592385836d903eee7b",
	"0x166dd41e1ef52fa2b43ea80a6589b7b6d17a916caeb4a07370b122a77ba11018",
	"0x390928f1064e82f5d59f58caefe91f23cf68ee0b9139daf74050cc1760924106",
	"0x28e1e3f62796c2c45c6fa883ff78dd187c5133ae301d228b916124c7cff773a3",
	"0x274fc9a73902860e8d4fb856673562a37946eb1e999153653cffda69fc9dab2c",
	"0x6e4fdfa60a518c501774fe603b19beb29eabfc48a9aa12d1060e01f751a1a29f",
	"0x0a46cc40013378507eaa4a576f9d9d4b49c57edc1aa970989ae53e6c17cacabf",
	"0x4888b56738125035ae8b28c2e0e70bf5dfd5f75138eb9750bcfcbc0a763adc20",
	"0x33491ac0609a9448cf74f6c8f23bdd88500ed91d7330b85b5550e304b63ecfa1",
	"0x4312051922d3cc5a7966694ecc7cdb830144d09de187d77b796c6ba5489b9ab2",
	"0x007e67673dbebe26ef3fb5f75b02d8e562b229a28c7bcff92170333d8ce4c28b",
	"0x6a40edb18dd63570fdd7010c84192bb52f130a69259735e46f9af94c572bfcf8",
	"0x1a0df113ffee14c2ad31cc6f005ad7da0d9e7849e88f4af1c55ffede3d330bd0",
	"0x63318a6544d4bac56279941c1804f3c303a966a020345cd2f2b3abbc20c94f08",
	"0x43c76d2e47974caeee205e7de5f06f9016fcb64ce66f508989484ce6049f1507",
	"0x1557c35cfc64844e2ab45038d0dae8f7b03616b4f0aa67ab9ac3511756ec12b7",
	"0x44fa5fca9ca36d28dc5c9be7f68fdb423b01da55db41d705189157f9ce25e762",
	"0x1496d1a33f0d8efc8c3b91a7bba3ec77cd7cadad1b1316c286b0c2a898679ac0",
	"0x63bdfc42fcf5e2262b96ae442ebf4b529d8b8588a1ef651ace4b921adb045e08",
	"0x13644053891cd4a04dfa1d363674817dd8493c0371669deea481b70e83fdd05d",
	"0x141d0eeb719d48bf157753b64cc6523ef7e360e1e238d0204e37a44d637e23bd",
	"0x40feadaae93b93871a50c2fade5bbc6545cd632025bbb5593b81c9e7f46fb0a3",
	"0x6e00e71a854b05906b521545502e34ef6b8e4f99434a9acfef53665211f6ef5c",
	"0x20202aec020aad60abffc6ceaf4927d6b4bfbff75f35830e0e5002e7ed5baefe",
	"0x017c0fc319efa06addf2328d4afb2e8edb543e82ef224aedcfe02d7fac9a1f62",
	"0x1fa3541724fea21a21457487709dd2c3e4c1deccbd5226931feca154aa8ba951",
	"0x202742e15e130cf7a8212212f78db982c2e6a15ceecb5f0f34238cdcf00be743",
	"0x4cbaf07f304ff35d4742cbe9e9da054c7196b1fef605b66a6d83620c90fc422a",
	"0x17635751564e215f66bd04dd7ede293d36b5d1a5bd789c2fbc581fe6c6c28f44",
	"0x25d0fba06855f1e422c10b86af951bf498d22923ce408eb6d4f406784c465da6",
	"0x5173c7c7f5abc20c1b2d3e11756cfb14c7fd47aad4aee5eb1404d53c76a2f71c",
	"0x4b10acb48ec8539bf6924ec45e62f307b196b5e34864e488bbde656d3b53dd5c",
	"0x6a431a22499b87245ebe9aaa5dda231a33fd6e02d0d481347c139d23921df6bf",
	"0x413076c7bd01d3ab40520117aeb4eb01f7c2faee631ec805b213e90a179e902e",
	"0x12fa98d6972d47684be57a95da3d540462532db6173436a489f590aabfc0cb0d",
	"0x090ed921b8aae0c7aef16db1cb5e475838cae6e0e52778ecd6262db306cd44a3",
	"0x397842374cd85c36fe721f006e641d2c0f4b3a0447b13fca317025ed545c0ff8",
	"0x480b069201244ab3e8f2958fc0d1b2e8ddc2d4a7c2e8cd42a905aa98b82b6cb6",
	"0x4e0bfd8a3bf2518e97ac72cee55241afc1158650c54b448cd43fb92e6d5bc770",
	"0x541a39bd2a33b009b2256c4eeedd54476293723e591d47596a65e3a10f54619d",
	"0x4a4f3dc7a8f9b86f74de88b8f7f2d2cd7cc2673d415212e3da4a277cd44c42fb",
	"0x31281e6a27900c09cefd471ccbd7d0b5970479b6aff6deaf40dc486aeac9e20a",
	"0x2d879aec6cb6820af679cbbb6fe42ff948dd9851f6e5177aa677fb1e09a18e09",
	"0x448a7ff59e58e7e66305d3e56605b8c8c8b75bdc1b6ae066e1494b979758edfe",
	"0x6adc516f89102f0fc408190647bdf9f526d50d3eae5ecec41508af9b30ad60c9",
	"0x15b6cf13e37f89be6c04eb49aa110ee49e6bc6a6e01c390b267afa99f8a7ca08",
	"0x10d8b6354fa970139e9f86dfc4d8e5dbb54d7625858b706ca9d57320ce90023d",
	"0x1f1b16844612624b4eb94d3bb5e1624e58f7e3195ef51e968834e93da13dd9ed",
	"0x1b0bdc60d6c7afa7abdaf94ccc5921c404a7bbcab9f6f9f88a62ca04473eb01d",
	"0x6836c9768d26453bd4df5a5bfc20548796f69b8ec318f319733e360cf177b528",
	"0x1196185868a26f45d6254add7786c38228d94bd08e0998e8a15bd76622e07420",
	"0x0983d4c333909860338cea1db3d7430cc908875e1dd44438357dd4934c6f14c7",
	"0x36e346c5508e54413ac1e0de6eb9f1c5f84a59ae907bb5ab090c52ee05e1a0d3",
	"0x1c42b0e4773232f24b5e1ff3dbc2b992ed8f7a3b22fb56ef7a9db865814a9e3f",
	"0x445a3e0f6a43b4b6207a0ba8e1b77e05308f0ff1d73209310a35e14ccc56a42e",
	"0x6c39b1a595576e4f2e491fa45e19a478db6993730df79defe25819e737c4bc16",
	"0x025a698fb0e85ce25f82fb1ca3cb8451ba2ce67ec9567dab15904f87d8f5342a",
	"0x22b1c286433398ff9aa211c9b017acd78c68ccd5f44b7cb0b5107987aefd5127",
	"0x2c32b970166e9764be0a02e85df9de82f796f128b488c16ec901ec8238f0b549",
	"0x7295f42a982c0322635311bb63f73739399a77ba2e86b26fd31035fe4b4fc267",
	"0x3fc26e03c70ed03c2ae945e349390582a7ad6f969e4f102bebf699c7cad7f51a",
	"0x3ddb4e8ef64e62179de1ebf821cb0f6824c054b53dfb670d3f8a30183a8f88c0",
	"0x068f2c4788abb034f993bf62748afae0971a94e59687eb90a4193a8c44a8e8d0",
	"0x511fb8c084d3e695dc34a28a0920144872b233a6a69f099124baeca2ded9bcdc",
	"0x394e8b5b74aeae6d09dd51d46606ac5d724cd932709fcba28869efef43b44302",
	"0x38ec207abe41e7ef0e627a51a9090ceb35a4d6cfb28174f37a949664ee253f2d",
	"0x65f7481e0cacff17a1a7ef2821fb55fd48b2150f3f785d6ad1f8891c81426d4d",
	"0x5ef181e913b5b879e03c5f3d28cd16de74c5f06b9acee038f8b50e63619b0d1c",
	"0x390555693190279035ea63de5d7f3f1e9006deb93f05176e7e572ef2b5271b46",
	"0x453296b0eaa2f51ecdcbb3fd7bc01595f5b0367fd6632e6f900ce3b7dcd8072d",
	"0x0d707863030913e538becc99ac4dc74bc36563962560df94e7ebc1ea6a301d33",
	"0x1b3db53c40bc6ff29cfe6670fe8ac503814ab3f1f71792e99c71ff9758390ebe",
	"0x383548be8a6d247bca0be00d7d064a6789927ce956401dd3d1fcbc2859f9aaef",
	"0x71ac84a2213a2d9bee9f8ca97d1a7f785c5c94c157cd714de9295fdd7972f0eb",
	"0x56a724b56bfbb8e60b176b7871a258d614355f4af3458850c7956930ecb1d369",
	"0x51cdd1fe438c7d915bd4d28d446d3faeb0e119d9bb6ff9c813551773d03ee469",
	"0x66ce3afceeefbc63db6af2bad33ba7923b84f85ce76879d1cbe02728ed565838",
	"0x681543325fed4a089779aec97acdd39695c831c9a7eebf45e27909fcc57c8db5",
	"0x4ac4959271c3817347645ece39b68035e5fdaae9fde723c8851f53b6e4080ba3",
	"0x10bf602488f56ffaa6903a1a9ccf21088edfb0817420d66ad6b0fc57b4dc3ad4",
	"0x071792cc91d74eb70dc8ea01e467d9428b690ea04eb2061a5da49df52cd2aa74",
	"0x1cd877e45cad6c0e12ce59e88f80a530f74fb23e2407a33b9f346e3c2027e063",
	"0x3af4f4612b76581829bb5532978d732cac2ea7f8daa8bf7e997b51a409d892ea",
	"0x14faf155857728dd172b7c4ba5dc5143cd1a8a4f369d1e1dc31c3b2a928d1cba",
	"0x0b6e3749df3ac15267b834b97259c07eba6970d0c5666ac365ae102b80fc47fe",
	"0x7062095d4827814dc18743e5f4082af456193489388fc9e7143711d789b38069",
	"0x57d5ebc23010cbcc03af8193f00391652faa32ad02124765a9f2b8937bdf107f",
	"0x591e0b8ef679b3a7ad017cccb2e3b11156c966c7d620072ad174d20aa025698b",
	"0x1eddc1349a4b1a3aa47876ba786ba18392f604c1f11b8c851ac104dbb98c6324",
	"0x4703a99946f628d25769fdd4148975ee1c18a88365eaaebfabf6a6f110751188",
	"0x57ca3067a284de7e12931366f2beb400a3fe2a98046c0e03771c3f517745d456",
	"0x6807ebb2efec2e24a76cca575002c182349ec93cb33f54c7114d32ab0003104f",
	"0x666739e29e13a1db088b0ce0c06c0dcfa663299213ccce0c1e413efe4984124b",
	"0x5058c86d1c02b8bfecb1d41a4aaf59b701f18c53c44a6a4c270832b9ffc70777",
	"0x260d9fad65ebb5ac4f255ed29c12273d43c4b5cce2727ceb0c08eb571fa3a8e4",
	"0x60907f5efbd1453efbebe8b96f1f064f517c67b68a994f018e774647caa6b9c7",
	"0x0e2b7511af559802691913266078796ce260a2a8f7cb44a84b9c4e3a1d058e68",
	"0x6c7430939d58c99a6f16f8c34bcf077f871218f0bf231a38bb0b465006f1e94d",
	"0x2faef7633531a58bd2a4758172760e8a9de919d7c6642e1e1b71a9ff031b3971",
	"0x3d2eed987b9272381e58d350f9219ed1be4c593af4b0cbc0268ae856fcc0bf28",
	"0x3e0dfc1a148dabcc9e2dbbf855e343da1cd7d379d98e8e8f05cf17e2ed901a3b",
	"0x09133bab77ab28ce8b1054db9363651eeee583e6be1bb1107a6b61a9e5e7724b",
	"0x4375b6e7819875511050703c099efcd02c0b3475c4a9105fc06b4570b8c95f63",
	"0x50ee28b14d025bd56e7b298533b32d1ab330aaa30b134dbef842b369088e8766",
	"0x2324aff50bbb8371818a325396abe74742b4316f62cbf702913e4a05f7ef4b56",
	"0x41eff9df4b1a955ee3c938ef106897da8ded0857e6e563b5ed90ec97c1235275",
	"0x0f51d51d47577851ee470420fdefc0fafa3e67557779841bcd57cc30dac05403",
	"0x24ac189dd05d52fc76affe3e951892763a411ed49205349e47e078cd724fe000",
	"0x58019841e5083befbd87ed0ce42094046c3d56598d353b52e2393ff2b0ab672a",
	"0x30dc038229008a6d704cf36f488bf50e03fe3cc3f9367492b2a76dcea48a1fb4",
	"0x030f91545c598ef66e56bc27a91d1a843cc2422ba7943f6e1c4c8d2624f657f9",
	"0x5a83a6cddc093a6605c8802e13d7c3df64ff67503e3776d0df4d14cec11b947b",
	"0x4f5f962af26165ead7527df2928972bf8448e6adcce6357c066023751540fa27",
	"0x41a016d36256530af94f694c889f7498e8c5ab90d8b06dab7a124f98d1d1cf10",
	"0x21b75405f5efe55c9d7d4fb12f9edfff84c2919bd53207a4b0e34cb0aacfee59",
	"0x0d858b072f9e9a20b305808320ae991dfcbcfa769ccd7eb3114b2dc14f925ccd",
	"0x52059986ae34bf5026f429a0fb917545ed02d2c30b9fedfc4cfa17b4070e9fe7",
	"0x28b8d5c4c7e030b77fb0ccbafe43cba37518ab0f29bdeb8564fc44bc840488af",
	"0x210e52cad14fb6e4711ae194d94c06fd518c9129495c2e139af7fb974e5cf881",
	"0x294ee3bf3df5489eca8250d4d6e3aa57bc0eaab1514595528a8bc1df9aae958a",
	"0x2cf97551e6a5d33752317a875b2fe8ce319a49a52ca45f46966713c9956d2273",
	"0x68d866606fe93482e7f5800097b78e06dd5f57da82a8fbbadfeffb3baf4186ff",
	"0x0cdbd342ea09c29c7a244fe7be6349bd259867ef7b629a1f994324e750850997",
	"0x00ed7b61ffa91c50279004a317e7a17c39e2526889e43d48f5b8026033ed07bc",
	"0x4252063dbee27d0fb11237731d867c5c16e8a59319bdfade60e580493fffdb3e",
	"0x702954029fa3492a7cd9e4a0366e06e7f2878a6e459646ce98c5c395dc21f0af",
	"0x054aeb2f7ae07f47127f427bb115a2282c2f13c96e38a4f1f894d2382babb9b9",
	"0x35e9f1fb16c110076e61ce420146245499df52bf58d3ee265cc8b6d602284c58",
	"0x3d9280aeff9f20fd4219a223d24473fb959381aa21b44b6f0d760b53d192388c",
	"0x0849a98eb43cc4d7a6b17bb66d2c8f0f5373b36df8d96703501d38216c500889",
	"0x09cc3629b5e40feb6aaa425c7d5e981cf4bf2241658b9a8fa876d84fc8125f07",
	"0x0b537594f58cc49d9e1319dbb56502e94979fcf4d1481cd0b621bb67af34854c",
	"0x0e8a4b4ecf9d67e931d4362c6c1072846dcf77e8451fc570f144e0c87ad73e5d",
	"0x561490a9581833118ffdb18dc1a9728ccea43f87ebd49f7be4d6f0c6f4c02936",
];
pub const MDS_ENTRIES: [[&str; 3]; 3] = [
	[
		"0x2946cd7551a7116498e9636ae2b998e3a34c52f176f8b7a1c41e2e95f57180f4",
		"0x023b2d46ae42096d9c621e1475c715944dac20aa1f9b85a8c70e248c05372602",
		"0x2c5a6c3655dcdb9a15ff4624cd932ffb4f5144a52e1c918a523049150f197c68",
	],
	[
		"0x3a578a995291e3deab1113569316ad473d159212fc744a45bce58799cb1a4a3d",
		"0x217e489e37567c644617d8a92d3190f78ee2b56b6c429c3d241033afb7d11d68",
		"0x3a820ef90615e444d25f577659237155f028d55303122925cc241daacebf4669",
	],
	[
		"0x061fa4f13da70140daebcafb1a118748f7fb83c6ccfb2ecd783bab55307bea21",
		"0x2b1f625626fc599b95d5c934fb00bd6442279675fec3c123613588f440eb814c",
		"0x6065db3854a19fe31a5576b49e0933f0b695018d25a4117ac17d326553e7ec32",
	],
];
