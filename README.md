# cluProcCmdline

[![Build Status](https://travis-ci.org/clucompany/cluProcCmdline.svg?branch=master)](https://travis-ci.org/clucompany/cluProcCmdline)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/cluproccmdline)](https://crates.io/crates/cluproccmdline)
[![Documentation](https://docs.rs/cluuname/badge.svg)](https://docs.rs/cluproccmdline)

Fast secure parsing "/proc/cmdline".


# Use DefaultIter:

1

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

2

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

# Use OneIter:

1

	use cluproccmdline::Cmdline;

	let mut cmdline = cluproccmdline::array_slice(b"test=all rw");
	for value in cmdline.iter_one() {
			println!("{}", 
					String::from_utf8(value.to_vec()).unwrap()
			);
			// OUTPUT: 
			// rw
	}

2

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

# Use TwoIter:

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

# Hash proc cmdline

	use cluproccmdline::Cmdline;

	let mut cmdline = cluproccmdline::array_slice(b"test=all rw");
	assert_eq!(cmdline.cmdline_hash(), 1877887864476248772);


# Benchmark

Machine: Intel Core 2 Duo (2000 MHz), 2 Gb DDR2

	test tests::bench_oneslice_new ... bench:         101 ns/iter (+/- 14)
	test tests::bench_slice_new    ... bench:         105 ns/iter (+/- 23)

# License

Copyright 2018 #UlinProject Денис Котляров

Licensed under the Apache License, Version 2.0
