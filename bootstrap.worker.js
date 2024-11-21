/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
/******/ (() => { // webpackBootstrap
/******/ 	var __webpack_modules__ = ({

/***/ "../../../ngrams lazy recursive ^\\.\\/.*\\/1\\-grams\\.txt$":
/*!**********************************************************************!*\
  !*** ../../../ngrams/ lazy ^\.\/.*\/1\-grams\.txt$ namespace object ***!
  \**********************************************************************/
/***/ ((module, __unused_webpack_exports, __webpack_require__) => {

eval("var map = {\n\t\"./arne/1-grams.txt\": [\n\t\t\"../../../ngrams/arne/1-grams.txt\",\n\t\t\"ngrams_arne_1-grams_txt\"\n\t],\n\t\"./arne_basis/1-grams.txt\": [\n\t\t\"../../../ngrams/arne_basis/1-grams.txt\",\n\t\t\"ngrams_arne_basis_1-grams_txt\"\n\t],\n\t\"./arne_no_special/1-grams.txt\": [\n\t\t\"../../../ngrams/arne_no_special/1-grams.txt\",\n\t\t\"ngrams_arne_no_special_1-grams_txt\"\n\t],\n\t\"./code_actionScript/1-grams.txt\": [\n\t\t\"../../../ngrams/code_actionScript/1-grams.txt\",\n\t\t\"ngrams_code_actionScript_1-grams_txt\"\n\t],\n\t\"./deu_mixed_0.6_eng_news_typical_0.4/1-grams.txt\": [\n\t\t\"../../../ngrams/deu_mixed_0.6_eng_news_typical_0.4/1-grams.txt\",\n\t\t\"ngrams_deu_mixed_0_6_eng_news_typical_0_4_1-grams_txt\"\n\t],\n\t\"./deu_mixed_1m/1-grams.txt\": [\n\t\t\"../../../ngrams/deu_mixed_1m/1-grams.txt\",\n\t\t\"ngrams_deu_mixed_1m_1-grams_txt\"\n\t],\n\t\"./deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4/1-grams.txt\": [\n\t\t\"../../../ngrams/deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4/1-grams.txt\",\n\t\t\"ngrams_deu_mixed_wiki_web_0_6_eng_news_typical_wiki_web_0_4_1-grams_txt\"\n\t],\n\t\"./deu_web_0.6_eng_web_0.4/1-grams.txt\": [\n\t\t\"../../../ngrams/deu_web_0.6_eng_web_0.4/1-grams.txt\",\n\t\t\"ngrams_deu_web_0_6_eng_web_0_4_1-grams_txt\"\n\t],\n\t\"./deu_web_1m/1-grams.txt\": [\n\t\t\"../../../ngrams/deu_web_1m/1-grams.txt\",\n\t\t\"ngrams_deu_web_1m_1-grams_txt\"\n\t],\n\t\"./deu_wiki_0.6_eng_wiki_0.4/1-grams.txt\": [\n\t\t\"../../../ngrams/deu_wiki_0.6_eng_wiki_0.4/1-grams.txt\",\n\t\t\"ngrams_deu_wiki_0_6_eng_wiki_0_4_1-grams_txt\"\n\t],\n\t\"./deu_wiki_1m/1-grams.txt\": [\n\t\t\"../../../ngrams/deu_wiki_1m/1-grams.txt\",\n\t\t\"ngrams_deu_wiki_1m_1-grams_txt\"\n\t],\n\t\"./eng_news_typical_1m/1-grams.txt\": [\n\t\t\"../../../ngrams/eng_news_typical_1m/1-grams.txt\",\n\t\t\"ngrams_eng_news_typical_1m_1-grams_txt\"\n\t],\n\t\"./eng_shai/1-grams.txt\": [\n\t\t\"../../../ngrams/eng_shai/1-grams.txt\",\n\t\t\"ngrams_eng_shai_1-grams_txt\"\n\t],\n\t\"./eng_web_1m/1-grams.txt\": [\n\t\t\"../../../ngrams/eng_web_1m/1-grams.txt\",\n\t\t\"ngrams_eng_web_1m_1-grams_txt\"\n\t],\n\t\"./eng_wiki_1m/1-grams.txt\": [\n\t\t\"../../../ngrams/eng_wiki_1m/1-grams.txt\",\n\t\t\"ngrams_eng_wiki_1m_1-grams_txt\"\n\t],\n\t\"./irc_neo/1-grams.txt\": [\n\t\t\"../../../ngrams/irc_neo/1-grams.txt\",\n\t\t\"ngrams_irc_neo_1-grams_txt\"\n\t],\n\t\"./oxey_english/1-grams.txt\": [\n\t\t\"../../../ngrams/oxey_english/1-grams.txt\",\n\t\t\"ngrams_oxey_english_1-grams_txt\"\n\t],\n\t\"./oxey_english2/1-grams.txt\": [\n\t\t\"../../../ngrams/oxey_english2/1-grams.txt\",\n\t\t\"ngrams_oxey_english2_1-grams_txt\"\n\t],\n\t\"./oxey_german/1-grams.txt\": [\n\t\t\"../../../ngrams/oxey_german/1-grams.txt\",\n\t\t\"ngrams_oxey_german_1-grams_txt\"\n\t]\n};\nfunction webpackAsyncContext(req) {\n\tif(!__webpack_require__.o(map, req)) {\n\t\treturn Promise.resolve().then(() => {\n\t\t\tvar e = new Error(\"Cannot find module '\" + req + \"'\");\n\t\t\te.code = 'MODULE_NOT_FOUND';\n\t\t\tthrow e;\n\t\t});\n\t}\n\n\tvar ids = map[req], id = ids[0];\n\treturn __webpack_require__.e(ids[1]).then(() => {\n\t\treturn __webpack_require__(id);\n\t});\n}\nwebpackAsyncContext.keys = () => (Object.keys(map));\nwebpackAsyncContext.id = \"../../../ngrams lazy recursive ^\\\\.\\\\/.*\\\\/1\\\\-grams\\\\.txt$\";\nmodule.exports = webpackAsyncContext;\n\n//# sourceURL=webpack://create-wasm-app/../../../ngrams/_lazy_^\\.\\/.*\\/1\\-grams\\.txt$_namespace_object?");

/***/ }),

/***/ "../../../ngrams lazy recursive ^\\.\\/.*\\/2\\-grams\\.txt$":
/*!**********************************************************************!*\
  !*** ../../../ngrams/ lazy ^\.\/.*\/2\-grams\.txt$ namespace object ***!
  \**********************************************************************/
/***/ ((module, __unused_webpack_exports, __webpack_require__) => {

eval("var map = {\n\t\"./arne/2-grams.txt\": [\n\t\t\"../../../ngrams/arne/2-grams.txt\",\n\t\t\"ngrams_arne_2-grams_txt\"\n\t],\n\t\"./arne_basis/2-grams.txt\": [\n\t\t\"../../../ngrams/arne_basis/2-grams.txt\",\n\t\t\"ngrams_arne_basis_2-grams_txt\"\n\t],\n\t\"./arne_no_special/2-grams.txt\": [\n\t\t\"../../../ngrams/arne_no_special/2-grams.txt\",\n\t\t\"ngrams_arne_no_special_2-grams_txt\"\n\t],\n\t\"./code_actionScript/2-grams.txt\": [\n\t\t\"../../../ngrams/code_actionScript/2-grams.txt\",\n\t\t\"ngrams_code_actionScript_2-grams_txt\"\n\t],\n\t\"./deu_mixed_0.6_eng_news_typical_0.4/2-grams.txt\": [\n\t\t\"../../../ngrams/deu_mixed_0.6_eng_news_typical_0.4/2-grams.txt\",\n\t\t\"ngrams_deu_mixed_0_6_eng_news_typical_0_4_2-grams_txt\"\n\t],\n\t\"./deu_mixed_1m/2-grams.txt\": [\n\t\t\"../../../ngrams/deu_mixed_1m/2-grams.txt\",\n\t\t\"ngrams_deu_mixed_1m_2-grams_txt\"\n\t],\n\t\"./deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4/2-grams.txt\": [\n\t\t\"../../../ngrams/deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4/2-grams.txt\",\n\t\t\"ngrams_deu_mixed_wiki_web_0_6_eng_news_typical_wiki_web_0_4_2-grams_txt\"\n\t],\n\t\"./deu_web_0.6_eng_web_0.4/2-grams.txt\": [\n\t\t\"../../../ngrams/deu_web_0.6_eng_web_0.4/2-grams.txt\",\n\t\t\"ngrams_deu_web_0_6_eng_web_0_4_2-grams_txt\"\n\t],\n\t\"./deu_web_1m/2-grams.txt\": [\n\t\t\"../../../ngrams/deu_web_1m/2-grams.txt\",\n\t\t\"ngrams_deu_web_1m_2-grams_txt\"\n\t],\n\t\"./deu_wiki_0.6_eng_wiki_0.4/2-grams.txt\": [\n\t\t\"../../../ngrams/deu_wiki_0.6_eng_wiki_0.4/2-grams.txt\",\n\t\t\"ngrams_deu_wiki_0_6_eng_wiki_0_4_2-grams_txt\"\n\t],\n\t\"./deu_wiki_1m/2-grams.txt\": [\n\t\t\"../../../ngrams/deu_wiki_1m/2-grams.txt\",\n\t\t\"ngrams_deu_wiki_1m_2-grams_txt\"\n\t],\n\t\"./eng_news_typical_1m/2-grams.txt\": [\n\t\t\"../../../ngrams/eng_news_typical_1m/2-grams.txt\",\n\t\t\"ngrams_eng_news_typical_1m_2-grams_txt\"\n\t],\n\t\"./eng_shai/2-grams.txt\": [\n\t\t\"../../../ngrams/eng_shai/2-grams.txt\",\n\t\t\"ngrams_eng_shai_2-grams_txt\"\n\t],\n\t\"./eng_web_1m/2-grams.txt\": [\n\t\t\"../../../ngrams/eng_web_1m/2-grams.txt\",\n\t\t\"ngrams_eng_web_1m_2-grams_txt\"\n\t],\n\t\"./eng_wiki_1m/2-grams.txt\": [\n\t\t\"../../../ngrams/eng_wiki_1m/2-grams.txt\",\n\t\t\"ngrams_eng_wiki_1m_2-grams_txt\"\n\t],\n\t\"./irc_neo/2-grams.txt\": [\n\t\t\"../../../ngrams/irc_neo/2-grams.txt\",\n\t\t\"ngrams_irc_neo_2-grams_txt\"\n\t],\n\t\"./oxey_english/2-grams.txt\": [\n\t\t\"../../../ngrams/oxey_english/2-grams.txt\",\n\t\t\"ngrams_oxey_english_2-grams_txt\"\n\t],\n\t\"./oxey_english2/2-grams.txt\": [\n\t\t\"../../../ngrams/oxey_english2/2-grams.txt\",\n\t\t\"ngrams_oxey_english2_2-grams_txt\"\n\t],\n\t\"./oxey_german/2-grams.txt\": [\n\t\t\"../../../ngrams/oxey_german/2-grams.txt\",\n\t\t\"ngrams_oxey_german_2-grams_txt\"\n\t]\n};\nfunction webpackAsyncContext(req) {\n\tif(!__webpack_require__.o(map, req)) {\n\t\treturn Promise.resolve().then(() => {\n\t\t\tvar e = new Error(\"Cannot find module '\" + req + \"'\");\n\t\t\te.code = 'MODULE_NOT_FOUND';\n\t\t\tthrow e;\n\t\t});\n\t}\n\n\tvar ids = map[req], id = ids[0];\n\treturn __webpack_require__.e(ids[1]).then(() => {\n\t\treturn __webpack_require__(id);\n\t});\n}\nwebpackAsyncContext.keys = () => (Object.keys(map));\nwebpackAsyncContext.id = \"../../../ngrams lazy recursive ^\\\\.\\\\/.*\\\\/2\\\\-grams\\\\.txt$\";\nmodule.exports = webpackAsyncContext;\n\n//# sourceURL=webpack://create-wasm-app/../../../ngrams/_lazy_^\\.\\/.*\\/2\\-grams\\.txt$_namespace_object?");

/***/ }),

/***/ "../../../ngrams lazy recursive ^\\.\\/.*\\/3\\-grams\\.txt$":
/*!**********************************************************************!*\
  !*** ../../../ngrams/ lazy ^\.\/.*\/3\-grams\.txt$ namespace object ***!
  \**********************************************************************/
/***/ ((module, __unused_webpack_exports, __webpack_require__) => {

eval("var map = {\n\t\"./arne/3-grams.txt\": [\n\t\t\"../../../ngrams/arne/3-grams.txt\",\n\t\t\"ngrams_arne_3-grams_txt\"\n\t],\n\t\"./arne_basis/3-grams.txt\": [\n\t\t\"../../../ngrams/arne_basis/3-grams.txt\",\n\t\t\"ngrams_arne_basis_3-grams_txt\"\n\t],\n\t\"./arne_no_special/3-grams.txt\": [\n\t\t\"../../../ngrams/arne_no_special/3-grams.txt\",\n\t\t\"ngrams_arne_no_special_3-grams_txt\"\n\t],\n\t\"./code_actionScript/3-grams.txt\": [\n\t\t\"../../../ngrams/code_actionScript/3-grams.txt\",\n\t\t\"ngrams_code_actionScript_3-grams_txt\"\n\t],\n\t\"./deu_mixed_0.6_eng_news_typical_0.4/3-grams.txt\": [\n\t\t\"../../../ngrams/deu_mixed_0.6_eng_news_typical_0.4/3-grams.txt\",\n\t\t\"ngrams_deu_mixed_0_6_eng_news_typical_0_4_3-grams_txt\"\n\t],\n\t\"./deu_mixed_1m/3-grams.txt\": [\n\t\t\"../../../ngrams/deu_mixed_1m/3-grams.txt\",\n\t\t\"ngrams_deu_mixed_1m_3-grams_txt\"\n\t],\n\t\"./deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4/3-grams.txt\": [\n\t\t\"../../../ngrams/deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4/3-grams.txt\",\n\t\t\"ngrams_deu_mixed_wiki_web_0_6_eng_news_typical_wiki_web_0_4_3-grams_txt\"\n\t],\n\t\"./deu_web_0.6_eng_web_0.4/3-grams.txt\": [\n\t\t\"../../../ngrams/deu_web_0.6_eng_web_0.4/3-grams.txt\",\n\t\t\"ngrams_deu_web_0_6_eng_web_0_4_3-grams_txt\"\n\t],\n\t\"./deu_web_1m/3-grams.txt\": [\n\t\t\"../../../ngrams/deu_web_1m/3-grams.txt\",\n\t\t\"ngrams_deu_web_1m_3-grams_txt\"\n\t],\n\t\"./deu_wiki_0.6_eng_wiki_0.4/3-grams.txt\": [\n\t\t\"../../../ngrams/deu_wiki_0.6_eng_wiki_0.4/3-grams.txt\",\n\t\t\"ngrams_deu_wiki_0_6_eng_wiki_0_4_3-grams_txt\"\n\t],\n\t\"./deu_wiki_1m/3-grams.txt\": [\n\t\t\"../../../ngrams/deu_wiki_1m/3-grams.txt\",\n\t\t\"ngrams_deu_wiki_1m_3-grams_txt\"\n\t],\n\t\"./eng_news_typical_1m/3-grams.txt\": [\n\t\t\"../../../ngrams/eng_news_typical_1m/3-grams.txt\",\n\t\t\"ngrams_eng_news_typical_1m_3-grams_txt\"\n\t],\n\t\"./eng_shai/3-grams.txt\": [\n\t\t\"../../../ngrams/eng_shai/3-grams.txt\",\n\t\t\"ngrams_eng_shai_3-grams_txt\"\n\t],\n\t\"./eng_web_1m/3-grams.txt\": [\n\t\t\"../../../ngrams/eng_web_1m/3-grams.txt\",\n\t\t\"ngrams_eng_web_1m_3-grams_txt\"\n\t],\n\t\"./eng_wiki_1m/3-grams.txt\": [\n\t\t\"../../../ngrams/eng_wiki_1m/3-grams.txt\",\n\t\t\"ngrams_eng_wiki_1m_3-grams_txt\"\n\t],\n\t\"./irc_neo/3-grams.txt\": [\n\t\t\"../../../ngrams/irc_neo/3-grams.txt\",\n\t\t\"ngrams_irc_neo_3-grams_txt\"\n\t],\n\t\"./oxey_english/3-grams.txt\": [\n\t\t\"../../../ngrams/oxey_english/3-grams.txt\",\n\t\t\"ngrams_oxey_english_3-grams_txt\"\n\t],\n\t\"./oxey_english2/3-grams.txt\": [\n\t\t\"../../../ngrams/oxey_english2/3-grams.txt\",\n\t\t\"ngrams_oxey_english2_3-grams_txt\"\n\t],\n\t\"./oxey_german/3-grams.txt\": [\n\t\t\"../../../ngrams/oxey_german/3-grams.txt\",\n\t\t\"ngrams_oxey_german_3-grams_txt\"\n\t]\n};\nfunction webpackAsyncContext(req) {\n\tif(!__webpack_require__.o(map, req)) {\n\t\treturn Promise.resolve().then(() => {\n\t\t\tvar e = new Error(\"Cannot find module '\" + req + \"'\");\n\t\t\te.code = 'MODULE_NOT_FOUND';\n\t\t\tthrow e;\n\t\t});\n\t}\n\n\tvar ids = map[req], id = ids[0];\n\treturn __webpack_require__.e(ids[1]).then(() => {\n\t\treturn __webpack_require__(id);\n\t});\n}\nwebpackAsyncContext.keys = () => (Object.keys(map));\nwebpackAsyncContext.id = \"../../../ngrams lazy recursive ^\\\\.\\\\/.*\\\\/3\\\\-grams\\\\.txt$\";\nmodule.exports = webpackAsyncContext;\n\n//# sourceURL=webpack://create-wasm-app/../../../ngrams/_lazy_^\\.\\/.*\\/3\\-grams\\.txt$_namespace_object?");

/***/ }),

/***/ "./worker.js":
/*!*******************!*\
  !*** ./worker.js ***!
  \*******************/
/***/ ((__unused_webpack_module, __unused_webpack_exports, __webpack_require__) => {

eval("importScripts(\"https://unpkg.com/comlink/dist/umd/comlink.js\")\n\nconst evaluator = {\n  wasm: null,\n  ngramProvider: null,\n  layoutEvaluator: null,\n  layoutOptimizer: null,\n\n  init() {\n    return __webpack_require__.e(/*! import() */ \"pkg_layout_evaluation_wasm_js\").then(__webpack_require__.bind(__webpack_require__, /*! evolve-keyboard-layout-wasm */ \"../pkg/layout_evaluation_wasm.js\"))\n      .then((wasm) => {\n        this.wasm = wasm\n      })\n  },\n\n  async initNgramProvider(ngramType, evalParams, ngramData) {\n    if (ngramType === 'prepared') {\n      let unigrams = await __webpack_require__(\"../../../ngrams lazy recursive ^\\\\.\\\\/.*\\\\/1\\\\-grams\\\\.txt$\")(`./${ngramData}/1-grams.txt`)\n        .then((ngrams) => ngrams.default)\n      let bigrams = await __webpack_require__(\"../../../ngrams lazy recursive ^\\\\.\\\\/.*\\\\/2\\\\-grams\\\\.txt$\")(`./${ngramData}/2-grams.txt`)\n        .then((ngrams) => ngrams.default)\n      let trigrams = await __webpack_require__(\"../../../ngrams lazy recursive ^\\\\.\\\\/.*\\\\/3\\\\-grams\\\\.txt$\")(`./${ngramData}/3-grams.txt`)\n        .then((ngrams) => ngrams.default)\n\n      this.ngramProvider = this.wasm.NgramProvider.with_frequencies(\n        evalParams,\n        unigrams,\n        bigrams,\n        trigrams\n      )\n    } else if (ngramType === 'from_text') {\n      this.ngramProvider = this.wasm.NgramProvider.with_text(\n        evalParams,\n        ngramData\n      )\n    }\n  },\n\n  initLayoutEvaluator(layoutConfig, evalParams) {\n    this.layoutEvaluator = this.wasm.LayoutEvaluator.new(\n      layoutConfig,\n      evalParams,\n      this.ngramProvider,\n    )\n  },\n\n  async saOptimize(layout, fixed_chars, optParamsStr, initCallbacks, setCurrentStepNr, setNewBest) {\n    // Needed to make the callbacks work in Firefox.\n    // In other browsers (for example in Chromium or Midori), this isn't necessary.\n    // In those browsers, the whole function can be turned into a syncronous one.\n    await initCallbacks()\n    this.wasm.sa_optimize(\n      layout,\n      optParamsStr,\n      this.layoutEvaluator,\n      fixed_chars,\n      true,\n      setCurrentStepNr,\n      setNewBest,\n    )\n  },\n\n  initGenLayoutOptimizer(layout, fixed_chars, optParamsStr) {\n    this.layoutOptimizer = this.wasm.LayoutOptimizer.new(\n      layout,\n      optParamsStr,\n      this.layoutEvaluator,\n      fixed_chars,\n      true,\n    )\n\n    return this.layoutOptimizer.parameters()\n  },\n  genOptimizationStep() {\n    return this.layoutOptimizer.step()\n  },\n\n  evaluateLayout(layout) {\n    let res = this.layoutEvaluator.evaluate(layout)\n    res.layout = layout\n    return res\n  },\n\n  permutableKeys() {\n    return this.layoutEvaluator.permutable_keys()\n  },\n}\n\nComlink.expose(evaluator)\n\n\n\n//# sourceURL=webpack://create-wasm-app/./worker.js?");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			id: moduleId,
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = __webpack_modules__;
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/async module */
/******/ 	(() => {
/******/ 		var webpackQueues = typeof Symbol === "function" ? Symbol("webpack queues") : "__webpack_queues__";
/******/ 		var webpackExports = typeof Symbol === "function" ? Symbol("webpack exports") : "__webpack_exports__";
/******/ 		var webpackError = typeof Symbol === "function" ? Symbol("webpack error") : "__webpack_error__";
/******/ 		var resolveQueue = (queue) => {
/******/ 			if(queue && queue.d < 1) {
/******/ 				queue.d = 1;
/******/ 				queue.forEach((fn) => (fn.r--));
/******/ 				queue.forEach((fn) => (fn.r-- ? fn.r++ : fn()));
/******/ 			}
/******/ 		}
/******/ 		var wrapDeps = (deps) => (deps.map((dep) => {
/******/ 			if(dep !== null && typeof dep === "object") {
/******/ 				if(dep[webpackQueues]) return dep;
/******/ 				if(dep.then) {
/******/ 					var queue = [];
/******/ 					queue.d = 0;
/******/ 					dep.then((r) => {
/******/ 						obj[webpackExports] = r;
/******/ 						resolveQueue(queue);
/******/ 					}, (e) => {
/******/ 						obj[webpackError] = e;
/******/ 						resolveQueue(queue);
/******/ 					});
/******/ 					var obj = {};
/******/ 					obj[webpackQueues] = (fn) => (fn(queue));
/******/ 					return obj;
/******/ 				}
/******/ 			}
/******/ 			var ret = {};
/******/ 			ret[webpackQueues] = x => {};
/******/ 			ret[webpackExports] = dep;
/******/ 			return ret;
/******/ 		}));
/******/ 		__webpack_require__.a = (module, body, hasAwait) => {
/******/ 			var queue;
/******/ 			hasAwait && ((queue = []).d = -1);
/******/ 			var depQueues = new Set();
/******/ 			var exports = module.exports;
/******/ 			var currentDeps;
/******/ 			var outerResolve;
/******/ 			var reject;
/******/ 			var promise = new Promise((resolve, rej) => {
/******/ 				reject = rej;
/******/ 				outerResolve = resolve;
/******/ 			});
/******/ 			promise[webpackExports] = exports;
/******/ 			promise[webpackQueues] = (fn) => (queue && fn(queue), depQueues.forEach(fn), promise["catch"](x => {}));
/******/ 			module.exports = promise;
/******/ 			body((deps) => {
/******/ 				currentDeps = wrapDeps(deps);
/******/ 				var fn;
/******/ 				var getResult = () => (currentDeps.map((d) => {
/******/ 					if(d[webpackError]) throw d[webpackError];
/******/ 					return d[webpackExports];
/******/ 				}))
/******/ 				var promise = new Promise((resolve) => {
/******/ 					fn = () => (resolve(getResult));
/******/ 					fn.r = 0;
/******/ 					var fnQueue = (q) => (q !== queue && !depQueues.has(q) && (depQueues.add(q), q && !q.d && (fn.r++, q.push(fn))));
/******/ 					currentDeps.map((dep) => (dep[webpackQueues](fnQueue)));
/******/ 				});
/******/ 				return fn.r ? promise : getResult();
/******/ 			}, (err) => ((err ? reject(promise[webpackError] = err) : outerResolve(exports)), resolveQueue(queue)));
/******/ 			queue && queue.d < 0 && (queue.d = 0);
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/ensure chunk */
/******/ 	(() => {
/******/ 		__webpack_require__.f = {};
/******/ 		// This file contains only the entry chunk.
/******/ 		// The chunk loading function for additional chunks
/******/ 		__webpack_require__.e = (chunkId) => {
/******/ 			return Promise.all(Object.keys(__webpack_require__.f).reduce((promises, key) => {
/******/ 				__webpack_require__.f[key](chunkId, promises);
/******/ 				return promises;
/******/ 			}, []));
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/get javascript chunk filename */
/******/ 	(() => {
/******/ 		// This function allow to reference async chunks
/******/ 		__webpack_require__.u = (chunkId) => {
/******/ 			// return url for filenames based on template
/******/ 			return "" + chunkId + ".bootstrap.worker.js";
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/global */
/******/ 	(() => {
/******/ 		__webpack_require__.g = (function() {
/******/ 			if (typeof globalThis === 'object') return globalThis;
/******/ 			try {
/******/ 				return this || new Function('return this')();
/******/ 			} catch (e) {
/******/ 				if (typeof window === 'object') return window;
/******/ 			}
/******/ 		})();
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/wasm loading */
/******/ 	(() => {
/******/ 		__webpack_require__.v = (exports, wasmModuleId, wasmModuleHash, importsObj) => {
/******/ 			var req = fetch(__webpack_require__.p + "" + wasmModuleHash + ".module.wasm");
/******/ 			var fallback = () => (req
/******/ 				.then((x) => (x.arrayBuffer()))
/******/ 				.then((bytes) => (WebAssembly.instantiate(bytes, importsObj)))
/******/ 				.then((res) => (Object.assign(exports, res.instance.exports))));
/******/ 			return req.then((res) => {
/******/ 				if (typeof WebAssembly.instantiateStreaming === "function") {
/******/ 					return WebAssembly.instantiateStreaming(res, importsObj)
/******/ 						.then(
/******/ 							(res) => (Object.assign(exports, res.instance.exports)),
/******/ 							(e) => {
/******/ 								if(res.headers.get("Content-Type") !== "application/wasm") {
/******/ 									console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
/******/ 									return fallback();
/******/ 								}
/******/ 								throw e;
/******/ 							}
/******/ 						);
/******/ 				}
/******/ 				return fallback();
/******/ 			});
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/publicPath */
/******/ 	(() => {
/******/ 		var scriptUrl;
/******/ 		if (__webpack_require__.g.importScripts) scriptUrl = __webpack_require__.g.location + "";
/******/ 		var document = __webpack_require__.g.document;
/******/ 		if (!scriptUrl && document) {
/******/ 			if (document.currentScript && document.currentScript.tagName.toUpperCase() === 'SCRIPT')
/******/ 				scriptUrl = document.currentScript.src;
/******/ 			if (!scriptUrl) {
/******/ 				var scripts = document.getElementsByTagName("script");
/******/ 				if(scripts.length) {
/******/ 					var i = scripts.length - 1;
/******/ 					while (i > -1 && (!scriptUrl || !/^http(s?):/.test(scriptUrl))) scriptUrl = scripts[i--].src;
/******/ 				}
/******/ 			}
/******/ 		}
/******/ 		// When supporting browsers where an automatic publicPath is not supported you must specify an output.publicPath manually via configuration
/******/ 		// or pass an empty string ("") and set the __webpack_public_path__ variable from your code to use your own logic.
/******/ 		if (!scriptUrl) throw new Error("Automatic publicPath is not supported in this browser");
/******/ 		scriptUrl = scriptUrl.replace(/#.*$/, "").replace(/\?.*$/, "").replace(/\/[^\/]+$/, "/");
/******/ 		__webpack_require__.p = scriptUrl;
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/importScripts chunk loading */
/******/ 	(() => {
/******/ 		// no baseURI
/******/ 		
/******/ 		// object to store loaded chunks
/******/ 		// "1" means "already loaded"
/******/ 		var installedChunks = {
/******/ 			"worker": 1
/******/ 		};
/******/ 		
/******/ 		// importScripts chunk loading
/******/ 		var installChunk = (data) => {
/******/ 			var [chunkIds, moreModules, runtime] = data;
/******/ 			for(var moduleId in moreModules) {
/******/ 				if(__webpack_require__.o(moreModules, moduleId)) {
/******/ 					__webpack_require__.m[moduleId] = moreModules[moduleId];
/******/ 				}
/******/ 			}
/******/ 			if(runtime) runtime(__webpack_require__);
/******/ 			while(chunkIds.length)
/******/ 				installedChunks[chunkIds.pop()] = 1;
/******/ 			parentChunkLoadingFunction(data);
/******/ 		};
/******/ 		__webpack_require__.f.i = (chunkId, promises) => {
/******/ 			// "1" is the signal for "already loaded"
/******/ 			if(!installedChunks[chunkId]) {
/******/ 				if(true) { // all chunks have JS
/******/ 					importScripts(__webpack_require__.p + __webpack_require__.u(chunkId));
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 		
/******/ 		var chunkLoadingGlobal = self["webpackChunkcreate_wasm_app"] = self["webpackChunkcreate_wasm_app"] || [];
/******/ 		var parentChunkLoadingFunction = chunkLoadingGlobal.push.bind(chunkLoadingGlobal);
/******/ 		chunkLoadingGlobal.push = installChunk;
/******/ 		
/******/ 		// no HMR
/******/ 		
/******/ 		// no HMR manifest
/******/ 	})();
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module can't be inlined because the eval devtool is used.
/******/ 	var __webpack_exports__ = __webpack_require__("./worker.js");
/******/ 	
/******/ })()
;