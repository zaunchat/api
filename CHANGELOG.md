## [1.12.2](https://github.com/itchatapp/api/compare/v1.12.1...v1.12.2) (2022-06-20)


### Bug Fixes

* **user:** refactor #fetch_by_token query ([c7fbee7](https://github.com/itchatapp/api/commit/c7fbee7aa3651dacafb80b17764a539b9095eb10))

## [1.12.1](https://github.com/itchatapp/api/compare/v1.12.0...v1.12.1) (2022-06-20)


### Bug Fixes

* deserialize permissions/badges ([5040776](https://github.com/itchatapp/api/commit/5040776ccfa85ef6d0536104bace8b241a242452))

# [1.12.0](https://github.com/itchatapp/api/compare/v1.11.1...v1.12.0) (2022-06-19)


### Bug Fixes

* subscribe to server channels ([b594257](https://github.com/itchatapp/api/commit/b5942575ef7c1a20e7684d0c0bfe494e49a40c8a))
* typo ([067bf2e](https://github.com/itchatapp/api/commit/067bf2e5981c252b9a23da35b4d982a202882236))
* unsubsribe from deleted objects ([49ac9a9](https://github.com/itchatapp/api/commit/49ac9a91ee2ca0a30da8a9f100d59e76376e1335))
* Update cached permissions ([417e5c1](https://github.com/itchatapp/api/commit/417e5c196c842a2504eb0647238416139fe39b90))


### Features

* Add redis connection ([ec7ec16](https://github.com/itchatapp/api/commit/ec7ec162ba3ba2c62631bd5a49da54a419423240))
* Add server edit route ([217461a](https://github.com/itchatapp/api/commit/217461a118218c2bbd10f4a0afdfe67f93d4e36e))
* basic events ([f4a0475](https://github.com/itchatapp/api/commit/f4a047541055482fd39600283eeaae2a0e246678))
* Cache permissions ([a9aed37](https://github.com/itchatapp/api/commit/a9aed371dbef969b3b7d5033582bb6804402ce0e))
* Emit server/channel creation events ([6a15173](https://github.com/itchatapp/api/commit/6a151735fa7ca8a5197fcceb7865bb661a6d3a36))
* handle outcoming data ([3e465d8](https://github.com/itchatapp/api/commit/3e465d8f4f46e0d19efc4a2ad9eda57be02dc398))
* Payload struct ([361e0d1](https://github.com/itchatapp/api/commit/361e0d12a3a06cc9fa883f7b6ae390ba5d7a2b63))
* **Permissions:** Add #fetch_cached ([c866874](https://github.com/itchatapp/api/commit/c86687430b1c0b0ccbee6fcd69740f4424e2be35))
* Publish other events ([18aa87a](https://github.com/itchatapp/api/commit/18aa87aed1a3184a3068ddb90dd86c06bfbad1d2))
* Send channel deletion events ([7377c0a](https://github.com/itchatapp/api/commit/7377c0a26e1c987ad36dab8a9f47e0522d28aba3))
* Send message events! ([a77faaa](https://github.com/itchatapp/api/commit/a77faaa53728266e08ad1b2e7c66de3441e24f0c))
* subsribe to servers & channels ([e7fa71e](https://github.com/itchatapp/api/commit/e7fa71e30a745c3cfa363a57adbcb72e8a0f4d3f))

## [1.11.1](https://github.com/itchatapp/api/compare/v1.11.0...v1.11.1) (2022-06-16)


### Bug Fixes

* replaced utoipa to opg as openapi generator and much more changes that i can't commit ([04e6657](https://github.com/itchatapp/api/commit/04e66577624fb4f30edae79ae0c9e4010a7b118f))

# [1.11.0](https://github.com/itchatapp/api/compare/v1.10.0...v1.11.0) (2022-06-14)


### Features

* Apply limitation of creation of x ([01d6ab0](https://github.com/itchatapp/api/commit/01d6ab04886d73f483bbce93c8c33ff39c433ddd))

# [1.10.0](https://github.com/itchatapp/api/compare/v1.9.1...v1.10.0) (2022-06-14)


### Bug Fixes

* **cors:** explicitly allowed headers ([778b195](https://github.com/itchatapp/api/commit/778b19541c4a6619f2faea510aab4e59291f69b6))
* **cors:** use from_str instead of from_static ([1c1ba35](https://github.com/itchatapp/api/commit/1c1ba35156e398507b9220de22b9db5e1b1c92cd))
* Remove support of HTTPs ([26fba0a](https://github.com/itchatapp/api/commit/26fba0a093d7656c245670f9e347327d1bfdba92))


### Features

* **base:** Add #count method ([d0fd90a](https://github.com/itchatapp/api/commit/d0fd90a66874ffd96391b61e139804784115b404))
* **error:** Provide more information about the occuret error ([a9f9a6b](https://github.com/itchatapp/api/commit/a9f9a6bcdba97011403cfecb8e7cc3389023ceaf))
* **routes:** limit creation of servers ([349b87d](https://github.com/itchatapp/api/commit/349b87de11d9dc04ea68b3010e5178ff33711fe2))

## [1.9.1](https://github.com/itchatapp/api/compare/v1.9.0...v1.9.1) (2022-06-13)


### Bug Fixes

* **ssl:** deal with http-1 challenge ([580efd0](https://github.com/itchatapp/api/commit/580efd0459537b220835a80959bdf8fd1b6dda93))

# [1.9.0](https://github.com/itchatapp/api/compare/v1.8.4...v1.9.0) (2022-06-13)


### Features

* HTTPs Support ([e8c25b2](https://github.com/itchatapp/api/commit/e8c25b2d650a73dff4163442b64a44c14c7014db))

## [1.8.4](https://github.com/itchatapp/api/compare/v1.8.3...v1.8.4) (2022-06-12)


### Bug Fixes

*  bug fixes ([b4246b3](https://github.com/itchatapp/api/commit/b4246b3bff7b1d3d72f3a4ab4983b5a01d7766f6))

## [1.8.3](https://github.com/itchatapp/api/compare/v1.8.2...v1.8.3) (2022-06-12)


### Bug Fixes

* **auth:** Remove swagger endpoint ([57babd5](https://github.com/itchatapp/api/commit/57babd5b008703802682fc4cc4d60a9b8b11d35d))
* **captcha:** Should return error instead of status code ([6b42a3b](https://github.com/itchatapp/api/commit/6b42a3b44587ec875eaf48a684e573884cef5dd7))

## [1.8.2](https://github.com/itchatapp/api/compare/v1.8.1...v1.8.2) (2022-06-11)


### Bug Fixes

* **routes:** Increase maiumum requests for auth routes ([92e16f9](https://github.com/itchatapp/api/commit/92e16f9fc2277accd6eb4ce8dcceff8ad43ab3c8))

## [1.8.1](https://github.com/itchatapp/api/compare/v1.8.0...v1.8.1) (2022-06-11)


### Bug Fixes

* **docker:** Remove old rocket options ([42ba75b](https://github.com/itchatapp/api/commit/42ba75be54ec78f5490e4f419a88b0dd3915442a))

# [1.8.0](https://github.com/itchatapp/api/compare/v1.7.0...v1.8.0) (2022-06-11)


### Features

* **docs:** document session/account routes ([83170a0](https://github.com/itchatapp/api/commit/83170a08adc8281873c31522a8e2527a54ab0e60))

# [1.7.0](https://github.com/itchatapp/api/compare/v1.6.0...v1.7.0) (2022-06-11)


### Features

* Add trust cloudflare option ([51f4205](https://github.com/itchatapp/api/commit/51f4205cd1963dcc7f4e7190b48430dd0e9d8ddc))
* **docs:** readd openapi & swagger ui ([f098ccd](https://github.com/itchatapp/api/commit/f098ccde44123c42af3602490b02d94849b1f74c))

# [1.6.0](https://github.com/itchatapp/api/compare/v1.5.1...v1.6.0) (2022-06-11)


### Features

* Move on to axum instead of rocket ([9d66431](https://github.com/itchatapp/api/commit/9d664316c54e6a0cebab0d3b58d50d6f210133de))

## [1.5.1](https://github.com/itchatapp/api/compare/v1.5.0...v1.5.1) (2022-06-10)


### Bug Fixes

* Remove bloated code ([e3ae67a](https://github.com/itchatapp/api/commit/e3ae67ac1a7433179cdad88a0965d5b18455ffc9))
* Remove unneeded header ([b6045c3](https://github.com/itchatapp/api/commit/b6045c30962f6073909d61c193ddde9c625b1c4e))
* **route:** mount routes the right way ([15df39e](https://github.com/itchatapp/api/commit/15df39e930c1bd9180b6a78a399edcb02dca5fa4))
* Save the channel ([c2d4732](https://github.com/itchatapp/api/commit/c2d4732e2886163e2e422814b53f4663a2e9488e))

# [1.5.0](https://github.com/itchatapp/api/compare/v1.4.0...v1.5.0) (2022-06-10)


### Features

* **migrations:** Add account invites table ([c49f33f](https://github.com/itchatapp/api/commit/c49f33f4ad21d600b11aeafa52a5a9633e900b1c))

# [1.4.0](https://github.com/itchatapp/api/compare/v1.3.3...v1.4.0) (2022-06-10)


### Features

* **routes:** Implement inviteation requirement ([fc94366](https://github.com/itchatapp/api/commit/fc94366ca555c98c5005f84df6c64060533e2bce))

## [1.3.3](https://github.com/itchatapp/api/compare/v1.3.2...v1.3.3) (2022-06-10)


### Bug Fixes

* **docker:** Use stable version of rust ([929caae](https://github.com/itchatapp/api/commit/929caae73cfd49ee3a2844ece283c644716ea8ed))
* **migration:** syntax error ([5d67550](https://github.com/itchatapp/api/commit/5d6755008d8f6e28faf489ac9752c16d3974f204))
* use include_str instead of std::fs ([fe1d2ad](https://github.com/itchatapp/api/commit/fe1d2adb5a818b9ffc142ac19fedd249a2e63836))

## [1.3.2](https://github.com/itchatapp/api/compare/v1.3.1...v1.3.2) (2022-06-09)


### Bug Fixes

* **docker:** Move assets to working dir ([6375162](https://github.com/itchatapp/api/commit/6375162d05d12bb744ddfdfb522756329f89bcb2))
* Follow clippy guide ([049b085](https://github.com/itchatapp/api/commit/049b085c3f1d28e78063bb3cbaaa8d0d9bce525a))

## [1.3.1](https://github.com/itchatapp/api/compare/v1.3.0...v1.3.1) (2022-06-09)


### Bug Fixes

* **docker:** Set default port to 8080 ([18b3d5d](https://github.com/itchatapp/api/commit/18b3d5d057d491d2f226792985531da6ac338afe))

# [1.3.0](https://github.com/itchatapp/api/compare/v1.2.1...v1.3.0) (2022-06-09)


### Features

* **docker:** Add Dockerfile ([5fdd09c](https://github.com/itchatapp/api/commit/5fdd09c439b41c843ba1f610ecaf890124f3b70a))

## [1.2.1](https://github.com/itchatapp/api/compare/v1.2.0...v1.2.1) (2022-06-09)


### Bug Fixes

* Permissions & Badges as integer ([917f327](https://github.com/itchatapp/api/commit/917f327e37c62156eaccdffb505a2d7dec072d04))

# [1.2.0](https://github.com/itchatapp/api/compare/v1.1.3...v1.2.0) (2022-06-09)


### Bug Fixes

* **Auth:** Ignore paths correctly ([236124e](https://github.com/itchatapp/api/commit/236124e93604fafb8bcf17925dea47e98c7aa565))
* serialize nullable property on structs ([52cbea9](https://github.com/itchatapp/api/commit/52cbea97e5d5730db3cdea08f0006a97d48e765f))


### Features

* OpenAPI v3 is here! ([3711d6a](https://github.com/itchatapp/api/commit/3711d6aa1bcb603849b230f04c44d1f54f9888f2))

## [1.1.3](https://github.com/itchatapp/api/compare/v1.1.2...v1.1.3) (2022-06-09)


### Bug Fixes

* use correct version of rocket_cors ([0328172](https://github.com/itchatapp/api/commit/0328172072156e6d3b63b01345e02ce5e1a8fe4f))

## [1.1.2](https://github.com/itchatapp/api/compare/v1.1.1...v1.1.2) (2022-06-09)


### Bug Fixes

* use correct version of rocket_cors ([9a46006](https://github.com/itchatapp/api/commit/9a46006929e12d48b6a1caa8ff5e4d901ac18e44))

## [1.1.1](https://github.com/itchatapp/api/compare/v1.1.0...v1.1.1) (2022-06-09)


### Bug Fixes

* Remove un-needed lifetimes ([39a27e5](https://github.com/itchatapp/api/commit/39a27e5d70b6a04397e572b14a82295bd77755ef))

# [1.1.0](https://github.com/itchatapp/api/compare/v1.0.1...v1.1.0) (2022-06-09)


### Features

* **fairings:** Add cors ([eca952b](https://github.com/itchatapp/api/commit/eca952b3edd1150ba5ebc05aed1c11afd0c9d37d))

## [1.0.1](https://github.com/itchatapp/api/compare/v1.0.0...v1.0.1) (2022-06-09)


### Bug Fixes

* bumping versions ([ccec7b7](https://github.com/itchatapp/api/commit/ccec7b74243d9f11d6151ee1290c5548642ec115))

# 1.0.0 (2022-06-09)


### Bug Fixes

* **accounts:** remove un-needed pub keywords ([c7062cd](https://github.com/itchatapp/api/commit/c7062cd9f579be27a211a301399dcefd322dee49))
* all kinds of bugs ([e1ef258](https://github.com/itchatapp/api/commit/e1ef25864a90024f0c4970a6ec50f1b634b93bac))
* **Channel:** Add missing property ([0387189](https://github.com/itchatapp/api/commit/0387189aba1b9a78ae7eb2fa7f4ed84c91d94080))
* **Permissions:** #fetch returns Result ([68ca42c](https://github.com/itchatapp/api/commit/68ca42c4715af88ac1e74a4f0660400e1f4b7443))
* **permissions:** Check group permissions ([006fa04](https://github.com/itchatapp/api/commit/006fa041efc3ae8ff765f1b3de7496b44be4e9dd))
* **permissions:** No casting needed ([a981e26](https://github.com/itchatapp/api/commit/a981e267b95f5975c451b5eda5a48b9e1adcbc94))
* **ratelimiter:** Send status code ([749c34c](https://github.com/itchatapp/api/commit/749c34ca4863bfe70226198a28c6ec4b90110f4d))
* **Ref:** Ensure the fatched role belongs to server ([797838f](https://github.com/itchatapp/api/commit/797838fe061e47b46d3ab2908b040bae9941c824))
* **Ref:** Make id public ([77feca5](https://github.com/itchatapp/api/commit/77feca508c021a2a6cb59e644e3576b68cfbf590))
* Remove some bloat here and there ([670062f](https://github.com/itchatapp/api/commit/670062fc1e3fdc1bbb65759182c8f2a745301bd5))
* **routes:** Mount /channels ([9bf1d29](https://github.com/itchatapp/api/commit/9bf1d2988605fccf5cd5dd0c6712e021e9942f72))
* separate guards from fairings ([cbd4f1a](https://github.com/itchatapp/api/commit/cbd4f1a09e18f32a2ca69184192c0a0d20b6cd8a))
* Use "Permissions" instead of u64 ([43c863b](https://github.com/itchatapp/api/commit/43c863b5222a8213aa7acf33c4d1b80814fc1c3a))
* Use once_cell instead of lazy_static ([3a9f4b7](https://github.com/itchatapp/api/commit/3a9f4b74b64b57c3e41eb7e8187abab1b7ef4070))
* Use u64 instead i64 ([b9152b4](https://github.com/itchatapp/api/commit/b9152b41a9e8ad444e7a1bcd4fa8fed3f803e9cd))
* **User#fetch_by_token:** Returns Result ([e3c39ba](https://github.com/itchatapp/api/commit/e3c39ba60c32d55b7a01e8de68cfe95bb0bdeca2))
* **utils:** Add unknown keyword for items that's not found ([60459cf](https://github.com/itchatapp/api/commit/60459cfa95f17a33111c67684ebedb3a5b42cf9d))


### Features

* Add global config ([63e781c](https://github.com/itchatapp/api/commit/63e781c6bfd7524077885a0186bd15cb3c3dda3e))
* Add user guard ([fee3322](https://github.com/itchatapp/api/commit/fee332240ec9af17652443a61be405323e6bc567))
* **Badges:** Implement (De)serialize & Default ([ef78aed](https://github.com/itchatapp/api/commit/ef78aede19034408a58172b9a9a6c3a306914f5b))
* **Base:** Add #delete ([f0eb856](https://github.com/itchatapp/api/commit/f0eb856ef506edcf8cf5321e3504bbf77e721f15))
* **Base:** Add #update ([db506a8](https://github.com/itchatapp/api/commit/db506a8b88b31e853dd682445c6c663a053648f2))
* Email verification ([87f35bb](https://github.com/itchatapp/api/commit/87f35bbd38509146d3a57683a04987da3b3ebaf1))
* **fairings:** Add rate limiter ([c9b9adf](https://github.com/itchatapp/api/commit/c9b9adffdc60be8746fbc7b95d6e874f08dd29e4))
* **guards:** Add authentication guard ([e7e3238](https://github.com/itchatapp/api/commit/e7e3238eeef3b05b64809d370c501982ff01846b))
* **guards:** Add captcha ([42abf12](https://github.com/itchatapp/api/commit/42abf12817fbed51e864451fa0add670c6a81951))
* **guards:** Add Ref guard ([7c93fa6](https://github.com/itchatapp/api/commit/7c93fa60231b7bed043117b630fa6ca01a5a1a22))
* Implement the rate limiter ([64a8907](https://github.com/itchatapp/api/commit/64a89079d4a842c49f527c12a9d391de6b5f19cb))
* **Permissions:** Add Manage Invites flag ([4095ca9](https://github.com/itchatapp/api/commit/4095ca9384d8f5068e35505008897d06bd0aad41))
* **Ref:** Add #invite ([bf756d4](https://github.com/itchatapp/api/commit/bf756d42edea7bdec0b2771b13bba89dd4097200))
* **Ref:** Add #member ([e78f286](https://github.com/itchatapp/api/commit/e78f2863bcbaa80c20d4326da358aa79e8327668))
* **Ref:** Add #session ([ce10dae](https://github.com/itchatapp/api/commit/ce10daec9bd1cdb27e2f4a200fb18172a7f9dfd7))
* **Ref:** Add more methods ([2b47cd7](https://github.com/itchatapp/api/commit/2b47cd7ea421bf4c15ba68403d86d295c0a3d937))
* **Ref:** Add option to fetch group/dm channel ([aebab44](https://github.com/itchatapp/api/commit/aebab44c7fe45f8e9df6200ec21f85a17803ed27))
* Replace snowflake module to rbatis built-in plugin ([752f915](https://github.com/itchatapp/api/commit/752f9150368c3f0d04974252776dfad23e4dce99))
* **routes:** Ability to create invites in servers ([c54aaf4](https://github.com/itchatapp/api/commit/c54aaf44cbf5ebaa8774396cfdb92cca908a1f6f))
* **routes:** Add accounts ([9b13fe9](https://github.com/itchatapp/api/commit/9b13fe97d70dbd02eb2b13584993c5dedce366fb))
* **routes:** Add basic server routes ([548f148](https://github.com/itchatapp/api/commit/548f148d2dc2edd07eb6eede1dea0a2f9c4aae6f))
* **routes:** Add basic user routes ([d2173e6](https://github.com/itchatapp/api/commit/d2173e645e428224f379afd955f3619b5fef4241))
* **routes:** Add bots routes ([01de32c](https://github.com/itchatapp/api/commit/01de32c50b790395970573b8064295e51e789e47))
* **routes:** Add DM/Group channel routes ([f932783](https://github.com/itchatapp/api/commit/f932783dbfc34cf2501640a7bb3cec2e94256291))
* **routes:** Add messages route ([c7cf19a](https://github.com/itchatapp/api/commit/c7cf19a6c89893fa04bae62d3cd249d35d272715))
* **routes:** Add server invite routes ([e203d92](https://github.com/itchatapp/api/commit/e203d92fd1350257c17abc8927f2b0cf095f6b27))
* **routes:** Add server members routes ([9b74e22](https://github.com/itchatapp/api/commit/9b74e22e0b3f8e09055f781613661f4c58daeb79))
* **routes:** Add server members routes ([b4d7e87](https://github.com/itchatapp/api/commit/b4d7e870c4fc4a71ecce939dc9c47cd9bb8b3c97))
* **routes:** Add server roles routes ([0de6d43](https://github.com/itchatapp/api/commit/0de6d43a5bfe0991c6f791e9e17b72b2083081ba))
* **routes:** Add sessions ([f023a3f](https://github.com/itchatapp/api/commit/f023a3f48e907b042a33b288e84117fcdb0de6fa))
* **routes:** Completely finish servers routes ([fb13767](https://github.com/itchatapp/api/commit/fb13767ca31b1853f71ef3d8a6a6a5df125c85c1))
* **routes:** Implement Invite routes ([c01b6e8](https://github.com/itchatapp/api/commit/c01b6e80319795ecc82df98dd5cb62f9884a1783))
* **routes:** mount everything ([4a9fd8e](https://github.com/itchatapp/api/commit/4a9fd8ec2dc6c6be5ca51e2618aa3e67bb125be9))
* **User:** Add #is_in_server helper ([736c561](https://github.com/itchatapp/api/commit/736c561b0ec44c836631c1ac5571b96ba039f01b))
* **util:** Implement migration system ([126ad38](https://github.com/itchatapp/api/commit/126ad387a318b0d0a5cbb51158ddc3b9592eadb7))
* **util:** Implement Permissions ([036ca89](https://github.com/itchatapp/api/commit/036ca89ce714ee823e2b266bbb3145c84d9a3e41))
