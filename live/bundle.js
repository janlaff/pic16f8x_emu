(function () {
    'use strict';

    let wasm;

    const heap = new Array(32);

    heap.fill(undefined);

    heap.push(undefined, null, true, false);

    function getObject(idx) { return heap[idx]; }

    let heap_next = heap.length;

    function dropObject(idx) {
        if (idx < 36) return;
        heap[idx] = heap_next;
        heap_next = idx;
    }

    function takeObject(idx) {
        const ret = getObject(idx);
        dropObject(idx);
        return ret;
    }

    let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

    cachedTextDecoder.decode();

    let cachegetUint8Memory0 = null;
    function getUint8Memory0() {
        if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory0;
    }

    function getStringFromWasm0(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
    }

    function addHeapObject(obj) {
        if (heap_next === heap.length) heap.push(heap.length + 1);
        const idx = heap_next;
        heap_next = heap[idx];

        heap[idx] = obj;
        return idx;
    }

    function debugString(val) {
        // primitive types
        const type = typeof val;
        if (type == 'number' || type == 'boolean' || val == null) {
            return  `${val}`;
        }
        if (type == 'string') {
            return `"${val}"`;
        }
        if (type == 'symbol') {
            const description = val.description;
            if (description == null) {
                return 'Symbol';
            } else {
                return `Symbol(${description})`;
            }
        }
        if (type == 'function') {
            const name = val.name;
            if (typeof name == 'string' && name.length > 0) {
                return `Function(${name})`;
            } else {
                return 'Function';
            }
        }
        // objects
        if (Array.isArray(val)) {
            const length = val.length;
            let debug = '[';
            if (length > 0) {
                debug += debugString(val[0]);
            }
            for(let i = 1; i < length; i++) {
                debug += ', ' + debugString(val[i]);
            }
            debug += ']';
            return debug;
        }
        // Test for built-in
        const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
        let className;
        if (builtInMatches.length > 1) {
            className = builtInMatches[1];
        } else {
            // Failed to match the standard '[object ClassName]'
            return toString.call(val);
        }
        if (className == 'Object') {
            // we're a user defined class or Object
            // JSON.stringify avoids problems with cycles, and is generally much
            // easier than looping through ownProperties of `val`.
            try {
                return 'Object(' + JSON.stringify(val) + ')';
            } catch (_) {
                return 'Object';
            }
        }
        // errors
        if (val instanceof Error) {
            return `${val.name}: ${val.message}\n${val.stack}`;
        }
        // TODO we could test for more things here, like `Set`s and `Map`s.
        return className;
    }

    let WASM_VECTOR_LEN = 0;

    let cachedTextEncoder = new TextEncoder('utf-8');

    const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
        ? function (arg, view) {
        return cachedTextEncoder.encodeInto(arg, view);
    }
        : function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    });

    function passStringToWasm0(arg, malloc, realloc) {

        if (realloc === undefined) {
            const buf = cachedTextEncoder.encode(arg);
            const ptr = malloc(buf.length);
            getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
            WASM_VECTOR_LEN = buf.length;
            return ptr;
        }

        let len = arg.length;
        let ptr = malloc(len);

        const mem = getUint8Memory0();

        let offset = 0;

        for (; offset < len; offset++) {
            const code = arg.charCodeAt(offset);
            if (code > 0x7F) break;
            mem[ptr + offset] = code;
        }

        if (offset !== len) {
            if (offset !== 0) {
                arg = arg.slice(offset);
            }
            ptr = realloc(ptr, len, len = offset + arg.length * 3);
            const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
            const ret = encodeString(arg, view);

            offset += ret.written;
        }

        WASM_VECTOR_LEN = offset;
        return ptr;
    }

    let cachegetInt32Memory0 = null;
    function getInt32Memory0() {
        if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
            cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
        }
        return cachegetInt32Memory0;
    }

    let stack_pointer = 32;

    function addBorrowedObject(obj) {
        if (stack_pointer == 1) throw new Error('out of js stack');
        heap[--stack_pointer] = obj;
        return stack_pointer;
    }
    function __wbg_adapter_16(arg0, arg1, arg2) {
        try {
            wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hacf707d242157e55(arg0, arg1, addBorrowedObject(arg2));
        } finally {
            heap[stack_pointer++] = undefined;
        }
    }

    /**
    */
    function run_app() {
        wasm.run_app();
    }

    function handleError(e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }

    function isLikeNone(x) {
        return x === undefined || x === null;
    }

    function init(module) {
        if (typeof module === 'undefined') {
            module = (document.currentScript && document.currentScript.src || new URL('bundle.js', document.baseURI).href).replace(/\.js$/, '_bg.wasm');
        }
        let result;
        const imports = {};
        imports.wbg = {};
        imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
            takeObject(arg0);
        };
        imports.wbg.__wbindgen_cb_drop = function(arg0) {
            const obj = takeObject(arg0).original;
            if (obj.cnt-- == 1) {
                obj.a = 0;
                return true;
            }
            var ret = false;
            return ret;
        };
        imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
            var ret = getStringFromWasm0(arg0, arg1);
            return addHeapObject(ret);
        };
        imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
            var ret = getObject(arg0);
            return addHeapObject(ret);
        };
        imports.wbg.__wbindgen_is_undefined = function(arg0) {
            var ret = getObject(arg0) === undefined;
            return ret;
        };
        imports.wbg.__wbg_new_59cb74e423758ede = function() {
            var ret = new Error();
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_stack_558ba5917b466edd = function(arg0, arg1) {
            var ret = getObject(arg1).stack;
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        };
        imports.wbg.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
            try {
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(arg0, arg1);
            }
        };
        imports.wbg.__widl_f_debug_1_ = function(arg0) {
            console.debug(getObject(arg0));
        };
        imports.wbg.__widl_f_error_1_ = function(arg0) {
            console.error(getObject(arg0));
        };
        imports.wbg.__widl_f_info_1_ = function(arg0) {
            console.info(getObject(arg0));
        };
        imports.wbg.__widl_f_log_1_ = function(arg0) {
            console.log(getObject(arg0));
        };
        imports.wbg.__widl_f_warn_1_ = function(arg0) {
            console.warn(getObject(arg0));
        };
        imports.wbg.__widl_instanceof_Window = function(arg0) {
            var ret = getObject(arg0) instanceof Window;
            return ret;
        };
        imports.wbg.__widl_f_add_1_DOMTokenList = function(arg0, arg1, arg2) {
            try {
                getObject(arg0).add(getStringFromWasm0(arg1, arg2));
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__widl_f_remove_1_DOMTokenList = function(arg0, arg1, arg2) {
            try {
                getObject(arg0).remove(getStringFromWasm0(arg1, arg2));
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__widl_f_create_element_Document = function(arg0, arg1, arg2) {
            try {
                var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__widl_f_create_element_ns_Document = function(arg0, arg1, arg2, arg3, arg4) {
            try {
                var ret = getObject(arg0).createElementNS(arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__widl_f_create_text_node_Document = function(arg0, arg1, arg2) {
            var ret = getObject(arg0).createTextNode(getStringFromWasm0(arg1, arg2));
            return addHeapObject(ret);
        };
        imports.wbg.__widl_f_get_element_by_id_Document = function(arg0, arg1, arg2) {
            var ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__widl_f_query_selector_Document = function(arg0, arg1, arg2) {
            try {
                var ret = getObject(arg0).querySelector(getStringFromWasm0(arg1, arg2));
                return isLikeNone(ret) ? 0 : addHeapObject(ret);
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__widl_f_remove_attribute_Element = function(arg0, arg1, arg2) {
            try {
                getObject(arg0).removeAttribute(getStringFromWasm0(arg1, arg2));
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__widl_f_scroll_into_view_Element = function(arg0) {
            getObject(arg0).scrollIntoView();
        };
        imports.wbg.__widl_f_set_attribute_Element = function(arg0, arg1, arg2, arg3, arg4) {
            try {
                getObject(arg0).setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__widl_f_namespace_uri_Element = function(arg0, arg1) {
            var ret = getObject(arg1).namespaceURI;
            var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        };
        imports.wbg.__widl_f_class_list_Element = function(arg0) {
            var ret = getObject(arg0).classList;
            return addHeapObject(ret);
        };
        imports.wbg.__widl_f_stop_propagation_Event = function(arg0) {
            getObject(arg0).stopPropagation();
        };
        imports.wbg.__widl_f_add_event_listener_with_callback_and_add_event_listener_options_EventTarget = function(arg0, arg1, arg2, arg3, arg4) {
            try {
                getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), getObject(arg4));
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__widl_f_remove_event_listener_with_callback_and_bool_EventTarget = function(arg0, arg1, arg2, arg3, arg4) {
            try {
                getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), arg4 !== 0);
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__widl_instanceof_HTMLInputElement = function(arg0) {
            var ret = getObject(arg0) instanceof HTMLInputElement;
            return ret;
        };
        imports.wbg.__widl_f_set_checked_HTMLInputElement = function(arg0, arg1) {
            getObject(arg0).checked = arg1 !== 0;
        };
        imports.wbg.__widl_f_set_type_HTMLInputElement = function(arg0, arg1, arg2) {
            getObject(arg0).type = getStringFromWasm0(arg1, arg2);
        };
        imports.wbg.__widl_f_set_value_HTMLInputElement = function(arg0, arg1, arg2) {
            getObject(arg0).value = getStringFromWasm0(arg1, arg2);
        };
        imports.wbg.__widl_instanceof_HTMLTextAreaElement = function(arg0) {
            var ret = getObject(arg0) instanceof HTMLTextAreaElement;
            return ret;
        };
        imports.wbg.__widl_f_set_value_HTMLTextAreaElement = function(arg0, arg1, arg2) {
            getObject(arg0).value = getStringFromWasm0(arg1, arg2);
        };
        imports.wbg.__widl_f_append_child_Node = function(arg0, arg1) {
            try {
                var ret = getObject(arg0).appendChild(getObject(arg1));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__widl_f_insert_before_Node = function(arg0, arg1, arg2) {
            try {
                var ret = getObject(arg0).insertBefore(getObject(arg1), getObject(arg2));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__widl_f_remove_child_Node = function(arg0, arg1) {
            try {
                var ret = getObject(arg0).removeChild(getObject(arg1));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__widl_f_last_child_Node = function(arg0) {
            var ret = getObject(arg0).lastChild;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__widl_f_next_sibling_Node = function(arg0) {
            var ret = getObject(arg0).nextSibling;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__widl_f_set_node_value_Node = function(arg0, arg1, arg2) {
            getObject(arg0).nodeValue = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
        };
        imports.wbg.__widl_f_alert_with_message_Window = function(arg0, arg1, arg2) {
            try {
                getObject(arg0).alert(getStringFromWasm0(arg1, arg2));
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__widl_f_document_Window = function(arg0) {
            var ret = getObject(arg0).document;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbg_call_12b949cfc461d154 = function(arg0, arg1) {
            try {
                var ret = getObject(arg0).call(getObject(arg1));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__wbg_newnoargs_c4b2cbbd30e2d057 = function(arg0, arg1) {
            var ret = new Function(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_new_7dd9b384a913884d = function() {
            var ret = new Object();
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_globalThis_22e06d4bea0084e3 = function() {
            try {
                var ret = globalThis.globalThis;
                return addHeapObject(ret);
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__wbg_self_00b0599bca667294 = function() {
            try {
                var ret = self.self;
                return addHeapObject(ret);
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__wbg_window_aa795c5aad79b8ac = function() {
            try {
                var ret = window.window;
                return addHeapObject(ret);
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__wbg_global_cc239dc2303f417c = function() {
            try {
                var ret = global.global;
                return addHeapObject(ret);
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__wbg_set_8d5fd23e838df6b0 = function(arg0, arg1, arg2) {
            try {
                var ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
                return ret;
            } catch (e) {
                handleError(e);
            }
        };
        imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
            var ret = debugString(getObject(arg1));
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        };
        imports.wbg.__wbindgen_throw = function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        };
        imports.wbg.__wbindgen_closure_wrapper341 = function(arg0, arg1, arg2) {

            const state = { a: arg0, b: arg1, cnt: 1 };
            const real = (arg0) => {
                state.cnt++;
                const a = state.a;
                state.a = 0;
                try {
                    return __wbg_adapter_16(a, state.b, arg0);
                } finally {
                    if (--state.cnt === 0) wasm.__wbindgen_export_2.get(156)(a, state.b);
                    else state.a = a;
                }
            }
            ;
            real.original = state;
            var ret = real;
            return addHeapObject(ret);
        };

        if ((typeof URL === 'function' && module instanceof URL) || typeof module === 'string' || (typeof Request === 'function' && module instanceof Request)) {

            const response = fetch(module);
            if (typeof WebAssembly.instantiateStreaming === 'function') {
                result = WebAssembly.instantiateStreaming(response, imports)
                .catch(e => {
                    return response
                    .then(r => {
                        if (r.headers.get('Content-Type') != 'application/wasm') {
                            console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                            return r.arrayBuffer();
                        } else {
                            throw e;
                        }
                    })
                    .then(bytes => WebAssembly.instantiate(bytes, imports));
                });
            } else {
                result = response
                .then(r => r.arrayBuffer())
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            }
        } else {

            result = WebAssembly.instantiate(module, imports)
            .then(result => {
                if (result instanceof WebAssembly.Instance) {
                    return { instance: result, module };
                } else {
                    return result;
                }
            });
        }
        return result.then(({instance, module}) => {
            wasm = instance.exports;
            init.__wbindgen_wasm_module = module;

            return wasm;
        });
    }

    async function main() {
       await init('./pic16f8x_emu_bg.wasm');
       run_app();
    }
    main();

}());
