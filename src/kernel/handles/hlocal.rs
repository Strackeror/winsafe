#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::guard::{LocalFreeGuard, LocalUnlockGuard};
use crate::kernel::privs::{
	LMEM_INVALID_HANDLE, ptr_to_sysresult, ptr_to_sysresult_handle,
};
use crate::prelude::Handle;

impl_handle! { HLOCAL;
	/// Handle to a
	/// [local memory block](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hlocal).
	///
	/// The allocated memory block is accessible through the
	/// [`LocalLock`](crate::prelude::kernel_Hlocal::LocalLock) method.
}

impl kernel_Hlocal for HLOCAL {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HLOCAL`](crate::HLOCAL).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hlocal: Handle {
	/// [`LocalAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localalloc)
	/// static method.
	#[must_use]
	fn LocalAlloc(
		flags: Option<co::LMEM>, num_bytes: usize) -> SysResult<LocalFreeGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				kernel::ffi::LocalAlloc(flags.unwrap_or_default().raw(), num_bytes),
			).map(|h| LocalFreeGuard::new(h))
		}
	}

	/// [`LocalFlags`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localflags)
	/// method.
	#[must_use]
	fn LocalFlags(&self) -> SysResult<co::LMEM> {
		match unsafe { kernel::ffi::LocalFlags(self.ptr()) } {
			LMEM_INVALID_HANDLE => Err(GetLastError()),
			flags => Ok(unsafe { co::LMEM::from_raw(flags) }),
		}
	}

	/// [`LocalLock`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-locallock)
	/// method.
	///
	/// Calls [`LocalSize`](crate::prelude::kernel_Hlocal::LocalSize) to
	/// retrieve the size of the memory block.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HLOCAL};
	///
	/// let hlocal = HLOCAL::LocalAlloc(
	///     Some(co::LMEM::FIXED | co::LMEM::ZEROINIT),
	///     120,
	/// )?;
	///
	/// let mut block = hlocal.LocalLock()?;
	///
	/// block.as_mut_slice()[0] = 40;
	///
	/// // LocalUnlock() called automatically
	///
	/// // LocalFree() called automatically
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn LocalLock(&self) -> SysResult<LocalUnlockGuard<'_, Self>> {
		let mem_sz = self.LocalSize()?;
		unsafe {
			ptr_to_sysresult(kernel::ffi::LocalLock(self.ptr()))
				.map(|ptr| LocalUnlockGuard::new(self, ptr, mem_sz))
		}
	}

	/// [`LocalReAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localrealloc)
	/// method.
	///
	/// Originally this method returns the handle to the reallocated memory
	/// object; here the original handle is automatically updated.
	fn LocalReAlloc(&mut self,
		num_bytes: usize, flags: Option<co::LMEM>) -> SysResult<()>
	{
		ptr_to_sysresult_handle(
			unsafe {
				kernel::ffi::LocalReAlloc(
					self.ptr(),
					num_bytes,
					flags.unwrap_or_default().raw(),
				)
			},
		).map(|h| { *self = h; })
	}

	/// [`LocalSize`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localsize)
	/// method.
	#[must_use]
	fn LocalSize(&self) -> SysResult<usize> {
		match unsafe { kernel::ffi::LocalSize(self.ptr()) } {
			0 => Err(GetLastError()),
			sz => Ok(sz),
		}
	}
}
