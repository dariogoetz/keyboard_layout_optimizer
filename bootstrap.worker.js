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
/******/ 		"../pkg/layout_evaluation_wasm_bg.wasm": function() {
/******/ 			return {
/******/ 				"./layout_evaluation_wasm_bg.js": {
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_bigint_new": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbindgen_bigint_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbindgen_string_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_number_new": function(p0f64) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbindgen_number_new"](p0f64);
/******/ 					},
/******/ 					"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbindgen_object_clone_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_json_parse": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbindgen_json_parse"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_new_693216e109162396": function() {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_new_693216e109162396"]();
/******/ 					},
/******/ 					"__wbg_stack_0ddaca5d1abfb52f": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_stack_0ddaca5d1abfb52f"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_error_09919627ac0992f5": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_error_09919627ac0992f5"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_now_20d2aadcf3cc17f7": function(p0i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_now_20d2aadcf3cc17f7"](p0i32);
/******/ 					},
/******/ 					"__wbg_getRandomValues_fb6b088efb6bead2": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_getRandomValues_fb6b088efb6bead2"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_randomFillSync_654a7797990fb8db": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_randomFillSync_654a7797990fb8db"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_static_accessor_NODE_MODULE_33b45247c55045b0": function() {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_static_accessor_NODE_MODULE_33b45247c55045b0"]();
/******/ 					},
/******/ 					"__wbg_process_70251ed1291754d5": function(p0i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_process_70251ed1291754d5"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_is_object": function(p0i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbindgen_is_object"](p0i32);
/******/ 					},
/******/ 					"__wbg_versions_b23f2588cdb2ddbb": function(p0i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_versions_b23f2588cdb2ddbb"](p0i32);
/******/ 					},
/******/ 					"__wbg_node_61b8c9a82499895d": function(p0i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_node_61b8c9a82499895d"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_is_string": function(p0i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbindgen_is_string"](p0i32);
/******/ 					},
/******/ 					"__wbg_require_2a93bc09fee45aca": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_require_2a93bc09fee45aca"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_crypto_2f56257a38275dbd": function(p0i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_crypto_2f56257a38275dbd"](p0i32);
/******/ 					},
/******/ 					"__wbg_msCrypto_d07655bf62361f21": function(p0i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_msCrypto_d07655bf62361f21"](p0i32);
/******/ 					},
/******/ 					"__wbg_newnoargs_e23b458e372830de": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_newnoargs_e23b458e372830de"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_get_a9cab131e3152c49": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_get_a9cab131e3152c49"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_ae78342adc33730a": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_call_ae78342adc33730a"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_3ed288a247f13ea5": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_call_3ed288a247f13ea5"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_call_a19d3173f3e1d3c5": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_call_a19d3173f3e1d3c5"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_getTime_bffb1c09df09618b": function(p0i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_getTime_bffb1c09df09618b"](p0i32);
/******/ 					},
/******/ 					"__wbg_getTimezoneOffset_9c9a2f880882a1d7": function(p0i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_getTimezoneOffset_9c9a2f880882a1d7"](p0i32);
/******/ 					},
/******/ 					"__wbg_new0_0ff7eb5c1486f3ec": function() {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_new0_0ff7eb5c1486f3ec"]();
/******/ 					},
/******/ 					"__wbg_self_99737b4dcdf6f0d8": function() {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_self_99737b4dcdf6f0d8"]();
/******/ 					},
/******/ 					"__wbg_window_9b61fbbf3564c4fb": function() {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_window_9b61fbbf3564c4fb"]();
/******/ 					},
/******/ 					"__wbg_globalThis_8e275ef40caea3a3": function() {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_globalThis_8e275ef40caea3a3"]();
/******/ 					},
/******/ 					"__wbg_global_5de1e0f82bddcd27": function() {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_global_5de1e0f82bddcd27"]();
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbg_buffer_7af23f65f6c64548": function(p0i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_buffer_7af23f65f6c64548"](p0i32);
/******/ 					},
/******/ 					"__wbg_new_cc9018bd6f283b6f": function(p0i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_new_cc9018bd6f283b6f"](p0i32);
/******/ 					},
/******/ 					"__wbg_set_f25e869e4565d2a2": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_set_f25e869e4565d2a2"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_length_0acb1cf9bbaf8519": function(p0i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_length_0acb1cf9bbaf8519"](p0i32);
/******/ 					},
/******/ 					"__wbg_newwithlength_8f0657faca9f1422": function(p0i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_newwithlength_8f0657faca9f1422"](p0i32);
/******/ 					},
/******/ 					"__wbg_subarray_da527dbd24eafb6b": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbg_subarray_da527dbd24eafb6b"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_memory": function() {
/******/ 						return installedModules["../pkg/layout_evaluation_wasm_bg.js"].exports["__wbindgen_memory"]();
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
/******/ 		var wasmModules = {"0":["../pkg/layout_evaluation_wasm_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"../pkg/layout_evaluation_wasm_bg.wasm":"0b74f9fd836697de5fa1"}[wasmModuleId] + ".module.wasm");
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

/***/ "../../../corpus lazy recursive ^\\.\\/.*\\/1\\-grams\\.txt$":
/*!**********************************************************************************************************************************!*\
  !*** /home/runner/work/keyboard_layout_optimizer/keyboard_layout_optimizer/corpus lazy ^\.\/.*\/1\-grams\.txt$ namespace object ***!
  \**********************************************************************************************************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("var map = {\n\t\"./arne/1-grams.txt\": [\n\t\t\"../../../corpus/arne/1-grams.txt\",\n\t\t1\n\t],\n\t\"./arne_basis/1-grams.txt\": [\n\t\t\"../../../corpus/arne_basis/1-grams.txt\",\n\t\t4\n\t],\n\t\"./arne_no_special/1-grams.txt\": [\n\t\t\"../../../corpus/arne_no_special/1-grams.txt\",\n\t\t7\n\t],\n\t\"./deu_mixed_0.6_eng_news_typical_0.4/1-grams.txt\": [\n\t\t\"../../../corpus/deu_mixed_0.6_eng_news_typical_0.4/1-grams.txt\",\n\t\t10\n\t],\n\t\"./deu_mixed_1m/1-grams.txt\": [\n\t\t\"../../../corpus/deu_mixed_1m/1-grams.txt\",\n\t\t13\n\t],\n\t\"./deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4/1-grams.txt\": [\n\t\t\"../../../corpus/deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4/1-grams.txt\",\n\t\t16\n\t],\n\t\"./deu_web_0.6_eng_web_0.4/1-grams.txt\": [\n\t\t\"../../../corpus/deu_web_0.6_eng_web_0.4/1-grams.txt\",\n\t\t19\n\t],\n\t\"./deu_web_1m/1-grams.txt\": [\n\t\t\"../../../corpus/deu_web_1m/1-grams.txt\",\n\t\t22\n\t],\n\t\"./deu_wiki_0.6_eng_wiki_0.4/1-grams.txt\": [\n\t\t\"../../../corpus/deu_wiki_0.6_eng_wiki_0.4/1-grams.txt\",\n\t\t25\n\t],\n\t\"./deu_wiki_1m/1-grams.txt\": [\n\t\t\"../../../corpus/deu_wiki_1m/1-grams.txt\",\n\t\t28\n\t],\n\t\"./eng_news_typical_1m/1-grams.txt\": [\n\t\t\"../../../corpus/eng_news_typical_1m/1-grams.txt\",\n\t\t31\n\t],\n\t\"./eng_web_1m/1-grams.txt\": [\n\t\t\"../../../corpus/eng_web_1m/1-grams.txt\",\n\t\t34\n\t],\n\t\"./eng_wiki_1m/1-grams.txt\": [\n\t\t\"../../../corpus/eng_wiki_1m/1-grams.txt\",\n\t\t37\n\t],\n\t\"./irc_neo/1-grams.txt\": [\n\t\t\"../../../corpus/irc_neo/1-grams.txt\",\n\t\t40\n\t]\n};\nfunction webpackAsyncContext(req) {\n\tif(!__webpack_require__.o(map, req)) {\n\t\treturn Promise.resolve().then(function() {\n\t\t\tvar e = new Error(\"Cannot find module '\" + req + \"'\");\n\t\t\te.code = 'MODULE_NOT_FOUND';\n\t\t\tthrow e;\n\t\t});\n\t}\n\n\tvar ids = map[req], id = ids[0];\n\treturn __webpack_require__.e(ids[1]).then(function() {\n\t\treturn __webpack_require__(id);\n\t});\n}\nwebpackAsyncContext.keys = function webpackAsyncContextKeys() {\n\treturn Object.keys(map);\n};\nwebpackAsyncContext.id = \"../../../corpus lazy recursive ^\\\\.\\\\/.*\\\\/1\\\\-grams\\\\.txt$\";\nmodule.exports = webpackAsyncContext;\n\n//# sourceURL=webpack:////home/runner/work/keyboard_layout_optimizer/keyboard_layout_optimizer/corpus_lazy_^\\.\\/.*\\/1\\-grams\\.txt$_namespace_object?");

/***/ }),

/***/ "../../../corpus lazy recursive ^\\.\\/.*\\/2\\-grams\\.txt$":
/*!**********************************************************************************************************************************!*\
  !*** /home/runner/work/keyboard_layout_optimizer/keyboard_layout_optimizer/corpus lazy ^\.\/.*\/2\-grams\.txt$ namespace object ***!
  \**********************************************************************************************************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("var map = {\n\t\"./arne/2-grams.txt\": [\n\t\t\"../../../corpus/arne/2-grams.txt\",\n\t\t2\n\t],\n\t\"./arne_basis/2-grams.txt\": [\n\t\t\"../../../corpus/arne_basis/2-grams.txt\",\n\t\t5\n\t],\n\t\"./arne_no_special/2-grams.txt\": [\n\t\t\"../../../corpus/arne_no_special/2-grams.txt\",\n\t\t8\n\t],\n\t\"./deu_mixed_0.6_eng_news_typical_0.4/2-grams.txt\": [\n\t\t\"../../../corpus/deu_mixed_0.6_eng_news_typical_0.4/2-grams.txt\",\n\t\t11\n\t],\n\t\"./deu_mixed_1m/2-grams.txt\": [\n\t\t\"../../../corpus/deu_mixed_1m/2-grams.txt\",\n\t\t14\n\t],\n\t\"./deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4/2-grams.txt\": [\n\t\t\"../../../corpus/deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4/2-grams.txt\",\n\t\t17\n\t],\n\t\"./deu_web_0.6_eng_web_0.4/2-grams.txt\": [\n\t\t\"../../../corpus/deu_web_0.6_eng_web_0.4/2-grams.txt\",\n\t\t20\n\t],\n\t\"./deu_web_1m/2-grams.txt\": [\n\t\t\"../../../corpus/deu_web_1m/2-grams.txt\",\n\t\t23\n\t],\n\t\"./deu_wiki_0.6_eng_wiki_0.4/2-grams.txt\": [\n\t\t\"../../../corpus/deu_wiki_0.6_eng_wiki_0.4/2-grams.txt\",\n\t\t26\n\t],\n\t\"./deu_wiki_1m/2-grams.txt\": [\n\t\t\"../../../corpus/deu_wiki_1m/2-grams.txt\",\n\t\t29\n\t],\n\t\"./eng_news_typical_1m/2-grams.txt\": [\n\t\t\"../../../corpus/eng_news_typical_1m/2-grams.txt\",\n\t\t32\n\t],\n\t\"./eng_web_1m/2-grams.txt\": [\n\t\t\"../../../corpus/eng_web_1m/2-grams.txt\",\n\t\t35\n\t],\n\t\"./eng_wiki_1m/2-grams.txt\": [\n\t\t\"../../../corpus/eng_wiki_1m/2-grams.txt\",\n\t\t38\n\t],\n\t\"./irc_neo/2-grams.txt\": [\n\t\t\"../../../corpus/irc_neo/2-grams.txt\",\n\t\t41\n\t]\n};\nfunction webpackAsyncContext(req) {\n\tif(!__webpack_require__.o(map, req)) {\n\t\treturn Promise.resolve().then(function() {\n\t\t\tvar e = new Error(\"Cannot find module '\" + req + \"'\");\n\t\t\te.code = 'MODULE_NOT_FOUND';\n\t\t\tthrow e;\n\t\t});\n\t}\n\n\tvar ids = map[req], id = ids[0];\n\treturn __webpack_require__.e(ids[1]).then(function() {\n\t\treturn __webpack_require__(id);\n\t});\n}\nwebpackAsyncContext.keys = function webpackAsyncContextKeys() {\n\treturn Object.keys(map);\n};\nwebpackAsyncContext.id = \"../../../corpus lazy recursive ^\\\\.\\\\/.*\\\\/2\\\\-grams\\\\.txt$\";\nmodule.exports = webpackAsyncContext;\n\n//# sourceURL=webpack:////home/runner/work/keyboard_layout_optimizer/keyboard_layout_optimizer/corpus_lazy_^\\.\\/.*\\/2\\-grams\\.txt$_namespace_object?");

/***/ }),

/***/ "../../../corpus lazy recursive ^\\.\\/.*\\/3\\-grams\\.txt$":
/*!**********************************************************************************************************************************!*\
  !*** /home/runner/work/keyboard_layout_optimizer/keyboard_layout_optimizer/corpus lazy ^\.\/.*\/3\-grams\.txt$ namespace object ***!
  \**********************************************************************************************************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("var map = {\n\t\"./arne/3-grams.txt\": [\n\t\t\"../../../corpus/arne/3-grams.txt\",\n\t\t3\n\t],\n\t\"./arne_basis/3-grams.txt\": [\n\t\t\"../../../corpus/arne_basis/3-grams.txt\",\n\t\t6\n\t],\n\t\"./arne_no_special/3-grams.txt\": [\n\t\t\"../../../corpus/arne_no_special/3-grams.txt\",\n\t\t9\n\t],\n\t\"./deu_mixed_0.6_eng_news_typical_0.4/3-grams.txt\": [\n\t\t\"../../../corpus/deu_mixed_0.6_eng_news_typical_0.4/3-grams.txt\",\n\t\t12\n\t],\n\t\"./deu_mixed_1m/3-grams.txt\": [\n\t\t\"../../../corpus/deu_mixed_1m/3-grams.txt\",\n\t\t15\n\t],\n\t\"./deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4/3-grams.txt\": [\n\t\t\"../../../corpus/deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4/3-grams.txt\",\n\t\t18\n\t],\n\t\"./deu_web_0.6_eng_web_0.4/3-grams.txt\": [\n\t\t\"../../../corpus/deu_web_0.6_eng_web_0.4/3-grams.txt\",\n\t\t21\n\t],\n\t\"./deu_web_1m/3-grams.txt\": [\n\t\t\"../../../corpus/deu_web_1m/3-grams.txt\",\n\t\t24\n\t],\n\t\"./deu_wiki_0.6_eng_wiki_0.4/3-grams.txt\": [\n\t\t\"../../../corpus/deu_wiki_0.6_eng_wiki_0.4/3-grams.txt\",\n\t\t27\n\t],\n\t\"./deu_wiki_1m/3-grams.txt\": [\n\t\t\"../../../corpus/deu_wiki_1m/3-grams.txt\",\n\t\t30\n\t],\n\t\"./eng_news_typical_1m/3-grams.txt\": [\n\t\t\"../../../corpus/eng_news_typical_1m/3-grams.txt\",\n\t\t33\n\t],\n\t\"./eng_web_1m/3-grams.txt\": [\n\t\t\"../../../corpus/eng_web_1m/3-grams.txt\",\n\t\t36\n\t],\n\t\"./eng_wiki_1m/3-grams.txt\": [\n\t\t\"../../../corpus/eng_wiki_1m/3-grams.txt\",\n\t\t39\n\t],\n\t\"./irc_neo/3-grams.txt\": [\n\t\t\"../../../corpus/irc_neo/3-grams.txt\",\n\t\t42\n\t]\n};\nfunction webpackAsyncContext(req) {\n\tif(!__webpack_require__.o(map, req)) {\n\t\treturn Promise.resolve().then(function() {\n\t\t\tvar e = new Error(\"Cannot find module '\" + req + \"'\");\n\t\t\te.code = 'MODULE_NOT_FOUND';\n\t\t\tthrow e;\n\t\t});\n\t}\n\n\tvar ids = map[req], id = ids[0];\n\treturn __webpack_require__.e(ids[1]).then(function() {\n\t\treturn __webpack_require__(id);\n\t});\n}\nwebpackAsyncContext.keys = function webpackAsyncContextKeys() {\n\treturn Object.keys(map);\n};\nwebpackAsyncContext.id = \"../../../corpus lazy recursive ^\\\\.\\\\/.*\\\\/3\\\\-grams\\\\.txt$\";\nmodule.exports = webpackAsyncContext;\n\n//# sourceURL=webpack:////home/runner/work/keyboard_layout_optimizer/keyboard_layout_optimizer/corpus_lazy_^\\.\\/.*\\/3\\-grams\\.txt$_namespace_object?");

/***/ }),

/***/ "./worker.js":
/*!*******************!*\
  !*** ./worker.js ***!
  \*******************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("importScripts(\"https://unpkg.com/comlink/dist/umd/comlink.js\")\n\nconst evaluator = {\n    wasm: null,\n    ngramProvider: null,\n    layoutEvaluator: null,\n    layoutOptimizer: null,\n\n    init() {\n        return __webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! evolve-keyboard-layout-wasm */ \"../pkg/layout_evaluation_wasm.js\"))\n            .then((wasm) => {\n                this.wasm = wasm\n            })\n    },\n\n    async initNgramProvider(ngramType, evalParams, ngramData) {\n        if (ngramType === 'prepared') {\n            let unigrams = await __webpack_require__(\"../../../corpus lazy recursive ^\\\\.\\\\/.*\\\\/1\\\\-grams\\\\.txt$\")(`./${ngramData}/1-grams.txt`)\n                .then((ngrams) => ngrams.default)\n            let bigrams = await __webpack_require__(\"../../../corpus lazy recursive ^\\\\.\\\\/.*\\\\/2\\\\-grams\\\\.txt$\")(`./${ngramData}/2-grams.txt`)\n                .then((ngrams) => ngrams.default)\n            let trigrams = await __webpack_require__(\"../../../corpus lazy recursive ^\\\\.\\\\/.*\\\\/3\\\\-grams\\\\.txt$\")(`./${ngramData}/3-grams.txt`)\n                .then((ngrams) => ngrams.default)\n\n            this.ngramProvider = this.wasm.NgramProvider.with_frequencies(\n                evalParams,\n                unigrams,\n                bigrams,\n                trigrams\n            )\n        } else if (ngramType === 'from_text') {\n            this.ngramProvider = this.wasm.NgramProvider.with_text(\n                evalParams,\n                ngramData\n            )\n        }\n    },\n\n    initLayoutEvaluator(layoutConfig, evalParams) {\n        this.layoutEvaluator = this.wasm.LayoutEvaluator.new(\n            layoutConfig,\n            evalParams,\n            this.ngramProvider,\n        )\n    },\n\n    async saOptimize(layout, fixed_chars, optParamsStr, initCallbacks, setCurrentStepNr, setNewBest) {\n        // Needed to make the callbacks work in Firefox.\n        // In other browsers (for example in Chromium or Midori), this isn't necessary.\n        // In those browsers, the whole function can be turned into a syncronous one.\n        await initCallbacks()\n        this.wasm.sa_optimize(\n            layout,\n            optParamsStr,\n            this.layoutEvaluator,\n            fixed_chars,\n            true,\n            setCurrentStepNr,\n            setNewBest,\n        )\n    },\n\n    initGenLayoutOptimizer(layout, fixed_chars, optParamsStr) {\n        this.layoutOptimizer = this.wasm.LayoutOptimizer.new(\n            layout,\n            optParamsStr,\n            this.layoutEvaluator,\n            fixed_chars,\n            true,\n        )\n\n        return this.layoutOptimizer.parameters()\n    },\n    genOptimizationStep() {\n        return this.layoutOptimizer.step()\n    },\n\n    evaluateLayout(layout) {\n        let res = this.layoutEvaluator.evaluate(layout)\n        res.layout = layout\n        return res\n    },\n\n    permutableKeys() {\n        return this.layoutEvaluator.permutable_keys()\n    },\n}\n\nComlink.expose(evaluator)\n\n\n\n//# sourceURL=webpack:///./worker.js?");

/***/ })

/******/ });