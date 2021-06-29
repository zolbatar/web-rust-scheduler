#![allow(clippy::all)]

pub const SCENARIO: &str = "
SchemaData(
  activity: {
    \"A1\": Activity(id: \"A1\", lat: 52.895461, lon: -1.702781),
    \"A10\": Activity(id: \"A10\", lat: 53.157758, lon: -1.76056),
    \"A100\": Activity(id: \"A100\", lat: 53.302244, lon: -1.540919),
    \"A101\": Activity(id: \"A101\", lat: 53.003741, lon: -1.60364),
    \"A102\": Activity(id: \"A102\", lat: 53.111961, lon: -1.713595),
    \"A103\": Activity(id: \"A103\", lat: 53.362528, lon: -1.962575),
    \"A104\": Activity(id: \"A104\", lat: 53.060147, lon: -1.656907),
    \"A105\": Activity(id: \"A105\", lat: 52.977311, lon: -1.635001),
    \"A106\": Activity(id: \"A106\", lat: 53.093224, lon: -1.608594),
    \"A107\": Activity(id: \"A107\", lat: 53.491053, lon: -1.956725),
    \"A108\": Activity(id: \"A108\", lat: 53.328724, lon: -1.765085),
    \"A109\": Activity(id: \"A109\", lat: 53.436296, lon: -1.815088),
    \"A11\": Activity(id: \"A11\", lat: 52.954781, lon: -1.807161),
    \"A110\": Activity(id: \"A110\", lat: 53.090331, lon: -1.779308),
    \"A111\": Activity(id: \"A111\", lat: 53.256376, lon: -1.642303),
    \"A112\": Activity(id: \"A112\", lat: 53.226335, lon: -1.426915),
    \"A113\": Activity(id: \"A113\", lat: 53.235168, lon: -1.501145),
    \"A114\": Activity(id: \"A114\", lat: 53.342574, lon: -1.825946),
    \"A115\": Activity(id: \"A115\", lat: 52.946245, lon: -1.647704),
    \"A116\": Activity(id: \"A116\", lat: 53.30077, lon: -1.952303),
    \"A117\": Activity(id: \"A117\", lat: 52.969088, lon: -1.334645),
    \"A118\": Activity(id: \"A118\", lat: 53.335003, lon: -1.423296),
    \"A119\": Activity(id: \"A119\", lat: 53.258822, lon: -1.871336),
    \"A12\": Activity(id: \"A12\", lat: 53.406814, lon: -1.741654),
    \"A120\": Activity(id: \"A120\", lat: 53.131492, lon: -1.46502),
    \"A121\": Activity(id: \"A121\", lat: 53.170703, lon: -1.409659),
    \"A122\": Activity(id: \"A122\", lat: 53.369993, lon: -1.67112),
    \"A123\": Activity(id: \"A123\", lat: 53.289086, lon: -1.776838),
    \"A124\": Activity(id: \"A124\", lat: 53.109431, lon: -1.770752),
    \"A125\": Activity(id: \"A125\", lat: 52.933132, lon: -1.785185),
    \"A126\": Activity(id: \"A126\", lat: 53.006136, lon: -1.689555),
    \"A127\": Activity(id: \"A127\", lat: 53.188072, lon: -1.589908),
    \"A128\": Activity(id: \"A128\", lat: 53.236173, lon: -1.286784),
    \"A129\": Activity(id: \"A129\", lat: 52.928848, lon: -1.686538),
    \"A13\": Activity(id: \"A13\", lat: 52.824353, lon: -1.459144),
    \"A130\": Activity(id: \"A130\", lat: 53.343623, lon: -1.660273),
    \"A131\": Activity(id: \"A131\", lat: 53.292066, lon: -1.629404),
    \"A132\": Activity(id: \"A132\", lat: 53.460162, lon: -1.92956),
    \"A133\": Activity(id: \"A133\", lat: 53.201532, lon: -1.846235),
    \"A134\": Activity(id: \"A134\", lat: 53.32616, lon: -1.312491),
    \"A135\": Activity(id: \"A135\", lat: 53.384027, lon: -1.826467),
    \"A136\": Activity(id: \"A136\", lat: 52.792122, lon: -1.475579),
    \"A137\": Activity(id: \"A137\", lat: 53.21477, lon: -1.783971),
    \"A138\": Activity(id: \"A138\", lat: 53.047506, lon: -1.599418),
    \"A139\": Activity(id: \"A139\", lat: 52.724984, lon: -1.560515),
    \"A14\": Activity(id: \"A14\", lat: 52.822398, lon: -1.58857),
    \"A140\": Activity(id: \"A140\", lat: 53.331498, lon: -1.919278),
    \"A141\": Activity(id: \"A141\", lat: 53.128405, lon: -1.690844),
    \"A142\": Activity(id: \"A142\", lat: 53.047964, lon: -1.385388),
    \"A143\": Activity(id: \"A143\", lat: 52.830612, lon: -1.396891),
    \"A144\": Activity(id: \"A144\", lat: 53.32442, lon: -1.673996),
    \"A145\": Activity(id: \"A145\", lat: 52.765984, lon: -1.513818),
    \"A146\": Activity(id: \"A146\", lat: 53.131835, lon: -1.640547),
    \"A147\": Activity(id: \"A147\", lat: 53.124498, lon: -1.777206),
    \"A148\": Activity(id: \"A148\", lat: 53.330363, lon: -1.992963),
    \"A149\": Activity(id: \"A149\", lat: 53.203068, lon: -1.238474),
    \"A15\": Activity(id: \"A15\", lat: 53.196949, lon: -1.497222),
    \"A150\": Activity(id: \"A150\", lat: 53.174016, lon: -1.417788),
    \"A151\": Activity(id: \"A151\", lat: 53.280986, lon: -1.99321),
    \"A152\": Activity(id: \"A152\", lat: 53.368666, lon: -2.017387),
    \"A153\": Activity(id: \"A153\", lat: 53.26186, lon: -1.889077),
    \"A154\": Activity(id: \"A154\", lat: 53.204186, lon: -1.690539),
    \"A155\": Activity(id: \"A155\", lat: 53.367123, lon: -1.696538),
    \"A156\": Activity(id: \"A156\", lat: 53.344521, lon: -2.001349),
    \"A157\": Activity(id: \"A157\", lat: 53.420076, lon: -1.992052),
    \"A158\": Activity(id: \"A158\", lat: 53.281417, lon: -1.947929),
    \"A159\": Activity(id: \"A159\", lat: 53.056457, lon: -1.567071),
    \"A16\": Activity(id: \"A16\", lat: 53.378869, lon: -1.987709),
    \"A160\": Activity(id: \"A160\", lat: 53.149712, lon: -1.369614),
    \"A161\": Activity(id: \"A161\", lat: 53.477366, lon: -1.932357),
    \"A162\": Activity(id: \"A162\", lat: 52.818869, lon: -1.413796),
    \"A163\": Activity(id: \"A163\", lat: 53.222625, lon: -1.494977),
    \"A164\": Activity(id: \"A164\", lat: 53.168128, lon: -1.555829),
    \"A165\": Activity(id: \"A165\", lat: 53.239681, lon: -1.205787),
    \"A166\": Activity(id: \"A166\", lat: 53.053485, lon: -1.403755),
    \"A167\": Activity(id: \"A167\", lat: 53.28622, lon: -1.798781),
    \"A168\": Activity(id: \"A168\", lat: 53.353646, lon: -1.717227),
    \"A169\": Activity(id: \"A169\", lat: 52.901794, lon: -1.784895),
    \"A17\": Activity(id: \"A17\", lat: 53.091206, lon: -1.601057),
    \"A170\": Activity(id: \"A170\", lat: 52.927151, lon: -1.331099),
    \"A171\": Activity(id: \"A171\", lat: 52.968995, lon: -1.641415),
    \"A172\": Activity(id: \"A172\", lat: 53.232371, lon: -1.609568),
    \"A173\": Activity(id: \"A173\", lat: 53.147449, lon: -1.686576),
    \"A174\": Activity(id: \"A174\", lat: 53.4107, lon: -1.713109),
    \"A175\": Activity(id: \"A175\", lat: 53.057276, lon: -1.456752),
    \"A176\": Activity(id: \"A176\", lat: 52.952174, lon: -1.35175),
    \"A177\": Activity(id: \"A177\", lat: 53.148252, lon: -1.531846),
    \"A178\": Activity(id: \"A178\", lat: 53.140294, lon: -1.673649),
    \"A179\": Activity(id: \"A179\", lat: 53.001331, lon: -1.560815),
    \"A18\": Activity(id: \"A18\", lat: 53.269399, lon: -1.687727),
    \"A180\": Activity(id: \"A180\", lat: 53.420637, lon: -1.801242),
    \"A181\": Activity(id: \"A181\", lat: 52.74258, lon: -1.657078),
    \"A182\": Activity(id: \"A182\", lat: 53.290278, lon: -1.76266),
    \"A183\": Activity(id: \"A183\", lat: 53.119458, lon: -1.481837),
    \"A184\": Activity(id: \"A184\", lat: 53.238248, lon: -1.28768),
    \"A185\": Activity(id: \"A185\", lat: 53.262886, lon: -1.397974),
    \"A186\": Activity(id: \"A186\", lat: 53.216243, lon: -1.218082),
    \"A187\": Activity(id: \"A187\", lat: 53.193009, lon: -1.372517),
    \"A188\": Activity(id: \"A188\", lat: 53.400516, lon: -1.719402),
    \"A189\": Activity(id: \"A189\", lat: 53.2324, lon: -1.433419),
    \"A19\": Activity(id: \"A19\", lat: 53.265344, lon: -1.814759),
    \"A190\": Activity(id: \"A190\", lat: 53.299376, lon: -1.226383),
    \"A191\": Activity(id: \"A191\", lat: 52.956144, lon: -1.450832),
    \"A192\": Activity(id: \"A192\", lat: 53.260321, lon: -1.233531),
    \"A193\": Activity(id: \"A193\", lat: 53.354296, lon: -1.926996),
    \"A194\": Activity(id: \"A194\", lat: 53.295993, lon: -1.722853),
    \"A195\": Activity(id: \"A195\", lat: 53.492103, lon: -1.828135),
    \"A196\": Activity(id: \"A196\", lat: 52.849991, lon: -1.411366),
    \"A197\": Activity(id: \"A197\", lat: 53.064366, lon: -1.780321),
    \"A198\": Activity(id: \"A198\", lat: 53.236873, lon: -1.56818),
    \"A199\": Activity(id: \"A199\", lat: 53.380292, lon: -1.925854),
    \"A2\": Activity(id: \"A2\", lat: 53.017264, lon: -1.661276),
    \"A20\": Activity(id: \"A20\", lat: 52.883217, lon: -1.746418),
    \"A200\": Activity(id: \"A200\", lat: 52.889563, lon: -1.271449),
    \"A201\": Activity(id: \"A201\", lat: 53.303507, lon: -1.903997),
    \"A202\": Activity(id: \"A202\", lat: 53.435866, lon: -1.9772),
    \"A203\": Activity(id: \"A203\", lat: 52.885656, lon: -1.420787),
    \"A204\": Activity(id: \"A204\", lat: 53.474378, lon: -1.910913),
    \"A205\": Activity(id: \"A205\", lat: 52.988367, lon: -1.595523),
    \"A206\": Activity(id: \"A206\", lat: 53.143872, lon: -1.601927),
    \"A207\": Activity(id: \"A207\", lat: 53.268901, lon: -1.819037),
    \"A208\": Activity(id: \"A208\", lat: 53.299187, lon: -1.444857),
    \"A209\": Activity(id: \"A209\", lat: 53.41614, lon: -1.99759),
    \"A21\": Activity(id: \"A21\", lat: 53.021597, lon: -1.445402),
    \"A210\": Activity(id: \"A210\", lat: 53.232915, lon: -1.857649),
    \"A211\": Activity(id: \"A211\", lat: 53.250224, lon: -1.346552),
    \"A212\": Activity(id: \"A212\", lat: 53.340342, lon: -1.97752),
    \"A213\": Activity(id: \"A213\", lat: 52.861015, lon: -1.392436),
    \"A214\": Activity(id: \"A214\", lat: 52.915007, lon: -1.844042),
    \"A215\": Activity(id: \"A215\", lat: 53.008206, lon: -1.348586),
    \"A216\": Activity(id: \"A216\", lat: 53.286645, lon: -1.798027),
    \"A217\": Activity(id: \"A217\", lat: 52.964313, lon: -1.823585),
    \"A218\": Activity(id: \"A218\", lat: 52.860917, lon: -1.576771),
    \"A219\": Activity(id: \"A219\", lat: 53.468706, lon: -1.823086),
    \"A22\": Activity(id: \"A22\", lat: 53.170751, lon: -1.722464),
    \"A220\": Activity(id: \"A220\", lat: 53.123541, lon: -1.412519),
    \"A221\": Activity(id: \"A221\", lat: 53.454633, lon: -1.805966),
    \"A222\": Activity(id: \"A222\", lat: 52.950795, lon: -1.782421),
    \"A223\": Activity(id: \"A223\", lat: 52.868583, lon: -1.515564),
    \"A224\": Activity(id: \"A224\", lat: 52.721439, lon: -1.570583),
    \"A225\": Activity(id: \"A225\", lat: 53.11083, lon: -1.409434),
    \"A226\": Activity(id: \"A226\", lat: 53.078846, lon: -1.521526),
    \"A227\": Activity(id: \"A227\", lat: 52.98787, lon: -1.50494),
    \"A228\": Activity(id: \"A228\", lat: 53.270855, lon: -1.992862),
    \"A229\": Activity(id: \"A229\", lat: 53.089788, lon: -1.351129),
    \"A23\": Activity(id: \"A23\", lat: 53.096364, lon: -1.696789),
    \"A230\": Activity(id: \"A230\", lat: 52.808678, lon: -1.420621),
    \"A231\": Activity(id: \"A231\", lat: 53.273298, lon: -1.791583),
    \"A232\": Activity(id: \"A232\", lat: 52.84775, lon: -1.441183),
    \"A233\": Activity(id: \"A233\", lat: 53.342244, lon: -1.717264),
    \"A234\": Activity(id: \"A234\", lat: 53.51084, lon: -1.872637),
    \"A235\": Activity(id: \"A235\", lat: 53.167896, lon: -1.748658),
    \"A236\": Activity(id: \"A236\", lat: 53.042137, lon: -1.47593),
    \"A237\": Activity(id: \"A237\", lat: 53.471398, lon: -1.979178),
    \"A238\": Activity(id: \"A238\", lat: 52.828708, lon: -1.468058),
    \"A239\": Activity(id: \"A239\", lat: 53.237055, lon: -1.266349),
    \"A24\": Activity(id: \"A24\", lat: 53.300303, lon: -1.259392),
    \"A240\": Activity(id: \"A240\", lat: 52.897553, lon: -1.611039),
    \"A241\": Activity(id: \"A241\", lat: 53.288665, lon: -1.740187),
    \"A242\": Activity(id: \"A242\", lat: 53.304599, lon: -1.91251),
    \"A243\": Activity(id: \"A243\", lat: 52.809838, lon: -1.569947),
    \"A244\": Activity(id: \"A244\", lat: 53.05, lon: -1.494993),
    \"A245\": Activity(id: \"A245\", lat: 53.341321, lon: -1.897862),
    \"A246\": Activity(id: \"A246\", lat: 53.26279, lon: -1.849513),
    \"A247\": Activity(id: \"A247\", lat: 52.851646, lon: -1.5092),
    \"A248\": Activity(id: \"A248\", lat: 53.182662, lon: -1.35157),
    \"A249\": Activity(id: \"A249\", lat: 53.359811, lon: -1.933053),
    \"A25\": Activity(id: \"A25\", lat: 53.194612, lon: -1.538533),
    \"A250\": Activity(id: \"A250\", lat: 52.736827, lon: -1.677467),
    \"A251\": Activity(id: \"A251\", lat: 53.043909, lon: -1.461499),
    \"A252\": Activity(id: \"A252\", lat: 53.243075, lon: -1.818523),
    \"A253\": Activity(id: \"A253\", lat: 53.480589, lon: -1.884952),
    \"A254\": Activity(id: \"A254\", lat: 52.986479, lon: -1.40144),
    \"A255\": Activity(id: \"A255\", lat: 53.278519, lon: -1.976291),
    \"A256\": Activity(id: \"A256\", lat: 53.245881, lon: -1.472619),
    \"A257\": Activity(id: \"A257\", lat: 53.447163, lon: -1.872359),
    \"A258\": Activity(id: \"A258\", lat: 53.101018, lon: -1.318505),
    \"A259\": Activity(id: \"A259\", lat: 53.294113, lon: -1.237886),
    \"A26\": Activity(id: \"A26\", lat: 52.949058, lon: -1.588481),
    \"A260\": Activity(id: \"A260\", lat: 53.20105, lon: -1.436085),
    \"A261\": Activity(id: \"A261\", lat: 53.186277, lon: -1.597598),
    \"A262\": Activity(id: \"A262\", lat: 52.880919, lon: -1.520913),
    \"A263\": Activity(id: \"A263\", lat: 53.475374, lon: -1.844016),
    \"A264\": Activity(id: \"A264\", lat: 53.293792, lon: -1.284614),
    \"A265\": Activity(id: \"A265\", lat: 52.986783, lon: -1.632825),
    \"A266\": Activity(id: \"A266\", lat: 53.249626, lon: -1.87524),
    \"A267\": Activity(id: \"A267\", lat: 53.157233, lon: -1.548882),
    \"A268\": Activity(id: \"A268\", lat: 52.873592, lon: -1.339804),
    \"A269\": Activity(id: \"A269\", lat: 52.760505, lon: -1.613011),
    \"A27\": Activity(id: \"A27\", lat: 52.928912, lon: -1.713495),
    \"A270\": Activity(id: \"A270\", lat: 52.887979, lon: -1.301632),
    \"A271\": Activity(id: \"A271\", lat: 53.333099, lon: -1.837163),
    \"A272\": Activity(id: \"A272\", lat: 53.085708, lon: -1.542234),
    \"A273\": Activity(id: \"A273\", lat: 53.228546, lon: -1.690212),
    \"A274\": Activity(id: \"A274\", lat: 52.959489, lon: -1.311193),
    \"A275\": Activity(id: \"A275\", lat: 53.337783, lon: -1.716267),
    \"A276\": Activity(id: \"A276\", lat: 52.902604, lon: -1.542454),
    \"A277\": Activity(id: \"A277\", lat: 53.222418, lon: -1.867287),
    \"A278\": Activity(id: \"A278\", lat: 53.294497, lon: -1.980553),
    \"A279\": Activity(id: \"A279\", lat: 53.285831, lon: -1.778673),
    \"A28\": Activity(id: \"A28\", lat: 53.09807, lon: -1.421342),
    \"A280\": Activity(id: \"A280\", lat: 52.860392, lon: -1.374045),
    \"A281\": Activity(id: \"A281\", lat: 52.961136, lon: -1.362867),
    \"A282\": Activity(id: \"A282\", lat: 52.832524, lon: -1.48973),
    \"A283\": Activity(id: \"A283\", lat: 53.307381, lon: -1.849985),
    \"A284\": Activity(id: \"A284\", lat: 52.977953, lon: -1.331897),
    \"A285\": Activity(id: \"A285\", lat: 52.844231, lon: -1.553895),
    \"A286\": Activity(id: \"A286\", lat: 52.896011, lon: -1.647761),
    \"A287\": Activity(id: \"A287\", lat: 53.060305, lon: -1.685304),
    \"A288\": Activity(id: \"A288\", lat: 53.454527, lon: -1.798711),
    \"A289\": Activity(id: \"A289\", lat: 53.263796, lon: -1.389613),
    \"A29\": Activity(id: \"A29\", lat: 53.308228, lon: -1.581307),
    \"A290\": Activity(id: \"A290\", lat: 53.273302, lon: -1.818144),
    \"A291\": Activity(id: \"A291\", lat: 53.352408, lon: -1.683063),
    \"A292\": Activity(id: \"A292\", lat: 53.027303, lon: -1.495461),
    \"A293\": Activity(id: \"A293\", lat: 53.152314, lon: -1.803471),
    \"A294\": Activity(id: \"A294\", lat: 52.872447, lon: -1.484113),
    \"A295\": Activity(id: \"A295\", lat: 53.013974, lon: -1.337265),
    \"A296\": Activity(id: \"A296\", lat: 53.294662, lon: -1.77954),
    \"A297\": Activity(id: \"A297\", lat: 52.930391, lon: -1.602513),
    \"A298\": Activity(id: \"A298\", lat: 53.333415, lon: -1.672731),
    \"A299\": Activity(id: \"A299\", lat: 53.066765, lon: -1.683283),
    \"A3\": Activity(id: \"A3\", lat: 52.853447, lon: -1.421925),
    \"A30\": Activity(id: \"A30\", lat: 53.345618, lon: -1.716767),
    \"A300\": Activity(id: \"A300\", lat: 53.377213, lon: -1.914478),
    \"A301\": Activity(id: \"A301\", lat: 53.152098, lon: -1.531613),
    \"A302\": Activity(id: \"A302\", lat: 52.835175, lon: -1.608345),
    \"A303\": Activity(id: \"A303\", lat: 53.253982, lon: -1.298671),
    \"A304\": Activity(id: \"A304\", lat: 52.951183, lon: -1.617319),
    \"A305\": Activity(id: \"A305\", lat: 53.159561, lon: -1.511656),
    \"A306\": Activity(id: \"A306\", lat: 52.874192, lon: -1.594772),
    \"A307\": Activity(id: \"A307\", lat: 53.194373, lon: -1.826719),
    \"A308\": Activity(id: \"A308\", lat: 53.335955, lon: -1.646242),
    \"A309\": Activity(id: \"A309\", lat: 53.251988, lon: -1.462683),
    \"A31\": Activity(id: \"A31\", lat: 52.997224, lon: -1.750269),
    \"A310\": Activity(id: \"A310\", lat: 53.263076, lon: -1.946183),
    \"A311\": Activity(id: \"A311\", lat: 53.194798, lon: -1.531571),
    \"A312\": Activity(id: \"A312\", lat: 52.9639, lon: -1.479179),
    \"A313\": Activity(id: \"A313\", lat: 52.797571, lon: -1.455035),
    \"A314\": Activity(id: \"A314\", lat: 53.27326, lon: -1.91086),
    \"A315\": Activity(id: \"A315\", lat: 53.209721, lon: -1.683791),
    \"A316\": Activity(id: \"A316\", lat: 53.388276, lon: -1.702803),
    \"A317\": Activity(id: \"A317\", lat: 53.363278, lon: -1.969355),
    \"A318\": Activity(id: \"A318\", lat: 52.745878, lon: -1.643253),
    \"A319\": Activity(id: \"A319\", lat: 53.366992, lon: -1.993477),
    \"A32\": Activity(id: \"A32\", lat: 53.248787, lon: -1.21252),
    \"A320\": Activity(id: \"A320\", lat: 53.373365, lon: -1.890693),
    \"A321\": Activity(id: \"A321\", lat: 53.210913, lon: -1.607096),
    \"A322\": Activity(id: \"A322\", lat: 52.88559, lon: -1.820358),
    \"A323\": Activity(id: \"A323\", lat: 52.92046, lon: -1.471585),
    \"A324\": Activity(id: \"A324\", lat: 53.082714, lon: -1.62502),
    \"A325\": Activity(id: \"A325\", lat: 53.380904, lon: -1.975926),
    \"A326\": Activity(id: \"A326\", lat: 53.298654, lon: -1.436648),
    \"A327\": Activity(id: \"A327\", lat: 53.220142, lon: -1.563642),
    \"A328\": Activity(id: \"A328\", lat: 53.390658, lon: -1.684257),
    \"A329\": Activity(id: \"A329\", lat: 53.075047, lon: -1.635613),
    \"A33\": Activity(id: \"A33\", lat: 53.392995, lon: -1.784665),
    \"A330\": Activity(id: \"A330\", lat: 53.2175, lon: -1.340501),
    \"A331\": Activity(id: \"A331\", lat: 53.274318, lon: -1.679263),
    \"A332\": Activity(id: \"A332\", lat: 53.348353, lon: -1.842712),
    \"A333\": Activity(id: \"A333\", lat: 53.176491, lon: -1.254203),
    \"A334\": Activity(id: \"A334\", lat: 53.501256, lon: -1.863441),
    \"A335\": Activity(id: \"A335\", lat: 53.435886, lon: -1.98357),
    \"A336\": Activity(id: \"A336\", lat: 53.033146, lon: -1.582961),
    \"A337\": Activity(id: \"A337\", lat: 53.376749, lon: -1.700681),
    \"A338\": Activity(id: \"A338\", lat: 53.394419, lon: -1.892288),
    \"A339\": Activity(id: \"A339\", lat: 53.358037, lon: -1.931724),
    \"A34\": Activity(id: \"A34\", lat: 53.06775, lon: -1.443167),
    \"A340\": Activity(id: \"A340\", lat: 53.290725, lon: -1.508049),
    \"A341\": Activity(id: \"A341\", lat: 53.161724, lon: -1.813916),
    \"A342\": Activity(id: \"A342\", lat: 53.062894, lon: -1.650108),
    \"A343\": Activity(id: \"A343\", lat: 53.481683, lon: -1.976595),
    \"A344\": Activity(id: \"A344\", lat: 53.24118, lon: -1.939691),
    \"A345\": Activity(id: \"A345\", lat: 52.958963, lon: -1.344107),
    \"A346\": Activity(id: \"A346\", lat: 53.284835, lon: -1.664746),
    \"A347\": Activity(id: \"A347\", lat: 52.88491, lon: -1.316082),
    \"A348\": Activity(id: \"A348\", lat: 53.318902, lon: -1.682998),
    \"A349\": Activity(id: \"A349\", lat: 53.468879, lon: -1.968936),
    \"A35\": Activity(id: \"A35\", lat: 53.189373, lon: -1.218481),
    \"A350\": Activity(id: \"A350\", lat: 53.334408, lon: -1.865757),
    \"A351\": Activity(id: \"A351\", lat: 53.09876, lon: -1.514937),
    \"A352\": Activity(id: \"A352\", lat: 53.026837, lon: -1.541276),
    \"A353\": Activity(id: \"A353\", lat: 52.854873, lon: -1.546077),
    \"A354\": Activity(id: \"A354\", lat: 53.244597, lon: -1.815372),
    \"A355\": Activity(id: \"A355\", lat: 52.915106, lon: -1.775061),
    \"A356\": Activity(id: \"A356\", lat: 52.955428, lon: -1.628927),
    \"A357\": Activity(id: \"A357\", lat: 53.294105, lon: -1.221945),
    \"A358\": Activity(id: \"A358\", lat: 53.251348, lon: -1.680541),
    \"A359\": Activity(id: \"A359\", lat: 53.320682, lon: -1.699642),
    \"A36\": Activity(id: \"A36\", lat: 53.276409, lon: -1.382221),
    \"A360\": Activity(id: \"A360\", lat: 53.298263, lon: -1.899204),
    \"A361\": Activity(id: \"A361\", lat: 52.912373, lon: -1.748691),
    \"A362\": Activity(id: \"A362\", lat: 53.252445, lon: -1.621241),
    \"A363\": Activity(id: \"A363\", lat: 52.934, lon: -1.681377),
    \"A364\": Activity(id: \"A364\", lat: 53.219499, lon: -1.626522),
    \"A365\": Activity(id: \"A365\", lat: 53.272999, lon: -1.970053),
    \"A366\": Activity(id: \"A366\", lat: 53.3707, lon: -1.86995),
    \"A367\": Activity(id: \"A367\", lat: 53.214615, lon: -1.476523),
    \"A368\": Activity(id: \"A368\", lat: 52.893235, lon: -1.704581),
    \"A369\": Activity(id: \"A369\", lat: 53.292336, lon: -1.949444),
    \"A37\": Activity(id: \"A37\", lat: 53.239115, lon: -1.954457),
    \"A370\": Activity(id: \"A370\", lat: 52.720544, lon: -1.568702),
    \"A371\": Activity(id: \"A371\", lat: 53.145712, lon: -1.765019),
    \"A372\": Activity(id: \"A372\", lat: 52.95817, lon: -1.633874),
    \"A373\": Activity(id: \"A373\", lat: 52.908175, lon: -1.288014),
    \"A374\": Activity(id: \"A374\", lat: 53.494554, lon: -1.860928),
    \"A375\": Activity(id: \"A375\", lat: 53.281127, lon: -1.648983),
    \"A376\": Activity(id: \"A376\", lat: 52.915235, lon: -1.702199),
    \"A377\": Activity(id: \"A377\", lat: 53.089769, lon: -1.347454),
    \"A378\": Activity(id: \"A378\", lat: 52.951426, lon: -1.699051),
    \"A379\": Activity(id: \"A379\", lat: 52.95235, lon: -1.43183),
    \"A38\": Activity(id: \"A38\", lat: 52.853051, lon: -1.500005),
    \"A380\": Activity(id: \"A380\", lat: 52.93266, lon: -1.480134),
    \"A381\": Activity(id: \"A381\", lat: 53.365809, lon: -1.967201),
    \"A382\": Activity(id: \"A382\", lat: 53.081526, lon: -1.544023),
    \"A383\": Activity(id: \"A383\", lat: 52.981228, lon: -1.342728),
    \"A384\": Activity(id: \"A384\", lat: 52.826656, lon: -1.60056),
    \"A385\": Activity(id: \"A385\", lat: 52.741292, lon: -1.577162),
    \"A386\": Activity(id: \"A386\", lat: 52.95563, lon: -1.82363),
    \"A387\": Activity(id: \"A387\", lat: 52.946932, lon: -1.520254),
    \"A388\": Activity(id: \"A388\", lat: 52.991118, lon: -1.499832),
    \"A389\": Activity(id: \"A389\", lat: 53.320769, lon: -1.851326),
    \"A39\": Activity(id: \"A39\", lat: 52.964071, lon: -1.757419),
    \"A390\": Activity(id: \"A390\", lat: 52.887326, lon: -1.732402),
    \"A391\": Activity(id: \"A391\", lat: 52.948133, lon: -1.328279),
    \"A392\": Activity(id: \"A392\", lat: 53.278316, lon: -1.441839),
    \"A393\": Activity(id: \"A393\", lat: 53.488166, lon: -1.831044),
    \"A394\": Activity(id: \"A394\", lat: 53.330352, lon: -1.634426),
    \"A395\": Activity(id: \"A395\", lat: 53.284403, lon: -1.398876),
    \"A396\": Activity(id: \"A396\", lat: 53.054644, lon: -1.495943),
    \"A397\": Activity(id: \"A397\", lat: 53.02094, lon: -1.400515),
    \"A398\": Activity(id: \"A398\", lat: 53.17824, lon: -1.383992),
    \"A399\": Activity(id: \"A399\", lat: 53.056656, lon: -1.674705),
    \"A4\": Activity(id: \"A4\", lat: 53.456315, lon: -1.938831),
    \"A40\": Activity(id: \"A40\", lat: 52.962609, lon: -1.298392),
    \"A400\": Activity(id: \"A400\", lat: 53.242542, lon: -1.522038),
    \"A401\": Activity(id: \"A401\", lat: 53.093855, lon: -1.51691),
    \"A402\": Activity(id: \"A402\", lat: 53.292557, lon: -1.193528),
    \"A403\": Activity(id: \"A403\", lat: 52.945326, lon: -1.674717),
    \"A404\": Activity(id: \"A404\", lat: 52.944174, lon: -1.698331),
    \"A405\": Activity(id: \"A405\", lat: 52.874737, lon: -1.410662),
    \"A406\": Activity(id: \"A406\", lat: 53.363599, lon: -1.851945),
    \"A407\": Activity(id: \"A407\", lat: 53.363548, lon: -1.955202),
    \"A408\": Activity(id: \"A408\", lat: 52.961086, lon: -1.711434),
    \"A409\": Activity(id: \"A409\", lat: 53.117539, lon: -1.401815),
    \"A41\": Activity(id: \"A41\", lat: 52.849239, lon: -1.567405),
    \"A410\": Activity(id: \"A410\", lat: 53.082292, lon: -1.377611),
    \"A411\": Activity(id: \"A411\", lat: 53.333941, lon: -1.882775),
    \"A412\": Activity(id: \"A412\", lat: 52.970549, lon: -1.396183),
    \"A413\": Activity(id: \"A413\", lat: 53.060047, lon: -1.69394),
    \"A414\": Activity(id: \"A414\", lat: 52.975226, lon: -1.580557),
    \"A415\": Activity(id: \"A415\", lat: 53.125995, lon: -1.338938),
    \"A416\": Activity(id: \"A416\", lat: 53.084924, lon: -1.346203),
    \"A417\": Activity(id: \"A417\", lat: 52.814947, lon: -1.572609),
    \"A418\": Activity(id: \"A418\", lat: 53.078543, lon: -1.414319),
    \"A419\": Activity(id: \"A419\", lat: 52.914469, lon: -1.45553),
    \"A42\": Activity(id: \"A42\", lat: 53.224717, lon: -1.548871),
    \"A420\": Activity(id: \"A420\", lat: 53.116948, lon: -1.755561),
    \"A421\": Activity(id: \"A421\", lat: 53.086729, lon: -1.319432),
    \"A422\": Activity(id: \"A422\", lat: 53.166087, lon: -1.409332),
    \"A423\": Activity(id: \"A423\", lat: 53.063094, lon: -1.658877),
    \"A424\": Activity(id: \"A424\", lat: 52.894409, lon: -1.471499),
    \"A425\": Activity(id: \"A425\", lat: 53.407812, lon: -1.795975),
    \"A426\": Activity(id: \"A426\", lat: 53.246651, lon: -1.787108),
    \"A427\": Activity(id: \"A427\", lat: 52.847759, lon: -1.408824),
    \"A428\": Activity(id: \"A428\", lat: 53.479052, lon: -1.841059),
    \"A429\": Activity(id: \"A429\", lat: 52.765804, lon: -1.528912),
    \"A43\": Activity(id: \"A43\", lat: 53.219331, lon: -1.610057),
    \"A430\": Activity(id: \"A430\", lat: 53.366188, lon: -1.937097),
    \"A431\": Activity(id: \"A431\", lat: 53.070116, lon: -1.360459),
    \"A432\": Activity(id: \"A432\", lat: 53.400829, lon: -1.734086),
    \"A433\": Activity(id: \"A433\", lat: 53.025994, lon: -1.661312),
    \"A434\": Activity(id: \"A434\", lat: 53.111149, lon: -1.572826),
    \"A435\": Activity(id: \"A435\", lat: 52.956482, lon: -1.507907),
    \"A436\": Activity(id: \"A436\", lat: 52.877214, lon: -1.528643),
    \"A437\": Activity(id: \"A437\", lat: 52.984477, lon: -1.752706),
    \"A438\": Activity(id: \"A438\", lat: 52.914053, lon: -1.721192),
    \"A439\": Activity(id: \"A439\", lat: 53.332707, lon: -1.830553),
    \"A44\": Activity(id: \"A44\", lat: 53.273162, lon: -1.47),
    \"A440\": Activity(id: \"A440\", lat: 53.43098, lon: -1.770276),
    \"A441\": Activity(id: \"A441\", lat: 53.129048, lon: -1.395214),
    \"A442\": Activity(id: \"A442\", lat: 53.228466, lon: -1.884525),
    \"A443\": Activity(id: \"A443\", lat: 53.295666, lon: -1.6698),
    \"A444\": Activity(id: \"A444\", lat: 53.18657, lon: -1.797845),
    \"A445\": Activity(id: \"A445\", lat: 53.20495, lon: -1.293644),
    \"A446\": Activity(id: \"A446\", lat: 53.26562, lon: -1.398309),
    \"A447\": Activity(id: \"A447\", lat: 53.232097, lon: -1.800436),
    \"A448\": Activity(id: \"A448\", lat: 52.821922, lon: -1.463052),
    \"A449\": Activity(id: \"A449\", lat: 53.249979, lon: -1.535328),
    \"A45\": Activity(id: \"A45\", lat: 52.884377, lon: -1.600451),
    \"A450\": Activity(id: \"A450\", lat: 53.27744, lon: -2.000087),
    \"A451\": Activity(id: \"A451\", lat: 53.0181, lon: -1.505363),
    \"A452\": Activity(id: \"A452\", lat: 53.282658, lon: -1.286859),
    \"A453\": Activity(id: \"A453\", lat: 53.076445, lon: -1.631166),
    \"A454\": Activity(id: \"A454\", lat: 53.328098, lon: -1.983478),
    \"A455\": Activity(id: \"A455\", lat: 53.315843, lon: -1.889687),
    \"A456\": Activity(id: \"A456\", lat: 53.319146, lon: -1.910055),
    \"A457\": Activity(id: \"A457\", lat: 53.467618, lon: -1.80971),
    \"A458\": Activity(id: \"A458\", lat: 52.954112, lon: -1.730794),
    \"A459\": Activity(id: \"A459\", lat: 52.90781, lon: -1.410795),
    \"A46\": Activity(id: \"A46\", lat: 53.318464, lon: -1.434432),
    \"A460\": Activity(id: \"A460\", lat: 52.873254, lon: -1.334265),
    \"A461\": Activity(id: \"A461\", lat: 53.208337, lon: -1.270018),
    \"A462\": Activity(id: \"A462\", lat: 53.191659, lon: -1.442761),
    \"A463\": Activity(id: \"A463\", lat: 53.080587, lon: -1.465793),
    \"A464\": Activity(id: \"A464\", lat: 53.282889, lon: -1.921572),
    \"A465\": Activity(id: \"A465\", lat: 53.490712, lon: -1.830792),
    \"A466\": Activity(id: \"A466\", lat: 52.982847, lon: -1.727329),
    \"A467\": Activity(id: \"A467\", lat: 53.148933, lon: -1.433652),
    \"A468\": Activity(id: \"A468\", lat: 52.769495, lon: -1.600818),
    \"A469\": Activity(id: \"A469\", lat: 53.112414, lon: -1.515076),
    \"A47\": Activity(id: \"A47\", lat: 53.374198, lon: -1.754786),
    \"A470\": Activity(id: \"A470\", lat: 53.290689, lon: -1.295722),
    \"A471\": Activity(id: \"A471\", lat: 52.764137, lon: -1.548128),
    \"A472\": Activity(id: \"A472\", lat: 53.250858, lon: -1.867588),
    \"A473\": Activity(id: \"A473\", lat: 53.460591, lon: -1.780312),
    \"A474\": Activity(id: \"A474\", lat: 52.972763, lon: -1.459385),
    \"A475\": Activity(id: \"A475\", lat: 53.123278, lon: -1.575332),
    \"A476\": Activity(id: \"A476\", lat: 53.412625, lon: -1.727333),
    \"A477\": Activity(id: \"A477\", lat: 53.358398, lon: -1.986541),
    \"A478\": Activity(id: \"A478\", lat: 53.242286, lon: -1.309946),
    \"A479\": Activity(id: \"A479\", lat: 53.065831, lon: -1.381648),
    \"A48\": Activity(id: \"A48\", lat: 53.394356, lon: -1.97847),
    \"A480\": Activity(id: \"A480\", lat: 53.000672, lon: -1.378069),
    \"A481\": Activity(id: \"A481\", lat: 53.00669, lon: -1.524029),
    \"A482\": Activity(id: \"A482\", lat: 53.039846, lon: -1.696042),
    \"A483\": Activity(id: \"A483\", lat: 53.281574, lon: -1.504146),
    \"A484\": Activity(id: \"A484\", lat: 52.96476, lon: -1.781217),
    \"A485\": Activity(id: \"A485\", lat: 52.862261, lon: -1.52767),
    \"A486\": Activity(id: \"A486\", lat: 53.133531, lon: -1.634838),
    \"A487\": Activity(id: \"A487\", lat: 53.394783, lon: -1.80511),
    \"A488\": Activity(id: \"A488\", lat: 53.389977, lon: -1.849766),
    \"A489\": Activity(id: \"A489\", lat: 53.284252, lon: -1.504242),
    \"A49\": Activity(id: \"A49\", lat: 52.894157, lon: -1.593967),
    \"A490\": Activity(id: \"A490\", lat: 53.131108, lon: -1.539769),
    \"A491\": Activity(id: \"A491\", lat: 53.391711, lon: -1.77286),
    \"A492\": Activity(id: \"A492\", lat: 52.78365, lon: -1.5172),
    \"A493\": Activity(id: \"A493\", lat: 53.205933, lon: -1.290005),
    \"A494\": Activity(id: \"A494\", lat: 53.125268, lon: -1.633345),
    \"A495\": Activity(id: \"A495\", lat: 53.267446, lon: -1.636776),
    \"A496\": Activity(id: \"A496\", lat: 53.344936, lon: -1.622277),
    \"A497\": Activity(id: \"A497\", lat: 53.297222, lon: -1.394429),
    \"A498\": Activity(id: \"A498\", lat: 53.070688, lon: -1.394259),
    \"A499\": Activity(id: \"A499\", lat: 53.1071, lon: -1.380987),
    \"A5\": Activity(id: \"A5\", lat: 53.04544, lon: -1.653596),
    \"A50\": Activity(id: \"A50\", lat: 52.980995, lon: -1.687655),
    \"A500\": Activity(id: \"A500\", lat: 53.075312, lon: -1.514646),
    \"A51\": Activity(id: \"A51\", lat: 53.081827, lon: -1.338886),
    \"A52\": Activity(id: \"A52\", lat: 53.305782, lon: -1.399702),
    \"A53\": Activity(id: \"A53\", lat: 52.898776, lon: -1.385245),
    \"A54\": Activity(id: \"A54\", lat: 53.270796, lon: -1.383411),
    \"A55\": Activity(id: \"A55\", lat: 53.346368, lon: -1.929756),
    \"A56\": Activity(id: \"A56\", lat: 53.255629, lon: -1.377794),
    \"A57\": Activity(id: \"A57\", lat: 52.928268, lon: -1.710506),
    \"A58\": Activity(id: \"A58\", lat: 53.039406, lon: -1.448758),
    \"A59\": Activity(id: \"A59\", lat: 53.027457, lon: -1.508424),
    \"A6\": Activity(id: \"A6\", lat: 52.881123, lon: -1.699716),
    \"A60\": Activity(id: \"A60\", lat: 53.120566, lon: -1.595411),
    \"A61\": Activity(id: \"A61\", lat: 53.065032, lon: -1.366517),
    \"A62\": Activity(id: \"A62\", lat: 53.460849, lon: -1.946062),
    \"A63\": Activity(id: \"A63\", lat: 53.011494, lon: -1.665914),
    \"A64\": Activity(id: \"A64\", lat: 52.762773, lon: -1.566517),
    \"A65\": Activity(id: \"A65\", lat: 53.106076, lon: -1.393463),
    \"A66\": Activity(id: \"A66\", lat: 53.141061, lon: -1.777693),
    \"A67\": Activity(id: \"A67\", lat: 52.900893, lon: -1.430698),
    \"A68\": Activity(id: \"A68\", lat: 53.265366, lon: -1.839951),
    \"A69\": Activity(id: \"A69\", lat: 53.026581, lon: -1.446926),
    \"A7\": Activity(id: \"A7\", lat: 53.356282, lon: -1.98683),
    \"A70\": Activity(id: \"A70\", lat: 53.140561, lon: -1.355278),
    \"A71\": Activity(id: \"A71\", lat: 52.963467, lon: -1.644309),
    \"A72\": Activity(id: \"A72\", lat: 52.930169, lon: -1.302114),
    \"A73\": Activity(id: \"A73\", lat: 53.046974, lon: -1.736582),
    \"A74\": Activity(id: \"A74\", lat: 53.199876, lon: -1.777463),
    \"A75\": Activity(id: \"A75\", lat: 52.75809, lon: -1.588261),
    \"A76\": Activity(id: \"A76\", lat: 53.08767, lon: -1.431045),
    \"A77\": Activity(id: \"A77\", lat: 52.907125, lon: -1.423618),
    \"A78\": Activity(id: \"A78\", lat: 52.996284, lon: -1.74657),
    \"A79\": Activity(id: \"A79\", lat: 52.921935, lon: -1.36243),
    \"A8\": Activity(id: \"A8\", lat: 52.878424, lon: -1.412139),
    \"A80\": Activity(id: \"A80\", lat: 52.94032, lon: -1.753777),
    \"A81\": Activity(id: \"A81\", lat: 53.215379, lon: -1.595284),
    \"A82\": Activity(id: \"A82\", lat: 53.263962, lon: -1.567333),
    \"A83\": Activity(id: \"A83\", lat: 53.155083, lon: -1.529034),
    \"A84\": Activity(id: \"A84\", lat: 53.222542, lon: -1.33159),
    \"A85\": Activity(id: \"A85\", lat: 53.102216, lon: -1.407565),
    \"A86\": Activity(id: \"A86\", lat: 53.023731, lon: -1.750332),
    \"A87\": Activity(id: \"A87\", lat: 53.217449, lon: -1.912982),
    \"A88\": Activity(id: \"A88\", lat: 53.476779, lon: -1.92496),
    \"A89\": Activity(id: \"A89\", lat: 53.503041, lon: -1.965831),
    \"A9\": Activity(id: \"A9\", lat: 52.976237, lon: -1.401738),
    \"A90\": Activity(id: \"A90\", lat: 53.061543, lon: -1.568773),
    \"A91\": Activity(id: \"A91\", lat: 53.252783, lon: -1.719633),
    \"A92\": Activity(id: \"A92\", lat: 53.30846, lon: -1.393857),
    \"A93\": Activity(id: \"A93\", lat: 53.086748, lon: -1.388024),
    \"A94\": Activity(id: \"A94\", lat: 53.210029, lon: -1.252932),
    \"A95\": Activity(id: \"A95\", lat: 52.827881, lon: -1.408901),
    \"A96\": Activity(id: \"A96\", lat: 52.751808, lon: -1.674164),
    \"A97\": Activity(id: \"A97\", lat: 52.80051, lon: -1.577267),
    \"A98\": Activity(id: \"A98\", lat: 52.980786, lon: -1.439274),
    \"A99\": Activity(id: \"A99\", lat: 53.128494, lon: -1.407428)
  },
  resource: {
    \"R1\": Resource(id: \"R1\", lat: 52.98186, lon: -1.48284),
    \"R10\": Resource(id: \"R10\", lat: 52.915582, lon: -1.822536),
    \"R2\": Resource(id: \"R2\", lat: 53.244035, lon: -1.712563),
    \"R3\": Resource(id: \"R3\", lat: 52.865613, lon: -1.508073),
    \"R4\": Resource(id: \"R4\", lat: 52.875805, lon: -1.698446),
    \"R5\": Resource(id: \"R5\", lat: 53.393973, lon: -1.789481),
    \"R6\": Resource(id: \"R6\", lat: 52.924522, lon: -1.684105),
    \"R7\": Resource(id: \"R7\", lat: 53.030762, lon: -1.43239),
    \"R8\": Resource(id: \"R8\", lat: 53.232308, lon: -1.798809),
    \"R9\": Resource(id: \"R9\", lat: 52.942768, lon: -1.610388)
  },
  route: [
  ],
  allocation: {
  },
  status: Some(Status(
    start_time: 0.0,
    new_data: true,
    changed: false,
    quality: 0.0,
    distance: 0.0,
    value: 0.0,
    travel_time: 0.0)),
)";
