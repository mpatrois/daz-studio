// {todo} `NOISE_TAB_SIZE` is useless since `NOISE_TAB` is an `array`:
// you can get its size with `NOISE_TAB.len()`. `array::len` does not
// exists, but when you call a method on a type, the compiler first
// checks if an inherent implementation is available, if none is
// found, the compiler will then sometimes try to convert the type to
// another one that might have an implementation for this method. This
// is entirely done at compile time. The conversion from `array` to
// `slice` is hard-coded, but there are other automatic conversions,
// most notably via the `Deref` `trait`:
//
// https://doc.rust-lang.org/std/ops/trait.Deref.html
//
// It's not the same thing but you can also have a look at `From` and
// `Into`:
//
// https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
pub const NOISE_TAB_SIZE : usize = 512;
// {todo} You can easily generate and cache such a table with a
// pseudo-random number generator. If caching this at startup is
// really an issue, you can use `const fn` to run code at
// compile-time.
pub const NOISE_TAB : [f32; 512] = [
-0.25007062135201713,
-0.3495725435116406,
0.3257733499100047,
-0.29320285107440625,
-0.14282538745951748,
0.16811426916557004,
0.29610631421552924,
0.006078594090128276,
-0.16766015577192317,
0.27995380143243187,
-0.27443930587157006,
-0.25562827220591533,
0.11038616514393743,
-0.38190314007523396,
-0.0651315544164619,
0.11950197961395928,
-0.3050136631606716,
-0.1873689462203486,
0.12803589397473222,
-0.36995858846576357,
-0.2589726288802354,
0.342759920089495,
-0.21945161570548236,
-0.2917654789076112,
0.25197300578102877,
-0.20441804220063695,
-0.1395811586574129,
0.38823138255368983,
0.15421378171849265,
-0.2398162642744664,
0.10846853844310145,
0.07924134750052758,
-0.03371373925588568,
0.24258387688957317,
-0.1559023336486464,
-0.3114559431045738,
-0.060109142633551785,
-0.3739612042738636,
-0.3076636767393177,
-0.18138995409217373,
0.3648816429020696,
0.3804712245656764,
0.14551663426572822,
0.22273705933764215,
-0.14419854874230809,
-0.18674628650249867,
-0.34641290596128516,
0.09495383508147617,
0.26167855278091584,
0.3555130852586612,
-0.32094877289128865,
-0.37418615301127933,
0.1689857049260186,
-0.18157855165392367,
-0.187327140032775,
-0.385173068622834,
-0.07757076155245396,
-0.241297061572022,
0.3509112723352344,
0.04869061097437628,
-0.2867624284806055,
-0.31924698760134773,
0.007930966073283674,
0.35503291202163206,
0.032285732007060024,
-0.32328263670014507,
-0.2174418298016123,
-0.16229315449865778,
-0.3422562662118611,
-0.3038275662609676,
-0.26964091999232287,
0.38883859617513533,
0.21767594174416965,
0.16589998585808552,
-0.04692779629229032,
-0.21159381361838295,
-0.22961915029377128,
-0.2682923113441636,
0.33516399160052335,
0.06068509088367598,
-0.08382145141292213,
0.36150828308795346,
0.29259813439591603,
0.11893816331924985,
-0.05120000506932128,
0.11758100487811096,
-0.07419414573953223,
-0.1495438919092691,
0.06586328209860626,
-0.1707973573391887,
-0.33766985868153826,
-0.15856631578889357,
0.2694483269453133,
-0.19111825632120982,
0.27517283644733065,
-0.16830670037310483,
0.0487936931355669,
0.36099896220608496,
0.27348008973161964,
0.35360532760034613,
0.2084644755627091,
0.13197859925109134,
-0.018089607618653636,
0.31238125475096723,
0.3428462625677646,
-0.14592455693998407,
0.18456945162460234,
-0.16380605161116912,
0.39268278088894926,
0.34897002948043676,
-0.0414485312761713,
0.30814386007709954,
0.04623855724155193,
-0.21781497509380535,
-0.28618058986441836,
-0.24620995787909875,
-0.08840331797433235,
-0.2284740709378065,
-0.20115541845501814,
-0.2980164038792039,
-0.21933033307025773,
0.24523912068022047,
0.32155513897770344,
-0.22982692984088554,
0.33688361769899644,
0.20308310999455936,
0.23362276396902193,
0.3827650557795934,
0.26115073379552617,
0.2995990907818894,
0.11852303687649508,
-0.13082210268555938,
-0.30535059398988934,
-0.3352429379552765,
-0.025683690002853733,
0.12866019995499328,
0.2990888197286976,
-0.3397236129719513,
0.013682933692774846,
0.047472998757096097,
0.1198774130758618,
0.3917521462005422,
0.17990528424339647,
-0.07904651768451272,
-0.3670299975495938,
-0.059313971512646724,
0.36107284642079024,
-0.29512709164242434,
0.24748010638747747,
0.19862884654253,
-0.09375471150468254,
-0.34082722510574875,
-0.39990878834571597,
0.057923793931982104,
0.2289124845868079,
0.3536475303569804,
-0.04354976642101445,
-0.27243310090838563,
-0.3499610866100414,
-0.005530409179451291,
0.3628616333599207,
0.09553266756936313,
-0.0013399594270731897,
0.038126511219872275,
0.04174455215773834,
0.042930033336567246,
-0.06282920330659607,
0.215115147127749,
0.2955247590488526,
0.13964740850335913,
0.13365386258725806,
0.22057822911920544,
-0.37926483768502645,
0.17779453396118505,
0.3175637225517688,
-0.22685042378872422,
0.12429087430777873,
0.37953388902782087,
0.11270691582718402,
-0.3065304524453879,
-0.07064925434207932,
0.3636330792626554,
-0.07185247232546606,
-0.21280208332316097,
0.01971517587766458,
0.2739327348434168,
-0.15321101857360098,
0.06833028323336365,
0.240807784473743,
-0.11097960553809379,
-0.3959385595817169,
-0.12751789489067197,
0.35974682217495824,
0.31407281883891497,
0.00015502182396271992,
0.06423021810020071,
0.055315869279248725,
0.17520722501953656,
0.1346325328660177,
-0.1914394930297593,
-0.2344767292717491,
0.3912505856392408,
0.1279479019029679,
-0.23074281235522412,
-0.05260096666687417,
0.2669993567883521,
-0.09083972558960302,
0.24805597657444972,
0.200060285750899,
0.3446036753983415,
0.2211016865205981,
-0.2472384462148355,
-0.026077598810099235,
-0.3972486918764591,
-0.34798239215513654,
0.2985350482951668,
0.38277490252404106,
-0.38321600374065756,
-0.30197657605993544,
-0.0866980831543915,
0.011734680836946822,
-0.16397185157308736,
-0.08974619502997215,
0.3362776354780981,
0.38036034636402444,
-0.29327226552669233,
-0.06932750449194995,
0.11020097823261535,
-0.3814884470264805,
0.15349462233420172,
0.3250683895683237,
-0.10230318983531053,
-0.3796821990317927,
0.2245538998625464,
-0.25887500299981603,
-0.37206901967841544,
0.26572642777745453,
-0.010747707186790923,
-0.1669996879247072,
0.15981277743751265,
0.2992621961072633,
0.2715155380072366,
0.39647878886228893,
-0.2370976089978885,
-0.20638136484827682,
-0.13082448374423858,
0.3849427037603645,
-0.274337270602322,
0.028109632171613663,
0.31481778134524535,
0.10150287718137539,
0.350459989614496,
-0.367077913991721,
-0.39430853955142703,
-0.30064970935315427,
0.011777895845522846,
0.13127468225948968,
-0.3706790984475474,
-0.172673479033628,
-0.17212437893824373,
-0.33326297307190056,
0.38313592437731714,
0.04394189623382188,
0.06213028452621963,
0.2860067828834011,
0.12961853490466302,
0.1417142726870236,
0.3195172866894512,
-0.22555478805588713,
0.10970336618153823,
-0.030132517308666975,
-0.35170770090815856,
0.08128465600670949,
0.2659396138325715,
0.09159073610311347,
-0.021912927998726375,
-0.14302164207497792,
-0.3157542801809224,
-0.3290394600391342,
-0.2937798922804523,
-0.17459708199962154,
-0.36973116021018404,
-0.3592150153455999,
-0.12933844044691495,
-0.3163054915735134,
0.1934818988618191,
0.20003790237368754,
-0.027345237304992588,
-0.15962017407194884,
0.15107801634205958,
0.05513877966682604,
0.06461000929226342,
0.23679181339873634,
0.20634154186903608,
0.14872108172502438,
-0.1243797142480422,
0.32720288570191713,
-0.08984952278626204,
0.09745064402770498,
0.34165166797760327,
0.2300123782040725,
-0.04701499148605093,
0.11712159994592444,
0.1812075561201812,
-0.3846982096834967,
0.342622627492301,
-0.18790739684305224,
-0.32549907377740195,
-0.16718414820043465,
-0.2156848023895376,
0.0988894655074522,
0.3227124134090382,
0.15517232352460006,
-0.1743984417506529,
-0.03490263908457072,
-0.38441264649884,
-0.3869270056172711,
-0.11299096017103621,
0.27622978112618485,
0.05836265727314998,
0.015135342126599305,
-0.3781734381325648,
0.11435818579910301,
0.3124136731466106,
-0.33592209943118145,
-0.30387580643221446,
-0.27645624748233427,
-0.08405369276122424,
-0.2047223872409279,
0.3892182269181751,
0.347885802101048,
0.08013007350315947,
-0.02579886771956477,
0.021556566616136853,
-0.23196196169206865,
0.14874948839937183,
0.07599006800456137,
-0.1137310817642891,
-0.2571333868066306,
-0.10522778616905396,
-0.13788332333729678,
-0.23432228611826356,
0.335183503598621,
0.16261134864099214,
-0.09639798419545778,
-0.1096762458649022,
-0.1924615523430295,
-0.32174970726856217,
0.2267483133005353,
-0.30352224320864724,
-0.22375657228182508,
0.17135567394354775,
-0.09872063462319432,
-0.17378917603853325,
0.0763421569586964,
-0.3088307626016551,
0.25992112744196455,
0.31267846691972945,
-0.2878142003748218,
0.03992665228713657,
0.019679112678521363,
0.25135059994781594,
0.28872401326988095,
0.12472833237696182,
-0.037827751381537,
-0.13642804397998906,
-0.3910807810347176,
-0.002449734777277435,
-0.27356550739740965,
-0.37322966332902463,
-0.2706502021430049,
-0.008541293559167907,
-0.3774388696994867,
-0.21548395235028336,
0.1329143984174502,
0.321714783494709,
0.2896355980114815,
0.36138238917711574,
-0.25069512953152706,
-0.18608940287710488,
0.28236921014033195,
-0.04681537577238762,
-0.087676288349101,
0.21446970230453957,
-0.24308732025954316,
-0.12322408333052337,
0.15754459322711548,
-0.09296477131846391,
0.32914225097104327,
0.2351136876970323,
0.2625620842244635,
0.14470664813776488,
-0.20124098469535437,
0.30748302911795733,
-0.11435618685549441,
0.24930531591897387,
0.2531468611705099,
-0.2342986714875706,
0.28400718380505613,
0.13375269497462547,
0.31655038929474,
-0.286091659802432,
-0.1582623713173598,
0.09630753629586009,
0.3667851791563795,
-0.16039363755289654,
0.3581523253537937,
-0.3148799773585793,
0.05649195996995063,
0.3157150208605559,
0.3498600337648507,
0.37000261878304536,
-0.23489960435863322,
-0.09790472357879235,
-0.22466262616332192,
-0.028566599538274364,
-0.18602807565445736,
0.23446626448720168,
0.17160702003726538,
0.3188418956187892,
0.326256034655009,
0.22549275324441598,
0.17018856515037106,
0.1745543545785444,
0.09602613394690543,
-0.17328892718297573,
-0.043458925825070785,
-0.017969400054037046,
-0.33799804956528845,
-0.09953801573367715,
-0.3932955838128958,
0.22072674127001807,
0.06699788926261392,
-0.261592122987201,
0.12872845736653837,
-0.31538319354306354,
-0.017345951095118297,
0.38327329795694093,
-0.2325365251876395,
-0.3670913221521829,
-0.3586204036506049,
-0.08681576266128169,
-0.3854708221295096,
0.26045119583449433,
0.39921891048808433,
-0.20602614390570723,
0.34162726200493027,
-0.3484996972577043,
0.17846804681729356,
-0.04949317711355339,
0.3606326016694381,
0.12694306313353226,
0.01897835949391502,
0.0037942964756303432,
-0.14947688557574015,
0.3540528175337954,
-0.18672509348508962,
0.38833095746327334,
-0.22159667008269165,
0.22610272296556647,
0.01074590540755005,
0.06882192380653628,
0.07613685397438355,
0.07716596183386706,
0.007244775494886513,
0.09510307362774123,
-0.29594404680717934,
-0.39330196864415307,
-0.37732809121297134,
-0.23549506308405746,
0.08812851585729634,
0.0799910752490562,
0.3230938757146606,
-0.19835234829224502,
0.057369386936432724,
-0.15478916527152592,
0.3021518712315742,
0.1560675807554402,
0.16040427956436185,
0.23821128667728222,
0.2091467921673832,
-0.2978743174192293,
0.1184233586651887,
0.0067240764533662125,
0.23620774079398243,
-0.13127789939136095,
0.3684040930455658,
0.21018729340178202,
0.31234184475159193,
-0.39767191666475354,
-0.2900783959902786,
0.3677589158208494,
0.24103690285631962,
0.1463853403085986,
0.380978367234482,
-0.21503458307813164,
-0.24493452399686572,
-0.20805863448972853,
0.23980798275511825,
-0.07346494088697564,
-0.2630127342524036,
0.03528060219195624,
-0.2640424873006108,
-0.25406329172453135,
-0.38992587261421635,
-0.25603650126993927,
0.04041933652065502,
-0.24259171695665108,
0.3266542660927574,
-0.14529300164362877,
0.066610152298979,
0.3130507430724122
];
