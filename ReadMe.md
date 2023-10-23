<p align="center">
Windows API constants data dump
<br>
</p>

<p align="center">  
</p>


## Introduction
A work-in-progress attempt to leverage the official [windows-bindgen](https://crates.io/crates/windows-bindgen) Windows metadata compiler Rust library to create a database of all the constants for further use in a [DLL](https://github.com/eugenesvk/winAPIconst/) that would allow looking up constant values by full/short name in, e.g., AutoHotkey scripts' calls to Windows APIs.
Resulting tab-separated data is at [windows_sys 185k constants](../../raw/data/data/1%20WinMD/winConst_bindgen_All_185k.rar)

A simpler alternative approach of using the [Trustfall adapter library](https://docs.rs/trustfall-rustdoc-adapter) that parsed cargo docs (behind the `rustdoc` feature) couldn't retrieve the values for ~14k out of ~126k constants since the docs don't contain all the information (e.g., `PCWSTR` string values in `w!` macros). Resulting tab-separated data is at [windows_sys non-blank 112k constants](../../raw/data/data/2%20rustdoc/winConst_Valid_112k.txt.rar) and with added extra data from [a Ziggle tool](https://www.autohotkey.com/boards/viewtopic.php?f=83&t=99581) at [windows_sys+ziggle 180k constants](../../raw/data/data/2%20rustdoc/winConst_Valid_ziggle_180k.txt.rar)

## Install

## Use

- the output data file has 4 tab-separated columns: `name` `type` `type_primitive` `value`, e.g.,
  - `D3DDDIFMT_X8R8G8B8	D3DDDIFORMAT	u32	22`for
  - WinMD: `D3DDDIFMT_X8R8G8B8 = 22u` in `enum D3DDDIFORMAT: uint`
- enums use underlying type (primitive converted, rest are `_`)
- Constant structs with only GUID `FaxAccountFolders` (no fields) are prefixed with `CLSID_`
- COM interfaces like `ILearningModel` are prefixed with `IID_` for their GUIDs
- Constant structs like `DEVPKEY_Device_ActivityId`(type `DEVPROPKEY`) with fields `fmtid`(`Guid`) and `pid`(`uint`) are stored as a full string representation as well as individual fields (`_`-appended to the constant name), e.g.:
  ```
  Name                                	TypeNative              	TypePrimitive	Value
  DEVPKEY_Device_ActivityId           	DEVPROPKEY              	_            	{fmtid:{c50a3f10-aa5c-4247-b830-d6a6f8eaa310},pid:4,}
  DEVPKEY_Device_ActivityId_fmtid     	GUID                    	str          	{c50a3f10-aa5c-4247-b830-d6a6f8eaa310}
  DEVPKEY_Device_ActivityId_pid       	u32                     	u32          	4
  SECURITY_APP_PACKAGE_AUTHORITY      	SID_IDENTIFIER_AUTHORITY	_            	{Value:[0,0,0,0,0,15,],}
  SECURITY_APP_PACKAGE_AUTHORITY_Value	_                       	array        	[0,0,0,0,0,15,]
  ```
- Generic interfaces like `IObservableVector<T>` are not included
- Duplicate names (~150 mostly IIDs [src](../../raw/data/data/1%20WinMD/winConst_dupe%20150.log.rar)) are disambiguated adding `_`-separated namespaces to the name in a separate [file](../../raw/data/data/1%20WinMD/winConst_bindgen_All_185k_dedupe.rar)
  ```
  Name                                       	TypeNative	TypePrimitive	Value
  IID_IPowerManagerStatics@Phone_System_Power	GUID      	str          	{25de8fd0-1c5b-11e1-bddb-0800200c9a66}	Phone_System_Power
  IID_IPowerManagerStatics@System_Power      	GUID      	str          	{1394825d-62ce-4364-98d5-aa28c7fbd15b}	System_Power
  ```

## Known issues

## Credits
