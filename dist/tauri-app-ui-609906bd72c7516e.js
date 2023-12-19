let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedFloat64Memory0 = null;

function getFloat64Memory0() {
    if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
        cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64Memory0;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

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
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

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
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
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

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_36(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures__invoke1_mut__h3449d11c7c19466a(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_47(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures__invoke0_mut__h73ce67872324d65f(arg0, arg1);
}

function __wbg_adapter_50(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures__invoke0_mut__h82072aa498de5558(arg0, arg1);
}

function __wbg_adapter_53(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures__invoke1_mut__hf71f9dd85f262fac(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_56(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures__invoke0_mut__h84d1cc807f8d5f4f(arg0, arg1);
}

function __wbg_adapter_59(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4cb4c6d895790c13(arg0, arg1, addHeapObject(arg2));
}

/**
*/
export function ripple_effect() {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.ripple_effect(retptr);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

function getCachedStringFromWasm0(ptr, len) {
    if (ptr === 0) {
        return getObject(len);
    } else {
        return getStringFromWasm0(ptr, len);
    }
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        return ret;
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = getObject(arg0);
        const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
        return ret;
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'number' ? obj : undefined;
        getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_set_bd72c078edfa51ad = function(arg0, arg1, arg2) {
        getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
        const ret = getObject(arg0) === getObject(arg1);
        return ret;
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'string';
        return ret;
    };
    imports.wbg.__wbindgen_is_null = function(arg0) {
        const ret = getObject(arg0) === null;
        return ret;
    };
    imports.wbg.__wbindgen_is_falsy = function(arg0) {
        const ret = !getObject(arg0);
        return ret;
    };
    imports.wbg.__wbg_instanceof_Window_3e5cd1f48c152d01 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Window;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_document_d609202d16c38224 = function(arg0) {
        const ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_location_176c34e89c2c9d80 = function(arg0) {
        const ret = getObject(arg0).location;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_history_80998b7456bf367e = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).history;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_navigator_96ba491902f8f083 = function(arg0) {
        const ret = getObject(arg0).navigator;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_scrollTo_eb21c4452d7b3cd6 = function(arg0, arg1, arg2) {
        getObject(arg0).scrollTo(arg1, arg2);
    };
    imports.wbg.__wbg_requestAnimationFrame_74309aadebde12fa = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setTimeout_06458eba2b40711c = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).setTimeout(getObject(arg1), arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_body_64abc9aba1891e91 = function(arg0) {
        const ret = getObject(arg0).body;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createComment_529b047c02bbe600 = function(arg0, arg1, arg2) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        const ret = getObject(arg0).createComment(v0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createDocumentFragment_1c6d6aeeb8a8eb2e = function(arg0) {
        const ret = getObject(arg0).createDocumentFragment();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createElement_fdd5c113cb84539e = function() { return handleError(function (arg0, arg1, arg2) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        const ret = getObject(arg0).createElement(v0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createTextNode_7ff0c034b2855f66 = function(arg0, arg1, arg2) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        const ret = getObject(arg0).createTextNode(v0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getElementById_65b9547a428b5eb4 = function(arg0, arg1, arg2) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        const ret = getObject(arg0).getElementById(v0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_Element_3f326a19cc457941 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Element;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_setclassName_4b720df7b12cb674 = function(arg0, arg1, arg2) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        getObject(arg0).className = v0;
    };
    imports.wbg.__wbg_classList_82893a9100db6428 = function(arg0) {
        const ret = getObject(arg0).classList;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setinnerHTML_ce0d6527ce4086f2 = function(arg0, arg1, arg2) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        getObject(arg0).innerHTML = v0;
    };
    imports.wbg.__wbg_getAttribute_bff489553dd803cc = function(arg0, arg1, arg2, arg3) {
        var v0 = getCachedStringFromWasm0(arg2, arg3);
        const ret = getObject(arg1).getAttribute(v0);
        var ptr2 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len2;
        getInt32Memory0()[arg0 / 4 + 0] = ptr2;
    };
    imports.wbg.__wbg_hasAttribute_bfb8f7140cf587f1 = function(arg0, arg1, arg2) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        const ret = getObject(arg0).hasAttribute(v0);
        return ret;
    };
    imports.wbg.__wbg_removeAttribute_2e200daefb9f3ed4 = function() { return handleError(function (arg0, arg1, arg2) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        getObject(arg0).removeAttribute(v0);
    }, arguments) };
    imports.wbg.__wbg_scrollIntoView_3de22d537ed95550 = function(arg0) {
        getObject(arg0).scrollIntoView();
    };
    imports.wbg.__wbg_setAttribute_e7b72a5e7cfcb5a3 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        var v1 = getCachedStringFromWasm0(arg3, arg4);
        getObject(arg0).setAttribute(v0, v1);
    }, arguments) };
    imports.wbg.__wbg_before_74a825a7b3d13d06 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).before(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_remove_0d26d36fd4f25c4e = function(arg0) {
        getObject(arg0).remove();
    };
    imports.wbg.__wbg_append_df44ca631c3c1657 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).append(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_append_962e199b73af5069 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).append(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_addEventListener_9bf60ea8a362e5e4 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        getObject(arg0).addEventListener(v0, getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_addEventListener_374cbfd2bbc19ccf = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        getObject(arg0).addEventListener(v0, getObject(arg3), getObject(arg4));
    }, arguments) };
    imports.wbg.__wbg_removeEventListener_66ee1536a0b32c11 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        getObject(arg0).removeEventListener(v0, getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_width_cfc58d9656d60465 = function(arg0) {
        const ret = getObject(arg0).width;
        return ret;
    };
    imports.wbg.__wbg_height_1ba9072bd4001d19 = function(arg0) {
        const ret = getObject(arg0).height;
        return ret;
    };
    imports.wbg.__wbg_getContext_164dc98953ddbc68 = function() { return handleError(function (arg0, arg1, arg2) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        const ret = getObject(arg0).getContext(v0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_instanceof_CanvasRenderingContext2d_b43c8f92c4744b7b = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof CanvasRenderingContext2D;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_setstrokeStyle_1b4551387606453b = function(arg0, arg1) {
        getObject(arg0).strokeStyle = getObject(arg1);
    };
    imports.wbg.__wbg_setfillStyle_1ebd7d8f502888fa = function(arg0, arg1) {
        getObject(arg0).fillStyle = getObject(arg1);
    };
    imports.wbg.__wbg_setlineWidth_d101118d79143f42 = function(arg0, arg1) {
        getObject(arg0).lineWidth = arg1;
    };
    imports.wbg.__wbg_setfont_a622e8f71fcc67bd = function(arg0, arg1, arg2) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        getObject(arg0).font = v0;
    };
    imports.wbg.__wbg_settextAlign_425d121bce67eea6 = function(arg0, arg1, arg2) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        getObject(arg0).textAlign = v0;
    };
    imports.wbg.__wbg_settextBaseline_ff6ae71131150241 = function(arg0, arg1, arg2) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        getObject(arg0).textBaseline = v0;
    };
    imports.wbg.__wbg_beginPath_5c4c10cc904edc64 = function(arg0) {
        getObject(arg0).beginPath();
    };
    imports.wbg.__wbg_fill_854920f07573dffd = function(arg0) {
        getObject(arg0).fill();
    };
    imports.wbg.__wbg_stroke_d6cd6e7c98952ff8 = function(arg0) {
        getObject(arg0).stroke();
    };
    imports.wbg.__wbg_arc_1e8c0f1f2b86edf7 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).arc(arg1, arg2, arg3, arg4, arg5);
    }, arguments) };
    imports.wbg.__wbg_lineTo_ef21aebc726bbda0 = function(arg0, arg1, arg2) {
        getObject(arg0).lineTo(arg1, arg2);
    };
    imports.wbg.__wbg_moveTo_23c53eed282c0730 = function(arg0, arg1, arg2) {
        getObject(arg0).moveTo(arg1, arg2);
    };
    imports.wbg.__wbg_fillRect_ae135cf52671cb3d = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).fillRect(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_restore_7c05e5338cb2f7dc = function(arg0) {
        getObject(arg0).restore();
    };
    imports.wbg.__wbg_save_7222a99e05e4831b = function(arg0) {
        getObject(arg0).save();
    };
    imports.wbg.__wbg_fillText_2dc739bc08581cf8 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        getObject(arg0).fillText(v0, arg3, arg4);
    }, arguments) };
    imports.wbg.__wbg_rotate_24eb4d29166252d6 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).rotate(arg1);
    }, arguments) };
    imports.wbg.__wbg_translate_52a4e2eb4836f205 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).translate(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_clearData_b615a9d35fbf4460 = function() { return handleError(function (arg0) {
        getObject(arg0).clearData();
    }, arguments) };
    imports.wbg.__wbg_getData_9bb88de49d7a730d = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        var v0 = getCachedStringFromWasm0(arg2, arg3);
        const ret = getObject(arg1).getData(v0);
        const ptr2 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len2;
        getInt32Memory0()[arg0 / 4 + 0] = ptr2;
    }, arguments) };
    imports.wbg.__wbg_setData_f6ba0eb4b1d5c0a1 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        var v1 = getCachedStringFromWasm0(arg3, arg4);
        getObject(arg0).setData(v0, v1);
    }, arguments) };
    imports.wbg.__wbg_setondisconnect_6b2f0c474afd60ca = function(arg0, arg1) {
        getObject(arg0).ondisconnect = getObject(arg1);
    };
    imports.wbg.__wbg_requestDevice_c7d909703080c26e = function(arg0, arg1) {
        const ret = getObject(arg0).requestDevice(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_HidDevice_5d064a324c3537dd = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof HIDDevice;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_setoninputreport_2cca6a5fb0674306 = function(arg0, arg1) {
        getObject(arg0).oninputreport = getObject(arg1);
    };
    imports.wbg.__wbg_opened_5d046d3097655711 = function(arg0) {
        const ret = getObject(arg0).opened;
        return ret;
    };
    imports.wbg.__wbg_productName_8fea172dfca16d1c = function(arg0, arg1) {
        const ret = getObject(arg1).productName;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_open_fca066845f8e8363 = function(arg0) {
        const ret = getObject(arg0).open();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_sendReport_10fcf412819fbd45 = function(arg0, arg1, arg2, arg3) {
        const ret = getObject(arg0).sendReport(arg1, getArrayU8FromWasm0(arg2, arg3));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_origin_595edc88be6e66b8 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg1).origin;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    }, arguments) };
    imports.wbg.__wbg_pathname_1ab7e82aaa4511ff = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg1).pathname;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    }, arguments) };
    imports.wbg.__wbg_search_9f7ca8896c2d0804 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg1).search;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    }, arguments) };
    imports.wbg.__wbg_hash_be2940ca236b5efc = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg1).hash;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    }, arguments) };
    imports.wbg.__wbg_origin_aab6d2be79bcec84 = function(arg0, arg1) {
        const ret = getObject(arg1).origin;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_pathname_aeafa820be91c325 = function(arg0, arg1) {
        const ret = getObject(arg1).pathname;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_search_f6e95882a48d3f69 = function(arg0, arg1) {
        const ret = getObject(arg1).search;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_searchParams_00f98167a3c8c4da = function(arg0) {
        const ret = getObject(arg0).searchParams;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_hash_0087751acddc8f2a = function(arg0, arg1) {
        const ret = getObject(arg1).hash;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_newwithbase_f4989aa5bbd5cc29 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        var v0 = getCachedStringFromWasm0(arg0, arg1);
        var v1 = getCachedStringFromWasm0(arg2, arg3);
        const ret = new URL(v0, v1);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_removeProperty_15ae1463504df541 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        var v0 = getCachedStringFromWasm0(arg2, arg3);
        const ret = getObject(arg1).removeProperty(v0);
        const ptr2 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len2;
        getInt32Memory0()[arg0 / 4 + 0] = ptr2;
    }, arguments) };
    imports.wbg.__wbg_setProperty_ecf331459a4d3891 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        var v1 = getCachedStringFromWasm0(arg3, arg4);
        getObject(arg0).setProperty(v0, v1);
    }, arguments) };
    imports.wbg.__wbg_add_e0f3c5b6e421c311 = function() { return handleError(function (arg0, arg1, arg2) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        getObject(arg0).add(v0);
    }, arguments) };
    imports.wbg.__wbg_contains_4e8a9651077b966b = function(arg0, arg1, arg2) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        const ret = getObject(arg0).contains(v0);
        return ret;
    };
    imports.wbg.__wbg_remove_c6ba26a0a6906129 = function() { return handleError(function (arg0, arg1, arg2) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        getObject(arg0).remove(v0);
    }, arguments) };
    imports.wbg.__wbg_reportId_2f7b63eccae3e006 = function(arg0) {
        const ret = getObject(arg0).reportId;
        return ret;
    };
    imports.wbg.__wbg_data_b402ebc851dd84ea = function(arg0) {
        const ret = getObject(arg0).data;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_offsetX_e8c2e5379a90ae29 = function(arg0) {
        const ret = getObject(arg0).offsetX;
        return ret;
    };
    imports.wbg.__wbg_offsetY_b8587366f6d36a25 = function(arg0) {
        const ret = getObject(arg0).offsetY;
        return ret;
    };
    imports.wbg.__wbg_ctrlKey_643b17aaac67db50 = function(arg0) {
        const ret = getObject(arg0).ctrlKey;
        return ret;
    };
    imports.wbg.__wbg_shiftKey_8fb7301f56e7e01c = function(arg0) {
        const ret = getObject(arg0).shiftKey;
        return ret;
    };
    imports.wbg.__wbg_altKey_c6c2a7e797d9a669 = function(arg0) {
        const ret = getObject(arg0).altKey;
        return ret;
    };
    imports.wbg.__wbg_metaKey_2a8dbd51a3f59e9c = function(arg0) {
        const ret = getObject(arg0).metaKey;
        return ret;
    };
    imports.wbg.__wbg_button_cd87b6dabbde9631 = function(arg0) {
        const ret = getObject(arg0).button;
        return ret;
    };
    imports.wbg.__wbg_parentNode_92a7017b3a4fad43 = function(arg0) {
        const ret = getObject(arg0).parentNode;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_childNodes_a5762b4b3e073cf6 = function(arg0) {
        const ret = getObject(arg0).childNodes;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_previousSibling_ef843c512fac0d77 = function(arg0) {
        const ret = getObject(arg0).previousSibling;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_nextSibling_bafccd3347d24543 = function(arg0) {
        const ret = getObject(arg0).nextSibling;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_settextContent_3ebccdd9354e1601 = function(arg0, arg1, arg2) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        getObject(arg0).textContent = v0;
    };
    imports.wbg.__wbg_appendChild_d30e6b83791d04c0 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).appendChild(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_cloneNode_405d5ea3f7e0098a = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).cloneNode();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_removeChild_942eb9c02243d84d = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).removeChild(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_instanceof_ShadowRoot_0bd39e89ab117f86 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof ShadowRoot;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_host_09eee5e3d9cf59a1 = function(arg0) {
        const ret = getObject(arg0).host;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setdata_86ad1e8da020aa68 = function(arg0, arg1, arg2) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        getObject(arg0).data = v0;
    };
    imports.wbg.__wbg_device_c3675b9afde3389f = function(arg0) {
        const ret = getObject(arg0).device;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_HtmlAnchorElement_76fafcefedd51299 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof HTMLAnchorElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_target_b68f65aba6338cfb = function(arg0, arg1) {
        const ret = getObject(arg1).target;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_href_829df0adc5a7228a = function(arg0, arg1) {
        const ret = getObject(arg1).href;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_length_f845c1c304d9837a = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_error_e60eff06f24ab7a4 = function(arg0) {
        console.error(getObject(arg0));
    };
    imports.wbg.__wbg_log_a4530b4fe289336f = function(arg0) {
        console.log(getObject(arg0));
    };
    imports.wbg.__wbg_warn_f260f49434e45e62 = function(arg0) {
        console.warn(getObject(arg0));
    };
    imports.wbg.__wbg_dataTransfer_86945596a7c803c5 = function(arg0) {
        const ret = getObject(arg0).dataTransfer;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_HtmlElement_55a0f0f0f0f0118e = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof HTMLElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_style_97c680a5cbdf49cd = function(arg0) {
        const ret = getObject(arg0).style;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_offsetWidth_b1977ba06d409033 = function(arg0) {
        const ret = getObject(arg0).offsetWidth;
        return ret;
    };
    imports.wbg.__wbg_offsetHeight_5f0a16cdf45abb93 = function(arg0) {
        const ret = getObject(arg0).offsetHeight;
        return ret;
    };
    imports.wbg.__wbg_value_e024243a9dae20bc = function(arg0, arg1) {
        const ret = getObject(arg1).value;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_pushState_e159043fce8f87bc = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
        var v0 = getCachedStringFromWasm0(arg2, arg3);
        var v1 = getCachedStringFromWasm0(arg4, arg5);
        getObject(arg0).pushState(getObject(arg1), v0, v1);
    }, arguments) };
    imports.wbg.__wbg_replaceState_b51dd62c7235b1ac = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
        var v0 = getCachedStringFromWasm0(arg2, arg3);
        var v1 = getCachedStringFromWasm0(arg4, arg5);
        getObject(arg0).replaceState(getObject(arg1), v0, v1);
    }, arguments) };
    imports.wbg.__wbg_hid_c78ed8e31a1a83f2 = function(arg0) {
        const ret = getObject(arg0).hid;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_target_52ddf6955f636bf5 = function(arg0) {
        const ret = getObject(arg0).target;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_defaultPrevented_ae7d433108dd159d = function(arg0) {
        const ret = getObject(arg0).defaultPrevented;
        return ret;
    };
    imports.wbg.__wbg_cancelBubble_976cfdf7ac449a6c = function(arg0) {
        const ret = getObject(arg0).cancelBubble;
        return ret;
    };
    imports.wbg.__wbg_composedPath_12a068e57a98cf90 = function(arg0) {
        const ret = getObject(arg0).composedPath();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_preventDefault_7f821f72e7c6b5d4 = function(arg0) {
        getObject(arg0).preventDefault();
    };
    imports.wbg.__wbg_queueMicrotask_4d890031a6a5a50c = function(arg0) {
        queueMicrotask(getObject(arg0));
    };
    imports.wbg.__wbg_queueMicrotask_adae4bc085237231 = function(arg0) {
        const ret = getObject(arg0).queueMicrotask;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'function';
        return ret;
    };
    imports.wbg.__wbg_get_f01601b5a68d10e3 = function(arg0, arg1) {
        const ret = getObject(arg0)[arg1 >>> 0];
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_length_1009b1af0c481d7b = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_new_ffc6d4d085022169 = function() {
        const ret = new Array();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newnoargs_c62ea9419c21fbac = function(arg0, arg1) {
        var v0 = getCachedStringFromWasm0(arg0, arg1);
        const ret = new Function(v0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_next_9b877f231f476d01 = function(arg0) {
        const ret = getObject(arg0).next;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_next_6529ee0cca8d57ed = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).next();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_done_5fe336b092d60cf2 = function(arg0) {
        const ret = getObject(arg0).done;
        return ret;
    };
    imports.wbg.__wbg_value_0c248a78fdc8e19f = function(arg0) {
        const ret = getObject(arg0).value;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_iterator_db7ca081358d4fb2 = function() {
        const ret = Symbol.iterator;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_get_7b48513de5dc5ea4 = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_90c26b09837aba1c = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_9fb8d994e1c0aaac = function() {
        const ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_self_f0e34d89f33b99fd = function() { return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_d3b084224f4774d7 = function() { return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_9caa27ff917c6860 = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_35dfdd59a4da3e74 = function() { return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_decodeURI_1e508fc8ed99cae7 = function() { return handleError(function (arg0, arg1) {
        var v0 = getCachedStringFromWasm0(arg0, arg1);
        const ret = decodeURI(v0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_at_52fa04dd29154681 = function(arg0, arg1) {
        const ret = getObject(arg0).at(arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_f2740edb12e318cd = function(arg0, arg1, arg2) {
        getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
    };
    imports.wbg.__wbg_isArray_74fb723e24f76012 = function(arg0) {
        const ret = Array.isArray(getObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_byteLength_8d93f51346bae2bd = function(arg0) {
        const ret = getObject(arg0).byteLength;
        return ret;
    };
    imports.wbg.__wbg_byteOffset_f9aaddef53c4bbe1 = function(arg0) {
        const ret = getObject(arg0).byteOffset;
        return ret;
    };
    imports.wbg.__wbg_getUint8_4e5871213962b43d = function(arg0, arg1) {
        const ret = getObject(arg0).getUint8(arg1 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_call_5da1969d7cd31ccd = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_is_ff7acd231c75c0e4 = function(arg0, arg1) {
        const ret = Object.is(getObject(arg0), getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_exec_42513e2d2ddabd95 = function(arg0, arg1, arg2) {
        var v0 = getCachedStringFromWasm0(arg1, arg2);
        const ret = getObject(arg0).exec(v0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_new_e145ee1b0ed9b4aa = function(arg0, arg1, arg2, arg3) {
        var v0 = getCachedStringFromWasm0(arg0, arg1);
        var v1 = getCachedStringFromWasm0(arg2, arg3);
        const ret = new RegExp(v0, v1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_resolve_6e1c6553a82f85b7 = function(arg0) {
        const ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_3ab08cd4fbb91ae9 = function(arg0, arg1) {
        const ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_8371cc12cfedc5a2 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stringify_e1b19966d964d242 = function() { return handleError(function (arg0) {
        const ret = JSON.stringify(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_set_759f75cd92b612d2 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_closure_wrapper410 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 158, __wbg_adapter_36);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper412 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 158, __wbg_adapter_36);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper414 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 158, __wbg_adapter_36);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper415 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 158, __wbg_adapter_36);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper416 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 158, __wbg_adapter_36);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper417 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 158, __wbg_adapter_47);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1030 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 424, __wbg_adapter_50);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1290 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 537, __wbg_adapter_53);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1426 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 582, __wbg_adapter_56);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper2835 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 593, __wbg_adapter_59);
        return addHeapObject(ret);
    };

    return imports;
}

function __wbg_init_memory(imports, maybe_memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedFloat64Memory0 = null;
    cachedInt32Memory0 = null;
    cachedUint8Memory0 = null;

    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(input) {
    if (wasm !== undefined) return wasm;

    if (typeof input === 'undefined') {
        input = new URL('tauri-app-ui-609906bd72c7516e_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await input, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync }
export default __wbg_init;
