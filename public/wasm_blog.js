import { ChangeListInterpreter } from './snippets/dodrio-fb10e775fcadd85a/js/change-list-interpreter.js';

let wasm;

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

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

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

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error('expected a boolean argument');
    }
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

    if (typeof(arg) !== 'string') throw new Error('expected a string argument');

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
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
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

function logError(e) {
    let error = (function () {
        try {
            return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
        } catch(_) {
            return "<failed to stringify thrown value>";
        }
    }());
    console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
    throw e;
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error('expected a number argument');
}
function __wbg_adapter_20(arg0, arg1, arg2, arg3, arg4) {
    _assertNum(arg0);
    _assertNum(arg1);
    _assertNum(arg3);
    _assertNum(arg4);
    wasm._dyn_core__ops__function__Fn__A__B__C___Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h9a38811a60c6233f(arg0, arg1, addHeapObject(arg2), arg3, arg4);
}

function __wbg_adapter_23(arg0, arg1) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h3a9d8dc04b3f58ea(arg0, arg1);
}

function __wbg_adapter_26(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__ha66839986d819f5e(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_29(arg0, arg1) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0f8a6fa30b6db6ff(arg0, arg1);
}

/**
*/
export function start() {
    wasm.start();
}

function handleError(e) {
    wasm.__wbindgen_exn_store(addHeapObject(e));
}

function isLikeNone(x) {
    return x === undefined || x === null;
}
function __wbg_adapter_103(arg0, arg1, arg2, arg3) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm.wasm_bindgen__convert__closures__invoke2_mut__h52dcbde0503e77eb(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}

function init(module) {
    if (typeof module === 'undefined') {
        module = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    let result;
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        var ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_json_parse = function(arg0, arg1) {
        var ret = JSON.parse(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        var ret = false;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
        try {
            try {
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(arg0, arg1);
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_new_59cb74e423758ede = function() {
        try {
            var ret = new Error();
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_stack_558ba5917b466edd = function(arg0, arg1) {
        try {
            var ret = getObject(arg1).stack;
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_new_5d8d0f7e7d6a14c0 = function(arg0) {
        try {
            var ret = new ChangeListInterpreter(getObject(arg0));
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_unmount_50cf7e18a03f5914 = function(arg0) {
        try {
            getObject(arg0).unmount();
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_addChangeListRange_0d991173a03fc15c = function(arg0, arg1, arg2) {
        try {
            getObject(arg0).addChangeListRange(arg1 >>> 0, arg2 >>> 0);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_applyChanges_25b20264911bf1af = function(arg0, arg1) {
        try {
            getObject(arg0).applyChanges(takeObject(arg1));
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_initEventsTrampoline_cf7bb7e86502a9dc = function(arg0, arg1) {
        try {
            getObject(arg0).initEventsTrampoline(getObject(arg1));
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_new_with_event_init_dict_CustomEvent = function(arg0, arg1, arg2) {
        try {
            try {
                var ret = new CustomEvent(getStringFromWasm0(arg0, arg1), getObject(arg2));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_create_element_Document = function(arg0, arg1, arg2) {
        try {
            try {
                var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_query_selector_all_Document = function(arg0, arg1, arg2) {
        try {
            try {
                var ret = getObject(arg0).querySelectorAll(getStringFromWasm0(arg1, arg2));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_body_Document = function(arg0) {
        try {
            var ret = getObject(arg0).body;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_set_id_Element = function(arg0, arg1, arg2) {
        try {
            getObject(arg0).id = getStringFromWasm0(arg1, arg2);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_client_width_Element = function(arg0) {
        try {
            var ret = getObject(arg0).clientWidth;
            _assertNum(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_client_height_Element = function(arg0) {
        try {
            var ret = getObject(arg0).clientHeight;
            _assertNum(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_set_inner_html_Element = function(arg0, arg1, arg2) {
        try {
            getObject(arg0).innerHTML = getStringFromWasm0(arg1, arg2);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_dispatch_event_EventTarget = function(arg0, arg1) {
        try {
            try {
                var ret = getObject(arg0).dispatchEvent(getObject(arg1));
                _assertBoolean(ret);
                return ret;
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_instanceof_HTMLElement = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof HTMLElement;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_append_child_Node = function(arg0, arg1) {
        try {
            try {
                var ret = getObject(arg0).appendChild(getObject(arg1));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_get_NodeList = function(arg0, arg1) {
        try {
            var ret = getObject(arg0)[arg1 >>> 0];
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_instanceof_Window = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof Window;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_request_animation_frame_Window = function(arg0, arg1) {
        try {
            try {
                var ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
                _assertNum(ret);
                return ret;
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_document_Window = function(arg0) {
        try {
            var ret = getObject(arg0).document;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_clear_timeout_with_handle_Window = function(arg0, arg1) {
        try {
            getObject(arg0).clearTimeout(arg1);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_set_timeout_with_callback_and_timeout_and_arguments_0_Window = function(arg0, arg1, arg2) {
        try {
            try {
                var ret = getObject(arg0).setTimeout(getObject(arg1), arg2);
                _assertNum(ret);
                return ret;
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_debug_2_ = function(arg0, arg1) {
        try {
            console.debug(getObject(arg0), getObject(arg1));
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_debug_3_ = function(arg0, arg1, arg2) {
        try {
            console.debug(getObject(arg0), getObject(arg1), getObject(arg2));
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_error_2_ = function(arg0, arg1) {
        try {
            console.error(getObject(arg0), getObject(arg1));
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_error_3_ = function(arg0, arg1, arg2) {
        try {
            console.error(getObject(arg0), getObject(arg1), getObject(arg2));
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_info_2_ = function(arg0, arg1) {
        try {
            console.info(getObject(arg0), getObject(arg1));
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_info_3_ = function(arg0, arg1, arg2) {
        try {
            console.info(getObject(arg0), getObject(arg1), getObject(arg2));
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_warn_2_ = function(arg0, arg1) {
        try {
            console.warn(getObject(arg0), getObject(arg1));
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_warn_3_ = function(arg0, arg1, arg2) {
        try {
            console.warn(getObject(arg0), getObject(arg1), getObject(arg2));
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_newnoargs_c4b2cbbd30e2d057 = function(arg0, arg1) {
        try {
            var ret = new Function(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_call_12b949cfc461d154 = function(arg0, arg1) {
        try {
            try {
                var ret = getObject(arg0).call(getObject(arg1));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_new_7dd9b384a913884d = function() {
        try {
            var ret = new Object();
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_new_d3eff62d5c013634 = function(arg0, arg1) {
        try {
            try {
                var state0 = {a: arg0, b: arg1};
                var cb0 = (arg0, arg1) => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return __wbg_adapter_103(a, state0.b, arg0, arg1);
                    } finally {
                        state0.a = a;
                    }
                };
                var ret = new Promise(cb0);
                return addHeapObject(ret);
            } finally {
                state0.a = state0.b = 0;
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_resolve_6885947099a907d3 = function(arg0) {
        try {
            var ret = Promise.resolve(getObject(arg0));
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_then_b6fef331fde5cf0a = function(arg0, arg1) {
        try {
            var ret = getObject(arg0).then(getObject(arg1));
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_then_7d828a330efec051 = function(arg0, arg1, arg2) {
        try {
            var ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_globalThis_22e06d4bea0084e3 = function() {
        try {
            try {
                var ret = globalThis.globalThis;
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_self_00b0599bca667294 = function() {
        try {
            try {
                var ret = self.self;
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_window_aa795c5aad79b8ac = function() {
        try {
            try {
                var ret = window.window;
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_global_cc239dc2303f417c = function() {
        try {
            try {
                var ret = global.global;
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = getObject(arg0) === undefined;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbg_set_8d5fd23e838df6b0 = function(arg0, arg1, arg2) {
        try {
            try {
                var ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
                _assertBoolean(ret);
                return ret;
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
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
    imports.wbg.__wbindgen_memory = function() {
        var ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper11906 = function(arg0, arg1, arg2) {
        try {

            const state = { a: arg0, b: arg1, cnt: 1 };
            const real = (arg0, arg1, arg2) => {
                state.cnt++;
                try {
                    return __wbg_adapter_20(state.a, state.b, arg0, arg1, arg2);
                } finally {
                    if (--state.cnt === 0) {
                        wasm.__wbindgen_export_2.get(174)(state.a, state.b);
                        state.a = 0;
                    }
                }
            }
            ;
            real.original = state;
            var ret = real;
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbindgen_closure_wrapper9160 = function(arg0, arg1, arg2) {
        try {

            const state = { a: arg0, b: arg1, cnt: 1 };
            const real = () => {
                state.cnt++;
                const a = state.a;
                state.a = 0;
                try {
                    return __wbg_adapter_23(a, state.b, );
                } finally {
                    if (--state.cnt === 0) wasm.__wbindgen_export_2.get(138)(a, state.b);
                    else state.a = a;
                }
            }
            ;
            real.original = state;
            var ret = real;
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbindgen_closure_wrapper11908 = function(arg0, arg1, arg2) {
        try {

            const state = { a: arg0, b: arg1, cnt: 1 };
            const real = () => {
                state.cnt++;
                const a = state.a;
                state.a = 0;
                try {
                    return __wbg_adapter_29(a, state.b, );
                } finally {
                    if (--state.cnt === 0) wasm.__wbindgen_export_2.get(172)(a, state.b);
                    else state.a = a;
                }
            }
            ;
            real.original = state;
            var ret = real;
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbindgen_closure_wrapper14165 = function(arg0, arg1, arg2) {
        try {

            const state = { a: arg0, b: arg1, cnt: 1 };
            const real = (arg0) => {
                state.cnt++;
                const a = state.a;
                state.a = 0;
                try {
                    return __wbg_adapter_26(a, state.b, arg0);
                } finally {
                    if (--state.cnt === 0) wasm.__wbindgen_export_2.get(195)(a, state.b);
                    else state.a = a;
                }
            }
            ;
            real.original = state;
            var ret = real;
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
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

export default init;

