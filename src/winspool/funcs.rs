#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::winspool::ffi;

/// [`EnumPrinters`](https://learn.microsoft.com/en-us/windows/win32/printdocs/enumprinters)
/// function for Level 2.
///
/// **Note:** This function doesn't seem to work with `raw-dylib` Cargo feature.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let printers = w::EnumPrinters2(
///     co::PRINTER_ENUM::LOCAL | co::PRINTER_ENUM::CONNECTIONS)?;
///
/// for p in printers {
///     println!("{}", p.pPrinterName().unwrap());
/// }
/// # w::SysResult::Ok(())
/// ```
///
/// # Related functions
///
/// * [`EnumPrinters4`](crate::EnumPrinters4)
/// * [`GetDefaultPrinter`](crate::GetDefaultPrinter)
#[must_use]
pub fn EnumPrinters2<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm>(
	flags: co::PRINTER_ENUM,
) -> SysResult<Vec<PRINTER_INFO_2<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm>>>
{
	enum_printers::<PRINTER_INFO_2>(flags, 2)
}

/// [`EnumPrinters`](https://learn.microsoft.com/en-us/windows/win32/printdocs/enumprinters)
/// function for Level 4.
///
/// **Note:** This function doesn't seem to work with `raw-dylib` Cargo feature.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let printers = w::EnumPrinters4(
///     co::PRINTER_ENUM::LOCAL | co::PRINTER_ENUM::CONNECTIONS)?;
///
/// for p in printers {
///     println!("{}", p.pPrinterName().unwrap());
/// }
/// # w::SysResult::Ok(())
/// ```
///
/// # Related functions
///
/// * [`EnumPrinters2`](crate::EnumPrinters2)
/// * [`GetDefaultPrinter`](crate::GetDefaultPrinter)
#[must_use]
pub fn EnumPrinters4<'a, 'b>(
	flags: co::PRINTER_ENUM,
) -> SysResult<Vec<PRINTER_INFO_4<'a, 'b>>>
{
	enum_printers::<PRINTER_INFO_4>(flags, 4)
}

#[must_use]
fn enum_printers<T: Default + Clone>(
	flags: co::PRINTER_ENUM, lvl: u32,
) -> SysResult<Vec<T>>
{
	let (mut sz_buf, mut count) = (u32::default(), u32::default());
	unsafe {
		ffi::EnumPrintersW(
			flags.raw(),
			std::ptr::null_mut(),
			lvl,
			std::ptr::null_mut(),
			0,
			&mut sz_buf,
			&mut count,
		);
	}

	let sz_unit = std::mem::size_of::<T>() as u32;
	let num_elems = ((sz_buf + sz_unit) - ((sz_buf + sz_unit) % sz_unit)) / sz_unit;
	let mut buf = vec![T::default(); num_elems as _];

	bool_to_sysresult(
		unsafe {
			ffi::EnumPrintersW(
				flags.raw(),
				std::ptr::null_mut(),
				lvl,
				buf.as_mut_ptr() as _,
				(buf.len() * std::mem::size_of::<T>()) as _,
				&mut sz_buf,
				&mut count,
			)
		},
	).map(|_| {
		buf.truncate(count as _);
		buf
	})
}

/// [`GetDefaultPrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/getdefaultprinter)
/// function.
///
/// **Note:** This function doesn't seem to work with `raw-dylib` Cargo feature.
///
/// * [`EnumPrinters2`](crate::EnumPrinters2)
/// * [`EnumPrinters4`](crate::EnumPrinters4)
#[must_use]
pub fn GetDefaultPrinter() -> SysResult<String> {
	let mut sz = u32::default();
	unsafe { ffi::GetDefaultPrinterW(std::ptr::null_mut(), &mut sz); }
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let mut name_buf = WString::new_alloc_buf(sz as _);
	bool_to_sysresult(
		unsafe { ffi::GetDefaultPrinterW(name_buf.as_mut_ptr(), &mut sz) },
	).map(|_| name_buf.to_string())
}
