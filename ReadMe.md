<p align="center">
Windows API constants data dump
<br>
</p>

<p align="center">  
</p>


## Introduction
A work-in-progress attempt to leverage the official [windows-bindgen](https://crates.io/crates/windows-bindgen) Windows metadata compiler Rust library to create a database of all the constants for further use in a [DLL](https://github.com/eugenesvk/winAPIconst/) that would allow looking up constant values by full/short name in, e.g., AutoHotkey scripts' calls to Windows APIs.

Resulting tab-separated data is at [windows_sys non-blank 112k constants](../../raw/data/data/winConst_Valid_112k.txt.rar) and with added extra data from [a Ziggle tool](https://www.autohotkey.com/boards/viewtopic.php?f=83&t=99581) at [windows_sys+ziggle 175k constants](../../raw/data/data/winConst_Valid_ziggle_175k.rar)

## Install

## Use

- the output data file has 4 tab-separated columns: `name` `type` `type_primitive` `value`, e.g.,
  - `D3DDDIFMT_X8R8G8B8	D3DDDIFORMAT	u32	22`for
  - WinMD: `D3DDDIFMT_X8R8G8B8 = 22u` in `enum D3DDDIFORMAT: uint`
- enums use underlying type (primitive converted, rest are `_`)
- Constant structs with only GUID `FaxAccountFolders` (no fields) are prefixed with `CLSID_`
- Constant structs like `DEVPKEY_Device_ActivityId`(type `DEVPROPKEY`) with fields `fmtid`(`Guid`) and `pid`(`uint`) are stored as a full string representation as well as individual fields (`_`-appended to the constant name), e.g.:
  ```
  Name                           	TypeNative	TypePrimitive	Value
  DEVPKEY_Device_ActivityId      	DEVPROPKEY	_            	{fmtid:{c50a3f10-aa5c-4247-b830-d6a6f8eaa310},pid:4,}
  DEVPKEY_Device_ActivityId_fmtid	GUID      	str          	{c50a3f10-aa5c-4247-b830-d6a6f8eaa310}
  DEVPKEY_Device_ActivityId_pid  	u32       	u32          	4
  ```

## Known issues

## Credits
