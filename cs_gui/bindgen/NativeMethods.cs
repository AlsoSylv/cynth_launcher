using System.Runtime.InteropServices;

namespace csbindings;

public static partial class NativeMethods {
	private const string __DllName = "csbindings";

	[DllImport(__DllName, EntryPoint = "new_rust_state", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
	public static extern unsafe State* new_rust_state(char* raw_path_ptr, nuint raw_path_len);

	[DllImport(__DllName, EntryPoint = "get_version_manifest", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
	public static extern unsafe ManifestTaskWrapper* get_version_manifest(State* state);

	[DllImport(__DllName, EntryPoint = "poll_manifest_task", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
	public static extern unsafe bool poll_manifest_task(ManifestTaskWrapper* raw_task);

	[DllImport(__DllName, EntryPoint = "await_version_manifest", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
	public static extern unsafe NativeReturn await_version_manifest(State* state, ManifestTaskWrapper* raw_task);

	[DllImport(__DllName, EntryPoint = "cancel_version_manifest", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
	public static extern unsafe void cancel_version_manifest(ManifestTaskWrapper* task);

	[DllImport(__DllName, EntryPoint = "get_latest_release", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
	public static extern unsafe RefStringWrapper get_latest_release(State* state);

	[DllImport(__DllName, EntryPoint = "get_name", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
	public static extern unsafe RefStringWrapper get_name(State* state, nuint index);

	[DllImport(__DllName, EntryPoint = "get_manifest_len", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
	public static extern unsafe nuint get_manifest_len(State* state);

	[DllImport(__DllName, EntryPoint = "is_manifest_null", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
	public static extern unsafe bool is_manifest_null(State* state);

	[DllImport(__DllName, EntryPoint = "free_owned_string_wrapper", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
	public static extern void free_owned_string_wrapper(OwnedStringWrapper string_wrapper);
}

struct RustString {
	private unsafe fixed nuint repr[3];
}