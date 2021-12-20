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
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_rethrow": function(p0i32) {
/******/ 						return installedModules["../pkg/evolve_keyboard_layout_wasm_bg.js"].exports["__wbindgen_rethrow"](p0i32);
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
/******/ 				var req = fetch(__webpack_require__.p + "" + {"../pkg/evolve_keyboard_layout_wasm_bg.wasm":"cd5b2d13ac627b4ffd3e"}[wasmModuleId] + ".module.wasm");
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

/***/ "./worker.js":
/*!*******************!*\
  !*** ./worker.js ***!
  \*******************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("// Create promise to handle Worker calls whilst\n// module is still initialising\nlet initResolve;\nlet ready = new Promise((resolve) => {\n    initResolve = resolve;\n})\n\n// instantiate wasm module\nlet wasm_import = __webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! evolve-keyboard-layout-wasm */ \"../pkg/evolve_keyboard_layout_wasm.js\"))\nlet unigram_import = __webpack_require__.e(/*! import() */ 1).then(__webpack_require__.bind(null, /*! ../../1-gramme.arne.no-special.txt */ \"../../1-gramme.arne.no-special.txt\"))\nlet bigram_import = __webpack_require__.e(/*! import() */ 2).then(__webpack_require__.bind(null, /*! ../../2-gramme.arne.no-special.txt */ \"../../2-gramme.arne.no-special.txt\"))\nlet trigram_import = __webpack_require__.e(/*! import() */ 3).then(__webpack_require__.bind(null, /*! ../../3-gramme.arne.no-special.txt */ \"../../3-gramme.arne.no-special.txt\"))\n\n\nclass Evaluator {\n    constructor (wasm, unigrams, bigrams, trigrams) {\n        this.wasm = wasm\n        this.unigrams = unigrams\n        this.bigrams = bigrams\n        this.trigrams = trigrams\n\n        this.ngramProvider = null\n        this.layoutEvaluator = null\n    }\n\n    initNgramProvider (ngramType, evalParams, corpusText) {\n        if (ngramType === 'prepared') {\n            this.ngramProvider = this.wasm.NgramProvider.with_frequencies(\n                evalParams,\n                this.unigrams,\n                this.bigrams,\n                this.trigrams\n            )\n        } else if (ngramType === 'from_text') {\n            this.ngramProvider = this.wasm.NgramProvider.with_text(\n                evalParams,\n                corpusText\n            )\n        }\n    }\n\n    initLayoutEvaluator (layoutConfig, evalParams) {\n        this.layoutEvaluator = this.wasm.LayoutEvaluator.new(\n            layoutConfig,\n            evalParams,\n            this.ngramProvider,\n        )\n    }\n\n    evaluateLayout (layout) {\n        let res = this.layoutEvaluator.evaluate(layout)\n        res.layout = layout\n        return res\n    }\n}\n\n\n// listen to messages sent from main thread\nself.addEventListener('message', function(event) {\n    const { eventType, eventData, eventId } = event.data;\n\n\n    if (eventType === \"initialise\") {\n        Promise.all([wasm_import, unigram_import, bigram_import, trigram_import])\n            .then((imports) => {\n                let wasm = imports[0]\n                let unigrams = imports[1].default\n                let bigrams = imports[2].default\n                let trigrams = imports[3].default\n                let evaluator = new Evaluator(wasm, unigrams, bigrams, trigrams)\n                initResolve(evaluator)\n\n                // Send back initialised message to main thread\n                self.postMessage({\n                    eventType: \"initialised\",\n                    eventData: Object.getOwnPropertyNames(Object.getPrototypeOf(evaluator))\n                });\n\n            });\n    } else if (eventType === \"call\") {\n        ready\n            .then((evaluator) => {\n                const method = evaluator[eventData.method].bind(evaluator);\n                const result = method.apply(null, eventData.arguments);\n                self.postMessage({\n                    eventType: \"result\",\n                    eventData: result,\n                    eventId: eventId\n                });\n            })\n            .catch((error) => {\n                self.postMessage({\n                    eventType: \"error\",\n                    eventData: \"An error occured executing WASM instance function: \" + error.toString(),\n                    eventId: eventId\n                });\n            })\n    }\n\n\n\n\n\n}, false)\n\n\n//# sourceURL=webpack:///./worker.js?");

/***/ })

/******/ });