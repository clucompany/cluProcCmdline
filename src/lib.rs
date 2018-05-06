//Copyright 2018 #UlinProject Денис Котляров

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.


/*!
Fast secure parsing /proc/cmdline.

# Use:

DefaultIter

```rust
use cluproccmdline::Cmdline;

let mut cmdline = cluproccmdline::this_machine().unwrap();
for (name, value) in cmdline.iter() {
	if let Some(name) = name {
		println!(
			"Cmdline_str: {} {}", 
			String::from_utf8(name.to_vec()).unwrap(), 
			String::from_utf8(value.to_vec()).unwrap()
		);
	}
	// OUTPUT: 
	// IF /proc/cmdline = "BOOT_IMAGE=/boot/vmlinuz-linux-zen nmi_watchdog=0"
	// TO -> "Cmdline_str: BOOT_IMAGE /boot/vmlinuz-linux-zen"
	// TO -> "Cmdline_str: nmi_watchdog 0"
}	
```

```rust
use cluproccmdline::Cmdline;

let mut cmdline = cluproccmdline::array_slice(b"test=all rw");
let mut iter = cmdline.iter();

while let Some((Some(name), value)) = iter.next() {
	println!("{} {}", 
		String::from_utf8(name.to_vec()).unwrap(), 
		String::from_utf8(value.to_vec()).unwrap()
	);
	// OUTPUT:
	// rw
}
```

OneIter

```rust
use cluproccmdline::Cmdline;

let mut cmdline = cluproccmdline::array_slice(b"test=all rw");
for value in cmdline.iter_one() {
	println!("{}", 
		String::from_utf8(value.to_vec()).unwrap()
	);
	// OUTPUT: 
	// rw
}
```
```rust
use cluproccmdline::Cmdline;

let mut cmdline = cluproccmdline::array_slice(b"test=all rw");
let mut iter = cmdline.iter_one();

while let Some(value) = iter.next() {
	println!("{}", 
		String::from_utf8(value.to_vec()).unwrap()
	);
	// OUTPUT:
	// rw
}
```

TwoIter

```rust
use cluproccmdline::Cmdline;

let mut cmdline = cluproccmdline::array_slice(b"test=all rw");
for (value, name) in cmdline.iter_two() {
	println!("{} {}", 
		String::from_utf8(name.to_vec()).unwrap(),
		String::from_utf8(value.to_vec()).unwrap()
	);
	// OUTPUT: 
	// test all
}
```

# Hash proc cmdline
```rust
use cluproccmdline::Cmdline;

let mut cmdline = cluproccmdline::array_slice(b"test=all rw");
assert_eq!(cmdline.cmdline_hash(), 1877887864476248772);
```

*/


#![feature(test)]

extern crate test;

//#Ulin Project 1718
//




pub mod structs;
pub mod iter;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::fmt::Debug;

use structs::slice::CmdlineSlice;
use structs::buf::CmdlineBuf;
use iter::two::CmdlineTwoIter;
use iter::one::CmdlineOneIter;
use iter::CmdlineIter;

use std::hash::Hash;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;


///Main functions of Cmdline.
pub trait Cmdline: /* Deref<Target = [u8]> + */ AsRef<[u8]> + Hash + Debug + Clone + Eq + PartialEq + PartialOrd {
	///```rust
	///use cluproccmdline::Cmdline;
	///
	///let mut cmdline = cluproccmdline::this_machine().unwrap();
	///for (name, value) in cmdline.iter() {
	///	if let Some(name) = name {
	///		println!(
	///			"Cmdline_str: {} {}", 
	///			String::from_utf8(name.to_vec()).unwrap(), 
	///			String::from_utf8(value.to_vec()).unwrap()
	///		);
	///	}
	///	// OUTPUT: 
	///	// IF /proc/cmdline = "BOOT_IMAGE=/boot/vmlinuz-linux-zen nmi_watchdog=0"
	///	// TO -> "Cmdline_str: BOOT_IMAGE /boot/vmlinuz-linux-zen"
	///	// TO -> "Cmdline_str: nmi_watchdog 0"
	///}	
	///```
	///```rust
	///use cluproccmdline::Cmdline;
	///
	///let mut cmdline = cluproccmdline::array_slice(b"test=all rw");
	///let mut iter = cmdline.iter();
	///
	///while let Some((Some(name), value)) = iter.next() {
	///	println!("{} {}", 
	///		String::from_utf8(name.to_vec()).unwrap(), 
	///		String::from_utf8(value.to_vec()).unwrap()
	///	);
	///	// OUTPUT:
	///	// rw
	///}
	///```
	
	fn iter<'i>(&'i mut self) -> CmdlineIter<'i>;
	
	
	///```rust
	///use cluproccmdline::Cmdline;
	///
	///let mut cmdline = cluproccmdline::array_slice(b"test=all rw");
	///for value in cmdline.iter_one() {
	///	println!("{}", 
	///		String::from_utf8(value.to_vec()).unwrap()
	///	);
	///	// OUTPUT: 
	///	// rw
	///}
	///```
	///```rust
	///use cluproccmdline::Cmdline;
	///
	///let mut cmdline = cluproccmdline::array_slice(b"test=all rw");
	///let mut iter = cmdline.iter_one();
	///
	///while let Some(value) = iter.next() {
	///	println!("{}", 
	///		String::from_utf8(value.to_vec()).unwrap()
	///	);
	///	// OUTPUT:
	///	// rw
	///}
	///```
	fn iter_one<'i>(&'i mut self) -> CmdlineOneIter<'i>;
	
	
	
	///```rust
	///use cluproccmdline::Cmdline;
	///
	///let mut cmdline = cluproccmdline::array_slice(b"test=all rw");
	///for (value, name) in cmdline.iter_two() {
	///	println!("{} {}", 
	///		String::from_utf8(name.to_vec()).unwrap(),
	///		String::from_utf8(value.to_vec()).unwrap()
	///	);
	///	// OUTPUT: 
	///	// test all
	///}
	///```
	fn iter_two<'i>(&'i mut self) -> CmdlineTwoIter<'i>;
	
	///```rust
	///use cluproccmdline::Cmdline;
	///
	///let mut cmdline = cluproccmdline::array_slice(b"test=all rw");
	///assert_eq!(cmdline.cmdline_hash(), 1877887864476248772);
	///```
	fn cmdline_hash(&self) -> u64 {
		let mut hasher = DefaultHasher::new();
		self.hash(&mut hasher);	
		hasher.finish()
	}
}

///Opens the cmdline of the current machine. Equivalent `open_file("/proc/cmdline")`.
///```rust
///use cluproccmdline::Cmdline;
///
///let mut cmdline = cluproccmdline::this_machine().unwrap();
///for (name, value) in cmdline.iter() {
///	if let Some(name) = name {
///		println!(
///			"Cmdline_str: {} {}", 
///			String::from_utf8(name.to_vec()).unwrap(), 
///			String::from_utf8(value.to_vec()).unwrap()
///		);
///	}
///
///	// OUTPUT: 
///	// IF /proc/cmdline = "BOOT_IMAGE=/boot/vmlinuz-linux-zen nmi_watchdog=0"
///	// TO -> "Cmdline_str: BOOT_IMAGE /boot/vmlinuz-linux-zen"
///	// TO -> "Cmdline_str: nmi_watchdog 0"
///}	
///```
#[inline]
pub fn this_machine() -> Result<CmdlineBuf, CmdlineErr> {
	open_file("/proc/cmdline")
}

///Opens the cmdline from the file.
pub fn open_file<P: AsRef<Path> >(path: P) -> Result<CmdlineBuf, CmdlineErr> {
	match File::open(path) {
		Ok(mut file) => {
			let mut vec = Vec::with_capacity(226);
			match file.read_to_end(&mut vec) {
				Ok(size) => {
					if size == 0 {
						return Err(CmdlineErr::EmptyFile);
					}
					
					return Ok(
						array_buf(vec)
					);
				},
				Err(e) => return Err(CmdlineErr::ReadFile(e)),
			}
		},
		Err(e) => return Err(CmdlineErr::OpenFile(e)),
	}
}


///Creates cmdline from `Vec`. Equivalent `CmdlineBuf::array(array)`.
#[inline]
pub fn array_buf(array: Vec<u8>) -> CmdlineBuf {
	CmdlineBuf::array(array)
}

///Creates cmdline from `&[u8]`. Equivalent `CmdlineSlice::array(array)`.
#[inline]
pub fn array_slice<'a>(array: &'a [u8]) -> CmdlineSlice<'a> {
	CmdlineSlice::array(array)
}	


///Description of errors
#[derive(Debug)]
pub enum CmdlineErr {
	///Open file err
	OpenFile(::std::io::Error),
	
	///Read file err
	ReadFile(::std::io::Error),
	
	///File empty, size = 0
	EmptyFile,
}



#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;
	
	#[test]
	fn test_basic_functional() {
		let mut cmdline = array_slice(b"BOOT_IMAGE=/boot/vmlinuz-linux-zen rw quiet");
		
		{
			//default iter
			let mut iter = cmdline.iter();
			
			assert_eq!(iter.next(), Some((Some(&b"BOOT_IMAGE"[..]), &b"/boot/vmlinuz-linux-zen"[..])));
			assert_eq!(iter.next(), Some((None, &b"rw"[..])));
			assert_eq!(iter.next(), Some((None, &b"quiet"[..])));
			assert_eq!(iter.next(), None);
		}
		
		{
			//one iter
			let mut iter = cmdline.iter_one();
			assert_eq!(iter.next(), Some(&b"rw"[..]));
			assert_eq!(iter.next(), Some(&b"quiet"[..]));
			assert_eq!(iter.next(), None);
		}
		
		{
			//two iter
			let mut iter = cmdline.iter_two();
			assert_eq!(iter.next(), Some((&b"BOOT_IMAGE"[..], &b"/boot/vmlinuz-linux-zen"[..])) );
			assert_eq!(iter.next(), None);
		}
	}
	
	
	#[bench]
	fn bench_slice_new(b: &mut Bencher) {
		let mut cmdline = array_slice(b"BOOT_IMAGE=/boot/vmlinuz-linux-zen rw quiet");
		
		b.iter(|| {			
			for (_a, _v) in cmdline.iter() {
				
			}
		});
	}
	
	#[bench]
	fn bench_oneslice_new(b: &mut Bencher) {
		let mut cmdline = array_slice(b"BOOT_IMAGE=/boot/vmlinuz-linux-zen rw quiet");
		
		b.iter(|| {
			for _n in cmdline.iter_one() {
				
			}
		});
	}
}

