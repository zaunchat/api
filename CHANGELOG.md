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
