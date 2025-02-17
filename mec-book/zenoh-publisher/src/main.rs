//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//
use zenoh::net::*;

#[async_std::main]
async fn main() {
    // Open a Zenoh session using the default configuration
    let session = open(Config::default(), None).await.unwrap();
    // Periodically publish the house temperature
    loop {
        let (res, val) = ("/region01/house01/temp", "25");
        let bytes = val.as_bytes().into();
        session.write(&res.into(), bytes).await.unwrap();
        println!(">> Published ('{}': '{}')", res, val);
        async_std::task::sleep(std::time::Duration::from_secs(5)).await;
    }
}
