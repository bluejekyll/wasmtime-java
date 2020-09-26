package net.bluejekyll.wasmtime;

import jnr.ffi.LibraryLoader;
import jnr.ffi.Pointer;
import jnr.ffi.Runtime;
import jnr.ffi.Struct;
import jnr.ffi.annotations.In;
import jnr.ffi.annotations.Out;
import jnr.ffi.byref.AddressByReference;
import jnr.ffi.byref.PointerByReference;
import jnr.ffi.types.size_t;

import java.nio.ByteBuffer;

/**
 * Wasmtime
 * <p>
 * A wrapper over the FFI of the Rust wasmtime library, which uses Wasmtime for the WASM runtime
 */
public class Wasmtime {
    private final WasmtimeFFI ffi;
    private final Runtime ffiRuntime;

    /**
     * This FFI is derived from https://bytecodealliance.github.io/wasmtime/c-api/
     * <p>
     * Strategy here is to use opaque pointers for everything.
     */
    public static interface WasmtimeFFI {
        // ===== wasm.h
//        void 	wasm_byte_vec_new_empty (own wasm_byte_vec_t *out)
//        Initializes an empty byte vector.
//
//        WASM_API_EXTERN void 	wasm_byte_vec_new_uninitialized (own wasm_byte_vec_t *out, size_t)
//        Initializes an byte vector with the specified capacity. More...
//
//        WASM_API_EXTERN void 	wasm_byte_vec_new (own wasm_byte_vec_t *out, size_t, own wasm_byte_t const[])
//        Copies the specified data into a new byte vector. More...
//
//        WASM_API_EXTERN void 	wasm_byte_vec_copy (own wasm_byte_vec_t *out, const wasm_byte_vec_t *)
//        Copies one vector into a new vector. More...
//
//        WASM_API_EXTERN void 	wasm_byte_vec_delete (own wasm_byte_vec_t *)
//        Deletes a byte vector. More...
//
//        void 	wasm_config_delete (own wasm_config_t *)
//        Deletes a configuration object.
//
//        wasm_config_t * 	wasm_config_new ()
//        Creates a new empty configuration object. More...
//

        //        void 	wasm_engine_delete (own wasm_engine_t *)
        //        Deletes an engine.
        void wasm_engine_delete(@In Pointer wasm_engine_t);

        //                wasm_engine_t * 	wasm_engine_new ()
        //        Creates a new engine with the default configuration. More...
        Pointer wasm_engine_new();

//        wasm_engine_t * 	wasm_engine_new_with_config (wasm_config_t *)
//        Creates a new engine with the specified configuration. More...
//

        //        void 	wasm_store_delete (own wasm_store_t *)
        //        Deletes the specified store.
        void wasm_store_delete(@In Pointer wasm_store_t);

        //        wasm_store_t * 	wasm_store_new (wasm_engine_t *)
        //        Creates a new store within the specified engine. More...
        Pointer wasm_store_new(@In Pointer wasm_engine_t);

//
//        void 	wasm_valtype_delete (own wasm_valtype_t *)
//        Deletes a type.
//
//                WASM_API_EXTERN void 	wasm_valtype_vec_new_empty (own wasm_valtype_vec_t *out)
//        Creates an empty vector. More...
//
//        WASM_API_EXTERN void 	wasm_valtype_vec_new_uninitialized (own wasm_valtype_vec_t *out, size_t)
//        Creates a vector with the given capacity. More...
//
//        WASM_API_EXTERN void 	wasm_valtype_vec_new (own wasm_valtype_vec_t *out, size_t, own wasm_valtype_t *const[])
//        Creates a vector with the provided contents. More...
//
//        WASM_API_EXTERN void 	wasm_valtype_vec_copy (own wasm_valtype_vec_t *out, const wasm_valtype_vec_t *)
//        Copies one vector to another. More...
//
//        WASM_API_EXTERN void 	wasm_valtype_vec_delete (own wasm_valtype_vec_t *)
//        Deallocates memory for a vector. More...
//
//        WASM_API_EXTERN own wasm_valtype_t * 	wasm_valtype_copy (wasm_valtype_t *)
//        Creates a new value which matches the provided one. More...
//
//        wasm_valtype_t * 	wasm_valtype_new (wasm_valkind_t)
//        Creates a new value type from the specified kind. More...
//
//        wasm_valkind_t 	wasm_valtype_kind (const wasm_valtype_t *)
//        Returns the associated kind for this value type.
//
//        void 	wasm_functype_delete (own wasm_functype_t *)
//        Deletes a type.
//
//                WASM_API_EXTERN void 	wasm_functype_vec_new_empty (own wasm_functype_vec_t *out)
//        Creates an empty vector. More...
//
//        WASM_API_EXTERN void 	wasm_functype_vec_new_uninitialized (own wasm_functype_vec_t *out, size_t)
//        Creates a vector with the given capacity. More...
//
//        WASM_API_EXTERN void 	wasm_functype_vec_new (own wasm_functype_vec_t *out, size_t, own wasm_functype_t *const[])
//        Creates a vector with the provided contents. More...
//
//        WASM_API_EXTERN void 	wasm_functype_vec_copy (own wasm_functype_vec_t *out, const wasm_functype_vec_t *)
//        Copies one vector to another. More...
//
//        WASM_API_EXTERN void 	wasm_functype_vec_delete (own wasm_functype_vec_t *)
//        Deallocates memory for a vector. More...
//
//        WASM_API_EXTERN own wasm_functype_t * 	wasm_functype_copy (wasm_functype_t *)
//        Creates a new value which matches the provided one. More...
//
//        wasm_functype_t * 	wasm_functype_new (wasm_valtype_vec_t *params, wasm_valtype_vec_t *results)
//        Creates a new function type with the provided parameter and result types. More...
//
//                const wasm_valtype_vec_t * 	wasm_functype_params (const wasm_functype_t *)
//        Returns the list of parameters of this function type. More...
//
//                const wasm_valtype_vec_t * 	wasm_functype_results (const wasm_functype_t *)
//        Returns the list of results of this function type. More...
//
//        void 	wasm_globaltype_delete (own wasm_globaltype_t *)
//        Deletes a type.
//
//                WASM_API_EXTERN void 	wasm_globaltype_vec_new_empty (own wasm_globaltype_vec_t *out)
//        Creates an empty vector. More...
//
//        WASM_API_EXTERN void 	wasm_globaltype_vec_new_uninitialized (own wasm_globaltype_vec_t *out, size_t)
//        Creates a vector with the given capacity. More...
//
//        WASM_API_EXTERN void 	wasm_globaltype_vec_new (own wasm_globaltype_vec_t *out, size_t, own wasm_globaltype_t *const[])
//        Creates a vector with the provided contents. More...
//
//        WASM_API_EXTERN void 	wasm_globaltype_vec_copy (own wasm_globaltype_vec_t *out, const wasm_globaltype_vec_t *)
//        Copies one vector to another. More...
//
//        WASM_API_EXTERN void 	wasm_globaltype_vec_delete (own wasm_globaltype_vec_t *)
//        Deallocates memory for a vector. More...
//
//        WASM_API_EXTERN own wasm_globaltype_t * 	wasm_globaltype_copy (wasm_globaltype_t *)
//        Creates a new value which matches the provided one. More...
//
//        wasm_globaltype_t * 	wasm_globaltype_new (wasm_valtype_t *, wasm_mutability_t)
//        Creates a new global type. More...
//
//                const wasm_valtype_t * 	wasm_globaltype_content (const wasm_globaltype_t *)
//        Returns the type of value contained in a global. More...
//
//        wasm_mutability_t 	wasm_globaltype_mutability (const wasm_globaltype_t *)
//        Returns whether or not a global is mutable.
//
//        void 	wasm_tabletype_delete (own wasm_tabletype_t *)
//        Deletes a type.
//
//                WASM_API_EXTERN void 	wasm_tabletype_vec_new_empty (own wasm_tabletype_vec_t *out)
//        Creates an empty vector. More...
//
//        WASM_API_EXTERN void 	wasm_tabletype_vec_new_uninitialized (own wasm_tabletype_vec_t *out, size_t)
//        Creates a vector with the given capacity. More...
//
//        WASM_API_EXTERN void 	wasm_tabletype_vec_new (own wasm_tabletype_vec_t *out, size_t, own wasm_tabletype_t *const[])
//        Creates a vector with the provided contents. More...
//
//        WASM_API_EXTERN void 	wasm_tabletype_vec_copy (own wasm_tabletype_vec_t *out, const wasm_tabletype_vec_t *)
//        Copies one vector to another. More...
//
//        WASM_API_EXTERN void 	wasm_tabletype_vec_delete (own wasm_tabletype_vec_t *)
//        Deallocates memory for a vector. More...
//
//        WASM_API_EXTERN own wasm_tabletype_t * 	wasm_tabletype_copy (wasm_tabletype_t *)
//        Creates a new value which matches the provided one. More...
//
//        wasm_tabletype_t * 	wasm_tabletype_new (wasm_valtype_t *, const wasm_limits_t *)
//        Creates a new table type. More...
//
//                const wasm_valtype_t * 	wasm_tabletype_element (const wasm_tabletype_t *)
//        Returns the element type of this table. More...
//
//                const wasm_limits_t * 	wasm_tabletype_limits (const wasm_tabletype_t *)
//        Returns the limits of this table. More...
//
//        void 	wasm_memorytype_delete (own wasm_memorytype_t *)
//        Deletes a type.
//
//                WASM_API_EXTERN void 	wasm_memorytype_vec_new_empty (own wasm_memorytype_vec_t *out)
//        Creates an empty vector. More...
//
//        WASM_API_EXTERN void 	wasm_memorytype_vec_new_uninitialized (own wasm_memorytype_vec_t *out, size_t)
//        Creates a vector with the given capacity. More...
//
//        WASM_API_EXTERN void 	wasm_memorytype_vec_new (own wasm_memorytype_vec_t *out, size_t, own wasm_memorytype_t *const[])
//        Creates a vector with the provided contents. More...
//
//        WASM_API_EXTERN void 	wasm_memorytype_vec_copy (own wasm_memorytype_vec_t *out, const wasm_memorytype_vec_t *)
//        Copies one vector to another. More...
//
//        WASM_API_EXTERN void 	wasm_memorytype_vec_delete (own wasm_memorytype_vec_t *)
//        Deallocates memory for a vector. More...
//
//        WASM_API_EXTERN own wasm_memorytype_t * 	wasm_memorytype_copy (wasm_memorytype_t *)
//        Creates a new value which matches the provided one. More...
//
//        wasm_memorytype_t * 	wasm_memorytype_new (const wasm_limits_t *)
//        Creates a new memory type. More...
//
//                const wasm_limits_t * 	wasm_memorytype_limits (const wasm_memorytype_t *)
//        Returns the limits of this memory. More...
//
//        void 	wasm_externtype_delete (own wasm_externtype_t *)
//        Deletes a type.
//
//                WASM_API_EXTERN void 	wasm_externtype_vec_new_empty (own wasm_externtype_vec_t *out)
//        Creates an empty vector. More...
//
//        WASM_API_EXTERN void 	wasm_externtype_vec_new_uninitialized (own wasm_externtype_vec_t *out, size_t)
//        Creates a vector with the given capacity. More...
//
//        WASM_API_EXTERN void 	wasm_externtype_vec_new (own wasm_externtype_vec_t *out, size_t, own wasm_externtype_t *const[])
//        Creates a vector with the provided contents. More...
//
//        WASM_API_EXTERN void 	wasm_externtype_vec_copy (own wasm_externtype_vec_t *out, const wasm_externtype_vec_t *)
//        Copies one vector to another. More...
//
//        WASM_API_EXTERN void 	wasm_externtype_vec_delete (own wasm_externtype_vec_t *)
//        Deallocates extern for a vector. More...
//
//        WASM_API_EXTERN own wasm_externtype_t * 	wasm_externtype_copy (wasm_externtype_t *)
//        Creates a new value which matches the provided one. More...
//
//        wasm_externkind_t 	wasm_externtype_kind (const wasm_externtype_t *)
//        Returns the kind of external item this type represents.
//
//        wasm_externtype_t * 	wasm_functype_as_externtype (wasm_functype_t *)
//        Converts a wasm_functype_t to a wasm_externtype_t. More...
//
//        wasm_externtype_t * 	wasm_globaltype_as_externtype (wasm_globaltype_t *)
//        Converts a wasm_globaltype_t to a wasm_externtype_t. More...
//
//        wasm_externtype_t * 	wasm_tabletype_as_externtype (wasm_tabletype_t *)
//        Converts a wasm_tabletype_t to a wasm_externtype_t. More...
//
//        wasm_externtype_t * 	wasm_memorytype_as_externtype (wasm_memorytype_t *)
//        Converts a wasm_memorytype_t to a wasm_externtype_t. More...
//
//        wasm_functype_t * 	wasm_externtype_as_functype (wasm_externtype_t *)
//        Attempts to convert a wasm_externtype_t to a wasm_functype_t. More...
//
//        wasm_globaltype_t * 	wasm_externtype_as_globaltype (wasm_externtype_t *)
//        Attempts to convert a wasm_externtype_t to a wasm_globaltype_t. More...
//
//        wasm_tabletype_t * 	wasm_externtype_as_tabletype (wasm_externtype_t *)
//        Attempts to convert a wasm_externtype_t to a wasm_tabletype_t. More...
//
//        wasm_memorytype_t * 	wasm_externtype_as_memorytype (wasm_externtype_t *)
//        Attempts to convert a wasm_externtype_t to a wasm_memorytype_t. More...
//
//                const wasm_externtype_t * 	wasm_functype_as_externtype_const (const wasm_functype_t *)
//        Converts a wasm_functype_t to a wasm_externtype_t. More...
//
//                const wasm_externtype_t * 	wasm_globaltype_as_externtype_const (const wasm_globaltype_t *)
//        Converts a wasm_globaltype_t to a wasm_externtype_t. More...
//
//                const wasm_externtype_t * 	wasm_tabletype_as_externtype_const (const wasm_tabletype_t *)
//        Converts a wasm_tabletype_t to a wasm_externtype_t. More...
//
//                const wasm_externtype_t * 	wasm_memorytype_as_externtype_const (const wasm_memorytype_t *)
//        Converts a wasm_memorytype_t to a wasm_externtype_t. More...
//
//                const wasm_functype_t * 	wasm_externtype_as_functype_const (const wasm_externtype_t *)
//        Attempts to convert a wasm_externtype_t to a wasm_functype_t. More...
//
//                const wasm_globaltype_t * 	wasm_externtype_as_globaltype_const (const wasm_externtype_t *)
//        Attempts to convert a wasm_externtype_t to a wasm_globaltype_t. More...
//
//                const wasm_tabletype_t * 	wasm_externtype_as_tabletype_const (const wasm_externtype_t *)
//        Attempts to convert a wasm_externtype_t to a wasm_tabletype_t. More...
//
//                const wasm_memorytype_t * 	wasm_externtype_as_memorytype_const (const wasm_externtype_t *)
//        Attempts to convert a wasm_externtype_t to a wasm_memorytype_t. More...
//
//        void 	wasm_importtype_delete (own wasm_importtype_t *)
//        Deletes a type.
//
//                WASM_API_EXTERN void 	wasm_importtype_vec_new_empty (own wasm_importtype_vec_t *out)
//        Creates an empty vector. More...
//
//        WASM_API_EXTERN void 	wasm_importtype_vec_new_uninitialized (own wasm_importtype_vec_t *out, size_t)
//        Creates a vector with the given capacity. More...
//
//        WASM_API_EXTERN void 	wasm_importtype_vec_new (own wasm_importtype_vec_t *out, size_t, own wasm_importtype_t *const[])
//        Creates a vector with the provided contents. More...
//
//        WASM_API_EXTERN void 	wasm_importtype_vec_copy (own wasm_importtype_vec_t *out, const wasm_importtype_vec_t *)
//        Copies one vector to another. More...
//
//        WASM_API_EXTERN void 	wasm_importtype_vec_delete (own wasm_importtype_vec_t *)
//        Deallocates import for a vector. More...
//
//        WASM_API_EXTERN own wasm_importtype_t * 	wasm_importtype_copy (wasm_importtype_t *)
//        Creates a new value which matches the provided one. More...
//
//        wasm_importtype_t * 	wasm_importtype_new (wasm_name_t *module, wasm_name_t *name, wasm_externtype_t *)
//        Creates a new import type. More...
//
//                const wasm_name_t * 	wasm_importtype_module (const wasm_importtype_t *)
//        Returns the module this import is importing from. More...
//
//                const wasm_name_t * 	wasm_importtype_name (const wasm_importtype_t *)
//        Returns the name this import is importing from. More...
//
//                const wasm_externtype_t * 	wasm_importtype_type (const wasm_importtype_t *)
//        Returns the type of item this import is importing. More...
//
//        void 	wasm_exporttype_delete (own wasm_exporttype_t *)
//        Deletes a type.
//
//                WASM_API_EXTERN void 	wasm_exporttype_vec_new_empty (own wasm_exporttype_vec_t *out)
//        Creates an empty vector. More...
//
//        WASM_API_EXTERN void 	wasm_exporttype_vec_new_uninitialized (own wasm_exporttype_vec_t *out, size_t)
//        Creates a vector with the given capacity. More...
//
//        WASM_API_EXTERN void 	wasm_exporttype_vec_new (own wasm_exporttype_vec_t *out, size_t, own wasm_exporttype_t *const[])
//        Creates a vector with the provided contents. More...
//
//        WASM_API_EXTERN void 	wasm_exporttype_vec_copy (own wasm_exporttype_vec_t *out, const wasm_exporttype_vec_t *)
//        Copies one vector to another. More...
//
//        WASM_API_EXTERN void 	wasm_exporttype_vec_delete (own wasm_exporttype_vec_t *)
//        Deallocates export for a vector. More...
//
//        WASM_API_EXTERN own wasm_exporttype_t * 	wasm_exporttype_copy (wasm_exporttype_t *)
//        Creates a new value which matches the provided one. More...
//
//        wasm_exporttype_t * 	wasm_exporttype_new (wasm_name_t *, wasm_externtype_t *)
//        Creates a new export type. More...
//
//                const wasm_name_t * 	wasm_exporttype_name (const wasm_exporttype_t *)
//        Returns the name of this export. More...
//
//                const wasm_externtype_t * 	wasm_exporttype_type (const wasm_exporttype_t *)
//        Returns the type of this export. More...
//
//        void 	wasm_val_delete (wasm_val_t *v)
//        Deletes a type. More...
//
//        void 	wasm_val_copy (wasm_val_t *out, const wasm_val_t *)
//        Copies a wasm_val_t to a new one. More...
//
//        void 	wasm_val_vec_new_empty (own wasm_val_vec_t *out)
//        Creates an empty vector. More...
//
//        WASM_API_EXTERN void 	wasm_val_vec_new_uninitialized (own wasm_val_vec_t *out, size_t)
//        Creates a vector with the given capacity. More...
//
//        WASM_API_EXTERN void 	wasm_val_vec_new (own wasm_val_vec_t *out, size_t, own wasm_val_t const[])
//        Creates a vector with the provided contents. More...
//
//        WASM_API_EXTERN void 	wasm_val_vec_copy (own wasm_val_vec_t *out, const wasm_val_vec_t *)
//        Copies one vector to another. More...
//
//        WASM_API_EXTERN void 	wasm_val_vec_delete (own wasm_val_vec_t *)
//        Deallocates export for a vector. More...
//
//        void 	wasm_ref_delete (own wasm_ref_t *)
//        Delete a reference.
//
//        WASM_API_EXTERN own wasm_ref_t * 	wasm_ref_copy (const wasm_ref_t *)
//        Copy a reference.
//
//        WASM_API_EXTERN bool 	wasm_ref_same (const wasm_ref_t *, const wasm_ref_t *)
//        Are the given references pointing to the same externref? More...
//
//        WASM_API_EXTERN void * 	wasm_ref_get_host_info (const wasm_ref_t *)
//        Unimplemented in Wasmtime, always returns NULL.
//
//                WASM_API_EXTERN void 	wasm_ref_set_host_info (wasm_ref_t *, void *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void 	wasm_ref_set_host_info_with_finalizer (wasm_ref_t *, void *, void(*)(void *))
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        void 	wasm_frame_delete (own wasm_frame_t *)
//        Deletes a frame.
//
//        void 	wasm_frame_vec_new_empty (own wasm_frame_vec_t *out)
//        Creates an empty vector. More...
//
//        WASM_API_EXTERN void 	wasm_frame_vec_new_uninitialized (own wasm_frame_vec_t *out, size_t)
//        Creates a vector with the given capacity. More...
//
//        WASM_API_EXTERN void 	wasm_frame_vec_new (own wasm_frame_vec_t *out, size_t, own wasm_frame_t *const[])
//        Creates a vector with the provided contents. More...
//
//        WASM_API_EXTERN void 	wasm_frame_vec_copy (own wasm_frame_vec_t *out, const wasm_frame_vec_t *)
//        Copies one vector to another. More...
//
//        WASM_API_EXTERN void 	wasm_frame_vec_delete (own wasm_frame_vec_t *)
//        Deallocates export for a vector. More...
//
//        wasm_frame_t * 	wasm_frame_copy (const wasm_frame_t *)
//        Copies a wasm_frame_t to a new one. More...
//
//        struct wasm_instance_t * 	wasm_frame_instance (const wasm_frame_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                uint32_t 	wasm_frame_func_index (const wasm_frame_t *)
//        Returns the function index in the original wasm module that this frame corresponds to.
//
//                size_t 	wasm_frame_func_offset (const wasm_frame_t *)
//        Returns the byte offset from the beginning of the function in the original wasm file to the instruction this frame points to.
//
//                size_t 	wasm_frame_module_offset (const wasm_frame_t *)
//        Returns the byte offset from the beginning of the original wasm file to the instruction this frame points to.
//
//        void 	wasm_trap_delete (own wasm_trap_t *)
//        Deletes a trap.
//
//        WASM_API_EXTERN own wasm_trap_t * 	wasm_trap_copy (const wasm_trap_t *)
//        Copies a wasm_trap_t to a new one. More...
//
//        WASM_API_EXTERN bool 	wasm_trap_same (const wasm_trap_t *, const wasm_trap_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void * 	wasm_trap_get_host_info (const wasm_trap_t *)
//        Unimplemented in Wasmtime, always returns NULL.
//
//                WASM_API_EXTERN void 	wasm_trap_set_host_info (wasm_trap_t *, void *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void 	wasm_trap_set_host_info_with_finalizer (wasm_trap_t *, void *, void(*)(void *))
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN wasm_ref_t * 	wasm_trap_as_ref (wasm_trap_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN wasm_trap_t * 	wasm_ref_as_trap (wasm_ref_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//const WASM_API_EXTERN wasm_ref_t * 	wasm_trap_as_ref_const (const wasm_trap_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//const WASM_API_EXTERN wasm_trap_t * 	wasm_ref_as_trap_const (const wasm_ref_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                wasm_trap_t * 	wasm_trap_new (wasm_store_t *store, const wasm_message_t *)
//        Creates a new wasm_trap_t with the provided message. More...
//
//        void 	wasm_trap_message (const wasm_trap_t *, wasm_message_t *out)
//        Retrieves the message associated with this trap. More...
//
//        wasm_frame_t * 	wasm_trap_origin (const wasm_trap_t *)
//        Returns the top frame of the wasm stack responsible for this trap. More...
//
//        void 	wasm_trap_trace (const wasm_trap_t *, wasm_frame_vec_t *out)
//        Returns the trace of wasm frames for this trap. More...
//
//        void 	wasm_foreign_delete (own wasm_foreign_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN own wasm_foreign_t * 	wasm_foreign_copy (const wasm_foreign_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN bool 	wasm_foreign_same (const wasm_foreign_t *, const wasm_foreign_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void * 	wasm_foreign_get_host_info (const wasm_foreign_t *)
//        Unimplemented in Wasmtime, always returns NULL.
//
//                WASM_API_EXTERN void 	wasm_foreign_set_host_info (wasm_foreign_t *, void *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void 	wasm_foreign_set_host_info_with_finalizer (wasm_foreign_t *, void *, void(*)(void *))
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN wasm_ref_t * 	wasm_foreign_as_ref (wasm_foreign_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN wasm_foreign_t * 	wasm_ref_as_foreign (wasm_ref_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//const WASM_API_EXTERN wasm_ref_t * 	wasm_foreign_as_ref_const (const wasm_foreign_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//const WASM_API_EXTERN wasm_foreign_t * 	wasm_ref_as_foreign_const (const wasm_ref_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                wasm_foreign_t * 	wasm_foreign_new (wasm_store_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
          //        void 	wasm_module_delete (own wasm_module_t *)
          //        Deletes a module.
          void wasm_module_delete(@In Pointer wasm_module_t);

//        WASM_API_EXTERN own wasm_module_t * 	wasm_module_copy (const wasm_module_t *)
//        Copies a wasm_module_t to a new one. More...
//
//        WASM_API_EXTERN bool 	wasm_module_same (const wasm_module_t *, const wasm_module_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void * 	wasm_module_get_host_info (const wasm_module_t *)
//        Unimplemented in Wasmtime, always returns NULL.
//
//                WASM_API_EXTERN void 	wasm_module_set_host_info (wasm_module_t *, void *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void 	wasm_module_set_host_info_with_finalizer (wasm_module_t *, void *, void(*)(void *))
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN wasm_ref_t * 	wasm_module_as_ref (wasm_module_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN wasm_module_t * 	wasm_ref_as_module (wasm_ref_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//const WASM_API_EXTERN wasm_ref_t * 	wasm_module_as_ref_const (const wasm_module_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//const WASM_API_EXTERN wasm_module_t * 	wasm_ref_as_module_const (const wasm_ref_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void 	wasm_shared_module_delete (own wasm_shared_module_t *)
//        Deletes the provided module.
//
//        WASM_API_EXTERN own wasm_shared_module_t * 	wasm_module_share (const wasm_module_t *)
//        Creates a shareable module from the provided module. More...
//
//        WASM_API_EXTERN own wasm_module_t * 	wasm_module_obtain (wasm_store_t *, const wasm_shared_module_t *)
//        Attempts to create a wasm_module_t from the shareable module. More...
//
//        wasm_module_t * 	wasm_module_new (wasm_store_t *, const wasm_byte_vec_t *binary)
//        Compiles a raw WebAssembly binary to a wasm_module_t. More...
//
//        bool 	wasm_module_validate (wasm_store_t *, const wasm_byte_vec_t *binary)
//        Validates whether a provided byte sequence is a valid wasm binary. More...
//
//        void 	wasm_module_imports (const wasm_module_t *, wasm_importtype_vec_t *out)
//        Returns the list of imports that this module expects. More...
//
//        void 	wasm_module_exports (const wasm_module_t *, wasm_exporttype_vec_t *out)
//        Returns the list of exports that this module provides. More...
//
//        void 	wasm_module_serialize (const wasm_module_t *, wasm_byte_vec_t *out)
//        Unimplemented in Wasmtime.
//
//                wasm_module_t * 	wasm_module_deserialize (wasm_store_t *, const wasm_byte_vec_t *)
//        Unimplemented in Wasmtime.
//
//        void 	wasm_func_delete (own wasm_func_t *)
//        Deletes a func.
//
//        WASM_API_EXTERN own wasm_func_t * 	wasm_func_copy (const wasm_func_t *)
//        Copies a wasm_func_t to a new one. More...
//
//        WASM_API_EXTERN bool 	wasm_func_same (const wasm_func_t *, const wasm_func_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void * 	wasm_func_get_host_info (const wasm_func_t *)
//        Unimplemented in Wasmtime, always returns NULL.
//
//                WASM_API_EXTERN void 	wasm_func_set_host_info (wasm_func_t *, void *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void 	wasm_func_set_host_info_with_finalizer (wasm_func_t *, void *, void(*)(void *))
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN wasm_ref_t * 	wasm_func_as_ref (wasm_func_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN wasm_func_t * 	wasm_ref_as_func (wasm_ref_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//const WASM_API_EXTERN wasm_ref_t * 	wasm_func_as_ref_const (const wasm_func_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//const WASM_API_EXTERN wasm_func_t * 	wasm_ref_as_func_const (const wasm_ref_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                wasm_func_t * 	wasm_func_new (wasm_store_t *, const wasm_functype_t *, wasm_func_callback_t)
//        Creates a new WebAssembly function with host functionality. More...
//
//        wasm_func_t * 	wasm_func_new_with_env (wasm_store_t *, const wasm_functype_t *type, wasm_func_callback_with_env_t, void *env, void(*finalizer)(void *))
//        Creates a new WebAssembly function with host functionality. More...
//
//        wasm_functype_t * 	wasm_func_type (const wasm_func_t *)
//        Returns the type of this function. More...
//
//        size_t 	wasm_func_param_arity (const wasm_func_t *)
//        Returns the number of arguments expected by this function.
//
//                size_t 	wasm_func_result_arity (const wasm_func_t *)
//        Returns the number of results returned by this function.
//
//                wasm_trap_t * 	wasm_func_call (const wasm_func_t *, const wasm_val_t args[], wasm_val_t results[])
//        Calls the provided function with the arguments given. More...
//
//        void 	wasm_global_delete (own wasm_global_t *)
//        Deletes a global.
//
//        WASM_API_EXTERN own wasm_global_t * 	wasm_global_copy (const wasm_global_t *)
//        Copies a wasm_global_t to a new one. More...
//
//        WASM_API_EXTERN bool 	wasm_global_same (const wasm_global_t *, const wasm_global_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void * 	wasm_global_get_host_info (const wasm_global_t *)
//        Unimplemented in Wasmtime, always returns NULL.
//
//                WASM_API_EXTERN void 	wasm_global_set_host_info (wasm_global_t *, void *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void 	wasm_global_set_host_info_with_finalizer (wasm_global_t *, void *, void(*)(void *))
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN wasm_ref_t * 	wasm_global_as_ref (wasm_global_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN wasm_global_t * 	wasm_ref_as_global (wasm_ref_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//const WASM_API_EXTERN wasm_ref_t * 	wasm_global_as_ref_const (const wasm_global_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//const WASM_API_EXTERN wasm_global_t * 	wasm_ref_as_global_const (const wasm_ref_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                wasm_global_t * 	wasm_global_new (wasm_store_t *, const wasm_globaltype_t *, const wasm_val_t *)
//        Creates a new WebAssembly global. More...
//
//        wasm_globaltype_t * 	wasm_global_type (const wasm_global_t *)
//        Returns the type of this global. More...
//
//        void 	wasm_global_get (const wasm_global_t *, wasm_val_t *out)
//        Gets the value of this global. More...
//
//        void 	wasm_global_set (wasm_global_t *, const wasm_val_t *)
//        Sets the value of this global. More...
//
//        void 	wasm_table_delete (own wasm_table_t *)
//        Deletes a table.
//
//        WASM_API_EXTERN own wasm_table_t * 	wasm_table_copy (const wasm_table_t *)
//        Copies a wasm_table_t to a new one. More...
//
//        WASM_API_EXTERN bool 	wasm_table_same (const wasm_table_t *, const wasm_table_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void * 	wasm_table_get_host_info (const wasm_table_t *)
//        Unimplemented in Wasmtime, always returns NULL.
//
//                WASM_API_EXTERN void 	wasm_table_set_host_info (wasm_table_t *, void *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void 	wasm_table_set_host_info_with_finalizer (wasm_table_t *, void *, void(*)(void *))
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN wasm_ref_t * 	wasm_table_as_ref (wasm_table_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN wasm_table_t * 	wasm_ref_as_table (wasm_ref_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//const WASM_API_EXTERN wasm_ref_t * 	wasm_table_as_ref_const (const wasm_table_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//const WASM_API_EXTERN wasm_table_t * 	wasm_ref_as_table_const (const wasm_ref_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                wasm_table_t * 	wasm_table_new (wasm_store_t *, const wasm_tabletype_t *, wasm_ref_t *init)
//        Creates a new WebAssembly table. More...
//
//        wasm_tabletype_t * 	wasm_table_type (const wasm_table_t *)
//        Returns the type of this table. More...
//
//        wasm_ref_t * 	wasm_table_get (const wasm_table_t *, wasm_table_size_t index)
//        Gets an element from this table. More...
//
//        bool 	wasm_table_set (wasm_table_t *, wasm_table_size_t index, wasm_ref_t *)
//        Sets an element in this table. More...
//
//        wasm_table_size_t 	wasm_table_size (const wasm_table_t *)
//        Gets the current size, in elements, of this table.
//
//                bool 	wasm_table_grow (wasm_table_t *, wasm_table_size_t delta, wasm_ref_t *init)
//        Attempts to grow this table by delta elements. More...
//
//        void 	wasm_memory_delete (own wasm_memory_t *)
//        Deletes a memory.
//
//        WASM_API_EXTERN own wasm_memory_t * 	wasm_memory_copy (const wasm_memory_t *)
//        Copies a wasm_memory_t to a new one. More...
//
//        WASM_API_EXTERN bool 	wasm_memory_same (const wasm_memory_t *, const wasm_memory_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void * 	wasm_memory_get_host_info (const wasm_memory_t *)
//        Unimplemented in Wasmtime, always returns NULL.
//
//                WASM_API_EXTERN void 	wasm_memory_set_host_info (wasm_memory_t *, void *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void 	wasm_memory_set_host_info_with_finalizer (wasm_memory_t *, void *, void(*)(void *))
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN wasm_ref_t * 	wasm_memory_as_ref (wasm_memory_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN wasm_memory_t * 	wasm_ref_as_memory (wasm_ref_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//const WASM_API_EXTERN wasm_ref_t * 	wasm_memory_as_ref_const (const wasm_memory_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//const WASM_API_EXTERN wasm_memory_t * 	wasm_ref_as_memory_const (const wasm_ref_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                wasm_memory_t * 	wasm_memory_new (wasm_store_t *, const wasm_memorytype_t *)
//        Creates a new WebAssembly memory.
//
//        wasm_memorytype_t * 	wasm_memory_type (const wasm_memory_t *)
//        Returns the type of this memory. More...
//
//        byte_t * 	wasm_memory_data (wasm_memory_t *)
//        Returns the base address, in memory, where this memory is located. More...
//
//        size_t 	wasm_memory_data_size (const wasm_memory_t *)
//        Returns the size, in bytes, of this memory.
//
//                wasm_memory_pages_t 	wasm_memory_size (const wasm_memory_t *)
//        Returns the size, in wasm pages, of this memory.
//
//                bool 	wasm_memory_grow (wasm_memory_t *, wasm_memory_pages_t delta)
//        Attempts to grow this memory by delta wasm pages. More...
//
//        void 	wasm_extern_delete (own wasm_extern_t *)
//        Deletes a extern.
//
//        WASM_API_EXTERN own wasm_extern_t * 	wasm_extern_copy (const wasm_extern_t *)
//        Copies a wasm_extern_t to a new one. More...
//
//        WASM_API_EXTERN bool 	wasm_extern_same (const wasm_extern_t *, const wasm_extern_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void * 	wasm_extern_get_host_info (const wasm_extern_t *)
//        Unimplemented in Wasmtime, always returns NULL.
//
//                WASM_API_EXTERN void 	wasm_extern_set_host_info (wasm_extern_t *, void *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void 	wasm_extern_set_host_info_with_finalizer (wasm_extern_t *, void *, void(*)(void *))
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN wasm_ref_t * 	wasm_extern_as_ref (wasm_extern_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN wasm_extern_t * 	wasm_ref_as_extern (wasm_ref_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//const WASM_API_EXTERN wasm_ref_t * 	wasm_extern_as_ref_const (const wasm_extern_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//const WASM_API_EXTERN wasm_extern_t * 	wasm_ref_as_extern_const (const wasm_ref_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        void 	wasm_extern_vec_new_empty (own wasm_extern_vec_t *out)
//        Creates an empty vector. More...
//
//        WASM_API_EXTERN void 	wasm_extern_vec_new_uninitialized (own wasm_extern_vec_t *out, size_t)
//        Creates a vector with the given capacity. More...
//
//        WASM_API_EXTERN void 	wasm_extern_vec_new (own wasm_extern_vec_t *out, size_t, own wasm_extern_t *const[])
//        Creates a vector with the provided contents. More...
//
//        WASM_API_EXTERN void 	wasm_extern_vec_copy (own wasm_extern_vec_t *out, const wasm_extern_vec_t *)
//        Copies one vector to another. More...
//
//        WASM_API_EXTERN void 	wasm_extern_vec_delete (own wasm_extern_vec_t *)
//        Deallocates import for a vector. More...
//
//        wasm_externkind_t 	wasm_extern_kind (const wasm_extern_t *)
//        Returns the kind of this extern, indicating what it will downcast as.
//
//        wasm_externtype_t * 	wasm_extern_type (const wasm_extern_t *)
//        Returns the type of this extern. More...
//
//        wasm_extern_t * 	wasm_func_as_extern (wasm_func_t *)
//        Converts a wasm_func_t to wasm_extern_t. More...
//
//        wasm_extern_t * 	wasm_global_as_extern (wasm_global_t *)
//        Converts a wasm_global_t to wasm_extern_t. More...
//
//        wasm_extern_t * 	wasm_table_as_extern (wasm_table_t *)
//        Converts a wasm_table_t to wasm_extern_t. More...
//
//        wasm_extern_t * 	wasm_memory_as_extern (wasm_memory_t *)
//        Converts a wasm_memory_t to wasm_extern_t. More...
//
//        wasm_func_t * 	wasm_extern_as_func (wasm_extern_t *)
//        Converts a wasm_extern_t to wasm_func_t. More...
//
//        wasm_global_t * 	wasm_extern_as_global (wasm_extern_t *)
//        Converts a wasm_extern_t to wasm_global_t. More...
//
//        wasm_table_t * 	wasm_extern_as_table (wasm_extern_t *)
//        Converts a wasm_extern_t to wasm_table_t. More...
//
//        wasm_memory_t * 	wasm_extern_as_memory (wasm_extern_t *)
//        Converts a wasm_extern_t to wasm_memory_t. More...
//
//                const wasm_extern_t * 	wasm_func_as_extern_const (const wasm_func_t *)
//        Converts a wasm_func_t to wasm_extern_t. More...
//
//                const wasm_extern_t * 	wasm_global_as_extern_const (const wasm_global_t *)
//        Converts a wasm_global_t to wasm_extern_t. More...
//
//                const wasm_extern_t * 	wasm_table_as_extern_const (const wasm_table_t *)
//        Converts a wasm_table_t to wasm_extern_t. More...
//
//                const wasm_extern_t * 	wasm_memory_as_extern_const (const wasm_memory_t *)
//        Converts a wasm_memory_t to wasm_extern_t. More...
//
//                const wasm_func_t * 	wasm_extern_as_func_const (const wasm_extern_t *)
//        Converts a wasm_extern_t to wasm_func_t. More...
//
//                const wasm_global_t * 	wasm_extern_as_global_const (const wasm_extern_t *)
//        Converts a wasm_extern_t to wasm_global_t. More...
//
//                const wasm_table_t * 	wasm_extern_as_table_const (const wasm_extern_t *)
//        Converts a wasm_extern_t to wasm_table_t. More...
//
//                const wasm_memory_t * 	wasm_extern_as_memory_const (const wasm_extern_t *)
//        Converts a wasm_extern_t to wasm_memory_t. More...
//
//        void 	wasm_instance_delete (own wasm_instance_t *)
//        Deletes a instance.
//
//        WASM_API_EXTERN own wasm_instance_t * 	wasm_instance_copy (const wasm_instance_t *)
//        Copies a wasm_instance_t to a new one. More...
//
//        WASM_API_EXTERN bool 	wasm_instance_same (const wasm_instance_t *, const wasm_instance_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void * 	wasm_instance_get_host_info (const wasm_instance_t *)
//        Unimplemented in Wasmtime, always returns NULL.
//
//                WASM_API_EXTERN void 	wasm_instance_set_host_info (wasm_instance_t *, void *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                WASM_API_EXTERN void 	wasm_instance_set_host_info_with_finalizer (wasm_instance_t *, void *, void(*)(void *))
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN wasm_ref_t * 	wasm_instance_as_ref (wasm_instance_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//        WASM_API_EXTERN wasm_instance_t * 	wasm_ref_as_instance (wasm_ref_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//const WASM_API_EXTERN wasm_ref_t * 	wasm_instance_as_ref_const (const wasm_instance_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//const WASM_API_EXTERN wasm_instance_t * 	wasm_ref_as_instance_const (const wasm_ref_t *)
//        Unimplemented in Wasmtime, aborts the process if called.
//
//                wasm_instance_t * 	wasm_instance_new (wasm_store_t *, const wasm_module_t *, const wasm_extern_t *const imports[], wasm_trap_t **)
//        Instantiates a module with the provided imports. More...
//
//        void 	wasm_instance_exports (const wasm_instance_t *, wasm_extern_vec_t *out)
//        Returns the exports of an instance. More...

        // ===== wasmtime.h

        //        void 	wasmtime_error_delete (own wasmtime_error_t *)
        //        Deletes an error.
        void wasmtime_error_delete(@In Pointer wasmtime_error_t);

        //        void wasmtime_error_message (const wasmtime_error_t *error, wasm_name_t *message)
        //        Returns the string description of this error. More...
        void wasmtime_error_message(@In Pointer wasmtime_error_t, @Out String message);

//
//        void 	wasmtime_config_debug_info_set (wasm_config_t *, bool)
//        Configures whether DWARF debug information is constructed at runtime to describe JIT code. More...
//
//        void 	wasmtime_config_interruptable_set (wasm_config_t *, bool)
//        Enables WebAssembly code to be interrupted. More...
//
//        void 	wasmtime_config_max_wasm_stack_set (wasm_config_t *, size_t)
//        Configures the maximum stack size, in bytes, that JIT code can use. More...
//
//        void 	wasmtime_config_wasm_threads_set (wasm_config_t *, bool)
//        Configures whether the WebAssembly threading proposal is enabled. More...
//
//        void 	wasmtime_config_wasm_reference_types_set (wasm_config_t *, bool)
//        Configures whether the WebAssembly reference types proposal is enabled. More...
//
//        void 	wasmtime_config_wasm_simd_set (wasm_config_t *, bool)
//        Configures whether the WebAssembly SIMD proposal is enabled. More...
//
//        void 	wasmtime_config_wasm_bulk_memory_set (wasm_config_t *, bool)
//        Configures whether the WebAssembly bulk memory proposal is enabled. More...
//
//        void 	wasmtime_config_wasm_multi_value_set (wasm_config_t *, bool)
//        Configures whether the WebAssembly multi value proposal is enabled. More...
//
//        wasmtime_error_t * 	wasmtime_config_strategy_set (wasm_config_t *, wasmtime_strategy_t)
//        Configures how JIT code will be compiled. More...
//
//        void 	wasmtime_config_cranelift_debug_verifier_set (wasm_config_t *, bool)
//        Configures whether Cranelift's debug verifier is enabled. More...
//
//        void 	wasmtime_config_cranelift_opt_level_set (wasm_config_t *, wasmtime_opt_level_t)
//        Configures Cranelift's optimization level for JIT code. More...
//
//        wasmtime_error_t * 	wasmtime_config_profiler_set (wasm_config_t *, wasmtime_profiling_strategy_t)
//        Configures the profiling strategy used for JIT code. More...
//
//        void 	wasmtime_config_static_memory_maximum_size_set (wasm_config_t *, uint64_t)
//        Configures the maximum size for memory to be considered "static". More...
//
//        void 	wasmtime_config_static_memory_guard_size_set (wasm_config_t *, uint64_t)
//        Configures the guard region size for "static" memory. More...
//
//        void 	wasmtime_config_dynamic_memory_guard_size_set (wasm_config_t *, uint64_t)
//        Configures the guard region size for "dynamic" memory. More...
//
//        wasmtime_error_t * 	wasmtime_config_cache_config_load (wasm_config_t *, const char *)
//        Enables Wasmtime's cache and loads configuration from the specified path. More...
//
//        wasmtime_error_t * 	wasmtime_wat2wasm (const wasm_byte_vec_t *wat, wasm_byte_vec_t *ret)
//        Converts from the text format of WebAssembly to to the binary format. More...
//
//        void 	wasmtime_store_gc (wasm_store_t *store)
//        Perform garbage collection within the given store. More...
//
//        void 	wasmtime_linker_delete (own wasmtime_linker_t *)
//        Deletes a linker.
//
//                wasmtime_linker_t * 	wasmtime_linker_new (wasm_store_t *store)
//        Creates a new linker which will link together objects in the specified store. More...
//
//        void 	wasmtime_linker_allow_shadowing (wasmtime_linker_t *linker, bool allow_shadowing)
//        Configures whether this linker allows later definitions to shadow previous definitions. More...
//
//        wasmtime_error_t * 	wasmtime_linker_define (wasmtime_linker_t *linker, const wasm_name_t *module, const wasm_name_t *name, const wasm_extern_t *item)
//        Defines a new item in this linker. More...
//
//        wasmtime_error_t * 	wasmtime_linker_define_wasi (wasmtime_linker_t *linker, const wasi_instance_t *instance)
//        Defines a WASI instance in this linker. More...
//
//        wasmtime_error_t * 	wasmtime_linker_define_instance (wasmtime_linker_t *linker, const wasm_name_t *name, const wasm_instance_t *instance)
//        Defines an instance under the specified name in this linker. More...
//
//        wasmtime_error_t * 	wasmtime_linker_instantiate (const wasmtime_linker_t *linker, const wasm_module_t *module, wasm_instance_t **instance, wasm_trap_t **trap)
//        Instantiates a wasm_module_t with the items defined in this linker. More...
//
//        wasmtime_error_t * 	wasmtime_linker_module (const wasmtime_linker_t *linker, const wasm_name_t *name, const wasm_module_t *module)
//        Defines automatic instantiations of a wasm_module_t in this linker. More...
//
//        wasmtime_error_t * 	wasmtime_linker_get_default (const wasmtime_linker_t *linker, const wasm_name_t *name, wasm_func_t **func)
//        Acquires the "default export" of the named module in this linker. More...
//
//        wasmtime_error_t * 	wasmtime_linker_get_one_by_name (const wasmtime_linker_t *linker, const wasm_name_t *module, const wasm_name_t *name, wasm_extern_t **item)
//        Loads an item by name from this linker. More...
//
//        wasm_func_t * 	wasmtime_func_new (wasm_store_t *, const wasm_functype_t *, wasmtime_func_callback_t callback)
//        Creates a new host-defined function. More...
//
//        wasm_func_t * 	wasmtime_func_new_with_env (wasm_store_t *store, const wasm_functype_t *type, wasmtime_func_callback_with_env_t callback, void *env, void(*finalizer)(void *))
//        Creates a new host-defined function. More...
//
//        void 	wasmtime_func_as_funcref (const wasm_func_t *func, wasm_val_t *funcrefp)
//        Creates a new funcref value referencing func. More...
//
//        wasm_func_t * 	wasmtime_funcref_as_func (const wasm_val_t *val)
//        Get the wasm_func_t* referenced by the given funcref value. More...
//
//        wasm_extern_t * 	wasmtime_caller_export_get (const wasmtime_caller_t *caller, const wasm_name_t *name)
//        Loads a wasm_extern_t from the caller's context. More...
//
//        void 	wasmtime_interrupt_handle_delete (own wasmtime_interrupt_handle_t *)
//        Deletes an interrupt handle.
//
//        wasmtime_interrupt_handle_t * 	wasmtime_interrupt_handle_new (wasm_store_t *store)
//        Creates a new interrupt handle to interrupt executing WebAssembly from the provided store. More...
//
//        void 	wasmtime_interrupt_handle_interrupt (wasmtime_interrupt_handle_t *handle)
//        Requests that WebAssembly code running in the store attached to this interrupt handle is interrupted. More...
//
//        bool 	wasmtime_trap_exit_status (const wasm_trap_t *, int *status)
//        Attempts to extract a WASI-specific exit status from this trap. More...
//
//                const wasm_name_t * 	wasmtime_frame_func_name (const wasm_frame_t *)
//        Returns a human-readable name for this frame's function. More...
//
//                const wasm_name_t * 	wasmtime_frame_module_name (const wasm_frame_t *)
//        Returns a human-readable name for this frame's module. More...
//
//        wasmtime_error_t * 	wasmtime_func_call (wasm_func_t *func, const wasm_val_t *args, size_t num_args, wasm_val_t *results, size_t num_results, wasm_trap_t **trap)
//        Call a WebAssembly function. More...
//
//        wasmtime_error_t * 	wasmtime_global_new (wasm_store_t *store, const wasm_globaltype_t *type, const wasm_val_t *val, wasm_global_t **ret)
//        Creates a new global value. More...
//
//        wasmtime_error_t * 	wasmtime_global_set (wasm_global_t *global, const wasm_val_t *val)
//        Sets a global to a new value. More...
//
//        wasmtime_error_t * 	wasmtime_instance_new (wasm_store_t *store, const wasm_module_t *module, const wasm_extern_t *const imports[], size_t num_imports, wasm_instance_t **instance, wasm_trap_t **trap)
//        Wasmtime-specific function to instantiate a module. More...
        
//        wasmtime_error_t * 	wasmtime_module_new (wasm_engine_t *engine, const wasm_byte_vec_t *binary, wasm_module_t **ret)
//        Wasmtime-specific function to compile a module. More...
        Pointer wasmtime_module_new(@In Pointer wasm_engine_t, @In WasmByteVecT wasm_byte_vec_t, @Out PointerByReference wasm_module_t);

//
//        wasmtime_error_t * 	wasmtime_module_validate (wasm_store_t *store, const wasm_byte_vec_t *binary)
//        Wasmtime-specific function to validate a module. More...
//
//        wasmtime_error_t * 	wasmtime_funcref_table_new (wasm_store_t *store, const wasm_tabletype_t *element_ty, wasm_func_t *init, wasm_table_t **table)
//        Creates a new host-defined wasm table. More...
//
//        bool 	wasmtime_funcref_table_get (const wasm_table_t *table, wasm_table_size_t index, wasm_func_t **func)
//        Gets a value in a table. More...
//
//        wasmtime_error_t * 	wasmtime_funcref_table_set (wasm_table_t *table, wasm_table_size_t index, const wasm_func_t *value)
//        Sets a value in a table. More...
//
//        wasmtime_error_t * 	wasmtime_funcref_table_grow (wasm_table_t *table, wasm_table_size_t delta, const wasm_func_t *init, wasm_table_size_t *prev_size)
//        Grows a table. More...
//
//        void 	wasmtime_externref_new (void *data, wasm_val_t *valp)
//        Create a new externref value. More...
//
//        void 	wasmtime_externref_new_with_finalizer (void *data, wasmtime_externref_finalizer_t finalizer, wasm_val_t *valp)
//        Create a new externref value with a finalizer. More...
//
//        bool 	wasmtime_externref_data (wasm_val_t *val, void **datap)
//        Get an externref's wrapped data. More...
//
//        wasmtime_error_t * 	wasmtime_module_serialize (wasm_module_t *module, wasm_byte_vec_t *ret)
//        This function serializes compiled module artifacts as blob data. More...
//
//        wasmtime_error_t * 	wasmtime_module_deserialize (wasm_engine_t *engine, const wasm_byte_vec_t *serialized, wasm_module_t **ret)
//        Build a module from serialized data.
//
//        This function does not take ownership of any of its arguments, but the returned error and module are owned by the caller.


    }

    public static final class WasmByteVecT extends Struct {
        public final Struct.UnsignedLong size = new UnsignedLong();
        public final Struct.Pointer data = new Pointer();

        public WasmByteVecT(Runtime runtime) {
            super(runtime);
        }

        public void setData(byte[] bytes) {
            ByteBuffer buf = ByteBuffer.wrap(bytes);
            jnr.ffi.Pointer ptr = jnr.ffi.Pointer.wrap(this.getRuntime(), buf);

            this.size.set(bytes.length);
            this.data.set(ptr);
        }
    }

    public Wasmtime() {
        this.ffi = LibraryLoader.create(WasmtimeFFI.class).library("wasmtime").load();
        this.ffiRuntime = Runtime.getRuntime(this.ffi);
    }

    public WasmEngineT newWasmEngine() {
        Pointer ptr = this.ffi.wasm_engine_new();
        return new WasmEngineT(this.ffi, this.ffiRuntime, ptr);
    }

    public static void main(String[] args) {
        System.out.println("Hello World!");
    }
}

