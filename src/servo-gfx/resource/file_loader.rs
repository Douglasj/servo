/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use core::task::spawn;
use resource::resource_task::{Payload, Done, LoaderTask};
use core::io::{file_reader, ReaderUtil};

static READ_SIZE: uint = 1024;

pub fn factory() -> LoaderTask {
	let f: LoaderTask = |url, progress_chan| {
		assert!(url.scheme == ~"file");
		do spawn {
			// FIXME: Resolve bug prevents us from moving the path out of the URL.
			match file_reader(&Path(url.path)) {
				Ok(reader) => {
					while !reader.eof() {
						let data = reader.read_bytes(READ_SIZE);
						progress_chan.send(Payload(data));
					}
					progress_chan.send(Done(Ok(())));
				}
				Err(*) => {
					progress_chan.send(Done(Err(())));
				}
			};
		}
	};
	f
}
