<p align="center">
Windows API constants data dump
<br>
</p>

<p align="center">  
</p>


## Introduction
A work-in-progress and only partially successful attempt to find a relatively easy way to parse [windows-sys](https://docs.rs/windows-sys/) Rust library and extract a database of all the constants for further use in a [DLL](https://github.com/eugenesvk/winAPIconst/) that would allow looking up constant values by full/short name in, e.g., AutoHotkey scripts' calls to Windows APIs.

The parsing relied on a [Trustfall adapter library](https://docs.rs/trustfall-rustdoc-adapter) that parsed cargo docs, but those docs don't contain all the information, so the values for ~14k out of ~126k constants couldn't have been retrieved

Resulting tab-separated data is at [windows_sys non-blank 112k constants](/raw/data/data/winConst_Valid_112k.txt.rar) and with added extra data from [a Ziggle tool](https://www.autohotkey.com/boards/viewtopic.php?f=83&t=99581) at [windows_sys+ziggle 175k constants](/raw/data/data/winConst_Valid_ziggle_175k.rar)

## Install

## Use

## Known issues

Since Rust doesn't have reflections and the rustdocs don't have full information, ~14k out of ~126k constants of the custom type or string pointer variety like ↓
```rs
pub const Dot11AdHocManager        	:GUID 	= GUID::from_u128(0xdd06a84f_83bd_4d01_8ab9_2389fea0869e);
pub const IHV_INIT_VS_FUNCTION_NAME	:PCSTR	= s!("Dot11ExtIhvInitVirtualStation");
```

have no value

## Credits
