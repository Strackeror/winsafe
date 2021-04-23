#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::kernel32;
use crate::funcs::GetLastError;
use crate::privs::{bool_to_winresult, ptr_as_opt};
use crate::structs::SECURITY_ATTRIBUTES;
use crate::WString;

handle_type! {
	/// Handle to a
	/// [file](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hfile).
	HFILE
}

impl HFILE {
	/// [`CloseHandle`](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)
	/// method.
	pub fn CloseHandle(&self) -> WinResult<()> {
		bool_to_winresult(unsafe { kernel32::CloseHandle(self.ptr) })
	}

	/// [`CreateFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew)
	/// static method.
	///
	/// **Note:** Must be paired with a
	/// [`CloseHandle`](crate::HFILE::CloseHandle) call.
	///
	/// # Examples
	///
	/// Opening an existing file as read-only:
	///
	/// ```rust,ignore
	/// use winsafe::{co, HFILE};
	///
	/// let hfile = HFILE::CreateFile(
	///     "C:\\Temp\\something.txt",
	///     co::GENERIC::READ,
	///     co::FILE_SHARE::READ,
	///     None,
	///     co::DISPOSITION::OPEN_EXISTING,
	///     co::FILE_ATTRIBUTE::NORMAL,
	///     None,
	/// ).unwrap();
	///
	/// hfile.CloseHandle().unwrap();
	/// ```
	///
	/// Opening a file for read and write. If the file doesn't exist, create it:
	///
	/// ```rust,ignore
	/// use winsafe::{co, HFILE};
	///
	/// let hfile = w::HFILE::CreateFile(
	///     "C:\\Temp\\something.txt",
	///     co::GENERIC::READ | co::GENERIC::WRITE,
	///     co::FILE_SHARE::NONE,
	///     None,
	///     co::DISPOSITION::OPEN_ALWAYS,
	///     co::FILE_ATTRIBUTE::NORMAL,
	///     None,
	/// ).unwrap();
	///
	/// hfile.CloseHandle().unwrap();
	/// ```
	pub fn CreateFile(
		lpFileName: &str,
		dwDesiredAccess: co::GENERIC,
		dwShareMode: co::FILE_SHARE,
		lpSecurityAttributes: Option<&mut SECURITY_ATTRIBUTES>,
		dwCreationDisposition: co::DISPOSITION,
		dwFlagsAndAttributes: co::FILE_ATTRIBUTE,
		hTemplateFile: Option<HFILE>) -> WinResult<HFILE>
	{
		match ptr_as_opt(
			unsafe {
				kernel32::CreateFileW(
					WString::from_str(lpFileName).as_ptr(),
					dwDesiredAccess.0,
					dwShareMode.0,
					match lpSecurityAttributes {
						Some(lp) => lp as *mut _ as *mut _,
						None => std::ptr::null_mut(),
					},
					dwCreationDisposition.0,
					dwFlagsAndAttributes.0,
					match hTemplateFile {
						Some(h) => h.ptr,
						None => std::ptr::null_mut(),
					},
				)
			},
		) {
			Some(ptr) => Ok(Self { ptr }),
			None => Err(GetLastError()),
		}
	}
}