/******/ (function(modules) { // webpackBootstrap
/******/ 	self["webpackChunk"] = function webpackChunkCallback(chunkIds, moreModules) {
/******/ 		for(var moduleId in moreModules) {
/******/ 			modules[moduleId] = moreModules[moduleId];
/******/ 		}
/******/ 		while(chunkIds.length)
/******/ 			installedChunks[chunkIds.pop()] = 1;
/******/ 	};
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded chunks
/******/ 	// "1" means "already loaded"
/******/ 	var installedChunks = {
/******/ 		"worker": 1
/******/ 	};
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"../pkg/evolve_keyboard_layout_wasm_bg.wasm": function() {
/******/ 			return {
/******/ 				"./evolve_keyboard_layout_wasm_bg.js": {
/******/ 					"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbindgen_string_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_json_parse": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbindgen_json_parse"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_new_693216e109162396": function() {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_new_693216e109162396"]();
/******/ 					},
/******/ 					"__wbg_stack_0ddaca5d1abfb52f": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_stack_0ddaca5d1abfb52f"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_error_09919627ac0992f5": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_error_09919627ac0992f5"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbg_getRandomValues_98117e9a7e993920": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_getRandomValues_98117e9a7e993920"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_randomFillSync_64cc7d048f228ca8": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_randomFillSync_64cc7d048f228ca8"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_process_2f24d6544ea7b200": function(p0i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_process_2f24d6544ea7b200"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_is_object": function(p0i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbindgen_is_object"](p0i32);
/******/ 					},
/******/ 					"__wbg_versions_6164651e75405d4a": function(p0i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_versions_6164651e75405d4a"](p0i32);
/******/ 					},
/******/ 					"__wbg_node_4b517d861cbcb3bc": function(p0i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_node_4b517d861cbcb3bc"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_is_string": function(p0i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbindgen_is_string"](p0i32);
/******/ 					},
/******/ 					"__wbg_modulerequire_3440a4bcf44437db": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_modulerequire_3440a4bcf44437db"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_crypto_98fc271021c7d2ad": function(p0i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_crypto_98fc271021c7d2ad"](p0i32);
/******/ 					},
/******/ 					"__wbg_msCrypto_a2cdb043d2bfe57f": function(p0i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_msCrypto_a2cdb043d2bfe57f"](p0i32);
/******/ 					},
/******/ 					"__wbg_newnoargs_be86524d73f67598": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_newnoargs_be86524d73f67598"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_888d259a5fefc347": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_call_888d259a5fefc347"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getTime_10d33f4f2959e5dd": function(p0i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_getTime_10d33f4f2959e5dd"](p0i32);
/******/ 					},
/******/ 					"__wbg_getTimezoneOffset_d3e5a22a1b7fb1d8": function(p0i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_getTimezoneOffset_d3e5a22a1b7fb1d8"](p0i32);
/******/ 					},
/******/ 					"__wbg_new0_fd3a3a290b25cdac": function() {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_new0_fd3a3a290b25cdac"]();
/******/ 					},
/******/ 					"__wbg_self_c6fbdfc2918d5e58": function() {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_self_c6fbdfc2918d5e58"]();
/******/ 					},
/******/ 					"__wbg_window_baec038b5ab35c54": function() {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_window_baec038b5ab35c54"]();
/******/ 					},
/******/ 					"__wbg_globalThis_3f735a5746d41fbd": function() {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_globalThis_3f735a5746d41fbd"]();
/******/ 					},
/******/ 					"__wbg_global_1bc0b39582740e95": function() {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_global_1bc0b39582740e95"]();
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbg_buffer_397eaa4d72ee94dd": function(p0i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_buffer_397eaa4d72ee94dd"](p0i32);
/******/ 					},
/******/ 					"__wbg_new_a7ce447f15ff496f": function(p0i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_new_a7ce447f15ff496f"](p0i32);
/******/ 					},
/******/ 					"__wbg_set_969ad0a60e51d320": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_set_969ad0a60e51d320"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_length_1eb8fc608a0d4cdb": function(p0i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_length_1eb8fc608a0d4cdb"](p0i32);
/******/ 					},
/******/ 					"__wbg_newwithlength_929232475839a482": function(p0i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_newwithlength_929232475839a482"](p0i32);
/******/ 					},
/******/ 					"__wbg_subarray_8b658422a224f479": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbg_subarray_8b658422a224f479"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbindgen_object_clone_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_rethrow": function(p0i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbindgen_rethrow"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_memory": function() {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbindgen_memory"]();
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/ 		promises.push(Promise.resolve().then(function() {
/******/ 			// "1" is the signal for "already loaded"
/******/ 			if(!installedChunks[chunkId]) {
/******/ 				importScripts(__webpack_require__.p + "" + chunkId + ".bootstrap.worker.js");
/******/ 			}
/******/ 		}));
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"0":["../pkg/evolve_keyboard_layout_wasm_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"../pkg/evolve_keyboard_layout_wasm_bg.wasm":"1252227644c2fd6bd3ed"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./worker.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "../../corpus lazy recursive ^\\.\\/.*\\/1\\-grams\\.txt$":
/*!**********************************************************************************************************************************!*\
  !*** /home/runner/work/keyboard_layout_optimizer/keyboard_layout_optimizer/corpus lazy ^\.\/.*\/1\-grams\.txt$ namespace object ***!
  \**********************************************************************************************************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("var map = {\n\t\"./arne/1-grams.txt\": [\n\t\t\"../../corpus/arne/1-grams.txt\",\n\t\t1\n\t],\n\t\"./arne_basis/1-grams.txt\": [\n\t\t\"../../corpus/arne_basis/1-grams.txt\",\n\t\t4\n\t],\n\t\"./arne_no_special/1-grams.txt\": [\n\t\t\"../../corpus/arne_no_special/1-grams.txt\",\n\t\t7\n\t],\n\t\"./deu_mixed_0.6_eng_news_typical_0.4/1-grams.txt\": [\n\t\t\"../../corpus/deu_mixed_0.6_eng_news_typical_0.4/1-grams.txt\",\n\t\t10\n\t],\n\t\"./deu_mixed_1m/1-grams.txt\": [\n\t\t\"../../corpus/deu_mixed_1m/1-grams.txt\",\n\t\t13\n\t],\n\t\"./deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4/1-grams.txt\": [\n\t\t\"../../corpus/deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4/1-grams.txt\",\n\t\t16\n\t],\n\t\"./deu_web_0.6_eng_web_0.4/1-grams.txt\": [\n\t\t\"../../corpus/deu_web_0.6_eng_web_0.4/1-grams.txt\",\n\t\t19\n\t],\n\t\"./deu_web_1m/1-grams.txt\": [\n\t\t\"../../corpus/deu_web_1m/1-grams.txt\",\n\t\t22\n\t],\n\t\"./deu_wiki_0.6_eng_wiki_0.4/1-grams.txt\": [\n\t\t\"../../corpus/deu_wiki_0.6_eng_wiki_0.4/1-grams.txt\",\n\t\t25\n\t],\n\t\"./deu_wiki_1m/1-grams.txt\": [\n\t\t\"../../corpus/deu_wiki_1m/1-grams.txt\",\n\t\t28\n\t],\n\t\"./eng_news_typical_1m/1-grams.txt\": [\n\t\t\"../../corpus/eng_news_typical_1m/1-grams.txt\",\n\t\t31\n\t],\n\t\"./eng_web_1m/1-grams.txt\": [\n\t\t\"../../corpus/eng_web_1m/1-grams.txt\",\n\t\t34\n\t],\n\t\"./eng_wiki_1m/1-grams.txt\": [\n\t\t\"../../corpus/eng_wiki_1m/1-grams.txt\",\n\t\t37\n\t]\n};\nfunction webpackAsyncContext(req) {\n\tif(!__webpack_require__.o(map, req)) {\n\t\treturn Promise.resolve().then(function() {\n\t\t\tvar e = new Error(\"Cannot find module '\" + req + \"'\");\n\t\t\te.code = 'MODULE_NOT_FOUND';\n\t\t\tthrow e;\n\t\t});\n\t}\n\n\tvar ids = map[req], id = ids[0];\n\treturn __webpack_require__.e(ids[1]).then(function() {\n\t\treturn __webpack_require__(id);\n\t});\n}\nwebpackAsyncContext.keys = function webpackAsyncContextKeys() {\n\treturn Object.keys(map);\n};\nwebpackAsyncContext.id = \"../../corpus lazy recursive ^\\\\.\\\\/.*\\\\/1\\\\-grams\\\\.txt$\";\nmodule.exports = webpackAsyncContext;\n\n//# sourceURL=webpack:////home/runner/work/keyboard_layout_optimizer/keyboard_layout_optimizer/corpus_lazy_^\\.\\/.*\\/1\\-grams\\.txt$_namespace_object?");

/***/ }),

/***/ "../../corpus lazy recursive ^\\.\\/.*\\/2\\-grams\\.txt$":
/*!**********************************************************************************************************************************!*\
  !*** /home/runner/work/keyboard_layout_optimizer/keyboard_layout_optimizer/corpus lazy ^\.\/.*\/2\-grams\.txt$ namespace object ***!
  \**********************************************************************************************************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("var map = {\n\t\"./arne/2-grams.txt\": [\n\t\t\"../../corpus/arne/2-grams.txt\",\n\t\t2\n\t],\n\t\"./arne_basis/2-grams.txt\": [\n\t\t\"../../corpus/arne_basis/2-grams.txt\",\n\t\t5\n\t],\n\t\"./arne_no_special/2-grams.txt\": [\n\t\t\"../../corpus/arne_no_special/2-grams.txt\",\n\t\t8\n\t],\n\t\"./deu_mixed_0.6_eng_news_typical_0.4/2-grams.txt\": [\n\t\t\"../../corpus/deu_mixed_0.6_eng_news_typical_0.4/2-grams.txt\",\n\t\t11\n\t],\n\t\"./deu_mixed_1m/2-grams.txt\": [\n\t\t\"../../corpus/deu_mixed_1m/2-grams.txt\",\n\t\t14\n\t],\n\t\"./deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4/2-grams.txt\": [\n\t\t\"../../corpus/deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4/2-grams.txt\",\n\t\t17\n\t],\n\t\"./deu_web_0.6_eng_web_0.4/2-grams.txt\": [\n\t\t\"../../corpus/deu_web_0.6_eng_web_0.4/2-grams.txt\",\n\t\t20\n\t],\n\t\"./deu_web_1m/2-grams.txt\": [\n\t\t\"../../corpus/deu_web_1m/2-grams.txt\",\n\t\t23\n\t],\n\t\"./deu_wiki_0.6_eng_wiki_0.4/2-grams.txt\": [\n\t\t\"../../corpus/deu_wiki_0.6_eng_wiki_0.4/2-grams.txt\",\n\t\t26\n\t],\n\t\"./deu_wiki_1m/2-grams.txt\": [\n\t\t\"../../corpus/deu_wiki_1m/2-grams.txt\",\n\t\t29\n\t],\n\t\"./eng_news_typical_1m/2-grams.txt\": [\n\t\t\"../../corpus/eng_news_typical_1m/2-grams.txt\",\n\t\t32\n\t],\n\t\"./eng_web_1m/2-grams.txt\": [\n\t\t\"../../corpus/eng_web_1m/2-grams.txt\",\n\t\t35\n\t],\n\t\"./eng_wiki_1m/2-grams.txt\": [\n\t\t\"../../corpus/eng_wiki_1m/2-grams.txt\",\n\t\t38\n\t]\n};\nfunction webpackAsyncContext(req) {\n\tif(!__webpack_require__.o(map, req)) {\n\t\treturn Promise.resolve().then(function() {\n\t\t\tvar e = new Error(\"Cannot find module '\" + req + \"'\");\n\t\t\te.code = 'MODULE_NOT_FOUND';\n\t\t\tthrow e;\n\t\t});\n\t}\n\n\tvar ids = map[req], id = ids[0];\n\treturn __webpack_require__.e(ids[1]).then(function() {\n\t\treturn __webpack_require__(id);\n\t});\n}\nwebpackAsyncContext.keys = function webpackAsyncContextKeys() {\n\treturn Object.keys(map);\n};\nwebpackAsyncContext.id = \"../../corpus lazy recursive ^\\\\.\\\\/.*\\\\/2\\\\-grams\\\\.txt$\";\nmodule.exports = webpackAsyncContext;\n\n//# sourceURL=webpack:////home/runner/work/keyboard_layout_optimizer/keyboard_layout_optimizer/corpus_lazy_^\\.\\/.*\\/2\\-grams\\.txt$_namespace_object?");

/***/ }),

/***/ "../../corpus lazy recursive ^\\.\\/.*\\/3\\-grams\\.txt$":
/*!**********************************************************************************************************************************!*\
  !*** /home/runner/work/keyboard_layout_optimizer/keyboard_layout_optimizer/corpus lazy ^\.\/.*\/3\-grams\.txt$ namespace object ***!
  \**********************************************************************************************************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("var map = {\n\t\"./arne/3-grams.txt\": [\n\t\t\"../../corpus/arne/3-grams.txt\",\n\t\t3\n\t],\n\t\"./arne_basis/3-grams.txt\": [\n\t\t\"../../corpus/arne_basis/3-grams.txt\",\n\t\t6\n\t],\n\t\"./arne_no_special/3-grams.txt\": [\n\t\t\"../../corpus/arne_no_special/3-grams.txt\",\n\t\t9\n\t],\n\t\"./deu_mixed_0.6_eng_news_typical_0.4/3-grams.txt\": [\n\t\t\"../../corpus/deu_mixed_0.6_eng_news_typical_0.4/3-grams.txt\",\n\t\t12\n\t],\n\t\"./deu_mixed_1m/3-grams.txt\": [\n\t\t\"../../corpus/deu_mixed_1m/3-grams.txt\",\n\t\t15\n\t],\n\t\"./deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4/3-grams.txt\": [\n\t\t\"../../corpus/deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4/3-grams.txt\",\n\t\t18\n\t],\n\t\"./deu_web_0.6_eng_web_0.4/3-grams.txt\": [\n\t\t\"../../corpus/deu_web_0.6_eng_web_0.4/3-grams.txt\",\n\t\t21\n\t],\n\t\"./deu_web_1m/3-grams.txt\": [\n\t\t\"../../corpus/deu_web_1m/3-grams.txt\",\n\t\t24\n\t],\n\t\"./deu_wiki_0.6_eng_wiki_0.4/3-grams.txt\": [\n\t\t\"../../corpus/deu_wiki_0.6_eng_wiki_0.4/3-grams.txt\",\n\t\t27\n\t],\n\t\"./deu_wiki_1m/3-grams.txt\": [\n\t\t\"../../corpus/deu_wiki_1m/3-grams.txt\",\n\t\t30\n\t],\n\t\"./eng_news_typical_1m/3-grams.txt\": [\n\t\t\"../../corpus/eng_news_typical_1m/3-grams.txt\",\n\t\t33\n\t],\n\t\"./eng_web_1m/3-grams.txt\": [\n\t\t\"../../corpus/eng_web_1m/3-grams.txt\",\n\t\t36\n\t],\n\t\"./eng_wiki_1m/3-grams.txt\": [\n\t\t\"../../corpus/eng_wiki_1m/3-grams.txt\",\n\t\t39\n\t]\n};\nfunction webpackAsyncContext(req) {\n\tif(!__webpack_require__.o(map, req)) {\n\t\treturn Promise.resolve().then(function() {\n\t\t\tvar e = new Error(\"Cannot find module '\" + req + \"'\");\n\t\t\te.code = 'MODULE_NOT_FOUND';\n\t\t\tthrow e;\n\t\t});\n\t}\n\n\tvar ids = map[req], id = ids[0];\n\treturn __webpack_require__.e(ids[1]).then(function() {\n\t\treturn __webpack_require__(id);\n\t});\n}\nwebpackAsyncContext.keys = function webpackAsyncContextKeys() {\n\treturn Object.keys(map);\n};\nwebpackAsyncContext.id = \"../../corpus lazy recursive ^\\\\.\\\\/.*\\\\/3\\\\-grams\\\\.txt$\";\nmodule.exports = webpackAsyncContext;\n\n//# sourceURL=webpack:////home/runner/work/keyboard_layout_optimizer/keyboard_layout_optimizer/corpus_lazy_^\\.\\/.*\\/3\\-grams\\.txt$_namespace_object?");

/***/ }),

/***/ "./worker.js":
/*!*******************!*\
  !*** ./worker.js ***!
  \*******************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("importScripts(\"https://unpkg.com/comlink/dist/umd/comlink.js\");\n\nconst evaluator = {\n\n    wasm: null,\n    ngramProvider: null,\n    layoutEvaluator: null,\n    layoutOptimizer: null,\n\n    init () {\n        return __webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! evolve-keyboard-layout-wasm */ \"../pkg/evolve_keyboard_layout_wasm.js\"))\n            .then((wasm) => {\n                this.wasm = wasm\n            })\n    },\n\n    async initNgramProvider (ngramType, evalParams, ngramData) {\n        if (ngramType === 'prepared') {\n            let unigrams = await __webpack_require__(\"../../corpus lazy recursive ^\\\\.\\\\/.*\\\\/1\\\\-grams\\\\.txt$\")(`./${ngramData}/1-grams.txt`)\n                .then((ngrams) => ngrams.default)\n            let bigrams = await __webpack_require__(\"../../corpus lazy recursive ^\\\\.\\\\/.*\\\\/2\\\\-grams\\\\.txt$\")(`./${ngramData}/2-grams.txt`)\n                .then((ngrams) => ngrams.default)\n            let trigrams = await __webpack_require__(\"../../corpus lazy recursive ^\\\\.\\\\/.*\\\\/3\\\\-grams\\\\.txt$\")(`./${ngramData}/3-grams.txt`)\n                .then((ngrams) => ngrams.default)\n\n            this.ngramProvider = this.wasm.NgramProvider.with_frequencies(\n                evalParams,\n                unigrams,\n                bigrams,\n                trigrams\n            )\n        } else if (ngramType === 'from_text') {\n            this.ngramProvider = this.wasm.NgramProvider.with_text(\n                evalParams,\n                ngramData\n            )\n        }\n    },\n\n    initLayoutEvaluator (layoutConfig, evalParams) {\n        this.layoutEvaluator = this.wasm.LayoutEvaluator.new(\n            layoutConfig,\n            evalParams,\n            this.ngramProvider,\n        )\n    },\n\n    initLayoutOptimizer (layout, fixed_chars, optParams) {\n        this.layoutOptimizer = this.wasm.LayoutOptimizer.new(\n            layout,\n            optParams,\n            this.layoutEvaluator,\n            fixed_chars,\n            true,\n        )\n\n        return this.layoutOptimizer.parameters()\n    },\n\n    optimizationStep () {\n        return this.layoutOptimizer.step()\n    },\n\n    evaluateLayout (layout) {\n        let res = this.layoutEvaluator.evaluate(layout)\n        res.layout = layout\n        return res\n    },\n\n    permutableKeys () {\n        return this.layoutEvaluator.permutable_keys()\n    }\n}\n\nComlink.expose(evaluator)\n\n\n\n//# sourceURL=webpack:///./worker.js?");

/***/ })

/******/ });