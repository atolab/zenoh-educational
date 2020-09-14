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
    let session = open(Config::default(), None).await.unwrap();

    loop {
        let selector = "/region01/**";
        println!(">> Query '{}'", selector);
        let predicate = "";
        let target = QueryTarget::default();
        let consol = QueryConsolidation::default();
        let mut replies = session
            .query(&selector.into(), predicate, target, consol)
            .await
            .unwrap();
        while let Some(reply) = replies.next().await {
            let bytes = reply.data.payload.to_vec();
            let value = String::from_utf8_lossy(&bytes);
            println!("<< Received ('{}': '{}')", reply.data.res_name, value);
        }
        async_std::task::sleep(std::time::Duration::from_secs(5)).await;
    }
}
