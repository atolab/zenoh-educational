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
use async_std::prelude::*;
use zenoh::net::*;

#[async_std::main]
async fn main() {
    // Open a Zenoh session using the default configuration
    let session = open(Config::default(), None).await.unwrap();
    // Declare a subscriber for a given key selector
    let selector = "/region01/**";
    let mut subscriber = session
        .declare_subscriber(&selector.into(), &SubInfo::default())
        .await
        .unwrap();
    // Process the incoming publications
    while let Some(sample) = subscriber.stream().next().await {
        let bytes = sample.payload.to_vec();
        let value = String::from_utf8_lossy(&bytes);
        println!(">> Received ('{}': '{}')", sample.res_name, value);
    }
}
